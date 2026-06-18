use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5\u{5c0f}\u{65f6}\u{4f1a}\u{8bdd}");
    m.insert("Weekly (7-day)", "\u{6bcf}\u{5468} (7\u{5929})");
    m.insert("Opus (7-day)", "Opus (7\u{5929})");
    m.insert("Sonnet (7-day)", "Sonnet (7\u{5929})");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7\u{5929})");
    m.insert("resets in", "\u{91cd}\u{7f6e}\u{5269}\u{4f59}");
    m.insert("Plan", "\u{8ba1}\u{5212}");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert(
        "Claude Code not detected",
        "\u{672a}\u{68c0}\u{6d4b}\u{5230} Claude Code",
    );
    m.insert(
        "credentials_not_found",
        "\u{672a}\u{627e}\u{5230}\u{51ed}\u{636e}",
    );
    m.insert("connection_error", "\u{8fde}\u{63a5}\u{9519}\u{8bef}");
    m.insert("token_expired", "令牌已过期");
    m.insert(
        "token_expired_desc",
        "您的OAuth令牌已过期。请在终端中运行 `claude login` 以刷新它。",
    );
    m.insert("rate_limited", "请求受限");
    m.insert("server_error", "服务器错误");
    m.insert(
        "server_error_desc",
        "Anthropic API 暂时不可用。将自动重试。",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code \u{5df2}\u{5b89}\u{88c5}\u{4f46}\u{672a}\u{767b}\u{5f55}\u{3002}\u{8bf7}\u{5728}\u{7ec8}\u{7aef}\u{8fd0}\u{884c} `claude login` \u{6765}\u{8fde}\u{63a5}\u{60a8}\u{7684}\u{8d26}\u{6237}\u{3002}",
    );
    m.insert(
        "install_claude_desc",
        "\u{5b89}\u{88c5} Claude Code \u{5e76}\u{8fd0}\u{884c} `claude login` \u{4ee5}\u{542f}\u{7528}\u{81ea}\u{52a8}\u{4f7f}\u{7528}\u{91cf}\u{8ddf}\u{8e2a}\u{3002}",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "\u{5b89}\u{88c5} Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI \u{672a}\u{63d0}\u{4f9b}\u{7528}\u{4e8e}\u{8ddf}\u{8e2a} ChatGPT \u{8ba2}\u{9605}\u{4f7f}\u{7528}\u{91cf}\u{7684} API\u{3002}",
    );
    m.insert(
        "Check your usage manually:",
        "\u{624b}\u{52a8}\u{68c0}\u{67e5}\u{60a8}\u{7684}\u{4f7f}\u{7528}\u{91cf}:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "\u{6253}\u{5f00} ChatGPT \u{4f7f}\u{7528}\u{91cf} \u{2192}",
    );
    m.insert("Refresh Now", "\u{7acb}\u{5373}\u{5237}\u{65b0}");
    m.insert("Open Dashboard", "\u{6253}\u{5f00}\u{4eea}\u{8868}\u{76d8}");
    m.insert(
        "Export History (CSV)",
        "\u{5bfc}\u{51fa}\u{5386}\u{53f2} (CSV)",
    );
    m.insert("Settings", "\u{8bbe}\u{7f6e}");
    m.insert("Start with Windows", "\u{968f} Windows \u{542f}\u{52a8}");
    m.insert("About", "\u{5173}\u{4e8e}");
    m.insert("Exit", "\u{9000}\u{51fa}");
    m.insert("Last updated:", "\u{6700}\u{540e}\u{66f4}\u{65b0}:");
    m.insert("Refresh", "\u{5237}\u{65b0}");
    m.insert("Status", "\u{72b6}\u{6001}");
    m.insert("Usage Alert", "\u{4f7f}\u{7528}\u{91cf}\u{8b66}\u{544a}");
    m.insert("Usage Critical", "\u{4f7f}\u{7528}\u{91cf}\u{5371}\u{9669}");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "\u{6b63}\u{5728}\u{7cfb}\u{7edf}\u{6258}\u{76d8}\u{4e2d}\u{8fd0}\u{884c}\u{3002}\u{70b9}\u{51fb}\u{56fe}\u{6807}\u{67e5}\u{770b}\u{8be6}\u{60c5}\u{3002}",
    );
    m.insert("Compact mode", "\u{7d27}\u{51d1}\u{6a21}\u{5f0f}");
    m.insert("Theme", "\u{4e3b}\u{9898}");
    m.insert("Language", "\u{8bed}\u{8a00}");
    m.insert("Notifications", "\u{901a}\u{77e5}");
    m.insert("Dark", "\u{6df1}\u{8272}");
    m.insert("Light", "\u{6d45}\u{8272}");
    m.insert("Auto", "\u{81ea}\u{52a8}");
    m.insert(
        "Show ChatGPT section",
        "\u{663e}\u{793a} ChatGPT \u{90e8}\u{5206}",
    );
    m.insert("Enabled", "\u{5df2}\u{542f}\u{7528}");
    m.insert("Sound", "\u{58f0}\u{97f3}");
    m.insert("Thresholds", "\u{9608}\u{503c}");
    m.insert("Polling interval", "\u{8f6e}\u{8be2}\u{95f4}\u{9694}");
    m.insert("seconds", "\u{79d2}");
    m.insert("Startup", "\u{542f}\u{52a8}");
    m.insert("General", "\u{5e38}\u{89c4}");
    m.insert("Back", "\u{2190} \u{8fd4}\u{56de}");
    m.insert(
        "Open Claude.ai \u{2192}",
        "\u{6253}\u{5f00} Claude.ai \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "使用历史");
    m.insert(
        "Usage History (24h)",
        "\u{4f7f}\u{7528}\u{5386}\u{53f2} (24\u{5c0f}\u{65f6})",
    );
    m.insert("Auto (English)", "\u{81ea}\u{52a8} (\u{4e2d}\u{6587})");
    m.insert("at", "\u{5728}");
    m.insert("Resets in", "\u{91cd}\u{7f6e}\u{5269}\u{4f59}");
    m.insert(
        "Tray icon colors:",
        "\u{6258}\u{76d8}\u{56fe}\u{6807}\u{989c}\u{8272}:",
    );
    m.insert("< 50% usage", "< 50% \u{4f7f}\u{7528}");
    m.insert("50-79% usage", "50\u{2013}79% \u{4f7f}\u{7528}");
    m.insert(">= 80% usage", "\u{2265} 80% \u{4f7f}\u{7528}");
    m.insert("No data", "\u{65e0}\u{6570}\u{636e}");
    m.insert("exceeded", "\u{8d85}\u{51fa}");
    m.insert("Show widget", "\u{663e}\u{793a}\u{5c0f}\u{7ec4}\u{4ef6}");
    m.insert("Check for updates", "\u{68c0}\u{67e5}\u{66f4}\u{65b0}");
    m.insert(
        "Accessibility patterns",
        "\u{65e0}\u{969c}\u{788d}\u{6a21}\u{5f0f}",
    );
    m.insert(
        "Update available",
        "\u{6709}\u{53ef}\u{7528}\u{66f4}\u{65b0}",
    );
    m.insert(
        "is available. Click to download.",
        "\u{53ef}\u{7528}\u{3002}\u{70b9}\u{51fb}\u{4e0b}\u{8f7d}\u{3002}",
    );
    m.insert("Icon style", "\u{56fe}\u{6807}\u{6837}\u{5f0f}");
    m.insert("Number", "\u{6570}\u{5b57}");
    m.insert("Ring", "\u{5706}\u{73af}");
    m.insert("Bar", "\u{6761}\u{5f62}");
    m.insert("Pie", "饼图");
    m.insert("Dashboard layout", "仪表板布局");
    m.insert("Minimal", "最小");
    m.insert("Standard", "标准");
    m.insert("Detailed", "详细");
    m.insert("Hide Extra Usage", "隐藏Extra Usage");
    m
}
