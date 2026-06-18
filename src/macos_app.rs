use crate::config::ConfigManager;
use crate::db::Database;
use crate::providers::claude::{format_metric_name, ClaudeClient, UsageResponse};
use crate::providers::{Provider, ZaiClient};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MetricEntry {
    /// Raw API key, e.g. "five_hour" — used by the UI to pick the session metric.
    key: String,
    /// Human-readable name, e.g. "Weekly (7-day)"
    name: String,
    percent: u32,
    resets_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MacStatus {
    state: String,
    title: String,
    detail: String,
    plan: Option<String>,
    percent: Option<u32>,
    /// Per-limit breakdown (5-hour, weekly, Sonnet, Opus, ...). Empty until first fetch.
    #[serde(default)]
    metrics: Vec<MetricEntry>,
    /// Downgrade comparison, e.g. "On Max 5x: weekly ~96%, session ~124%".
    #[serde(default)]
    tier_note: Option<String>,
    last_api_update: Option<String>,
    data_age_seconds: Option<u64>,
    error: Option<String>,
    /// 24h usage history for the five_hour metric, 48 buckets oldest-first
    /// (index 0 = 24h ago, index 47 = now). Each value is a 0-100 percent.
    /// Mirrors the Windows popup's "Usage History (24h)" chart.
    #[serde(default)]
    chart: Vec<u32>,
    /// Past 5-hour session reset points, as "hours ago" (0-24). Drawn as dashed
    /// vertical lines on the chart, matching the Windows popup.
    #[serde(default)]
    chart_resets: Vec<f64>,
    /// Set while a "free flush" party is live. The Swift menu bar animates
    /// fireworks + rainbow text until weekly usage climbs back past the stop
    /// threshold. None the rest of the time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    celebrate: Option<Celebrate>,
}

/// Active free-flush celebration, surfaced to the Swift UI via status.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Celebrate {
    active: bool,
    /// RFC3339 timestamp of when the flush was detected.
    since: Option<String>,
    /// Short human label for the dropdown line.
    reason: String,
}

/// Tunables pulled from Config, threaded into the poll.
#[derive(Debug, Clone, Copy)]
struct CelebrationCfg {
    enabled: bool,
    stop_at_percent: f64,
    drop_threshold: f64,
    anchor_tolerance_seconds: i64,
}

/// Persisted across polls in celebrate_state.json so the party survives the
/// poll where usage sits flat at ~0. Tracks the previous weekly reading to
/// detect the drop, and whether a party is currently live.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct CelebrateState {
    last_util: Option<f64>,
    last_reset: Option<String>,
    active: bool,
    since: Option<String>,
}

impl MacStatus {
    fn refreshing() -> Self {
        Self {
            state: "refreshing".to_string(),
            title: "Refreshing...".to_string(),
            detail: "Requesting fresh usage data".to_string(),
            plan: None,
            percent: None,
            metrics: Vec::new(),
            tier_note: None,
            last_api_update: None,
            data_age_seconds: None,
            error: None,
            chart: Vec::new(),
            chart_resets: Vec::new(),
            celebrate: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            state: "error".to_string(),
            title: "API error".to_string(),
            detail: message.clone(),
            plan: None,
            percent: None,
            metrics: Vec::new(),
            tier_note: None,
            last_api_update: None,
            data_age_seconds: None,
            error: Some(message),
            chart: Vec::new(),
            chart_resets: Vec::new(),
            celebrate: None,
        }
    }
}

