use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "\u{09eb}-\u{0998}\u{09a8}\u{09cd}\u{099f}\u{09be}\u{09b0} \u{09b8}\u{09c7}\u{09b6}\u{09a8}");
    m.insert("Weekly (7-day)", "\u{09b8}\u{09be}\u{09aa}\u{09cd}\u{09a4}\u{09be}\u{09b9}\u{09bf}\u{0995} (\u{09ed}-\u{09a6}\u{09bf}\u{09a8})");
    m.insert("Opus (7-day)", "Opus (\u{09ed}-\u{09a6}\u{09bf}\u{09a8})");
    m.insert(
        "Sonnet (7-day)",
        "Sonnet (\u{09ed}-\u{09a6}\u{09bf}\u{09a8})",
    );
    m.insert(
        "OAuth Apps (7-day)",
        "OAuth Apps (\u{09ed}-\u{09a6}\u{09bf}\u{09a8})",
    );
    m.insert(
        "resets in",
        "\u{09b0}\u{09bf}\u{09b8}\u{09c7}\u{099f} \u{09b9}\u{09ac}\u{09c7}",
    );
    m.insert(
        "Plan",
        "\u{09aa}\u{09cd}\u{09b2}\u{09cd}\u{09af}\u{09be}\u{09a8}",
    );
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code \u{09aa}\u{09be}\u{0993}\u{09af}\u{09bc}\u{09be} \u{09af}\u{09be}\u{09af}\u{09bc}\u{09a8}\u{09bf}");
    m.insert("credentials_not_found", "\u{0995}\u{09cd}\u{09b0}\u{09c7}\u{09a1}\u{09c7}\u{09a8}\u{09b6}\u{09bf}\u{09af}\u{09bc}\u{09be}\u{09b2} \u{09aa}\u{09be}\u{0993}\u{09af}\u{09bc}\u{09be} \u{09af}\u{09be}\u{09af}\u{09bc}\u{09a8}\u{09bf}");
    m.insert("connection_error", "\u{0995}\u{09be}\u{09a8}\u{09c7}\u{0995}\u{09b6}\u{09a8} \u{09a4}\u{09cd}\u{09b0}\u{09c1}\u{099f}\u{09bf}");
    m.insert("token_expired", "\u{099f}\u{09cb}\u{0995}\u{09c7}\u{09a8} \u{09ae}\u{09c7}\u{09af}\u{09bc}\u{09be}\u{09a6} \u{0989}\u{09a4}\u{09cd}\u{09a4}\u{09c0}\u{09b0}\u{09cd}\u{09a3}");
    m.insert(
        "token_expired_desc",
        "\u{0986}\u{09aa}\u{09a8}\u{09be}\u{09b0} OAuth \u{099f}\u{09cb}\u{0995}\u{09c7}\u{09a8} \u{09ae}\u{09c7}\u{09af}\u{09bc}\u{09be}\u{09a6} \u{0989}\u{09a4}\u{09cd}\u{09a4}\u{09c0}\u{09b0}\u{09cd}\u{09a3}\u{0964} \u{099f}\u{09be}\u{09b0}\u{09cd}\u{09ae}\u{09bf}\u{09a8}\u{09be}\u{09b2}\u{09c7} `claude login` \u{099a}\u{09be}\u{09b2}\u{09be}\u{09a8}\u{0964}",
    );
    m.insert(
        "rate_limited",
        "\u{09b0}\u{09c7}\u{099f} \u{09b8}\u{09c0}\u{09ae}\u{09bf}\u{09a4}",
    );
    m.insert("server_error", "\u{09b8}\u{09be}\u{09b0}\u{09cd}\u{09ad}\u{09be}\u{09b0} \u{09a4}\u{09cd}\u{09b0}\u{09c1}\u{099f}\u{09bf}");
    m.insert(
        "server_error_desc",
        "Anthropic API \u{09b8}\u{09be}\u{09ae}\u{09af}\u{09bc}\u{09bf}\u{0995}\u{09ad}\u{09be}\u{09ac}\u{09c7} \u{0985}\u{09a8}\u{09c1}\u{09aa}\u{09b2}\u{09ac}\u{09cd}\u{09a7}\u{0964} \u{09b8}\u{09cd}\u{09ac}\u{09af}\u{09bc}\u{0982}\u{0995}\u{09cd}\u{09b0}\u{09bf}\u{09af}\u{09bc}\u{09ad}\u{09be}\u{09ac}\u{09c7} \u{09aa}\u{09c1}\u{09a8}\u{09b0}\u{09be}\u{09af}\u{09bc} \u{099a}\u{09c7}\u{09b7}\u{09cd}\u{099f}\u{09be} \u{0995}\u{09b0}\u{09ac}\u{09c7}\u{0964}",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code \u{0987}\u{09a8}\u{09b8}\u{09cd}\u{099f}\u{09b2} \u{0986}\u{099b}\u{09c7} \u{0995}\u{09bf}\u{09a8}\u{09cd}\u{09a4}\u{09c1} \u{09b2}\u{0997}\u{0987}\u{09a8} \u{0995}\u{09b0}\u{09be} \u{09b9}\u{09af}\u{09bc}\u{09a8}\u{09bf}\u{0964} `claude login` \u{099a}\u{09be}\u{09b2}\u{09be}\u{09a8}\u{0964}",
    );
    m.insert(
        "install_claude_desc",
        "Claude Code \u{0987}\u{09a8}\u{09b8}\u{09cd}\u{099f}\u{09b2} \u{0995}\u{09b0}\u{09c1}\u{09a8} \u{098f}\u{09ac}\u{0982} `claude login` \u{099a}\u{09be}\u{09b2}\u{09be}\u{09a8}\u{0964}",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Claude Code \u{0987}\u{09a8}\u{09b8}\u{09cd}\u{099f}\u{09b2} \u{0995}\u{09b0}\u{09c1}\u{09a8} \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI ChatGPT \u{09b8}\u{09be}\u{09ac}\u{09b8}\u{09cd}\u{0995}\u{09cd}\u{09b0}\u{09bf}\u{09aa}\u{09b6}\u{09a8} \u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09b9}\u{09be}\u{09b0} \u{099f}\u{09cd}\u{09b0}\u{09cd}\u{09af}\u{09be}\u{0995} \u{0995}\u{09b0}\u{09a4}\u{09c7} API \u{09aa}\u{09cd}\u{09b0}\u{09a6}\u{09be}\u{09a8} \u{0995}\u{09b0}\u{09c7} \u{09a8}\u{09be}\u{0964}",
    );
    m.insert("Check your usage manually:", "\u{09ae}\u{09cd}\u{09af}\u{09be}\u{09a8}\u{09c1}\u{09af}\u{09bc}\u{09be}\u{09b2}\u{09bf} \u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09b9}\u{09be}\u{09b0} \u{09a6}\u{09c7}\u{0996}\u{09c1}\u{09a8}:");
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "ChatGPT Usage \u{0996}\u{09c1}\u{09b2}\u{09c1}\u{09a8} \u{2192}",
    );
    m.insert("Refresh Now", "\u{098f}\u{0996}\u{09a8}\u{0987} \u{09b0}\u{09bf}\u{09ab}\u{09cd}\u{09b0}\u{09c7}\u{09b6} \u{0995}\u{09b0}\u{09c1}\u{09a8}");
    m.insert("Open Dashboard", "\u{09a1}\u{09cd}\u{09af}\u{09be}\u{09b6}\u{09ac}\u{09cb}\u{09b0}\u{09cd}\u{09a1} \u{0996}\u{09c1}\u{09b2}\u{09c1}\u{09a8}");
    m.insert("Export History (CSV)", "\u{0987}\u{09a4}\u{09bf}\u{09b9}\u{09be}\u{09b8} \u{09b0}\u{09aa}\u{09cd}\u{09a4}\u{09be}\u{09a8}\u{09bf} \u{0995}\u{09b0}\u{09c1}\u{09a8} (CSV)");
    m.insert(
        "Settings",
        "\u{09b8}\u{09c7}\u{099f}\u{09bf}\u{0982}\u{09b8}",
    );
    m.insert("Start with Windows", "Windows-\u{098f}\u{09b0} \u{09b8}\u{09be}\u{09a5}\u{09c7} \u{09b6}\u{09c1}\u{09b0}\u{09c1} \u{0995}\u{09b0}\u{09c1}\u{09a8}");
    m.insert(
        "About",
        "\u{09b8}\u{09ae}\u{09cd}\u{09aa}\u{09b0}\u{09cd}\u{0995}\u{09c7}",
    );
    m.insert(
        "Exit",
        "\u{09aa}\u{09cd}\u{09b0}\u{09b8}\u{09cd}\u{09a5}\u{09be}\u{09a8}",
    );
    m.insert(
        "Last updated:",
        "\u{09b6}\u{09c7}\u{09b7} \u{0986}\u{09aa}\u{09a1}\u{09c7}\u{099f}:",
    );
    m.insert(
        "Refresh",
        "\u{09b0}\u{09bf}\u{09ab}\u{09cd}\u{09b0}\u{09c7}\u{09b6}",
    );
    m.insert("Status", "\u{0985}\u{09ac}\u{09b8}\u{09cd}\u{09a5}\u{09be}");
    m.insert("Usage Alert", "\u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09b9}\u{09be}\u{09b0}\u{09c7}\u{09b0} \u{09b8}\u{09a4}\u{09b0}\u{09cd}\u{0995}\u{09a4}\u{09be}");
    m.insert("Usage Critical", "\u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09b9}\u{09be}\u{09b0} \u{09b8}\u{0999}\u{09cd}\u{0995}\u{099f}\u{09aa}\u{09c2}\u{09b0}\u{09cd}\u{09a3}");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "\u{09b8}\u{09bf}\u{09b8}\u{09cd}\u{099f}\u{09c7}\u{09ae} \u{099f}\u{09cd}\u{09b0}\u{09c7}\u{09a4}\u{09c7} \u{099a}\u{09b2}\u{099b}\u{09c7}\u{0964} \u{09ac}\u{09bf}\u{09b8}\u{09cd}\u{09a4}\u{09be}\u{09b0}\u{09bf}\u{09a4} \u{099c}\u{09be}\u{09a8}\u{09a4}\u{09c7} \u{0986}\u{0987}\u{0995}\u{09a8}\u{09c7} \u{0995}\u{09cd}\u{09b2}\u{09bf}\u{0995} \u{0995}\u{09b0}\u{09c1}\u{09a8}\u{0964}",
    );
    m.insert("Compact mode", "\u{0995}\u{09ae}\u{09cd}\u{09aa}\u{09cd}\u{09af}\u{09be}\u{0995}\u{09cd}\u{099f} \u{09ae}\u{09cb}\u{09a1}");
    m.insert("Theme", "\u{09a5}\u{09bf}\u{09ae}");
    m.insert("Language", "\u{09ad}\u{09be}\u{09b7}\u{09be}");
    m.insert(
        "Notifications",
        "\u{09ac}\u{09bf}\u{099c}\u{09cd}\u{099e}\u{09aa}\u{09cd}\u{09a4}\u{09bf}",
    );
    m.insert("Dark", "\u{09a1}\u{09be}\u{09b0}\u{09cd}\u{0995}");
    m.insert("Light", "\u{09b2}\u{09be}\u{0987}\u{099f}");
    m.insert("Auto", "\u{0985}\u{099f}\u{09cb}");
    m.insert(
        "Show ChatGPT section",
        "ChatGPT \u{09b8}\u{09c7}\u{0995}\u{09b6}\u{09a8} \u{09a6}\u{09c7}\u{0996}\u{09be}\u{09a8}",
    );
    m.insert(
        "Enabled",
        "\u{09b8}\u{0995}\u{09cd}\u{09b0}\u{09bf}\u{09af}\u{09bc}",
    );
    m.insert("Sound", "\u{09b6}\u{09ac}\u{09cd}\u{09a6}");
    m.insert("Thresholds", "\u{09b8}\u{09c0}\u{09ae}\u{09be}");
    m.insert("Polling interval", "\u{09aa}\u{09cb}\u{09b2}\u{09bf}\u{0982} \u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09a7}\u{09be}\u{09a8}");
    m.insert(
        "seconds",
        "\u{09b8}\u{09c7}\u{0995}\u{09c7}\u{09a8}\u{09cd}\u{09a1}",
    );
    m.insert(
        "Startup",
        "\u{09b8}\u{09cd}\u{099f}\u{09be}\u{09b0}\u{09cd}\u{099f}\u{0986}\u{09aa}",
    );
    m.insert(
        "General",
        "\u{09b8}\u{09be}\u{09a7}\u{09be}\u{09b0}\u{09a3}",
    );
    m.insert(
        "Back",
        "\u{2190} \u{09ab}\u{09bf}\u{09b0}\u{09c7} \u{09af}\u{09be}\u{09a8}",
    );
    m.insert(
        "Open Claude.ai \u{2192}",
        "Claude.ai \u{0996}\u{09c1}\u{09b2}\u{09c1}\u{09a8} \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "\u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09b9}\u{09be}\u{09b0}\u{09c7}\u{09b0} \u{0987}\u{09a4}\u{09bf}\u{09b9}\u{09be}\u{09b8}");
    m.insert("Usage History (24h)", "\u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09b9}\u{09be}\u{09b0}\u{09c7}\u{09b0} \u{0987}\u{09a4}\u{09bf}\u{09b9}\u{09be}\u{09b8} (24h)");
    m.insert("Auto (English)", "Auto (English)");
    m.insert("at", "\u{09a4}\u{09c7}");
    m.insert(
        "Resets in",
        "\u{09b0}\u{09bf}\u{09b8}\u{09c7}\u{099f} \u{09b9}\u{09ac}\u{09c7}",
    );
    m.insert("Tray icon colors:", "\u{099f}\u{09cd}\u{09b0}\u{09c7} \u{0986}\u{0987}\u{0995}\u{09a8}\u{09c7}\u{09b0} \u{09b0}\u{0999}:");
    m.insert(
        "< 50% usage",
        "< 50% \u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09b9}\u{09be}\u{09b0}",
    );
    m.insert(
        "50-79% usage",
        "50\u{2013}79% \u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09b9}\u{09be}\u{09b0}",
    );
    m.insert(
        ">= 80% usage",
        "\u{2265} 80% \u{09ac}\u{09cd}\u{09af}\u{09ac}\u{09b9}\u{09be}\u{09b0}",
    );
    m.insert("No data", "\u{0995}\u{09cb}\u{09a8}\u{09cb} \u{09a4}\u{09a5}\u{09cd}\u{09af} \u{09a8}\u{09c7}\u{0987}");
    m.insert(
        "exceeded",
        "\u{0985}\u{09a4}\u{09bf}\u{0995}\u{09cd}\u{09b0}\u{09ae}",
    );
    m.insert(
        "Show widget",
        "\u{0989}\u{0987}\u{099c}\u{09c7}\u{099f} \u{09a6}\u{09c7}\u{0996}\u{09be}\u{09a8}",
    );
    m.insert("Check for updates", "\u{0986}\u{09aa}\u{09a1}\u{09c7}\u{099f} \u{099a}\u{09c7}\u{0995} \u{0995}\u{09b0}\u{09c1}\u{09a8}");
    m.insert("Accessibility patterns", "\u{0985}\u{09cd}\u{09af}\u{09be}\u{0995}\u{09cd}\u{09b8}\u{09c7}\u{09b8}\u{09bf}\u{09ac}\u{09bf}\u{09b2}\u{09bf}\u{099f}\u{09bf} \u{09aa}\u{09cd}\u{09af}\u{09be}\u{099f}\u{09be}\u{09b0}\u{09cd}\u{09a8}");
    m.insert("Update available", "\u{0986}\u{09aa}\u{09a1}\u{09c7}\u{099f} \u{09aa}\u{09be}\u{0993}\u{09af}\u{09bc}\u{09be} \u{0997}\u{09c7}\u{099b}\u{09c7}");
    m.insert(
        "is available. Click to download.",
        "\u{09aa}\u{09be}\u{0993}\u{09af}\u{09bc}\u{09be} \u{0997}\u{09c7}\u{099b}\u{09c7}\u{0964} \u{09a1}\u{09be}\u{0989}\u{09a8}\u{09b2}\u{09cb}\u{09a1} \u{0995}\u{09b0}\u{09a4}\u{09c7} \u{0995}\u{09cd}\u{09b2}\u{09bf}\u{0995} \u{0995}\u{09b0}\u{09c1}\u{09a8}\u{0964}",
    );
    m.insert(
        "Icon style",
        "\u{0986}\u{0987}\u{0995}\u{09a8} \u{09b6}\u{09c8}\u{09b2}\u{09c0}",
    );
    m.insert("Number", "\u{09a8}\u{09ae}\u{09cd}\u{09ac}\u{09b0}");
    m.insert("Ring", "\u{09b0}\u{09bf}\u{0982}");
    m.insert("Bar", "\u{09ac}\u{09be}\u{09b0}");
    m.insert("Pie", "\u{09aa}\u{09be}\u{0987}");
    m.insert("Dashboard layout", "\u{09a1}\u{09cd}\u{09af}\u{09be}\u{09b6}\u{09ac}\u{09cb}\u{09b0}\u{09cd}\u{09a1} \u{09b2}\u{09c7}\u{0986}\u{0989}\u{099f}");
    m.insert(
        "Minimal",
        "\u{09a8}\u{09cd}\u{09af}\u{09c2}\u{09a8}\u{09a4}\u{09ae}",
    );
    m.insert("Standard", "\u{09b8}\u{09cd}\u{099f}\u{09cd}\u{09af}\u{09be}\u{09a8}\u{09cd}\u{09a1}\u{09be}\u{09b0}\u{09cd}\u{09a1}");
    m.insert(
        "Detailed",
        "\u{09ac}\u{09bf}\u{09b8}\u{09cd}\u{09a4}\u{09be}\u{09b0}\u{09bf}\u{09a4}",
    );
    m.insert(
        "Hide Extra Usage",
        "Extra Usage \u{09b2}\u{09c1}\u{0995}\u{09be}\u{09a8}",
    );
    m
}
