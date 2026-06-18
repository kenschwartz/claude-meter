use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert(
        "5-hour session",
        "5\u{6642}\u{9593}\u{30bb}\u{30c3}\u{30b7}\u{30e7}\u{30f3}",
    );
    m.insert("Weekly (7-day)", "\u{9031}\u{9593} (7\u{65e5})");
    m.insert("Opus (7-day)", "Opus (7\u{65e5})");
    m.insert("Sonnet (7-day)", "Sonnet (7\u{65e5})");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7\u{65e5})");
    m.insert(
        "resets in",
        "\u{30ea}\u{30bb}\u{30c3}\u{30c8}\u{307e}\u{3067}",
    );
    m.insert("Plan", "\u{30d7}\u{30e9}\u{30f3}");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert(
        "Claude Code not detected",
        "Claude Code\u{304c}\u{691c}\u{51fa}\u{3055}\u{308c}\u{307e}\u{305b}\u{3093}",
    );
    m.insert("credentials_not_found", "\u{8a8d}\u{8a3c}\u{60c5}\u{5831}\u{304c}\u{898b}\u{3064}\u{304b}\u{308a}\u{307e}\u{305b}\u{3093}");
    m.insert(
        "connection_error",
        "\u{63a5}\u{7d9a}\u{30a8}\u{30e9}\u{30fc}",
    );
    m.insert("token_expired", "トークン期限切れ");
    m.insert(
        "token_expired_desc",
        "OAuthトークンが期限切れです。ターミナルで `claude login` を実行して更新してください。",
    );
    m.insert("rate_limited", "レート制限");
    m.insert("server_error", "サーバーエラー");
    m.insert(
        "server_error_desc",
        "Anthropic APIが一時的に利用できません。自動的に再試行します。",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code\u{306f}\u{30a4}\u{30f3}\u{30b9}\u{30c8}\u{30fc}\u{30eb}\u{6e08}\u{307f}\u{3067}\u{3059}\u{304c}\u{30ed}\u{30b0}\u{30a4}\u{30f3}\u{3055}\u{308c}\u{3066}\u{3044}\u{307e}\u{305b}\u{3093}\u{3002}\u{30bf}\u{30fc}\u{30df}\u{30ca}\u{30eb}\u{3067} `claude login` \u{3092}\u{5b9f}\u{884c}\u{3057}\u{3066}\u{304f}\u{3060}\u{3055}\u{3044}\u{3002}",
    );
    m.insert(
        "install_claude_desc",
        "Claude Code\u{3092}\u{30a4}\u{30f3}\u{30b9}\u{30c8}\u{30fc}\u{30eb}\u{3057}\u{3001}`claude login` \u{3092}\u{5b9f}\u{884c}\u{3057}\u{3066}\u{4f7f}\u{7528}\u{91cf}\u{306e}\u{81ea}\u{52d5}\u{8ffd}\u{8de1}\u{3092}\u{6709}\u{52b9}\u{306b}\u{3057}\u{3066}\u{304f}\u{3060}\u{3055}\u{3044}\u{3002}",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Claude Code\u{3092}\u{30a4}\u{30f3}\u{30b9}\u{30c8}\u{30fc}\u{30eb} \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI\u{306f}ChatGPT\u{30b5}\u{30d6}\u{30b9}\u{30af}\u{30ea}\u{30d7}\u{30b7}\u{30e7}\u{30f3}\u{306e}\u{4f7f}\u{7528}\u{91cf}\u{3092}\u{8ffd}\u{8de1}\u{3059}\u{308b}API\u{3092}\u{63d0}\u{4f9b}\u{3057}\u{3066}\u{3044}\u{307e}\u{305b}\u{3093}\u{3002}",
    );
    m.insert(
        "Check your usage manually:",
        "\u{624b}\u{52d5}\u{3067}\u{4f7f}\u{7528}\u{91cf}\u{3092}\u{78ba}\u{8a8d}:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "ChatGPT\u{4f7f}\u{7528}\u{91cf}\u{3092}\u{958b}\u{304f} \u{2192}",
    );
    m.insert("Refresh Now", "\u{4eca}\u{3059}\u{3050}\u{66f4}\u{65b0}");
    m.insert(
        "Open Dashboard",
        "\u{30c0}\u{30c3}\u{30b7}\u{30e5}\u{30dc}\u{30fc}\u{30c9}\u{3092}\u{958b}\u{304f}",
    );
    m.insert(
        "Export History (CSV)",
        "\u{5c65}\u{6b74}\u{3092}\u{30a8}\u{30af}\u{30b9}\u{30dd}\u{30fc}\u{30c8} (CSV)",
    );
    m.insert("Settings", "\u{8a2d}\u{5b9a}");
    m.insert(
        "Start with Windows",
        "Windows\u{3068}\u{540c}\u{6642}\u{306b}\u{8d77}\u{52d5}",
    );
    m.insert(
        "About",
        "\u{30d0}\u{30fc}\u{30b8}\u{30e7}\u{30f3}\u{60c5}\u{5831}",
    );
    m.insert("Exit", "\u{7d42}\u{4e86}");
    m.insert("Last updated:", "\u{6700}\u{7d42}\u{66f4}\u{65b0}:");
    m.insert("Refresh", "\u{66f4}\u{65b0}");
    m.insert("Status", "\u{30b9}\u{30c6}\u{30fc}\u{30bf}\u{30b9}");
    m.insert("Usage Alert", "\u{4f7f}\u{7528}\u{91cf}\u{8b66}\u{544a}");
    m.insert("Usage Critical", "\u{4f7f}\u{7528}\u{91cf}\u{5371}\u{967a}");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "\u{30b7}\u{30b9}\u{30c6}\u{30e0}\u{30c8}\u{30ec}\u{30a4}\u{3067}\u{5b9f}\u{884c}\u{4e2d}\u{3002}\u{30a2}\u{30a4}\u{30b3}\u{30f3}\u{3092}\u{30af}\u{30ea}\u{30c3}\u{30af}\u{3057}\u{3066}\u{8a73}\u{7d30}\u{3092}\u{8868}\u{793a}\u{3002}",
    );
    m.insert(
        "Compact mode",
        "\u{30b3}\u{30f3}\u{30d1}\u{30af}\u{30c8}\u{30e2}\u{30fc}\u{30c9}",
    );
    m.insert("Theme", "\u{30c6}\u{30fc}\u{30de}");
    m.insert("Language", "\u{8a00}\u{8a9e}");
    m.insert("Notifications", "\u{901a}\u{77e5}");
    m.insert("Dark", "\u{30c0}\u{30fc}\u{30af}");
    m.insert("Light", "\u{30e9}\u{30a4}\u{30c8}");
    m.insert("Auto", "\u{81ea}\u{52d5}");
    m.insert(
        "Show ChatGPT section",
        "ChatGPT\u{30bb}\u{30af}\u{30b7}\u{30e7}\u{30f3}\u{3092}\u{8868}\u{793a}",
    );
    m.insert("Enabled", "\u{6709}\u{52b9}");
    m.insert("Sound", "\u{30b5}\u{30a6}\u{30f3}\u{30c9}");
    m.insert("Thresholds", "\u{3057}\u{304d}\u{3044}\u{5024}");
    m.insert(
        "Polling interval",
        "\u{30dd}\u{30fc}\u{30ea}\u{30f3}\u{30b0}\u{9593}\u{9694}",
    );
    m.insert("seconds", "\u{79d2}");
    m.insert("Startup", "\u{8d77}\u{52d5}");
    m.insert("General", "\u{4e00}\u{822c}");
    m.insert("Back", "\u{2190} \u{623b}\u{308b}");
    m.insert(
        "Open Claude.ai \u{2192}",
        "Claude.ai\u{3092}\u{958b}\u{304f} \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "使用履歴");
    m.insert(
        "Usage History (24h)",
        "\u{4f7f}\u{7528}\u{5c65}\u{6b74} (24\u{6642}\u{9593})",
    );
    m.insert(
        "Auto (English)",
        "\u{81ea}\u{52d5} (\u{65e5}\u{672c}\u{8a9e})",
    );
    m.insert("at", "\u{306b}");
    m.insert(
        "Resets in",
        "\u{30ea}\u{30bb}\u{30c3}\u{30c8}\u{307e}\u{3067}",
    );
    m.insert(
        "Tray icon colors:",
        "\u{30c8}\u{30ec}\u{30a4}\u{30a2}\u{30a4}\u{30b3}\u{30f3}\u{306e}\u{8272}:",
    );
    m.insert("< 50% usage", "< 50% \u{4f7f}\u{7528}");
    m.insert("50-79% usage", "50\u{2013}79% \u{4f7f}\u{7528}");
    m.insert(">= 80% usage", "\u{2265} 80% \u{4f7f}\u{7528}");
    m.insert("No data", "\u{30c7}\u{30fc}\u{30bf}\u{306a}\u{3057}");
    m.insert("exceeded", "\u{8d85}\u{904e}");
    m.insert(
        "Show widget",
        "\u{30a6}\u{30a3}\u{30b8}\u{30a7}\u{30c3}\u{30c8}\u{3092}\u{8868}\u{793a}",
    );
    m.insert(
        "Check for updates",
        "\u{30a2}\u{30c3}\u{30d7}\u{30c7}\u{30fc}\u{30c8}\u{3092}\u{78ba}\u{8a8d}",
    );
    m.insert("Accessibility patterns", "\u{30a2}\u{30af}\u{30bb}\u{30b7}\u{30d3}\u{30ea}\u{30c6}\u{30a3}\u{30d1}\u{30bf}\u{30fc}\u{30f3}");
    m.insert(
        "Update available",
        "\u{30a2}\u{30c3}\u{30d7}\u{30c7}\u{30fc}\u{30c8}\u{5229}\u{7528}\u{53ef}\u{80fd}",
    );
    m.insert("is available. Click to download.", "\u{304c}\u{5229}\u{7528}\u{53ef}\u{80fd}\u{3067}\u{3059}\u{3002}\u{30af}\u{30ea}\u{30c3}\u{30af}\u{3057}\u{3066}\u{30c0}\u{30a6}\u{30f3}\u{30ed}\u{30fc}\u{30c9}\u{3002}");
    m.insert(
        "Icon style",
        "\u{30a2}\u{30a4}\u{30b3}\u{30f3}\u{30b9}\u{30bf}\u{30a4}\u{30eb}",
    );
    m.insert("Number", "\u{6570}\u{5b57}");
    m.insert("Ring", "\u{30ea}\u{30f3}\u{30b0}");
    m.insert("Bar", "\u{30d0}\u{30fc}");
    m.insert("Pie", "円グラフ");
    m.insert("Dashboard layout", "ダッシュボードレイアウト");
    m.insert("Minimal", "最小");
    m.insert("Standard", "標準");
    m.insert("Detailed", "詳細");
    m.insert("Hide Extra Usage", "Extra Usageを非表示");
    m
}