pub fn run() {
    env_logger::init();

    let exe_dir = app_data_dir();
    if let Err(e) = std::fs::create_dir_all(&exe_dir) {
        log::warn!("Failed to create app data directory {:?}: {e}", exe_dir);
    }

    let args: Vec<String> = std::env::args().collect();
    let once = args.iter().any(|arg| arg == "--once" || arg == "--refresh");
    let status_only = args.iter().any(|arg| arg == "--status");

    if status_only {
        print_status(&exe_dir);
        return;
    }

    let config_mgr = ConfigManager::new(&exe_dir);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to create tokio runtime");

    rt.block_on(async move {
        let provider = match config_mgr.config.provider.as_str() {
            "claude" => match ClaudeClient::new() {
                Ok(client) => Provider::Claude(client),
                Err(e) => {
                    let message = format!("Failed to create Claude client: {e}");
                    append_log(&exe_dir, &message);
                    write_error(&exe_dir, message);
                    return;
                }
            },
            _ => match ZaiClient::new() {
                Ok(client) => Provider::Zai(client),
                Err(e) => {
                    let message = format!("Failed to create Z.ai client: {e}");
                    append_log(&exe_dir, &message);
                    write_error(&exe_dir, message);
                    return;
                }
            },
        };

        let plan_override = config_mgr.config.plan_override.clone();
        let login_warning = config_mgr.config.token_expiry_warning;
        let celebration = CelebrationCfg {
            enabled: config_mgr.config.celebrate_free_flush,
            stop_at_percent: config_mgr.config.celebrate_stop_at_percent,
            drop_threshold: config_mgr.config.celebrate_drop_threshold,
            anchor_tolerance_seconds: config_mgr.config.celebrate_anchor_tolerance_seconds,
        };

        // Resume a party that is still live in the history but was never
        // recorded (app down or pre-feature when the flush happened).
        backfill_celebration(&exe_dir, celebration, provider.name());

        if once {
            poll_once(
                &exe_dir,
                &provider,
                login_warning,
                plan_override.as_deref(),
                celebration,
            )
            .await;
            return;
        }

        if config_mgr.config.show_startup_notification {
            notify("ClaudeMeter", "Running in the macOS menu bar.");
        }

        loop {
            poll_once(
                &exe_dir,
                &provider,
                login_warning,
                plan_override.as_deref(),
                celebration,
            )
            .await;
            let interval = config_mgr.config.polling_interval_seconds.max(60);
            tokio::time::sleep(Duration::from_secs(interval)).await;
        }
    });
}

async fn poll_once(
    exe_dir: &Path,
    provider: &Provider,
    login_warning_enabled: bool,
    plan_override: Option<&str>,
    celebration: CelebrationCfg,
) {
    mark_refreshing(exe_dir);

    let usage = match provider.fetch().await {
        Ok(usage) => usage,
        Err(e) => {
            append_log(exe_dir, &format!("Usage poll failed: {e}"));
            // A missing credential (vs a transport/API failure) gets the
            // provider-appropriate login hint as a notification.
            if login_warning_enabled && e.starts_with("[cred]") {
                notify("ClaudeMeter", provider.login_hint());
            }
            write_error(exe_dir, e);
            return;
        }
    };

    save_history(exe_dir, &usage, provider.name());
    let celebrate = update_celebration(exe_dir, &usage, celebration);
    publish_status(exe_dir, &usage, plan_override, provider.name(), celebrate);
}

