use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5-hour session");
    m.insert("Weekly (7-day)", "Weekly (7-day)");
    m.insert("Opus (7-day)", "Opus (7-day)");
    m.insert("Sonnet (7-day)", "Sonnet (7-day)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7-day)");
    m.insert("resets in", "resets in");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code not detected");
    m.insert("credentials_not_found", "Credentials not found");
    m.insert("connection_error", "Connection error");
    m.insert("token_expired", "Token expired");
    m.insert(
        "token_expired_desc",
        "Your OAuth token has expired. Run `claude login` in your terminal to refresh it.",
    );
    m.insert("rate_limited", "Rate limited");
    m.insert("server_error", "Server error");
    m.insert(
        "server_error_desc",
        "Anthropic API is temporarily unavailable. Will retry automatically.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code is installed but not logged in. Run `claude login` in your terminal to connect your account.",
    );
    m.insert(
        "install_claude_desc",
        "Install Claude Code and run `claude login` to enable automatic usage tracking.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Install Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI does not provide an API to track ChatGPT subscription usage programmatically.",
    );
    m.insert("Check your usage manually:", "Check your usage manually:");
    m.insert("Open ChatGPT Usage \u{2192}", "Open ChatGPT Usage \u{2192}");
    m.insert("Refresh Now", "Refresh Now");
    m.insert("Open Dashboard", "Open Dashboard");
    m.insert("Export History (CSV)", "Export History (CSV)");
    m.insert("Settings", "Settings");
    m.insert("Start with Windows", "Start with Windows");
    m.insert("About", "About");
    m.insert("Exit", "Exit");
    m.insert("Last updated:", "Last updated:");
    m.insert("Refresh", "Refresh");
    m.insert("Status", "Status");
    m.insert("Usage Alert", "Usage Alert");
    m.insert("Usage Critical", "Usage Critical");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Running in system tray. Click the icon for details.",
    );
    m.insert("Compact mode", "Compact mode");
    m.insert("Theme", "Theme");
    m.insert("Language", "Language");
    m.insert("Notifications", "Notifications");
    m.insert("Dark", "Dark");
    m.insert("Light", "Light");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Show ChatGPT section");
    m.insert("Enabled", "Enabled");
    m.insert("Sound", "Sound");
    m.insert("Thresholds", "Thresholds");
    m.insert("Polling interval", "Polling interval");
    m.insert("seconds", "seconds");
    m.insert("Startup", "Startup");
    m.insert("General", "General");
    m.insert("Back", "\u{2190} Back");
    m.insert("Open Claude.ai \u{2192}", "Open Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Usage History");
    m.insert("Usage History (24h)", "Usage History (24h)");
    m.insert("Auto (English)", "Auto (English)");
    m.insert("at", "at");
    m.insert("Resets in", "Resets in");
    m.insert("Tray icon colors:", "Tray icon colors:");
    m.insert("< 50% usage", "< 50% usage");
    m.insert("50-79% usage", "50\u{2013}79% usage");
    m.insert(">= 80% usage", "\u{2265} 80% usage");
    m.insert("No data", "No data");
    m.insert("exceeded", "exceeded");
    m.insert("Show widget", "Show widget");
    m.insert("Check for updates", "Check for updates");
    m.insert("Accessibility patterns", "Accessibility patterns");
    m.insert("Update available", "Update available");
    m.insert(
        "is available. Click to download.",
        "is available. Click to download.",
    );
    m.insert("Icon style", "Icon style");
    m.insert("Number", "Number");
    m.insert("Ring", "Ring");
    m.insert("Bar", "Bar");
    m.insert("Pie", "Pie");
    m.insert("Dashboard layout", "Dashboard layout");
    m.insert("Minimal", "Minimal");
    m.insert("Standard", "Standard");
    m.insert("Detailed", "Detailed");
    m.insert("Hide Extra Usage", "Hide Extra Usage");
    m.insert("Show startup notification", "Show startup notification");
    m.insert("Show login expiry warning", "Show login expiry warning");
    m
}
