use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub thresholds: Vec<u8>,
    pub sound: bool,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            thresholds: vec![50, 75, 90],
            sound: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuietHoursConfig {
    pub enabled: bool,
    pub start: String,
    pub end: String,
}

impl Default for QuietHoursConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            start: "22:00".to_string(),
            end: "08:00".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomColors {
    pub background: Option<String>,
    pub surface: Option<String>,
    pub text_primary: Option<String>,
    pub text_secondary: Option<String>,
    pub progress_bg: Option<String>,
    pub green: Option<String>,
    pub yellow: Option<String>,
    pub red: Option<String>,
    pub accent: Option<String>,
    pub separator: Option<String>,
    pub hover: Option<String>,
    pub border: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub polling_interval_seconds: u64,
    pub notifications: NotificationConfig,
    pub autostart: bool,
    pub compact_mode: bool,
    pub theme: String,
    pub language: String,
    pub show_chatgpt_section: bool,
    pub chatgpt_usage_url: String,
    pub claude_install_url: String,
    #[serde(default)]
    pub show_widget: bool,
    #[serde(default = "default_true")]
    pub check_updates: bool,
    #[serde(default)]
    pub accessibility_patterns: bool,
    #[serde(default = "default_icon_style")]
    pub tray_icon_style: String,
    #[serde(default = "default_dashboard_layout")]
    pub dashboard_layout: String,
    #[serde(default)]
    pub hide_extra_usage: bool,
    #[serde(default)]
    pub token_expiry_warning: bool,
    #[serde(default = "default_true")]
    pub show_startup_notification: bool,
    #[serde(default)]
    pub custom_colors: CustomColors,
    #[serde(default)]
    pub quiet_hours: QuietHoursConfig,
    /// Session key for claude.ai web API fallback (optional)
    #[serde(default)]
    pub web_api_session_key: Option<String>,
    /// Organization ID for claude.ai web API fallback (optional)
    #[serde(default)]
    pub web_api_org_id: Option<String>,
    /// Manual plan label override, e.g. "Max 20x". The usage API does not
    /// return the plan tier, so set this for a correct label and the
    /// downgrade comparison. Recognized: "Pro", "Max 5x", "Max 20x".
    #[serde(default)]
    pub plan_override: Option<String>,
    /// Celebrate a "free flush": when Anthropic zeroes the weekly counter out
    /// of band (utilization drops hard while the reset anchor holds), the menu
    /// bar throws a fireworks + rainbow party until you start using the fresh
    /// bucket again. Tunables below.
    #[serde(default = "default_true")]
    pub celebrate_free_flush: bool,
    /// Stop the party once weekly utilization climbs back to at least this
    /// percent (you have started denting the fresh bucket).
    #[serde(default = "default_celebrate_stop_at")]
    pub celebrate_stop_at_percent: f64,
    /// Minimum weekly-utilization drop (percentage points) between two polls to
    /// count as a flush. Filters out normal poll-to-poll wobble.
    #[serde(default = "default_celebrate_drop")]
    pub celebrate_drop_threshold: f64,
    /// Max movement (seconds) of resets_at still treated as "anchor held". A
    /// scheduled weekly reset moves it by days; a free flush leaves it put.
    #[serde(default = "default_celebrate_tolerance_seconds")]
    pub celebrate_anchor_tolerance_seconds: i64,
}

fn default_celebrate_stop_at() -> f64 {
    5.0
}

fn default_celebrate_drop() -> f64 {
    15.0
}

fn default_celebrate_tolerance_seconds() -> i64 {
    3600
}

fn default_dashboard_layout() -> String {
    "standard".to_string()
}

fn default_icon_style() -> String {
    "number".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            polling_interval_seconds: 120,
            notifications: NotificationConfig::default(),
            autostart: false,
            compact_mode: false,
            theme: "auto".to_string(),
            language: "auto".to_string(),
            show_chatgpt_section: false,
            chatgpt_usage_url: "https://chatgpt.com/codex/settings/usage".to_string(),
            claude_install_url: "https://claude.ai/download".to_string(),
            show_widget: false,
            check_updates: true,
            accessibility_patterns: false,
            tray_icon_style: "number".to_string(),
            dashboard_layout: "standard".to_string(),
            hide_extra_usage: false,
            token_expiry_warning: false,
            show_startup_notification: true,
            custom_colors: CustomColors::default(),
            quiet_hours: QuietHoursConfig::default(),
            web_api_session_key: None,
            web_api_org_id: None,
            plan_override: None,
            celebrate_free_flush: true,
            celebrate_stop_at_percent: 5.0,
            celebrate_drop_threshold: 15.0,
            celebrate_anchor_tolerance_seconds: 3600,
        }
    }
}