/// Detect and track a "free flush". Returns Some(active party) when the menu
/// bar should celebrate. A flush is a weekly-utilization drop of at least
/// `drop_threshold` points between two polls while the reset anchor holds
/// (resets_at moved less than the tolerance) - that is Anthropic zeroing the
/// counter out of band, not a scheduled reset. Once live, the party persists
/// across polls until utilization climbs back to `stop_at_percent`, or a real
/// scheduled reset arrives (the anchor jumps forward). State is persisted so a
/// flat run of ~0% readings does not end it.
fn update_celebration(
    exe_dir: &Path,
    usage: &UsageResponse,
    cfg: CelebrationCfg,
) -> Option<Celebrate> {
    if !cfg.enabled {
        // Clear any stale state so re-enabling later starts clean.
        let _ = std::fs::remove_file(exe_dir.join("celebrate_state.json"));
        return None;
    }

    let week = usage.seven_day.as_ref();
    let cur_util = week.map(|m| m.utilization);
    let cur_reset = week.and_then(|m| m.resets_at.clone());

    let mut st = load_celebrate_state(exe_dir);

    // Did the anchor jump forward (a real scheduled reset)? Compared against the
    // previous reading. Used both to suppress false flushes and to end a party.
    let anchor_advanced = match (cur_reset.as_deref(), st.last_reset.as_deref()) {
        (Some(cur), Some(prev)) => reset_delta_seconds(cur, prev)
            .map(|d| d > cfg.anchor_tolerance_seconds)
            .unwrap_or(false),
        _ => false,
    };

    // Detect a fresh flush event: a real drop with the anchor held.
    if let (Some(cu), Some(pu)) = (cur_util, st.last_util) {
        if is_flush(pu, cu, anchor_advanced, cfg.drop_threshold) {
            st.active = true;
            st.since = Some(Local::now().to_rfc3339());
            append_log(
                exe_dir,
                &format!(
                    "Free flush detected: weekly {:.0}% -> {:.0}%, anchor held. Party on.",
                    pu, cu
                ),
            );
        }
    }

    // End conditions for an active party.
    if st.active {
        if anchor_advanced {
            st.active = false;
            st.since = None;
        } else if let Some(cu) = cur_util {
            if cu >= cfg.stop_at_percent {
                st.active = false;
                st.since = None;
                append_log(
                    exe_dir,
                    &format!("Free flush party ended: weekly back to {:.0}%.", cu),
                );
            }
        }
    }

    // Remember this reading for the next comparison.
    st.last_util = cur_util;
    st.last_reset = cur_reset;
    save_celebrate_state(exe_dir, &st);

    if st.active {
        Some(Celebrate {
            active: true,
            since: st.since.clone(),
            reason: "Free weekly flush - fresh bucket".to_string(),
        })
    } else {
        None
    }
}

/// Scan recent weekly history (oldest-first `(timestamp, utilization,
/// resets_at)`) for a free flush that should STILL be celebrating now, and
/// return the index of that flush reading. Used at startup so a flush that
/// happened while the app was down or being upgraded is not missed (the live
/// per-poll detector only sees drops between two of its own polls).
///
/// A party is live iff: the latest flush event (a drop >= `drop_threshold`
/// with the anchor held) is followed by NO reading that climbed back to
/// `stop_at_percent` and NO scheduled reset (anchor jump) since. The flush
/// reading itself is included in that check, so a drop that merely lands high
/// (e.g. 90% -> 70%) does not party.
fn detect_live_flush(
    readings: &[(String, f64, Option<String>)],
    cfg: CelebrationCfg,
) -> Option<usize> {
    if !cfg.enabled || readings.len() < 2 {
        return None;
    }

    let advanced = |cur: &Option<String>, prev: &Option<String>| -> bool {
        match (cur.as_deref(), prev.as_deref()) {
            (Some(c), Some(p)) => reset_delta_seconds(c, p)
                .map(|d| d > cfg.anchor_tolerance_seconds)
                .unwrap_or(false),
            _ => false,
        }
    };

    // Latest flush event wins (a later flush supersedes an earlier one).
    let mut flush_idx = None;
    for i in 1..readings.len() {
        if is_flush(
            readings[i - 1].1,
            readings[i].1,
            advanced(&readings[i].2, &readings[i - 1].2),
            cfg.drop_threshold,
        ) {
            flush_idx = Some(i);
        }
    }
    let idx = flush_idx?;
    let flush_reset = readings[idx].2.clone();

    // From the flush onward, the party must not have been consumed: no reading
    // back at/above the stop threshold, and no scheduled reset since.
    for (_, util, reset) in &readings[idx..] {
        if *util >= cfg.stop_at_percent {
            return None;
        }
        if advanced(reset, &flush_reset) {
            return None;
        }
    }
    Some(idx)
}

/// At startup, resume a free-flush party that is still live in the history but
/// was never recorded (app was down or pre-feature when the flush happened).
/// No-op if a party is already active (the per-poll detector owns it) or if the
/// db/history is unavailable.
fn backfill_celebration(exe_dir: &Path, cfg: CelebrationCfg, provider: &str) {
    if !cfg.enabled {
        return;
    }
    if load_celebrate_state(exe_dir).active {
        return;
    }
    let Ok(db) = Database::open(exe_dir) else {
        return;
    };
    let Ok(readings) = db.query_recent_readings("seven_day", 8, provider) else {
        return;
    };
    if let Some(idx) = detect_live_flush(&readings, cfg) {
        let last = readings
            .last()
            .expect("non-empty: detect_live_flush matched");
        let state = CelebrateState {
            last_util: Some(last.1),
            last_reset: last.2.clone(),
            active: true,
            since: Some(readings[idx].0.clone()),
        };
        save_celebrate_state(exe_dir, &state);
        append_log(
            exe_dir,
            "Startup backfill: a recent free flush is still live, resuming the party.",
        );
    }
}

