use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert(
        "5-hour session",
        "\u{062c}\u{0644}\u{0633}\u{0647} \u{06f5} \u{0633}\u{0627}\u{0639}\u{062a}\u{0647}",
    );
    m.insert(
        "Weekly (7-day)",
        "\u{0647}\u{0641}\u{062a}\u{06af}\u{06cc} (\u{06f7} \u{0631}\u{0648}\u{0632}\u{0647})",
    );
    m.insert(
        "Opus (7-day)",
        "Opus (\u{06f7} \u{0631}\u{0648}\u{0632}\u{0647})",
    );
    m.insert(
        "Sonnet (7-day)",
        "Sonnet (\u{06f7} \u{0631}\u{0648}\u{0632}\u{0647})",
    );
    m.insert(
        "OAuth Apps (7-day)",
        "OAuth Apps (\u{06f7} \u{0631}\u{0648}\u{0632}\u{0647})",
    );
    m.insert(
        "resets in",
        "\u{0628}\u{0627}\u{0632}\u{0646}\u{0634}\u{0627}\u{0646}\u{06cc} \u{062f}\u{0631}",
    );
    m.insert("Plan", "\u{067e}\u{0644}\u{0646}");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code \u{0634}\u{0646}\u{0627}\u{0633}\u{0627}\u{06cc}\u{06cc} \u{0646}\u{0634}\u{062f}");
    m.insert("credentials_not_found", "\u{0627}\u{0639}\u{062a}\u{0628}\u{0627}\u{0631}\u{0646}\u{0627}\u{0645}\u{0647} \u{06cc}\u{0627}\u{0641}\u{062a} \u{0646}\u{0634}\u{062f}");
    m.insert(
        "connection_error",
        "\u{062e}\u{0637}\u{0627}\u{06cc} \u{0627}\u{062a}\u{0635}\u{0627}\u{0644}",
    );
    m.insert("token_expired", "\u{062a}\u{0648}\u{06a9}\u{0646} \u{0645}\u{0646}\u{0642}\u{0636}\u{06cc} \u{0634}\u{062f}\u{0647}");
    m.insert(
        "token_expired_desc",
        "\u{062a}\u{0648}\u{06a9}\u{0646} OAuth \u{0634}\u{0645}\u{0627} \u{0645}\u{0646}\u{0642}\u{0636}\u{06cc} \u{0634}\u{062f}\u{0647} \u{0627}\u{0633}\u{062a}. \u{062f}\u{0631} \u{062a}\u{0631}\u{0645}\u{06cc}\u{0646}\u{0627}\u{0644} `claude login` \u{0631}\u{0627} \u{0627}\u{062c}\u{0631}\u{0627} \u{06a9}\u{0646}\u{06cc}\u{062f}.",
    );
    m.insert(
        "rate_limited",
        "\u{0645}\u{062d}\u{062f}\u{0648}\u{062f}\u{06cc}\u{062a} \u{0646}\u{0631}\u{062e}",
    );
    m.insert(
        "server_error",
        "\u{062e}\u{0637}\u{0627}\u{06cc} \u{0633}\u{0631}\u{0648}\u{0631}",
    );
    m.insert(
        "server_error_desc",
        "Anthropic API \u{0645}\u{0648}\u{0642}\u{062a}\u{0627}\u{064b} \u{062f}\u{0631} \u{062f}\u{0633}\u{062a}\u{0631}\u{0633} \u{0646}\u{06cc}\u{0633}\u{062a}. \u{0628}\u{0647} \u{0637}\u{0648}\u{0631} \u{062e}\u{0648}\u{062f}\u{06a9}\u{0627}\u{0631} \u{062a}\u{0644}\u{0627}\u{0634} \u{0645}\u{062c}\u{062f}\u{062f} \u{0645}\u{06cc}\u{200c}\u{06a9}\u{0646}\u{062f}.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code \u{0646}\u{0635}\u{0628} \u{0634}\u{062f}\u{0647} \u{0627}\u{0645}\u{0627} \u{0648}\u{0627}\u{0631}\u{062f} \u{0646}\u{0634}\u{062f}\u{0647}. `claude login` \u{0631}\u{0627} \u{0627}\u{062c}\u{0631}\u{0627} \u{06a9}\u{0646}\u{06cc}\u{062f}.",
    );
    m.insert(
        "install_claude_desc",
        "Claude Code \u{0631}\u{0627} \u{0646}\u{0635}\u{0628} \u{06a9}\u{0646}\u{06cc}\u{062f} \u{0648} `claude login` \u{0631}\u{0627} \u{0627}\u{062c}\u{0631}\u{0627} \u{06a9}\u{0646}\u{06cc}\u{062f}.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "\u{0646}\u{0635}\u{0628} Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI API \u{0628}\u{0631}\u{0627}\u{06cc} \u{0631}\u{062f}\u{06cc}\u{0627}\u{0628}\u{06cc} \u{0645}\u{0635}\u{0631}\u{0641} \u{0627}\u{0634}\u{062a}\u{0631}\u{0627}\u{06a9} ChatGPT \u{0627}\u{0631}\u{0627}\u{0626}\u{0647} \u{0646}\u{0645}\u{06cc}\u{200c}\u{062f}\u{0647}\u{062f}.",
    );
    m.insert("Check your usage manually:", "\u{0645}\u{0635}\u{0631}\u{0641} \u{062e}\u{0648}\u{062f} \u{0631}\u{0627} \u{062f}\u{0633}\u{062a}\u{06cc} \u{0628}\u{0631}\u{0631}\u{0633}\u{06cc} \u{06a9}\u{0646}\u{06cc}\u{062f}:");
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "\u{0628}\u{0627}\u{0632} \u{06a9}\u{0631}\u{062f}\u{0646} ChatGPT Usage \u{2192}",
    );
    m.insert("Refresh Now", "\u{0628}\u{0627}\u{0632}\u{062e}\u{0648}\u{0627}\u{0646}\u{06cc} \u{0627}\u{06a9}\u{0646}\u{0648}\u{0646}");
    m.insert("Open Dashboard", "\u{0628}\u{0627}\u{0632} \u{06a9}\u{0631}\u{062f}\u{0646} \u{062f}\u{0627}\u{0634}\u{0628}\u{0648}\u{0631}\u{062f}");
    m.insert("Export History (CSV)", "\u{062e}\u{0631}\u{0648}\u{062c}\u{06cc} \u{062a}\u{0627}\u{0631}\u{06cc}\u{062e}\u{0686}\u{0647} (CSV)");
    m.insert(
        "Settings",
        "\u{062a}\u{0646}\u{0638}\u{06cc}\u{0645}\u{0627}\u{062a}",
    );
    m.insert("Start with Windows", "\u{0634}\u{0631}\u{0648}\u{0639} \u{0628}\u{0627} \u{0648}\u{06cc}\u{0646}\u{062f}\u{0648}\u{0632}");
    m.insert("About", "\u{062f}\u{0631}\u{0628}\u{0627}\u{0631}\u{0647}");
    m.insert("Exit", "\u{062e}\u{0631}\u{0648}\u{062c}");
    m.insert("Last updated:", "\u{0622}\u{062e}\u{0631}\u{06cc}\u{0646} \u{0628}\u{0647}\u{200c}\u{0631}\u{0648}\u{0632}\u{0631}\u{0633}\u{0627}\u{0646}\u{06cc}:");
    m.insert(
        "Refresh",
        "\u{0628}\u{0627}\u{0632}\u{062e}\u{0648}\u{0627}\u{0646}\u{06cc}",
    );
    m.insert("Status", "\u{0648}\u{0636}\u{0639}\u{06cc}\u{062a}");
    m.insert(
        "Usage Alert",
        "\u{0647}\u{0634}\u{062f}\u{0627}\u{0631} \u{0645}\u{0635}\u{0631}\u{0641}",
    );
    m.insert(
        "Usage Critical",
        "\u{0645}\u{0635}\u{0631}\u{0641} \u{0628}\u{062d}\u{0631}\u{0627}\u{0646}\u{06cc}",
    );
    m.insert(
        "Running in system tray. Click the icon for details.",
        "\u{062f}\u{0631} \u{0633}\u{06cc}\u{0646}\u{06cc} \u{0633}\u{06cc}\u{0633}\u{062a}\u{0645} \u{0627}\u{062c}\u{0631}\u{0627} \u{0645}\u{06cc}\u{200c}\u{0634}\u{0648}\u{062f}. \u{0628}\u{0631}\u{0627}\u{06cc} \u{062c}\u{0632}\u{0626}\u{06cc}\u{0627}\u{062a} \u{0631}\u{0648}\u{06cc} \u{0622}\u{06cc}\u{06a9}\u{0648}\u{0646} \u{06a9}\u{0644}\u{06cc}\u{06a9} \u{06a9}\u{0646}\u{06cc}\u{062f}.",
    );
    m.insert(
        "Compact mode",
        "\u{062d}\u{0627}\u{0644}\u{062a} \u{0641}\u{0634}\u{0631}\u{062f}\u{0647}",
    );
    m.insert("Theme", "\u{067e}\u{0648}\u{0633}\u{062a}\u{0647}");
    m.insert("Language", "\u{0632}\u{0628}\u{0627}\u{0646}");
    m.insert(
        "Notifications",
        "\u{0627}\u{0639}\u{0644}\u{0627}\u{0646}\u{200c}\u{0647}\u{0627}",
    );
    m.insert("Dark", "\u{062a}\u{06cc}\u{0631}\u{0647}");
    m.insert("Light", "\u{0631}\u{0648}\u{0634}\u{0646}");
    m.insert("Auto", "\u{062e}\u{0648}\u{062f}\u{06a9}\u{0627}\u{0631}");
    m.insert(
        "Show ChatGPT section",
        "\u{0646}\u{0645}\u{0627}\u{06cc}\u{0634} \u{0628}\u{062e}\u{0634} ChatGPT",
    );
    m.insert("Enabled", "\u{0641}\u{0639}\u{0627}\u{0644}");
    m.insert("Sound", "\u{0635}\u{062f}\u{0627}");
    m.insert(
        "Thresholds",
        "\u{0622}\u{0633}\u{062a}\u{0627}\u{0646}\u{0647}\u{200c}\u{0647}\u{0627}",
    );
    m.insert(
        "Polling interval",
        "\u{0641}\u{0627}\u{0635}\u{0644}\u{0647} \u{0628}\u{0631}\u{0631}\u{0633}\u{06cc}",
    );
    m.insert("seconds", "\u{062b}\u{0627}\u{0646}\u{06cc}\u{0647}");
    m.insert("Startup", "\u{0634}\u{0631}\u{0648}\u{0639}");
    m.insert("General", "\u{0639}\u{0645}\u{0648}\u{0645}\u{06cc}");
    m.insert(
        "Back",
        "\u{2190} \u{0628}\u{0627}\u{0632}\u{06af}\u{0634}\u{062a}",
    );
    m.insert(
        "Open Claude.ai \u{2192}",
        "\u{0628}\u{0627}\u{0632} \u{06a9}\u{0631}\u{062f}\u{0646} Claude.ai \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert(
        "Usage History",
        "\u{062a}\u{0627}\u{0631}\u{06cc}\u{062e}\u{0686}\u{0647} \u{0645}\u{0635}\u{0631}\u{0641}",
    );
    m.insert("Usage History (24h)", "\u{062a}\u{0627}\u{0631}\u{06cc}\u{062e}\u{0686}\u{0647} \u{0645}\u{0635}\u{0631}\u{0641} (24h)");
    m.insert("Auto (English)", "Auto (English)");
    m.insert("at", "\u{062f}\u{0631}");
    m.insert(
        "Resets in",
        "\u{0628}\u{0627}\u{0632}\u{0646}\u{0634}\u{0627}\u{0646}\u{06cc} \u{062f}\u{0631}",
    );
    m.insert("Tray icon colors:", "\u{0631}\u{0646}\u{06af}\u{200c}\u{0647}\u{0627}\u{06cc} \u{0622}\u{06cc}\u{06a9}\u{0648}\u{0646}:");
    m.insert("< 50% usage", "< 50% \u{0645}\u{0635}\u{0631}\u{0641}");
    m.insert(
        "50-79% usage",
        "50\u{2013}79% \u{0645}\u{0635}\u{0631}\u{0641}",
    );
    m.insert(
        ">= 80% usage",
        "\u{2265} 80% \u{0645}\u{0635}\u{0631}\u{0641}",
    );
    m.insert(
        "No data",
        "\u{0628}\u{062f}\u{0648}\u{0646} \u{062f}\u{0627}\u{062f}\u{0647}",
    );
    m.insert(
        "exceeded",
        "\u{0641}\u{0631}\u{0627}\u{062a}\u{0631} \u{0631}\u{0641}\u{062a}\u{0647}",
    );
    m.insert(
        "Show widget",
        "\u{0646}\u{0645}\u{0627}\u{06cc}\u{0634} \u{0648}\u{06cc}\u{062c}\u{062a}",
    );
    m.insert("Check for updates", "\u{0628}\u{0631}\u{0631}\u{0633}\u{06cc} \u{0628}\u{0647}\u{200c}\u{0631}\u{0648}\u{0632}\u{0631}\u{0633}\u{0627}\u{0646}\u{06cc}");
    m.insert("Accessibility patterns", "\u{0627}\u{0644}\u{06af}\u{0648}\u{0647}\u{0627}\u{06cc} \u{062f}\u{0633}\u{062a}\u{0631}\u{0633}\u{06cc}\u{200c}\u{067e}\u{0630}\u{06cc}\u{0631}\u{06cc}");
    m.insert("Update available", "\u{0628}\u{0647}\u{200c}\u{0631}\u{0648}\u{0632}\u{0631}\u{0633}\u{0627}\u{0646}\u{06cc} \u{0645}\u{0648}\u{062c}\u{0648}\u{062f}");
    m.insert(
        "is available. Click to download.",
        "\u{0645}\u{0648}\u{062c}\u{0648}\u{062f} \u{0627}\u{0633}\u{062a}. \u{0628}\u{0631}\u{0627}\u{06cc} \u{062f}\u{0627}\u{0646}\u{0644}\u{0648}\u{062f} \u{06a9}\u{0644}\u{06cc}\u{06a9} \u{06a9}\u{0646}\u{06cc}\u{062f}.",
    );
    m.insert(
        "Icon style",
        "\u{0633}\u{0628}\u{06a9} \u{0622}\u{06cc}\u{06a9}\u{0648}\u{0646}",
    );
    m.insert("Number", "\u{0639}\u{062f}\u{062f}");
    m.insert("Ring", "\u{062d}\u{0644}\u{0642}\u{0647}");
    m.insert("Bar", "\u{0646}\u{0648}\u{0627}\u{0631}");
    m.insert(
        "Pie",
        "\u{062f}\u{0627}\u{06cc}\u{0631}\u{0647}\u{200c}\u{0627}\u{06cc}",
    );
    m.insert("Dashboard layout", "\u{0686}\u{06cc}\u{062f}\u{0645}\u{0627}\u{0646} \u{062f}\u{0627}\u{0634}\u{0628}\u{0648}\u{0631}\u{062f}");
    m.insert("Minimal", "\u{062d}\u{062f}\u{0627}\u{0642}\u{0644}");
    m.insert(
        "Standard",
        "\u{0627}\u{0633}\u{062a}\u{0627}\u{0646}\u{062f}\u{0627}\u{0631}\u{062f}",
    );
    m.insert(
        "Detailed",
        "\u{062c}\u{0632}\u{0626}\u{06cc}\u{0627}\u{062a}",
    );
    m.insert(
        "Hide Extra Usage",
        "\u{067e}\u{0646}\u{0647}\u{0627}\u{0646} \u{06a9}\u{0631}\u{062f}\u{0646} Extra Usage",
    );
    m
}
