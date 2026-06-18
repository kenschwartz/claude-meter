use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "\u{03a3}\u{03c5}\u{03bd}\u{03b5}\u{03b4}\u{03c1}\u{03af}\u{03b1} 5 \u{03c9}\u{03c1}\u{03ce}\u{03bd}");
    m.insert("Weekly (7-day)", "\u{0395}\u{03b2}\u{03b4}\u{03bf}\u{03bc}\u{03b1}\u{03b4}\u{03b9}\u{03b1}\u{03af}\u{03b1} (7 \u{03b7}\u{03bc}\u{03ad}\u{03c1}\u{03b5}\u{03c2})");
    m.insert("Opus (7-day)", "Opus (7 \u{03b7}\u{03bc}.)");
    m.insert("Sonnet (7-day)", "Sonnet (7 \u{03b7}\u{03bc}.)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 \u{03b7}\u{03bc}.)");
    m.insert(
        "resets in",
        "\u{03b5}\u{03c0}\u{03b1}\u{03bd}\u{03b1}\u{03c6}\u{03bf}\u{03c1}\u{03ac} \u{03c3}\u{03b5}",
    );
    m.insert("Plan", "\u{03a0}\u{03bb}\u{03ac}\u{03bd}\u{03bf}");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "\u{0394}\u{03b5}\u{03bd} \u{03b5}\u{03bd}\u{03c4}\u{03bf}\u{03c0}\u{03af}\u{03c3}\u{03c4}\u{03b7}\u{03ba}\u{03b5} \u{03c4}\u{03bf} Claude Code");
    m.insert("credentials_not_found", "\u{0394}\u{03b5}\u{03bd} \u{03b2}\u{03c1}\u{03ad}\u{03b8}\u{03b7}\u{03ba}\u{03b1}\u{03bd} \u{03b4}\u{03b9}\u{03b1}\u{03c0}\u{03b9}\u{03c3}\u{03c4}\u{03b5}\u{03c5}\u{03c4}\u{03ae}\u{03c1}\u{03b9}\u{03b1}");
    m.insert("connection_error", "\u{03a3}\u{03c6}\u{03ac}\u{03bb}\u{03bc}\u{03b1} \u{03c3}\u{03cd}\u{03bd}\u{03b4}\u{03b5}\u{03c3}\u{03b7}\u{03c2}");
    m.insert(
        "token_expired",
        "\u{03a4}\u{03bf} token \u{03ad}\u{03bb}\u{03b7}\u{03be}\u{03b5}",
    );
    m.insert(
        "token_expired_desc",
        "\u{03a4}\u{03bf} OAuth token \u{03ad}\u{03bb}\u{03b7}\u{03be}\u{03b5}. \u{0395}\u{03ba}\u{03c4}\u{03b5}\u{03bb}\u{03ad}\u{03c3}\u{03c4}\u{03b5} `claude login` \u{03c3}\u{03c4}\u{03bf} \u{03c4}\u{03b5}\u{03c1}\u{03bc}\u{03b1}\u{03c4}\u{03b9}\u{03ba}\u{03cc}.",
    );
    m.insert("rate_limited", "\u{03a0}\u{03b5}\u{03c1}\u{03b9}\u{03bf}\u{03c1}\u{03b9}\u{03c3}\u{03bc}\u{03cc}\u{03c2} \u{03c1}\u{03c5}\u{03b8}\u{03bc}\u{03bf}\u{03cd}");
    m.insert("server_error", "\u{03a3}\u{03c6}\u{03ac}\u{03bb}\u{03bc}\u{03b1} \u{03b4}\u{03b9}\u{03b1}\u{03ba}\u{03bf}\u{03bc}\u{03b9}\u{03c3}\u{03c4}\u{03ae}");
    m.insert(
        "server_error_desc",
        "\u{03a4}\u{03bf} Anthropic API \u{03b5}\u{03af}\u{03bd}\u{03b1}\u{03b9} \u{03c0}\u{03c1}\u{03bf}\u{03c3}\u{03c9}\u{03c1}\u{03b9}\u{03bd}\u{03ac} \u{03bc}\u{03b7} \u{03b4}\u{03b9}\u{03b1}\u{03b8}\u{03ad}\u{03c3}\u{03b9}\u{03bc}\u{03bf}.",
    );
    m.insert(
        "run_claude_login_desc",
        "\u{03a4}\u{03bf} Claude Code \u{03b5}\u{03af}\u{03bd}\u{03b1}\u{03b9} \u{03b5}\u{03b3}\u{03ba}\u{03b1}\u{03c4}\u{03b5}\u{03c3}\u{03c4}\u{03b7}\u{03bc}\u{03ad}\u{03bd}\u{03bf}. \u{0395}\u{03ba}\u{03c4}\u{03b5}\u{03bb}\u{03ad}\u{03c3}\u{03c4}\u{03b5} `claude login`.",
    );
    m.insert(
        "install_claude_desc",
        "\u{0395}\u{03b3}\u{03ba}\u{03b1}\u{03c4}\u{03b1}\u{03c3}\u{03c4}\u{03ae}\u{03c3}\u{03c4}\u{03b5} \u{03c4}\u{03bf} Claude Code \u{03ba}\u{03b1}\u{03b9} \u{03b5}\u{03ba}\u{03c4}\u{03b5}\u{03bb}\u{03ad}\u{03c3}\u{03c4}\u{03b5} `claude login`.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "\u{0395}\u{03b3}\u{03ba}\u{03b1}\u{03c4}\u{03ac}\u{03c3}\u{03c4}\u{03b1}\u{03c3}\u{03b7} Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "\u{0397} OpenAI \u{03b4}\u{03b5}\u{03bd} \u{03c0}\u{03b1}\u{03c1}\u{03ad}\u{03c7}\u{03b5}\u{03b9} API \u{03b3}\u{03b9}\u{03b1} \u{03c0}\u{03b1}\u{03c1}\u{03b1}\u{03ba}\u{03bf}\u{03bb}\u{03bf}\u{03cd}\u{03b8}\u{03b7}\u{03c3}\u{03b7} \u{03c7}\u{03c1}\u{03ae}\u{03c3}\u{03b7}\u{03c2}.",
    );
    m.insert("Check your usage manually:", "\u{0395}\u{03bb}\u{03ad}\u{03b3}\u{03be}\u{03c4}\u{03b5} \u{03c7}\u{03b5}\u{03b9}\u{03c1}\u{03bf}\u{03ba}\u{03af}\u{03bd}\u{03b7}\u{03c4}\u{03b1}:");
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "\u{0386}\u{03bd}\u{03bf}\u{03b9}\u{03b3}\u{03bc}\u{03b1} ChatGPT Usage \u{2192}",
    );
    m.insert("Refresh Now", "\u{0391}\u{03bd}\u{03b1}\u{03bd}\u{03ad}\u{03c9}\u{03c3}\u{03b7} \u{03c4}\u{03ce}\u{03c1}\u{03b1}");
    m.insert("Open Dashboard", "\u{0386}\u{03bd}\u{03bf}\u{03b9}\u{03b3}\u{03bc}\u{03b1} \u{03c0}\u{03af}\u{03bd}\u{03b1}\u{03ba}\u{03b1}");
    m.insert("Export History (CSV)", "\u{0395}\u{03be}\u{03b1}\u{03b3}\u{03c9}\u{03b3}\u{03ae} \u{03b9}\u{03c3}\u{03c4}\u{03bf}\u{03c1}\u{03b9}\u{03ba}\u{03bf}\u{03cd} (CSV)");
    m.insert(
        "Settings",
        "\u{03a1}\u{03c5}\u{03b8}\u{03bc}\u{03af}\u{03c3}\u{03b5}\u{03b9}\u{03c2}",
    );
    m.insert(
        "Start with Windows",
        "\u{0395}\u{03ba}\u{03ba}\u{03af}\u{03bd}\u{03b7}\u{03c3}\u{03b7} \u{03bc}\u{03b5} Windows",
    );
    m.insert(
        "About",
        "\u{03a3}\u{03c7}\u{03b5}\u{03c4}\u{03b9}\u{03ba}\u{03ac}",
    );
    m.insert("Exit", "\u{0388}\u{03be}\u{03bf}\u{03b4}\u{03bf}\u{03c2}");
    m.insert("Last updated:", "\u{03a4}\u{03b5}\u{03bb}\u{03b5}\u{03c5}\u{03c4}\u{03b1}\u{03af}\u{03b1} \u{03b5}\u{03bd}\u{03b7}\u{03bc}\u{03ad}\u{03c1}\u{03c9}\u{03c3}\u{03b7}:");
    m.insert(
        "Refresh",
        "\u{0391}\u{03bd}\u{03b1}\u{03bd}\u{03ad}\u{03c9}\u{03c3}\u{03b7}",
    );
    m.insert(
        "Status",
        "\u{039a}\u{03b1}\u{03c4}\u{03ac}\u{03c3}\u{03c4}\u{03b1}\u{03c3}\u{03b7}",
    );
    m.insert("Usage Alert", "\u{0395}\u{03b9}\u{03b4}\u{03bf}\u{03c0}\u{03bf}\u{03af}\u{03b7}\u{03c3}\u{03b7} \u{03c7}\u{03c1}\u{03ae}\u{03c3}\u{03b7}\u{03c2}");
    m.insert("Usage Critical", "\u{039a}\u{03c1}\u{03af}\u{03c3}\u{03b9}\u{03bc}\u{03b7} \u{03c7}\u{03c1}\u{03ae}\u{03c3}\u{03b7}");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "\u{0395}\u{03ba}\u{03c4}\u{03b5}\u{03bb}\u{03b5}\u{03af}\u{03c4}\u{03b1}\u{03b9} \u{03c3}\u{03c4}\u{03bf} system tray. \u{039a}\u{03ac}\u{03bd}\u{03c4}\u{03b5} \u{03ba}\u{03bb}\u{03b9}\u{03ba} \u{03c3}\u{03c4}\u{03bf} \u{03b5}\u{03b9}\u{03ba}\u{03bf}\u{03bd}\u{03af}\u{03b4}\u{03b9}\u{03bf}.",
    );
    m.insert("Compact mode", "\u{03a3}\u{03c5}\u{03bc}\u{03c0}\u{03b1}\u{03b3}\u{03ae}\u{03c2} \u{03bb}\u{03b5}\u{03b9}\u{03c4}\u{03bf}\u{03c5}\u{03c1}\u{03b3}\u{03af}\u{03b1}");
    m.insert("Theme", "\u{0398}\u{03ad}\u{03bc}\u{03b1}");
    m.insert(
        "Language",
        "\u{0393}\u{03bb}\u{03ce}\u{03c3}\u{03c3}\u{03b1}",
    );
    m.insert("Notifications", "\u{0395}\u{03b9}\u{03b4}\u{03bf}\u{03c0}\u{03bf}\u{03b9}\u{03ae}\u{03c3}\u{03b5}\u{03b9}\u{03c2}");
    m.insert(
        "Dark",
        "\u{03a3}\u{03ba}\u{03bf}\u{03c4}\u{03b5}\u{03b9}\u{03bd}\u{03cc}",
    );
    m.insert(
        "Light",
        "\u{03a6}\u{03c9}\u{03c4}\u{03b5}\u{03b9}\u{03bd}\u{03cc}",
    );
    m.insert(
        "Auto",
        "\u{0391}\u{03c5}\u{03c4}\u{03cc}\u{03bc}\u{03b1}\u{03c4}\u{03bf}",
    );
    m.insert(
        "Show ChatGPT section",
        "\u{0395}\u{03bc}\u{03c6}\u{03ac}\u{03bd}\u{03b9}\u{03c3}\u{03b7} ChatGPT",
    );
    m.insert(
        "Enabled",
        "\u{0395}\u{03bd}\u{03b5}\u{03c1}\u{03b3}\u{03cc}",
    );
    m.insert("Sound", "\u{0389}\u{03c7}\u{03bf}\u{03c2}");
    m.insert(
        "Thresholds",
        "\u{039a}\u{03b1}\u{03c4}\u{03ce}\u{03c6}\u{03bb}\u{03b9}\u{03b1}",
    );
    m.insert("Polling interval", "\u{0394}\u{03b9}\u{03ac}\u{03c3}\u{03c4}\u{03b7}\u{03bc}\u{03b1} \u{03b5}\u{03bb}\u{03ad}\u{03b3}\u{03c7}\u{03bf}\u{03c5}");
    m.insert("seconds", "\u{03b4}\u{03b5}\u{03c5}\u{03c4}\u{03b5}\u{03c1}\u{03cc}\u{03bb}\u{03b5}\u{03c0}\u{03c4}\u{03b1}");
    m.insert(
        "Startup",
        "\u{0395}\u{03ba}\u{03ba}\u{03af}\u{03bd}\u{03b7}\u{03c3}\u{03b7}",
    );
    m.insert(
        "General",
        "\u{0393}\u{03b5}\u{03bd}\u{03b9}\u{03ba}\u{03ac}",
    );
    m.insert("Back", "\u{2190} \u{03a0}\u{03af}\u{03c3}\u{03c9}");
    m.insert(
        "Open Claude.ai \u{2192}",
        "\u{0386}\u{03bd}\u{03bf}\u{03b9}\u{03b3}\u{03bc}\u{03b1} Claude.ai \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert(
        "Usage History",
        "\u{0399}\u{03c3}\u{03c4}\u{03bf}\u{03c1}\u{03b9}\u{03ba}\u{03cc}",
    );
    m.insert(
        "Usage History (24h)",
        "\u{0399}\u{03c3}\u{03c4}\u{03bf}\u{03c1}\u{03b9}\u{03ba}\u{03cc} (24\u{03ce})",
    );
    m.insert("Auto (English)", "Auto (English)");
    m.insert("at", "\u{03c3}\u{03c4}\u{03b9}\u{03c2}");
    m.insert(
        "Resets in",
        "\u{0395}\u{03c0}\u{03b1}\u{03bd}\u{03b1}\u{03c6}\u{03bf}\u{03c1}\u{03ac} \u{03c3}\u{03b5}",
    );
    m.insert("Tray icon colors:", "\u{03a7}\u{03c1}\u{03ce}\u{03bc}\u{03b1}\u{03c4}\u{03b1} \u{03b5}\u{03b9}\u{03ba}\u{03bf}\u{03bd}\u{03b9}\u{03b4}\u{03af}\u{03bf}\u{03c5}:");
    m.insert("< 50% usage", "< 50%");
    m.insert("50-79% usage", "50\u{2013}79%");
    m.insert(">= 80% usage", "\u{2265} 80%");
    m.insert("No data", "\u{03a7}\u{03c9}\u{03c1}\u{03af}\u{03c2} \u{03b4}\u{03b5}\u{03b4}\u{03bf}\u{03bc}\u{03ad}\u{03bd}\u{03b1}");
    m.insert(
        "exceeded",
        "\u{03c5}\u{03c0}\u{03b5}\u{03c1}\u{03b2}\u{03bf}\u{03bb}\u{03ae}",
    );
    m.insert(
        "Show widget",
        "\u{0395}\u{03bc}\u{03c6}\u{03ac}\u{03bd}\u{03b9}\u{03c3}\u{03b7} widget",
    );
    m.insert("Check for updates", "\u{0388}\u{03bb}\u{03b5}\u{03b3}\u{03c7}\u{03bf}\u{03c2} \u{03b5}\u{03bd}\u{03b7}\u{03bc}\u{03b5}\u{03c1}\u{03ce}\u{03c3}\u{03b5}\u{03c9}\u{03bd}");
    m.insert("Accessibility patterns", "\u{039c}\u{03bf}\u{03c4}\u{03af}\u{03b2}\u{03b1} \u{03c0}\u{03c1}\u{03bf}\u{03c3}\u{03b2}\u{03b1}\u{03c3}\u{03b9}\u{03bc}\u{03cc}\u{03c4}\u{03b7}\u{03c4}\u{03b1}\u{03c2}");
    m.insert("Update available", "\u{0394}\u{03b9}\u{03b1}\u{03b8}\u{03ad}\u{03c3}\u{03b9}\u{03bc}\u{03b7} \u{03b5}\u{03bd}\u{03b7}\u{03bc}\u{03ad}\u{03c1}\u{03c9}\u{03c3}\u{03b7}");
    m.insert(
        "is available. Click to download.",
        "\u{03b5}\u{03af}\u{03bd}\u{03b1}\u{03b9} \u{03b4}\u{03b9}\u{03b1}\u{03b8}\u{03ad}\u{03c3}\u{03b9}\u{03bc}\u{03b7}. \u{039a}\u{03ac}\u{03bd}\u{03c4}\u{03b5} \u{03ba}\u{03bb}\u{03b9}\u{03ba} \u{03b3}\u{03b9}\u{03b1} \u{03bb}\u{03ae}\u{03c8}\u{03b7}.",
    );
    m.insert("Icon style", "\u{03a3}\u{03c4}\u{03c5}\u{03bb} \u{03b5}\u{03b9}\u{03ba}\u{03bf}\u{03bd}\u{03b9}\u{03b4}\u{03af}\u{03bf}\u{03c5}");
    m.insert(
        "Number",
        "\u{0391}\u{03c1}\u{03b9}\u{03b8}\u{03bc}\u{03cc}\u{03c2}",
    );
    m.insert(
        "Ring",
        "\u{0394}\u{03b1}\u{03ba}\u{03c4}\u{03cd}\u{03bb}\u{03b9}\u{03bf}\u{03c2}",
    );
    m.insert("Bar", "\u{039c}\u{03c0}\u{03ac}\u{03c1}\u{03b1}");
    m.insert("Pie", "\u{03a0}\u{03af}\u{03c4}\u{03b1}");
    m.insert(
        "Dashboard layout",
        "\u{0394}\u{03b9}\u{03ac}\u{03c4}\u{03b1}\u{03be}\u{03b7}",
    );
    m.insert(
        "Minimal",
        "\u{0395}\u{03bb}\u{03ac}\u{03c7}\u{03b9}\u{03c3}\u{03c4}\u{03bf}",
    );
    m.insert(
        "Standard",
        "\u{039a}\u{03b1}\u{03bd}\u{03bf}\u{03bd}\u{03b9}\u{03ba}\u{03cc}",
    );
    m.insert(
        "Detailed",
        "\u{039b}\u{03b5}\u{03c0}\u{03c4}\u{03bf}\u{03bc}\u{03b5}\u{03c1}\u{03ad}\u{03c2}",
    );
    m.insert(
        "Hide Extra Usage",
        "\u{0391}\u{03c0}\u{03cc}\u{03ba}\u{03c1}\u{03c5}\u{03c8}\u{03b7} Extra Usage",
    );
    m
}