/// A flush is a weekly-utilization drop of at least `drop_threshold` points
/// while the reset anchor did not jump forward. A scheduled reset also drops
/// utilization, but it moves the anchor, so `anchor_advanced` suppresses it.
fn is_flush(prev_util: f64, cur_util: f64, anchor_advanced: bool, drop_threshold: f64) -> bool {
    (prev_util - cur_util) >= drop_threshold && !anchor_advanced
}

/// Signed seconds between two RFC3339 timestamps (a - b). None if either fails
/// to parse.
fn reset_delta_seconds(a: &str, b: &str) -> Option<i64> {
    let a: chrono::DateTime<chrono::Utc> = a.parse().ok()?;
    let b: chrono::DateTime<chrono::Utc> = b.parse().ok()?;
    Some(a.signed_duration_since(b).num_seconds())
}

fn load_celebrate_state(exe_dir: &Path) -> CelebrateState {
    std::fs::read_to_string(exe_dir.join("celebrate_state.json"))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_celebrate_state(exe_dir: &Path, st: &CelebrateState) {
    if let Ok(json) = serde_json::to_string_pretty(st) {
        let _ = std::fs::write(exe_dir.join("celebrate_state.json"), json);
    }
}

fn save_history(exe_dir: &Path, usage: &UsageResponse, provider: &str) {
    let db = match Database::open(exe_dir) {
        Ok(db) => db,
        Err(e) => {
            append_log(exe_dir, &format!("Database unavailable: {e}"));
            return;
        }
    };

    for (metric, value) in usage.all_metrics() {
        if let Err(e) = db.insert(
            provider,
            &metric,
            value.utilization,
            value.resets_at.as_deref(),
        ) {
            append_log(exe_dir, &format!("Failed to save metric {metric}: {e}"));
        }
    }
}

fn publish_status(
    exe_dir: &Path,
    usage: &UsageResponse,
    plan_override: Option<&str>,
    provider: &str,
    celebrate: Option<Celebrate>,
) {
    let percent = usage.max_utilization().unwrap_or(0.0).round() as u32;
    // plan_override is a Claude tier label (Pro/Max 5x/Max 20x); ignore it for
    // z.ai so the API's own level ("GLM Max") is shown.
    let plan = if provider == "claude" {
        plan_override
            .map(|s| s.to_string())
            .unwrap_or_else(|| usage.detected_plan())
    } else {
        usage.detected_plan()
    };
    let now = Local::now();
    let last_api_update = now.to_rfc3339();
    let message = format!("{plan}: {percent}% max usage");

    let metrics = usage
        .all_metrics()
        .into_iter()
        .map(|(key, m)| MetricEntry {
            name: format_metric_name(&key),
            percent: m.utilization.round() as u32,
            resets_at: m.resets_at.clone(),
            key,
        })
        .collect();

    // The downgrade-comparison tier math is Claude-specific (Pro/Max 5x/Max
    // 20x multipliers); skip it for z.ai.
    let tier_note = if provider == "claude" {
        plan_override.and_then(|p| build_tier_note(p, usage))
    } else {
        None
    };

    // 24h history for the menu-bar chart. save_history() already inserted this
    // poll's reading, so reopening the DB here picks up the freshest bucket.
    let chart = Database::open(exe_dir)
        .and_then(|db| db.query_24h_chart(provider))
        .map(|slots| slots.iter().map(|v| v.round() as u32).collect())
        .unwrap_or_default();

    // Past 5-hour session reset points as "hours ago", stepping back by 5h from
    // the most recent reset up to 24h. Same logic as windows_app.rs.
    let mut chart_resets = Vec::new();
    if let Some(secs) = usage
        .five_hour
        .as_ref()
        .and_then(|fh| fh.resets_at.as_deref())
        .and_then(seconds_until)
    {
        let hours_until = secs as f64 / 3600.0;
        let mut hours_ago = 5.0 - hours_until;
        while hours_ago <= 24.0 {
            if hours_ago > 0.0 {
                chart_resets.push(hours_ago);
            }
            hours_ago += 5.0;
        }
    }

    let status = MacStatus {
        state: "live".to_string(),
        title: format!("{percent}%"),
        detail: message.clone(),
        plan: Some(plan),
        percent: Some(percent),
        metrics,
        tier_note,
        last_api_update: Some(last_api_update),
        data_age_seconds: Some(0),
        error: None,
        chart,
        chart_resets,
        celebrate,
    };

    append_log(
        exe_dir,
        &format!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), message),
    );
    write_status(exe_dir, &status);

    if percent >= 90 {
        notify("ClaudeMeter: high usage", &message);
    }
}

