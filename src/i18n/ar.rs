use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert(
        "5-hour session",
        "\u{062c}\u{0644}\u{0633}\u{0629} 5 \u{0633}\u{0627}\u{0639}\u{0627}\u{062a}",
    );
    m.insert(
        "Weekly (7-day)",
        "\u{0623}\u{0633}\u{0628}\u{0648}\u{0639}\u{064a} (7 \u{0623}\u{064a}\u{0627}\u{0645})",
    );
    m.insert("Opus (7-day)", "Opus (7 \u{0623}\u{064a}\u{0627}\u{0645})");
    m.insert(
        "Sonnet (7-day)",
        "Sonnet (7 \u{0623}\u{064a}\u{0627}\u{0645})",
    );
    m.insert(
        "OAuth Apps (7-day)",
        "OAuth Apps (7 \u{0623}\u{064a}\u{0627}\u{0645})",
    );
    m.insert("resets in", "\u{064a}\u{0639}\u{0627}\u{062f} \u{0636}\u{0628}\u{0637}\u{0647} \u{062e}\u{0644}\u{0627}\u{0644}");
    m.insert("Plan", "\u{0627}\u{0644}\u{062e}\u{0637}\u{0629}");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "\u{0644}\u{0645} \u{064a}\u{062a}\u{0645} \u{0627}\u{0643}\u{062a}\u{0634}\u{0627}\u{0641} Claude Code");
    m.insert("credentials_not_found", "\u{0644}\u{0645} \u{064a}\u{062a}\u{0645} \u{0627}\u{0644}\u{0639}\u{062b}\u{0648}\u{0631} \u{0639}\u{0644}\u{0649} \u{0628}\u{064a}\u{0627}\u{0646}\u{0627}\u{062a} \u{0627}\u{0644}\u{0627}\u{0639}\u{062a}\u{0645}\u{0627}\u{062f}");
    m.insert("connection_error", "\u{062e}\u{0637}\u{0623} \u{0641}\u{064a} \u{0627}\u{0644}\u{0627}\u{062a}\u{0635}\u{0627}\u{0644}");
    m.insert("token_expired", "انتهت صلاحية الرمز");
    m.insert(
        "token_expired_desc",
        "انتهت صلاحية رمز OAuth الخاص بك. قم بتشغيل `claude login` في الطرفية لتجديده.",
    );
    m.insert("rate_limited", "تم تجاوز حد الطلبات");
    m.insert("server_error", "خطأ في الخادم");
    m.insert(
        "server_error_desc",
        "واجهة Anthropic API غير متاحة مؤقتاً. ستتم إعادة المحاولة تلقائياً.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code \u{0645}\u{062b}\u{0628}\u{062a} \u{0648}\u{0644}\u{0643}\u{0646} \u{0644}\u{0645} \u{064a}\u{062a}\u{0645} \u{062a}\u{0633}\u{062c}\u{064a}\u{0644} \u{0627}\u{0644}\u{062f}\u{062e}\u{0648}\u{0644}. \u{0634}\u{063a}\u{0651}\u{0644} `claude login` \u{0641}\u{064a} \u{0627}\u{0644}\u{0637}\u{0631}\u{0641}\u{064a}\u{0629} \u{0644}\u{0631}\u{0628}\u{0637} \u{062d}\u{0633}\u{0627}\u{0628}\u{0643}.",
    );
    m.insert(
        "install_claude_desc",
        "\u{062b}\u{0628}\u{0651}\u{062a} Claude Code \u{0648}\u{0634}\u{063a}\u{0651}\u{0644} `claude login` \u{0644}\u{062a}\u{0641}\u{0639}\u{064a}\u{0644} \u{062a}\u{062a}\u{0628}\u{0639} \u{0627}\u{0644}\u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645} \u{0627}\u{0644}\u{062a}\u{0644}\u{0642}\u{0627}\u{0626}\u{064a}.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "\u{062a}\u{062b}\u{0628}\u{064a}\u{062a} Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "\u{0644}\u{0627} \u{062a}\u{0648}\u{0641}\u{0631} OpenAI \u{0648}\u{0627}\u{062c}\u{0647}\u{0629} \u{0628}\u{0631}\u{0645}\u{062c}\u{0629} \u{0644}\u{062a}\u{062a}\u{0628}\u{0639} \u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645} \u{0627}\u{0634}\u{062a}\u{0631}\u{0627}\u{0643} ChatGPT.",
    );
    m.insert("Check your usage manually:", "\u{062a}\u{062d}\u{0642}\u{0642} \u{0645}\u{0646} \u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645}\u{0643} \u{064a}\u{062f}\u{0648}\u{064a}\u{064b}\u{0627}:");
    m.insert("Open ChatGPT Usage \u{2192}", "\u{0641}\u{062a}\u{062d} \u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645} ChatGPT \u{2192}");
    m.insert(
        "Refresh Now",
        "\u{062a}\u{062d}\u{062f}\u{064a}\u{062b} \u{0627}\u{0644}\u{0622}\u{0646}",
    );
    m.insert("Open Dashboard", "\u{0641}\u{062a}\u{062d} \u{0644}\u{0648}\u{062d}\u{0629} \u{0627}\u{0644}\u{0645}\u{0639}\u{0644}\u{0648}\u{0645}\u{0627}\u{062a}");
    m.insert(
        "Export History (CSV)",
        "\u{062a}\u{0635}\u{062f}\u{064a}\u{0631} \u{0627}\u{0644}\u{0633}\u{062c}\u{0644} (CSV)",
    );
    m.insert(
        "Settings",
        "\u{0627}\u{0644}\u{0625}\u{0639}\u{062f}\u{0627}\u{062f}\u{0627}\u{062a}",
    );
    m.insert(
        "Start with Windows",
        "\u{0628}\u{062f}\u{0621} \u{0645}\u{0639} Windows",
    );
    m.insert(
        "About",
        "\u{062d}\u{0648}\u{0644} \u{0627}\u{0644}\u{0628}\u{0631}\u{0646}\u{0627}\u{0645}\u{062c}",
    );
    m.insert("Exit", "\u{062e}\u{0631}\u{0648}\u{062c}");
    m.insert(
        "Last updated:",
        "\u{0622}\u{062e}\u{0631} \u{062a}\u{062d}\u{062f}\u{064a}\u{062b}:",
    );
    m.insert("Refresh", "\u{062a}\u{062d}\u{062f}\u{064a}\u{062b}");
    m.insert("Status", "\u{0627}\u{0644}\u{062d}\u{0627}\u{0644}\u{0629}");
    m.insert("Usage Alert", "\u{062a}\u{0646}\u{0628}\u{064a}\u{0647} \u{0627}\u{0644}\u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645}");
    m.insert(
        "Usage Critical",
        "\u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645} \u{062d}\u{0631}\u{062c}",
    );
    m.insert(
        "Running in system tray. Click the icon for details.",
        "\u{064a}\u{0639}\u{0645}\u{0644} \u{0641}\u{064a} \u{0639}\u{0644}\u{0628}\u{0629} \u{0627}\u{0644}\u{0646}\u{0638}\u{0627}\u{0645}. \u{0627}\u{0646}\u{0642}\u{0631} \u{0639}\u{0644}\u{0649} \u{0627}\u{0644}\u{0623}\u{064a}\u{0642}\u{0648}\u{0646}\u{0629} \u{0644}\u{0644}\u{062a}\u{0641}\u{0627}\u{0635}\u{064a}\u{0644}.",
    );
    m.insert(
        "Compact mode",
        "\u{0627}\u{0644}\u{0648}\u{0636}\u{0639} \u{0627}\u{0644}\u{0645}\u{062f}\u{0645}\u{062c}",
    );
    m.insert("Theme", "\u{0627}\u{0644}\u{0645}\u{0638}\u{0647}\u{0631}");
    m.insert("Language", "\u{0627}\u{0644}\u{0644}\u{063a}\u{0629}");
    m.insert(
        "Notifications",
        "\u{0627}\u{0644}\u{0625}\u{0634}\u{0639}\u{0627}\u{0631}\u{0627}\u{062a}",
    );
    m.insert("Dark", "\u{062f}\u{0627}\u{0643}\u{0646}");
    m.insert("Light", "\u{0641}\u{0627}\u{062a}\u{062d}");
    m.insert("Auto", "\u{062a}\u{0644}\u{0642}\u{0627}\u{0626}\u{064a}");
    m.insert(
        "Show ChatGPT section",
        "\u{0625}\u{0638}\u{0647}\u{0627}\u{0631} \u{0642}\u{0633}\u{0645} ChatGPT",
    );
    m.insert("Enabled", "\u{0645}\u{0641}\u{0639}\u{0651}\u{0644}");
    m.insert("Sound", "\u{0627}\u{0644}\u{0635}\u{0648}\u{062a}");
    m.insert(
        "Thresholds",
        "\u{0627}\u{0644}\u{062d}\u{062f}\u{0648}\u{062f}",
    );
    m.insert("Polling interval", "\u{0641}\u{062a}\u{0631}\u{0629} \u{0627}\u{0644}\u{0627}\u{0633}\u{062a}\u{0639}\u{0644}\u{0627}\u{0645}");
    m.insert("seconds", "\u{062b}\u{0648}\u{0627}\u{0646}\u{064a}");
    m.insert(
        "Startup",
        "\u{0628}\u{062f}\u{0621} \u{0627}\u{0644}\u{062a}\u{0634}\u{063a}\u{064a}\u{0644}",
    );
    m.insert("General", "\u{0639}\u{0627}\u{0645}");
    m.insert("Back", "\u{2190} \u{0631}\u{062c}\u{0648}\u{0639}");
    m.insert(
        "Open Claude.ai \u{2192}",
        "\u{0641}\u{062a}\u{062d} Claude.ai \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "سجل الاستخدام");
    m.insert("Usage History (24h)", "\u{0633}\u{062c}\u{0644} \u{0627}\u{0644}\u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645} (24\u{0633})");
    m.insert("Auto (English)", "\u{062a}\u{0644}\u{0642}\u{0627}\u{0626}\u{064a} (\u{0627}\u{0644}\u{0639}\u{0631}\u{0628}\u{064a}\u{0629})");
    m.insert("at", "\u{0639}\u{0646}\u{062f}");
    m.insert("Resets in", "\u{064a}\u{0639}\u{0627}\u{062f} \u{0636}\u{0628}\u{0637}\u{0647} \u{062e}\u{0644}\u{0627}\u{0644}");
    m.insert("Tray icon colors:", "\u{0623}\u{0644}\u{0648}\u{0627}\u{0646} \u{0623}\u{064a}\u{0642}\u{0648}\u{0646}\u{0629} \u{0627}\u{0644}\u{0639}\u{0644}\u{0628}\u{0629}:");
    m.insert(
        "< 50% usage",
        "< 50% \u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645}",
    );
    m.insert(
        "50-79% usage",
        "50\u{2013}79% \u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645}",
    );
    m.insert(
        ">= 80% usage",
        "\u{2265} 80% \u{0627}\u{0633}\u{062a}\u{062e}\u{062f}\u{0627}\u{0645}",
    );
    m.insert("No data", "\u{0644}\u{0627} \u{062a}\u{0648}\u{062c}\u{062f} \u{0628}\u{064a}\u{0627}\u{0646}\u{0627}\u{062a}");
    m.insert(
        "exceeded",
        "\u{062a}\u{0645} \u{062a}\u{062c}\u{0627}\u{0648}\u{0632}\u{0647}",
    );
    m.insert(
        "Show widget",
        "\u{0625}\u{0638}\u{0647}\u{0627}\u{0631} \u{0627}\u{0644}\u{0648}\u{064a}\u{062c}\u{062a}",
    );
    m.insert("Check for updates", "\u{0627}\u{0644}\u{062a}\u{062d}\u{0642}\u{0642} \u{0645}\u{0646} \u{0627}\u{0644}\u{062a}\u{062d}\u{062f}\u{064a}\u{062b}\u{0627}\u{062a}");
    m.insert("Accessibility patterns", "\u{0623}\u{0646}\u{0645}\u{0627}\u{0637} \u{0625}\u{0645}\u{0643}\u{0627}\u{0646}\u{064a}\u{0629} \u{0627}\u{0644}\u{0648}\u{0635}\u{0648}\u{0644}");
    m.insert(
        "Update available",
        "\u{062a}\u{062d}\u{062f}\u{064a}\u{062b} \u{0645}\u{062a}\u{0627}\u{062d}",
    );
    m.insert(
        "is available. Click to download.",
        "\u{0645}\u{062a}\u{0627}\u{062d}. \u{0627}\u{0646}\u{0642}\u{0631} \u{0644}\u{0644}\u{062a}\u{0646}\u{0632}\u{064a}\u{0644}.",
    );
    m.insert(
        "Icon style",
        "\u{0646}\u{0645}\u{0637} \u{0627}\u{0644}\u{0623}\u{064a}\u{0642}\u{0648}\u{0646}\u{0629}",
    );
    m.insert("Number", "\u{0631}\u{0642}\u{0645}");
    m.insert("Ring", "\u{062d}\u{0644}\u{0642}\u{0629}");
    m.insert("Bar", "\u{0634}\u{0631}\u{064a}\u{0637}");
    m.insert("Pie", "دائري");
    m.insert("Dashboard layout", "تخطيط اللوحة");
    m.insert("Minimal", "الحد الأدنى");
    m.insert("Standard", "قياسي");
    m.insert("Detailed", "مفصل");
    m.insert("Hide Extra Usage", "إخفاء Extra Usage");
    m
}