impl Config {
    /// Generate a random polling interval.
    /// Within 15 minutes before the top of the hour: 120-180 seconds.
    /// Otherwise: 120-300 seconds.
    /// Uses system time nanoseconds for simple randomness without external crate.
    #[cfg_attr(not(windows), allow(dead_code))] // consumed by the Windows app
    pub fn random_polling_interval() -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = now.as_secs();
        let minute_of_hour = (secs % 3600) / 60;
        let nanos = now.subsec_nanos() as u64;

        if minute_of_hour >= 45 {
            // Last 15 minutes of the hour: 120-180s
            120 + (nanos % 61)
        } else {
            // Normal: 120-300s
            120 + (nanos % 181)
        }
    }

    /// Validate and fix config values to safe ranges.
    pub fn validate(&mut self) {
        // Thresholds: keep only 1..=100, remove duplicates, sort
        self.notifications
            .thresholds
            .retain(|&t| (1..=100).contains(&t));
        self.notifications.thresholds.sort();
        self.notifications.thresholds.dedup();
        if self.notifications.thresholds.is_empty() {
            self.notifications.thresholds = vec![50, 75, 90];
        }

        // Validate theme
        if !["auto", "dark", "light"].contains(&self.theme.as_str()) {
            self.theme = "auto".to_string();
        }

        // Validate tray icon style
        if !["number", "ring", "bar", "pie"].contains(&self.tray_icon_style.as_str()) {
            self.tray_icon_style = "number".to_string();
        }

        // Validate dashboard layout
        if !["minimal", "standard", "detailed"].contains(&self.dashboard_layout.as_str()) {
            self.dashboard_layout = "standard".to_string();
        }

        // Celebration tunables: keep to sane ranges.
        self.celebrate_stop_at_percent = self.celebrate_stop_at_percent.clamp(0.0, 100.0);
        self.celebrate_drop_threshold = self.celebrate_drop_threshold.clamp(1.0, 100.0);
        // Below ~1 minute, normal integer-second jitter in resets_at could read
        // as "anchor advanced" and end a party early. Anything that small is a
        // misconfiguration; fall back to the default.
        if self.celebrate_anchor_tolerance_seconds < 60 {
            self.celebrate_anchor_tolerance_seconds = 3600;
        }

        // Validate language
        if ![
            "auto", "en", "uk", "es", "de", "fr", "pt", "ja", "ko", "zh", "it", "hi", "tr", "nl",
            "pl", "vi", "ru", "th", "id", "sv", "cs", "ar", "ro", "da", "fi", "hu", "bg", "el",
            "he", "ms", "no",
        ]
        .contains(&self.language.as_str())
        {
            self.language = "auto".to_string();
        }
    }
}

pub struct ConfigManager {
    path: PathBuf,
    last_modified: Option<SystemTime>,
    pub config: Config,
}

impl ConfigManager {
    pub fn new(exe_dir: &Path) -> Self {
        let path = exe_dir.join("config.json");
        let mut mgr = Self {
            path,
            last_modified: None,
            config: Config::default(),
        };
        mgr.load();
        mgr
    }

    fn load(&mut self) {
        if self.path.exists() {
            match fs::read_to_string(&self.path) {
                Ok(content) => match serde_json::from_str::<Config>(&content) {
                    Ok(mut cfg) => {
                        cfg.validate();
                        self.config = cfg;
                        self.last_modified = fs::metadata(&self.path)
                            .ok()
                            .and_then(|m| m.modified().ok());
                    }
                    Err(e) => {
                        log::warn!("Failed to parse config.json: {e}. Using defaults.");
                    }
                },
                Err(e) => {
                    log::warn!("Failed to read config.json: {e}. Using defaults.");
                }
            }
        } else {
            self.save();
        }
    }