/// Plan tier as a multiple of the Pro base allowance. Used to estimate what
/// usage would look like on a smaller plan. Recognized labels: Pro, Max 5x,
/// Max 20x (bare "Max" assumed 5x, its entry tier).
fn plan_multiplier(plan: &str) -> Option<f64> {
    let p = plan.to_lowercase();
    if p.contains("20x") {
        Some(20.0)
    } else if p.contains("5x") || p.contains("max") {
        Some(5.0)
    } else if p.contains("pro") {
        Some(1.0)
    } else {
        None
    }
}

/// The next cheaper tier and its multiplier, or None if already at the bottom.
fn lower_tier(mult: f64) -> Option<(&'static str, f64)> {
    if mult >= 20.0 {
        Some(("Max 5x", 5.0))
    } else if mult >= 5.0 {
        Some(("Pro", 1.0))
    } else {
        None
    }
}

/// "On Max 5x: weekly ~96%, session ~124% — would throttle": estimate the
/// session and weekly usage if the same work ran on the next tier down.
/// Assumes limits scale linearly with the tier multiplier.
fn build_tier_note(plan: &str, usage: &UsageResponse) -> Option<String> {
    let mult = plan_multiplier(plan)?;
    let (lower_name, lower_mult) = lower_tier(mult)?;
    let factor = mult / lower_mult;

    let week = usage.seven_day.as_ref().map(|m| m.utilization);
    let five = usage.five_hour.as_ref().map(|m| m.utilization);

    let mut parts = Vec::new();
    if let Some(w) = week {
        parts.push(format!("weekly ~{}%", (w * factor).round() as i64));
    }
    if let Some(f) = five {
        parts.push(format!("session ~{}%", (f * factor).round() as i64));
    }
    if parts.is_empty() {
        return None;
    }

    let over = week.is_some_and(|w| w * factor > 100.0) || five.is_some_and(|f| f * factor > 100.0);
    let verdict = if over {
        " — would throttle"
    } else {
        " — would fit"
    };
    Some(format!(
        "On {}: {}{}",
        lower_name,
        parts.join(", "),
        verdict
    ))
}

/// Seconds until an RFC3339 reset timestamp (negative if already past).
/// Local copy of the i18n helper, which is Windows-gated.
fn seconds_until(resets_at: &str) -> Option<i64> {
    let reset: chrono::DateTime<chrono::Utc> = resets_at.parse().ok()?;
    Some(
        reset
            .signed_duration_since(chrono::Utc::now())
            .num_seconds(),
    )
}

fn write_status(exe_dir: &Path, status: &MacStatus) {
    let path = exe_dir.join("status.json");
    match serde_json::to_string_pretty(status) {
        Ok(json) => {
            if let Err(e) = std::fs::write(path, json) {
                log::warn!("Failed to write macOS status: {e}");
            }
        }
        Err(e) => log::warn!("Failed to serialize macOS status: {e}"),
    }
}

