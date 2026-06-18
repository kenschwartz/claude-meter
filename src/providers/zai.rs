//! z.ai (GLM Coding Plan) usage provider.
//!
//! Polls the z.ai account-monitoring API with a bearer token (the same
//! `GLM_API_KEY` the `zai` shell launcher reads from ~/.hermes/.env). The
//! primary source is `/api/monitor/usage/quota/limit`, which returns a list of
//! limits. We decode them into the shared `UsageResponse` shape so the rest of
//! the pipeline (history DB, 24h chart, free-flush celebration, status.json)
//! is unchanged:
//!
//!   TOKENS_LIMIT unit=3 number=5  -> five_hour  (5-hour token limit)
//!   TOKENS_LIMIT unit=6 number=1  -> seven_day  (weekly token limit)
//!   TIME_LIMIT                    -> extra "tools_5h" (built-in tools quota)
//!
//! `data.level` becomes the plan label, e.g. "max" -> "GLM Max". The metric
//! KEY names (`five_hour`, `seven_day`) are intentionally identical to the
//! Claude provider's: the Swift menu-bar title and the 24h chart query both
//! select on those exact strings.
//!
//! Field semantics verified against `opencode-glm-quota/src/utils/token-limits.ts`
//! and against the live account (5-hour resets same day, weekly resets ~7d out).

use crate::providers::claude::{UsageMetric, UsageResponse};
use chrono::{TimeZone, Utc};
use std::collections::HashMap;

const QUOTA_LIMIT_URL: &str = "https://api.z.ai/api/monitor/usage/quota/limit";

pub struct ZaiClient {
    client: reqwest::Client,
}

impl ZaiClient {
    pub fn new() -> Result<Self, String> {
        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Self { client })
    }

    pub async fn fetch_usage(&self, token: &str) -> Result<UsageResponse, String> {
        let response = self
            .client
            .get(QUOTA_LIMIT_URL)
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {token}"))
            .header("Cache-Control", "no-cache, no-store, max-age=0")
            .header("Pragma", "no-cache")
            .send()
            .await
            .map_err(|e| format!("[network_error] {e}"))?;

        let status = response.status();
        if status.as_u16() == 429 {
            return Err("[rate_limited] z.ai quota API rate limited".to_string());
        }
        if status.as_u16() == 401 || status.as_u16() == 403 {
            return Err("[token_expired] z.ai token rejected (check GLM_API_KEY)".to_string());
        }
        if status.is_server_error() {
            return Err(format!("[server_error] z.ai returned {status}"));
        }
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(format!("[api_error] z.ai returned {status}: {body}"));
        }

        let value: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse z.ai response JSON: {e}"))?;

        parse_quota_response(value)
    }
}

/// Decode the `quota/limit` envelope into a `UsageResponse`.
///
/// Top-level shape: `{ code, msg, success, data: { level, limits: [...] } }`.
/// Each limit carries `type`, `unit`, `number`, `percentage`, `nextResetTime`
/// (epoch ms). Token limits expose only `percentage` (the utilization), which
/// is exactly what the meter displays.
fn parse_quota_response(value: serde_json::Value) -> Result<UsageResponse, String> {
    let success = value
        .get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let code = value.get("code").and_then(|v| v.as_i64()).unwrap_or(0);
    if !success && code != 200 {
        let msg = value
            .get("msg")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown error");
        return Err(format!(
            "[api_error] z.ai reported failure (code {code}): {msg}"
        ));
    }

    let data = value
        .get("data")
        .ok_or_else(|| "[api_error] z.ai response missing data".to_string())?;

    let level = data.get("level").and_then(|v| v.as_str()).unwrap_or("plan");
    let plan_label = format!("GLM {}", title_case(level));

    let limits = data
        .get("limits")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut resp = UsageResponse {
        five_hour: None,
        seven_day: None,
        seven_day_sonnet: None,
        seven_day_opus: None,
        seven_day_oauth_apps: None,
        extra: HashMap::new(),
        subscription_type: Some(plan_label),
        rate_limit_tier: None,
    };

    for limit in limits {
        let kind = limit.get("type").and_then(|v| v.as_str()).unwrap_or("");
        let unit = limit.get("unit").and_then(|v| v.as_i64());
        let number = limit.get("number").and_then(|v| v.as_i64());
        let metric = decode_metric(&limit);

        match (kind, unit, number) {
            ("TOKENS_LIMIT", Some(3), Some(5)) => resp.five_hour = metric,
            ("TOKENS_LIMIT", Some(6), Some(1)) => resp.seven_day = metric,
            ("TIME_LIMIT", _, _) => {
                if let Some(m) = metric {
                    resp.extra.insert("tools_5h".to_string(), m);
                }
            }
            _ => {
                if let Some(m) = metric {
                    let key = format!(
                        "zai_{}_{}_{}",
                        kind.to_lowercase(),
                        unit.unwrap_or(0),
                        number.unwrap_or(0)
                    );
                    resp.extra.insert(key, m);
                }
            }
        }
    }

    Ok(resp)
}