    /// Check if config file changed on disk and reload if so.
    #[cfg_attr(not(windows), allow(dead_code))] // consumed by the Windows app
    pub fn reload_if_changed(&mut self) {
        let mtime = fs::metadata(&self.path)
            .ok()
            .and_then(|m| m.modified().ok());
        if mtime != self.last_modified {
            log::debug!("Config file changed, reloading.");
            self.load();
        }
    }

    pub fn save(&self) {
        match serde_json::to_string_pretty(&self.config) {
            Ok(content) => {
                if let Err(e) = fs::write(&self.path, content) {
                    log::error!("Failed to write config.json: {e}");
                }
            }
            Err(e) => log::error!("Failed to serialize config: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_values() {
        let cfg = Config::default();
        assert_eq!(cfg.dashboard_layout, "standard");
        assert_eq!(cfg.tray_icon_style, "number");
        assert_eq!(cfg.theme, "auto");
        assert!(!cfg.compact_mode);
        assert!(!cfg.token_expiry_warning);
    }

    #[test]
    fn test_validate_dashboard_layout() {
        let mut cfg = Config::default();

        // Valid values should be preserved
        for valid in ["minimal", "standard", "detailed"] {
            cfg.dashboard_layout = valid.to_string();
            cfg.validate();
            assert_eq!(cfg.dashboard_layout, valid);
        }

        // Invalid value resets to default
        cfg.dashboard_layout = "invalid".to_string();
        cfg.validate();
        assert_eq!(cfg.dashboard_layout, "standard");
    }

    #[test]
    fn test_validate_tray_icon_style_includes_pie() {
        let mut cfg = Config::default();

        for valid in ["number", "ring", "bar", "pie"] {
            cfg.tray_icon_style = valid.to_string();
            cfg.validate();
            assert_eq!(cfg.tray_icon_style, valid);
        }

        cfg.tray_icon_style = "invalid".to_string();
        cfg.validate();
        assert_eq!(cfg.tray_icon_style, "number");
    }

    #[test]
    fn test_validate_theme() {
        let mut cfg = Config::default();
        for valid in ["auto", "dark", "light"] {
            cfg.theme = valid.to_string();
            cfg.validate();
            assert_eq!(cfg.theme, valid);
        }
        cfg.theme = "neon".to_string();
        cfg.validate();
        assert_eq!(cfg.theme, "auto");
    }

    #[test]
    fn test_validate_thresholds() {
        let mut cfg = Config::default();
        // Out-of-range and duplicate thresholds cleaned up
        cfg.notifications.thresholds = vec![0, 50, 50, 101, 75];
        cfg.validate();
        assert_eq!(cfg.notifications.thresholds, vec![50, 75]);
    }

    #[test]
    fn test_validate_empty_thresholds_reset() {
        let mut cfg = Config::default();
        cfg.notifications.thresholds = vec![0, 200]; // all invalid
        cfg.validate();
        assert_eq!(cfg.notifications.thresholds, vec![50, 75, 90]);
    }

    #[test]
    fn test_random_polling_interval_range() {
        for _ in 0..100 {
            let interval = Config::random_polling_interval();
            assert!((120..=300).contains(&interval));
        }
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let cfg = Config::default();
        let json = serde_json::to_string(&cfg).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.dashboard_layout, cfg.dashboard_layout);
        assert_eq!(parsed.tray_icon_style, cfg.tray_icon_style);
        assert_eq!(parsed.theme, cfg.theme);
    }

    #[test]
    fn test_config_deserialize_missing_new_fields() {
        // Simulates loading old config without new v2.0 fields
        let json = r#"{
            "version": "1.10.0",
            "polling_interval_seconds": 120,
            "notifications": {"enabled": true, "thresholds": [50, 75, 90], "sound": true},
            "autostart": false,
            "compact_mode": false,
            "theme": "auto",
            "language": "en",
            "show_chatgpt_section": false,
            "chatgpt_usage_url": "https://chatgpt.com/codex/settings/usage",
            "claude_install_url": "https://claude.ai/download"
        }"#;
        let cfg: Config = serde_json::from_str(json).unwrap();
        // New fields should get defaults
        assert_eq!(cfg.dashboard_layout, "standard");
        assert_eq!(cfg.tray_icon_style, "number");
        assert!(!cfg.show_widget);
        assert!(!cfg.token_expiry_warning);
    }
}