/// Begin a refresh without blanking the menu. If a prior good reading exists,
/// keep its numbers on screen (and clear any stale error) instead of flashing
/// an empty "refreshing" state every poll. Only show the blank refreshing
/// placeholder on the very first run, when there is no data yet.
fn mark_refreshing(exe_dir: &Path) {
    let path = exe_dir.join("status.json");
    if let Ok(contents) = std::fs::read_to_string(&path) {
        if let Ok(mut prev) = serde_json::from_str::<MacStatus>(&contents) {
            if prev.percent.is_some() {
                prev.error = None;
                write_status(exe_dir, &prev);
                return;
            }
        }
    }
    write_status(exe_dir, &MacStatus::refreshing());
}

/// Record a fetch failure without discarding the last good reading.
/// If a prior live status exists, keep its data and metrics (so the menu
/// keeps showing the numbers with a staleness indicator) and just attach
/// the error. Only blank out when there is no prior data to show.
fn write_error(exe_dir: &Path, message: String) {
    let path = exe_dir.join("status.json");
    if let Ok(contents) = std::fs::read_to_string(&path) {
        if let Ok(mut prev) = serde_json::from_str::<MacStatus>(&contents) {
            if prev.percent.is_some() {
                prev.error = Some(message);
                write_status(exe_dir, &prev);
                return;
            }
        }
    }
    write_status(exe_dir, &MacStatus::error(message));
}

fn print_status(exe_dir: &Path) {
    let path = exe_dir.join("status.json");
    match std::fs::read_to_string(path) {
        Ok(status) => println!("{status}"),
        Err(_) => println!(
            "{}",
            serde_json::to_string(&MacStatus::refreshing()).unwrap()
        ),
    }
}

fn append_log(exe_dir: &Path, message: &str) {
    let path = exe_dir.join("claudemeter.log");
    let line = format!("{}\n", message);
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut file| {
            use std::io::Write;
            file.write_all(line.as_bytes())
        });
}

fn notify(title: &str, message: &str) {
    let script = format!(
        "display notification \"{}\" with title \"{}\"",
        escape_applescript(message),
        escape_applescript(title)
    );

    let _ = Command::new("osascript").arg("-e").arg(script).status();
}