/// Build a `UsageMetric` from a limit's `percentage` + `nextResetTime` (epoch ms).
/// None when the limit carries no `percentage`.
fn decode_metric(limit: &serde_json::Value) -> Option<UsageMetric> {
    let utilization = limit.get("percentage").and_then(|v| v.as_f64())?;
    let resets_at = limit
        .get("nextResetTime")
        .and_then(|v| v.as_i64())
        .and_then(epoch_ms_to_iso);
    Some(UsageMetric {
        utilization,
        resets_at,
    })
}

/// Epoch milliseconds -> RFC3339 string. None if the timestamp is out of range.
fn epoch_ms_to_iso(ms: i64) -> Option<String> {
    Utc.timestamp_millis_opt(ms)
        .single()
        .map(|dt| dt.to_rfc3339())
}

/// "max" -> "Max". First letter uppercased, rest unchanged.
fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_verified_live_payload() {
        // The exact shape probed against the live account on 2026-06-18.
        let json = serde_json::json!({
            "code": 200,
            "msg": "Operation successful",
            "success": true,
            "data": {
                "level": "max",
                "limits": [
                    {
                        "type": "TIME_LIMIT", "unit": 5, "number": 1,
                        "usage": 4000, "currentValue": 2, "remaining": 3998, "percentage": 1,
                        "nextResetTime": 1784371630956_i64,
                        "usageDetails": [{"modelCode": "search-prime", "usage": 1}]
                    },
                    { "type": "TOKENS_LIMIT", "unit": 3, "number": 5, "percentage": 1,
                      "nextResetTime": 1781817824865_i64 },
                    { "type": "TOKENS_LIMIT", "unit": 6, "number": 1, "percentage": 2,
                      "nextResetTime": 1782384430998_i64 }
                ]
            }
        });
        let resp = parse_quota_response(json).unwrap();

        // 5-hour token limit -> five_hour.
        let fh = resp
            .five_hour
            .as_ref()
            .expect("five_hour should be populated");
        assert!((fh.utilization - 1.0).abs() < 1e-9);
        assert!(fh.resets_at.as_deref().unwrap().starts_with("2026-06-"));

        // Weekly token limit -> seven_day.
        let wk = resp
            .seven_day
            .as_ref()
            .expect("seven_day should be populated");
        assert!((wk.utilization - 2.0).abs() < 1e-9);
        // Weekly reset is ~7 days out (2026-06-25).
        assert!(wk.resets_at.as_deref().unwrap().starts_with("2026-06-25"));

        // Built-in tools quota -> extra.
        assert_eq!(resp.extra.get("tools_5h").unwrap().utilization, 1.0);

        // Plan label flows through detected_plan() via the `other` arm.
        assert_eq!(resp.subscription_type.as_deref(), Some("GLM Max"));
        assert_eq!(resp.detected_plan(), "GLM Max");
        // The two token limits are the only named metrics surfaced.
        assert_eq!(resp.max_utilization(), Some(2.0));
    }

    #[test]
    fn test_rejects_failed_envelope() {
        let json = serde_json::json!({ "code": 401, "success": false, "msg": "bad token" });
        assert!(parse_quota_response(json).is_err());
    }

    #[test]
    fn test_missing_data_is_error() {
        let json = serde_json::json!({ "code": 200, "success": true });
        assert!(parse_quota_response(json).is_err());
    }

    #[test]
    fn test_unknown_limit_goes_to_extra() {
        let json = serde_json::json!({
            "code": 200, "success": true,
            "data": { "level": "pro", "limits": [
                { "type": "TOKENS_LIMIT", "unit": 99, "number": 9, "percentage": 7,
                  "nextResetTime": 1781817824865_i64 }
            ]}
        });
        let resp = parse_quota_response(json).unwrap();
        assert!(resp.five_hour.is_none());
        assert!(resp.seven_day.is_none());
        assert_eq!(resp.extra.len(), 1);
        assert_eq!(resp.subscription_type.as_deref(), Some("GLM Pro"));
    }

    #[test]
    fn test_limit_without_percentage_skipped() {
        // A limit with no percentage contributes no metric.
        let json = serde_json::json!({
            "code": 200, "success": true,
            "data": { "level": "max", "limits": [
                { "type": "TOKENS_LIMIT", "unit": 3, "number": 5, "nextResetTime": 1781817824865_i64 }
            ]}
        });
        let resp = parse_quota_response(json).unwrap();
        assert!(resp.five_hour.is_none());
        assert!(resp.extra.is_empty());
    }

    #[test]
    fn test_epoch_ms_to_iso() {
        let iso = epoch_ms_to_iso(1781817824865_i64).unwrap();
        assert!(iso.starts_with("2026-06-18"));
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("max"), "Max");
        assert_eq!(title_case("pro"), "Pro");
        assert_eq!(title_case("free"), "Free");
        assert_eq!(title_case(""), "");
    }
}
