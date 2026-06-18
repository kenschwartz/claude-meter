use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5\u{c2dc}\u{ac04} \u{c138}\u{c158}");
    m.insert("Weekly (7-day)", "\u{c8fc}\u{ac04} (7\u{c77c})");
    m.insert("Opus (7-day)", "Opus (7\u{c77c})");
    m.insert("Sonnet (7-day)", "Sonnet (7\u{c77c})");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7\u{c77c})");
    m.insert("resets in", "\u{cd08}\u{ae30}\u{d654}\u{ae4c}\u{c9c0}");
    m.insert("Plan", "\u{d50c}\u{b79c}");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert(
        "Claude Code not detected",
        "Claude Code\u{b97c} \u{cc3e}\u{c744} \u{c218} \u{c5c6}\u{c2b5}\u{b2c8}\u{b2e4}",
    );
    m.insert("credentials_not_found", "\u{c778}\u{c99d} \u{c815}\u{bcf4}\u{b97c} \u{cc3e}\u{c744} \u{c218} \u{c5c6}\u{c2b5}\u{b2c8}\u{b2e4}");
    m.insert("connection_error", "\u{c5f0}\u{acb0} \u{c624}\u{b958}");
    m.insert("token_expired", "토큰 만료");
    m.insert(
        "token_expired_desc",
        "OAuth 토큰이 만료되었습니다. 터미널에서 `claude login`을 실행하여 갱신하세요.",
    );
    m.insert("rate_limited", "요청 제한");
    m.insert("server_error", "서버 오류");
    m.insert(
        "server_error_desc",
        "Anthropic API가 일시적으로 사용할 수 없습니다. 자동으로 재시도합니다.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code\u{ac00} \u{c124}\u{ce58}\u{b418}\u{c5c8}\u{c9c0}\u{b9cc} \u{b85c}\u{adf8}\u{c778}\u{b418}\u{c9c0} \u{c54a}\u{c558}\u{c2b5}\u{b2c8}\u{b2e4}. \u{d130}\u{bbf8}\u{b110}\u{c5d0}\u{c11c} `claude login`\u{c744} \u{c2e4}\u{d589}\u{d558}\u{c138}\u{c694}.",
    );
    m.insert(
        "install_claude_desc",
        "Claude Code\u{b97c} \u{c124}\u{ce58}\u{d558}\u{ace0} `claude login`\u{c744} \u{c2e4}\u{d589}\u{d558}\u{c5ec} \u{c790}\u{b3d9} \u{c0ac}\u{c6a9}\u{b7c9} \u{cd94}\u{c801}\u{c744} \u{d65c}\u{c131}\u{d654}\u{d558}\u{c138}\u{c694}.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Claude Code \u{c124}\u{ce58} \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI\u{b294} ChatGPT \u{ad6c}\u{b3c5} \u{c0ac}\u{c6a9}\u{b7c9}\u{c744} \u{cd94}\u{c801}\u{d558}\u{b294} API\u{b97c} \u{c81c}\u{acf5}\u{d558}\u{c9c0} \u{c54a}\u{c2b5}\u{b2c8}\u{b2e4}.",
    );
    m.insert(
        "Check your usage manually:",
        "\u{c0ac}\u{c6a9}\u{b7c9}\u{c744} \u{c218}\u{b3d9}\u{c73c}\u{b85c} \u{d655}\u{c778}\u{d558}\u{c138}\u{c694}:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "ChatGPT \u{c0ac}\u{c6a9}\u{b7c9} \u{c5f4}\u{ae30} \u{2192}",
    );
    m.insert(
        "Refresh Now",
        "\u{c9c0}\u{ae08} \u{c0c8}\u{b85c}\u{ace0}\u{ce68}",
    );
    m.insert(
        "Open Dashboard",
        "\u{b300}\u{c2dc}\u{bcf4}\u{b4dc} \u{c5f4}\u{ae30}",
    );
    m.insert(
        "Export History (CSV)",
        "\u{ae30}\u{b85d} \u{b0b4}\u{bcf4}\u{b0b4}\u{ae30} (CSV)",
    );
    m.insert("Settings", "\u{c124}\u{c815}");
    m.insert(
        "Start with Windows",
        "Windows\u{c640} \u{d568}\u{aed8} \u{c2dc}\u{c791}",
    );
    m.insert("About", "\u{c815}\u{bcf4}");
    m.insert("Exit", "\u{c885}\u{b8cc}");
    m.insert(
        "Last updated:",
        "\u{b9c8}\u{c9c0}\u{b9c9} \u{c5c5}\u{b370}\u{c774}\u{d2b8}:",
    );
    m.insert("Refresh", "\u{c0c8}\u{b85c}\u{ace0}\u{ce68}");
    m.insert("Status", "\u{c0c1}\u{d0dc}");
    m.insert("Usage Alert", "\u{c0ac}\u{c6a9}\u{b7c9} \u{acbd}\u{ace0}");
    m.insert(
        "Usage Critical",
        "\u{c0ac}\u{c6a9}\u{b7c9} \u{c704}\u{d5d8}",
    );
    m.insert(
        "Running in system tray. Click the icon for details.",
        "\u{c2dc}\u{c2a4}\u{d15c} \u{d2b8}\u{b808}\u{c774}\u{c5d0}\u{c11c} \u{c2e4}\u{d589} \u{c911}. \u{c544}\u{c774}\u{cf58}\u{c744} \u{d074}\u{b9ad}\u{d558}\u{c5ec} \u{c138}\u{bd80} \u{c815}\u{bcf4}\u{b97c} \u{d655}\u{c778}\u{d558}\u{c138}\u{c694}.",
    );
    m.insert("Compact mode", "\u{cf64}\u{d329}\u{d2b8} \u{baa8}\u{b4dc}");
    m.insert("Theme", "\u{d14c}\u{b9c8}");
    m.insert("Language", "\u{c5b8}\u{c5b4}");
    m.insert("Notifications", "\u{c54c}\u{b9bc}");
    m.insert("Dark", "\u{b2e4}\u{d06c}");
    m.insert("Light", "\u{b77c}\u{c774}\u{d2b8}");
    m.insert("Auto", "\u{c790}\u{b3d9}");
    m.insert(
        "Show ChatGPT section",
        "ChatGPT \u{c139}\u{c158} \u{d45c}\u{c2dc}",
    );
    m.insert("Enabled", "\u{d65c}\u{c131}\u{d654}");
    m.insert("Sound", "\u{c18c}\u{b9ac}");
    m.insert("Thresholds", "\u{c784}\u{acc4}\u{ac12}");
    m.insert("Polling interval", "\u{d3f4}\u{b9c1} \u{ac04}\u{aca9}");
    m.insert("seconds", "\u{cd08}");
    m.insert("Startup", "\u{c2dc}\u{c791}");
    m.insert("General", "\u{c77c}\u{bc18}");
    m.insert("Back", "\u{2190} \u{b4a4}\u{b85c}");
    m.insert(
        "Open Claude.ai \u{2192}",
        "Claude.ai \u{c5f4}\u{ae30} \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "사용 기록");
    m.insert(
        "Usage History (24h)",
        "\u{c0ac}\u{c6a9} \u{ae30}\u{b85d} (24\u{c2dc}\u{ac04})",
    );
    m.insert(
        "Auto (English)",
        "\u{c790}\u{b3d9} (\u{d55c}\u{ad6d}\u{c5b4})",
    );
    m.insert("at", "\u{c5d0}");
    m.insert("Resets in", "\u{cd08}\u{ae30}\u{d654}\u{ae4c}\u{c9c0}");
    m.insert(
        "Tray icon colors:",
        "\u{d2b8}\u{b808}\u{c774} \u{c544}\u{c774}\u{cf58} \u{c0c9}\u{c0c1}:",
    );
    m.insert("< 50% usage", "< 50% \u{c0ac}\u{c6a9}");
    m.insert("50-79% usage", "50\u{2013}79% \u{c0ac}\u{c6a9}");
    m.insert(">= 80% usage", "\u{2265} 80% \u{c0ac}\u{c6a9}");
    m.insert("No data", "\u{b370}\u{c774}\u{d130} \u{c5c6}\u{c74c}");
    m.insert("exceeded", "\u{cd08}\u{acfc}");
    m.insert("Show widget", "\u{c704}\u{c82f} \u{d45c}\u{c2dc}");
    m.insert(
        "Check for updates",
        "\u{c5c5}\u{b370}\u{c774}\u{d2b8} \u{d655}\u{c778}",
    );
    m.insert(
        "Accessibility patterns",
        "\u{c811}\u{adc0}\u{c131} \u{d328}\u{d134}",
    );
    m.insert(
        "Update available",
        "\u{c5c5}\u{b370}\u{c774}\u{d2b8} \u{c0ac}\u{c6a9} \u{ac00}\u{b2a5}",
    );
    m.insert("is available. Click to download.", "\u{c744} \u{c0ac}\u{c6a9}\u{d560} \u{c218} \u{c788}\u{c2b5}\u{b2c8}\u{b2e4}. \u{d074}\u{b9ad}\u{d558}\u{c5ec} \u{b2e4}\u{c6b4}\u{b85c}\u{b4dc}\u{d558}\u{c138}\u{c694}.");
    m.insert(
        "Icon style",
        "\u{c544}\u{c774}\u{cf58} \u{c2a4}\u{d0c0}\u{c77c}",
    );
    m.insert("Number", "\u{c22b}\u{c790}");
    m.insert("Ring", "\u{b9c1}");
    m.insert("Bar", "\u{bc14}");
    m.insert("Pie", "원형");
    m.insert("Dashboard layout", "대시보드 레이아웃");
    m.insert("Minimal", "최소");
    m.insert("Standard", "표준");
    m.insert("Detailed", "상세");
    m.insert("Hide Extra Usage", "Extra Usage 숨기기");
    m
}