fn escape_applescript(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn app_data_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("ClaudeMeter");
    }

    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refreshing_status_shape() {
        let status = MacStatus::refreshing();
        assert_eq!(status.state, "refreshing");
        assert_eq!(status.title, "Refreshing...");
        assert!(status.percent.is_none());
    }

    #[test]
    fn test_escape_applescript() {
        assert_eq!(escape_applescript(r#"a\b"c"#), r#"a\\b\"c"#);
    }

    #[test]
    fn test_is_flush_free_flush() {
        // 54% -> 0% with anchor held: a free flush.
        assert!(is_flush(54.0, 0.0, false, 15.0));
    }

    #[test]
    fn test_is_flush_scheduled_reset_suppressed() {
        // Same big drop, but the anchor jumped forward: a scheduled reset, not
        // a free flush. Must not celebrate.
        assert!(!is_flush(54.0, 0.0, true, 15.0));
    }

    #[test]
    fn test_is_flush_small_wobble_ignored() {
        // Normal poll-to-poll noise under the threshold is not a flush.
        assert!(!is_flush(54.0, 50.0, false, 15.0));
    }

    #[test]
    fn test_is_flush_threshold_boundary() {
        // Exactly at the threshold counts.
        assert!(is_flush(20.0, 5.0, false, 15.0));
    }

    #[test]
    fn test_is_flush_flat_zero() {
        // Both readings at 0%: no delta, no party.
        assert!(!is_flush(0.0, 0.0, false, 15.0));
    }

    #[test]
    fn test_is_flush_rising_utilization() {
        // Utilization went UP (negative drop): never a flush.
        assert!(!is_flush(10.0, 40.0, false, 15.0));
    }

    #[test]
    fn test_reset_delta_seconds_holds_vs_advances() {
        let a = "2026-06-18T04:00:00+00:00";
        let b = "2026-06-18T03:59:59+00:00";
        // Anchor held: ~1s apart.
        assert_eq!(reset_delta_seconds(a, b), Some(1));
        // Anchor advanced ~7 days.
        let later = "2026-06-25T04:00:00+00:00";
        assert!(reset_delta_seconds(later, a).unwrap() > 600_000);
    }

    fn cfg() -> CelebrationCfg {
        CelebrationCfg {
            enabled: true,
            stop_at_percent: 5.0,
            drop_threshold: 15.0,
            anchor_tolerance_seconds: 3600,
        }
    }

    fn reading(ts: &str, util: f64, reset: &str) -> (String, f64, Option<String>) {
        (ts.to_string(), util, Some(reset.to_string()))
    }

    const A: &str = "2026-06-18T04:00:00+00:00";
    const B: &str = "2026-06-25T04:00:00+00:00"; // ~7 days later (scheduled reset)

    #[test]
    fn test_detect_live_flush_active() {
        let rows = vec![reading("t0", 54.0, A), reading("t1", 0.0, A)];
        assert_eq!(detect_live_flush(&rows, cfg()), Some(1));
    }

    #[test]
    fn test_detect_live_flush_latest_flush_wins() {
        // Two real flushes (both drops exceed the 15pt threshold): 54->0, then
        // usage climbed to 20% (which consumed the first party), then 20->0.
        // The later flush at idx 3 wins; the consumed scan runs only from there,
        // so the intermediate 20% does NOT end this party. Note the reviewer's
        // suggested values (3->0, 8->0) are below threshold and would not be
        // flushes at all; these use real drops.
        let rows = vec![
            reading("t0", 54.0, A),
            reading("t1", 0.0, A),
            reading("t2", 20.0, A),
            reading("t3", 0.0, A),
        ];
        assert_eq!(detect_live_flush(&rows, cfg()), Some(3));
    }

    #[test]
    fn test_detect_live_flush_stop_threshold_boundary() {
        // Post-flush util exactly at the stop threshold (5.0, stop=5.0): the
        // consumed scan uses >=, so the party does NOT start.
        let at = vec![
            reading("t0", 54.0, A),
            reading("t1", 0.0, A),
            reading("t2", 5.0, A),
        ];
        assert_eq!(detect_live_flush(&at, cfg()), None);
        // A hair under the threshold: party starts.
        let under = vec![
            reading("t0", 54.0, A),
            reading("t1", 0.0, A),
            reading("t2", 4.999, A),
        ];
        assert_eq!(detect_live_flush(&under, cfg()), Some(1));
    }

    #[test]
    fn test_detect_live_flush_consumed_by_usage() {
        // Flush, then usage climbed back past the stop threshold: party over.
        let rows = vec![
            reading("t0", 54.0, A),
            reading("t1", 0.0, A),
            reading("t2", 8.0, A),
        ];
        assert_eq!(detect_live_flush(&rows, cfg()), None);
    }

    #[test]
    fn test_detect_live_flush_ended_by_scheduled_reset() {
        // Flush, then a scheduled reset advanced the anchor: party over.
        let rows = vec![
            reading("t0", 54.0, A),
            reading("t1", 0.0, A),
            reading("t2", 0.0, B),
        ];
        assert_eq!(detect_live_flush(&rows, cfg()), None);
    }

    #[test]
    fn test_detect_live_flush_no_flush() {
        let rows = vec![reading("t0", 3.0, A), reading("t1", 4.0, A)];
        assert_eq!(detect_live_flush(&rows, cfg()), None);
    }

    #[test]
    fn test_detect_live_flush_drop_landing_high_is_not_party() {
        // Big drop but still well above the stop threshold (90% -> 70%): you are
        // clearly using it, no party.
        let rows = vec![reading("t0", 90.0, A), reading("t1", 70.0, A)];
        assert_eq!(detect_live_flush(&rows, cfg()), None);
    }

    #[test]
    fn test_reset_delta_seconds_equal_and_bad() {
        // Identical timestamps: zero delta, treated as anchor held.
        let t = "2026-06-18T04:00:00+00:00";
        assert_eq!(reset_delta_seconds(t, t), Some(0));
        // Unparseable input yields None rather than panicking.
        assert_eq!(reset_delta_seconds("not-a-date", t), None);
        assert_eq!(reset_delta_seconds(t, ""), None);
    }
}
