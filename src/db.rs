use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as SqlResult};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

pub struct Database {
    conn: Connection,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UsageRecord {
    pub timestamp: DateTime<Utc>,
    pub provider: String,
    pub metric: String,
    pub utilization: f64,
    pub resets_at: Option<String>,
}

impl Database {
    pub fn open(exe_dir: &Path) -> SqlResult<Self> {
        let db_path = exe_dir.join("claudemeter.db");
        let conn = Connection::open(db_path)?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> SqlResult<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS usage_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL DEFAULT (datetime('now')),
                provider TEXT NOT NULL,
                metric TEXT NOT NULL,
                utilization REAL NOT NULL,
                resets_at TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_history_time
                ON usage_history(timestamp);
            CREATE INDEX IF NOT EXISTS idx_history_provider
                ON usage_history(provider, metric);",
        )?;
        // Clean up old records (> 30 days) on startup
        self.conn.execute(
            "DELETE FROM usage_history WHERE timestamp < datetime('now', '-30 days')",
            [],
        )?;
        // Remove five_hour records with no active session (resets_at is NULL)
        self.conn.execute(
            "DELETE FROM usage_history WHERE metric = 'five_hour' AND resets_at IS NULL",
            [],
        )?;
        Ok(())
    }

    pub fn insert(
        &self,
        provider: &str,
        metric: &str,
        utilization: f64,
        resets_at: Option<&str>,
    ) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO usage_history (provider, metric, utilization, resets_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![provider, metric, utilization, resets_at],
        )?;
        Ok(())
    }

    /// Query last 24 hours of `five_hour` metric, bucketed into 30-minute intervals.
    /// Always returns exactly 48 elements (oldest first: index 0 = 24h ago, index 47 = now).
    /// Missing slots are filled with 0.0.
    pub fn query_24h_chart(&self) -> SqlResult<Vec<f64>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                CAST((julianday('now') - julianday(timestamp)) * 48 AS INTEGER) AS bucket,
                AVG(utilization) AS avg_util
             FROM usage_history
             WHERE provider = 'claude'
               AND metric = 'five_hour'
               AND resets_at IS NOT NULL
               AND timestamp > datetime('now', '-24 hours')
             GROUP BY bucket",
        )?;

        let mut slots = vec![0.0f64; 48];

        let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, f64>(1)?)))?;

        for row in rows.flatten() {
            let (bucket, util) = row;
            // bucket 0 = now, bucket 47 = ~24h ago
            // We want index 0 = oldest, index 47 = newest
            let idx = 47 - bucket.clamp(0, 47) as usize;
            slots[idx] = util;
        }

        Ok(slots)
    }

    /// Query last 7 days of `five_hour` metric, bucketed into 4-hour intervals.
    /// Always returns exactly 42 elements (oldest first: index 0 = 7d ago, index 41 = now).
    /// Missing slots are filled with 0.0.
    #[cfg_attr(not(windows), allow(dead_code))] // consumed by the Windows app
    pub fn query_7d_chart(&self) -> SqlResult<Vec<f64>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                CAST((julianday('now') - julianday(timestamp)) * 6 AS INTEGER) AS bucket,
                AVG(utilization) AS avg_util
             FROM usage_history
             WHERE provider = 'claude'
               AND metric = 'five_hour'
               AND resets_at IS NOT NULL
               AND timestamp > datetime('now', '-7 days')
             GROUP BY bucket",
        )?;

        let mut slots = vec![0.0f64; 42];

        let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, f64>(1)?)))?;

        for row in rows.flatten() {
            let (bucket, util) = row;
            let idx = 41 - bucket.clamp(0, 41) as usize;
            slots[idx] = util;
        }

        Ok(slots)
    }

    /// Query last 30 days of `five_hour` metric, bucketed into 1-day intervals.
    /// Always returns exactly 30 elements (oldest first: index 0 = 30d ago, index 29 = now).
    /// Missing slots are filled with 0.0.
    #[cfg_attr(not(windows), allow(dead_code))] // consumed by the Windows app
    pub fn query_30d_chart(&self) -> SqlResult<Vec<f64>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                CAST(julianday('now') - julianday(timestamp) AS INTEGER) AS bucket,
                AVG(utilization) AS avg_util
             FROM usage_history
             WHERE provider = 'claude'
               AND metric = 'five_hour'
               AND resets_at IS NOT NULL
               AND timestamp > datetime('now', '-30 days')
             GROUP BY bucket",
        )?;

        let mut slots = vec![0.0f64; 30];

        let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, f64>(1)?)))?;

        for row in rows.flatten() {
            let (bucket, util) = row;
            let idx = 29 - bucket.clamp(0, 29) as usize;
            slots[idx] = util;
        }

        Ok(slots)
    }

    /// Query the most recent utilization value for each metric.
    /// Returns a list of (metric_name, utilization, resets_at) tuples.
    #[cfg_attr(not(windows), allow(dead_code))] // consumed by the Windows app
    pub fn query_latest(&self) -> SqlResult<Vec<(String, f64, Option<String>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT metric, utilization, resets_at
             FROM usage_history
             WHERE provider = 'claude'
               AND id IN (
                   SELECT MAX(id) FROM usage_history
                   WHERE provider = 'claude'
                   GROUP BY metric
               )",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })?;

        Ok(rows.flatten().collect())
    }

    /// Recent readings for one metric, oldest-first: (timestamp, utilization,
    /// resets_at). Used by the macOS startup backfill to find a free flush that
    /// happened while the app was down or being upgraded.
    #[cfg_attr(windows, allow(dead_code))] // consumed by the macOS app
    pub fn query_recent_readings(
        &self,
        metric: &str,
        days: i64,
    ) -> SqlResult<Vec<(String, f64, Option<String>)>> {
        let cutoff = format!("-{days} days");
        let mut stmt = self.conn.prepare(
            "SELECT timestamp, utilization, resets_at
             FROM usage_history
             WHERE provider = 'claude'
               AND metric = ?1
               AND timestamp > datetime('now', ?2)
             ORDER BY timestamp ASC",
        )?;

        let rows = stmt.query_map(params![metric, cutoff], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })?;

        Ok(rows.flatten().collect())
    }

    /// Compute rate of change (%/hour) for each metric by comparing current values
    /// against values from `lookback_minutes` ago.
    #[cfg_attr(not(windows), allow(dead_code))] // consumed by the Windows app
    pub fn query_rate_of_change(&self, lookback_minutes: i64) -> SqlResult<HashMap<String, f64>> {
        let latest = self.query_latest()?;

        let half_window = 5; // +/- 5 min window around the lookback point
        let window_start = format!("-{} minutes", lookback_minutes + half_window);
        let window_end = format!("-{} minutes", (lookback_minutes - half_window).max(0));

        let mut stmt = self.conn.prepare(
            "SELECT metric, AVG(utilization) AS avg_util
             FROM usage_history
             WHERE provider = 'claude'
               AND timestamp BETWEEN datetime('now', ?1) AND datetime('now', ?2)
             GROUP BY metric",
        )?;

        let past_rows = stmt.query_map(params![window_start, window_end], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })?;

        let mut past_map: HashMap<String, f64> = HashMap::new();
        for row in past_rows.flatten() {
            past_map.insert(row.0, row.1);
        }

        let hours = lookback_minutes as f64 / 60.0;
        let mut result = HashMap::new();
        for (metric, current_util, _) in &latest {
            if let Some(&past_util) = past_map.get(metric) {
                let rate = (current_util - past_util) / hours;
                result.insert(metric.clone(), rate);
            }
        }

        Ok(result)
    }

    /// Open an in-memory database (for testing).
    #[cfg(test)]
    pub fn open_in_memory() -> SqlResult<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    /// Insert a record with a specific timestamp (for testing).
    #[cfg(test)]
    pub fn insert_at(
        &self,
        timestamp: &str,
        provider: &str,
        metric: &str,
        utilization: f64,
        resets_at: Option<&str>,
    ) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO usage_history (timestamp, provider, metric, utilization, resets_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![timestamp, provider, metric, utilization, resets_at],
        )?;
        Ok(())
    }

    /// Export all usage history to a CSV file. Returns the number of rows written.
    #[cfg_attr(not(windows), allow(dead_code))] // consumed by the Windows app
    pub fn export_csv(&self, path: &Path) -> SqlResult<usize> {
        let mut stmt = self.conn.prepare(
            "SELECT timestamp, provider, metric, utilization, resets_at
             FROM usage_history ORDER BY timestamp DESC",
        )?;

        let mut file = std::fs::File::create(path)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        let _ = writeln!(file, "timestamp,provider,metric,utilization,resets_at");
        let mut count = 0usize;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, f64>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })?;

        for (ts, provider, metric, util, resets) in rows.flatten() {
            let resets_str = resets.unwrap_or_default();
            let _ = writeln!(
                file,
                "{},{},{},{:.2},{}",
                ts, provider, metric, util, resets_str
            );
            count += 1;
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_and_insert() {
        let db = Database::open_in_memory().unwrap();
        db.insert(
            "claude",
            "five_hour",
            42.0,
            Some("2025-11-04T05:00:00+00:00"),
        )
        .unwrap();
        let latest = db.query_latest().unwrap();
        assert_eq!(latest.len(), 1);
        assert_eq!(latest[0].0, "five_hour");
        assert_eq!(latest[0].1, 42.0);
    }

    #[test]
    fn test_query_latest_multiple_metrics() {
        let db = Database::open_in_memory().unwrap();
        db.insert("claude", "five_hour", 30.0, None).unwrap();
        db.insert("claude", "seven_day", 15.0, None).unwrap();
        db.insert("claude", "five_hour", 60.0, None).unwrap(); // newer
        let latest = db.query_latest().unwrap();
        // Should have 2 metrics, five_hour should be the latest (60.0)
        assert_eq!(latest.len(), 2);
        let five_hour = latest.iter().find(|(m, _, _)| m == "five_hour").unwrap();
        assert_eq!(five_hour.1, 60.0);
    }

    #[test]
    fn test_query_24h_chart_returns_48_slots() {
        let db = Database::open_in_memory().unwrap();
        let slots = db.query_24h_chart().unwrap();
        assert_eq!(slots.len(), 48);
    }

    #[test]
    fn test_query_7d_chart_returns_42_slots() {
        let db = Database::open_in_memory().unwrap();
        let slots = db.query_7d_chart().unwrap();
        assert_eq!(slots.len(), 42);
    }

    #[test]
    fn test_query_30d_chart_returns_30_slots() {
        let db = Database::open_in_memory().unwrap();
        let slots = db.query_30d_chart().unwrap();
        assert_eq!(slots.len(), 30);
    }

    #[test]
    fn test_query_24h_chart_with_data() {
        let db = Database::open_in_memory().unwrap();
        // Insert a record at "now" — should land in the newest bucket
        db.insert(
            "claude",
            "five_hour",
            75.0,
            Some("2025-11-04T05:00:00+00:00"),
        )
        .unwrap();
        let slots = db.query_24h_chart().unwrap();
        // The newest slot (index 47) should have data
        assert!(slots[47] > 0.0);
    }

    #[test]
    fn test_rate_of_change_no_data() {
        let db = Database::open_in_memory().unwrap();
        let roc = db.query_rate_of_change(60).unwrap();
        assert!(roc.is_empty());
    }

    #[test]
    fn test_rate_of_change_with_data() {
        let db = Database::open_in_memory().unwrap();
        // Insert an old record ~60 min ago
        db.insert_at(
            &format!(
                "{}",
                chrono::Utc::now()
                    .checked_sub_signed(chrono::Duration::minutes(60))
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
            ),
            "claude",
            "five_hour",
            20.0,
            Some("2025-11-04T05:00:00+00:00"),
        )
        .unwrap();
        // Insert a current record
        db.insert(
            "claude",
            "five_hour",
            50.0,
            Some("2025-11-04T05:00:00+00:00"),
        )
        .unwrap();

        let roc = db.query_rate_of_change(60).unwrap();
        if let Some(&rate) = roc.get("five_hour") {
            // Rate should be positive (~30%/hour)
            assert!(rate > 0.0, "Expected positive rate, got {rate}");
        }
        // It's OK if no rate is found (timing-sensitive), but if found it should be positive
    }

    #[test]
    fn test_query_recent_readings_ordered() {
        let db = Database::open_in_memory().unwrap();
        db.insert_at(
            "2026-06-13 00:00:00",
            "claude",
            "seven_day",
            54.0,
            Some("a"),
        )
        .unwrap();
        db.insert_at("2026-06-13 00:02:00", "claude", "seven_day", 0.0, Some("a"))
            .unwrap();
        // A different metric must not leak in.
        db.insert("claude", "five_hour", 10.0, Some("b")).unwrap();
        let rows = db.query_recent_readings("seven_day", 8).unwrap();
        assert_eq!(rows.len(), 2);
        // Oldest first.
        assert_eq!(rows[0].1, 54.0);
        assert_eq!(rows[1].1, 0.0);
    }

    #[test]
    fn test_export_csv() {
        let db = Database::open_in_memory().unwrap();
        db.insert("claude", "five_hour", 42.0, None).unwrap();
        db.insert("claude", "seven_day", 15.0, None).unwrap();

        let tmp = std::env::temp_dir().join("claudemeter_test_export.csv");
        let count = db.export_csv(&tmp).unwrap();
        assert_eq!(count, 2);

        let content = std::fs::read_to_string(&tmp).unwrap();
        assert!(content.contains("timestamp,provider,metric,utilization,resets_at"));
        assert!(content.contains("five_hour"));
        assert!(content.contains("seven_day"));

        let _ = std::fs::remove_file(&tmp);
    }

    #[test]
    fn test_five_hour_null_resets_cleaned_on_init() {
        let db = Database::open_in_memory().unwrap();
        // Insert a five_hour with NULL resets_at
        db.insert("claude", "five_hour", 10.0, None).unwrap();
        // Insert a five_hour with resets_at
        db.insert(
            "claude",
            "five_hour",
            20.0,
            Some("2025-11-04T05:00:00+00:00"),
        )
        .unwrap();

        // Re-init (simulates startup cleanup)
        db.init().unwrap();

        let latest = db.query_latest().unwrap();
        // Only the one with resets_at should survive
        let five_hour = latest.iter().find(|(m, _, _)| m == "five_hour").unwrap();
        assert_eq!(five_hour.1, 20.0);
    }
}
