use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "Phi\u{00ea}n 5 gi\u{1edd}");
    m.insert("Weekly (7-day)", "H\u{00e0}ng tu\u{1ea7}n (7 ng\u{00e0}y)");
    m.insert("Opus (7-day)", "Opus (7 ng\u{00e0}y)");
    m.insert("Sonnet (7-day)", "Sonnet (7 ng\u{00e0}y)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 ng\u{00e0}y)");
    m.insert("resets in", "\u{0111}\u{1eb7}t l\u{1ea1}i sau");
    m.insert("Plan", "G\u{00f3}i");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert(
        "Claude Code not detected",
        "Kh\u{00f4}ng t\u{00ec}m th\u{1ea5}y Claude Code",
    );
    m.insert(
        "credentials_not_found",
        "Kh\u{00f4}ng t\u{00ec}m th\u{1ea5}y th\u{00f4}ng tin \u{0111}\u{0103}ng nh\u{1ead}p",
    );
    m.insert("connection_error", "L\u{1ed7}i k\u{1ebf}t n\u{1ed1}i");
    m.insert("token_expired", "Token hết hạn");
    m.insert(
        "token_expired_desc",
        "Token OAuth của bạn đã hết hạn. Chạy `claude login` trong terminal để làm mới.",
    );
    m.insert("rate_limited", "Giới hạn yêu cầu");
    m.insert("server_error", "Lỗi máy chủ");
    m.insert(
        "server_error_desc",
        "API Anthropic tạm thời không khả dụng. Sẽ tự động thử lại.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code \u{0111}\u{00e3} c\u{00e0}i \u{0111}\u{1eb7}t nh\u{01b0}ng ch\u{01b0}a \u{0111}\u{0103}ng nh\u{1ead}p. Ch\u{1ea1}y `claude login` trong terminal.",
    );
    m.insert(
        "install_claude_desc",
        "C\u{00e0}i \u{0111}\u{1eb7}t Claude Code v\u{00e0} ch\u{1ea1}y `claude login` \u{0111}\u{1ec3} b\u{1eaf}t theo d\u{00f5}i t\u{1ef1} \u{0111}\u{1ed9}ng.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "C\u{00e0}i \u{0111}\u{1eb7}t Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI kh\u{00f4}ng cung c\u{1ea5}p API \u{0111}\u{1ec3} theo d\u{00f5}i m\u{1ee9}c s\u{1eed} d\u{1ee5}ng \u{0111}\u{0103}ng k\u{00fd} ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Ki\u{1ec3}m tra m\u{1ee9}c s\u{1eed} d\u{1ee5}ng th\u{1ee7} c\u{00f4}ng:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "M\u{1edf} m\u{1ee9}c d\u{00f9}ng ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "L\u{00e0}m m\u{1edb}i ngay");
    m.insert(
        "Open Dashboard",
        "M\u{1edf} b\u{1ea3}ng \u{0111}i\u{1ec1}u khi\u{1ec3}n",
    );
    m.insert(
        "Export History (CSV)",
        "Xu\u{1ea5}t l\u{1ecb}ch s\u{1eed} (CSV)",
    );
    m.insert("Settings", "C\u{00e0}i \u{0111}\u{1eb7}t");
    m.insert(
        "Start with Windows",
        "Kh\u{1edf}i \u{0111}\u{1ed9}ng c\u{00f9}ng Windows",
    );
    m.insert("About", "Gi\u{1edb}i thi\u{1ec7}u");
    m.insert("Exit", "Tho\u{00e1}t");
    m.insert(
        "Last updated:",
        "C\u{1ead}p nh\u{1ead}t l\u{1ea7}n cu\u{1ed1}i:",
    );
    m.insert("Refresh", "L\u{00e0}m m\u{1edb}i");
    m.insert("Status", "Tr\u{1ea1}ng th\u{00e1}i");
    m.insert(
        "Usage Alert",
        "C\u{1ea3}nh b\u{00e1}o s\u{1eed} d\u{1ee5}ng",
    );
    m.insert(
        "Usage Critical",
        "S\u{1eed} d\u{1ee5}ng nghi\u{00ea}m tr\u{1ecd}ng",
    );
    m.insert(
        "Running in system tray. Click the icon for details.",
        "\u{0110}ang ch\u{1ea1}y trong khay h\u{1ec7} th\u{1ed1}ng. Nh\u{1ea5}p v\u{00e0}o bi\u{1ec3}u t\u{01b0}\u{1ee3}ng \u{0111}\u{1ec3} xem chi ti\u{1ebf}t.",
    );
    m.insert("Compact mode", "Ch\u{1ebf} \u{0111}\u{1ed9} g\u{1ecd}n");
    m.insert("Theme", "Giao di\u{1ec7}n");
    m.insert("Language", "Ng\u{00f4}n ng\u{1eef}");
    m.insert("Notifications", "Th\u{00f4}ng b\u{00e1}o");
    m.insert("Dark", "T\u{1ed1}i");
    m.insert("Light", "S\u{00e1}ng");
    m.insert("Auto", "T\u{1ef1} \u{0111}\u{1ed9}ng");
    m.insert("Show ChatGPT section", "Hi\u{1ec3}n ph\u{1ea7}n ChatGPT");
    m.insert("Enabled", "B\u{1ead}t");
    m.insert("Sound", "\u{00c2}m thanh");
    m.insert("Thresholds", "Ng\u{01b0}\u{1ee1}ng");
    m.insert(
        "Polling interval",
        "Kho\u{1ea3}ng th\u{1edd}i gian ki\u{1ec3}m tra",
    );
    m.insert("seconds", "gi\u{00e2}y");
    m.insert("Startup", "Kh\u{1edf}i \u{0111}\u{1ed9}ng");
    m.insert("General", "Chung");
    m.insert("Back", "\u{2190} Quay l\u{1ea1}i");
    m.insert("Open Claude.ai \u{2192}", "M\u{1edf} Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Lịch sử sử dụng");
    m.insert(
        "Usage History (24h)",
        "L\u{1ecb}ch s\u{1eed} s\u{1eed} d\u{1ee5}ng (24h)",
    );
    m.insert(
        "Auto (English)",
        "T\u{1ef1} \u{0111}\u{1ed9}ng (Ti\u{1ebf}ng Vi\u{1ec7}t)",
    );
    m.insert("at", "l\u{00fa}c");
    m.insert("Resets in", "\u{0110}\u{1eb7}t l\u{1ea1}i sau");
    m.insert(
        "Tray icon colors:",
        "M\u{00e0}u bi\u{1ec3}u t\u{01b0}\u{1ee3}ng:",
    );
    m.insert("< 50% usage", "< 50% s\u{1eed} d\u{1ee5}ng");
    m.insert("50-79% usage", "50\u{2013}79% s\u{1eed} d\u{1ee5}ng");
    m.insert(">= 80% usage", "\u{2265} 80% s\u{1eed} d\u{1ee5}ng");
    m.insert("No data", "Kh\u{00f4}ng c\u{00f3} d\u{1eef} li\u{1ec7}u");
    m.insert("exceeded", "v\u{01b0}\u{1ee3}t");
    m.insert("Show widget", "Hi\u{1ec3}n widget");
    m.insert(
        "Check for updates",
        "Ki\u{1ec3}m tra c\u{1ead}p nh\u{1ead}t",
    );
    m.insert(
        "Accessibility patterns",
        "M\u{1eab}u h\u{1ed7} tr\u{1ee3} ti\u{1ebf}p c\u{1ead}n",
    );
    m.insert(
        "Update available",
        "C\u{00f3} b\u{1ea3}n c\u{1ead}p nh\u{1ead}t",
    );
    m.insert(
        "is available. Click to download.",
        "\u{0111}\u{00e3} c\u{00f3}. Nh\u{1ea5}p \u{0111}\u{1ec3} t\u{1ea3}i v\u{1ec1}.",
    );
    m.insert("Icon style", "Ki\u{1ec3}u bi\u{1ec3}u t\u{01b0}\u{1ee3}ng");
    m.insert("Number", "S\u{1ed1}");
    m.insert("Ring", "V\u{00f2}ng");
    m.insert("Bar", "Thanh");
    m.insert("Pie", "Biểu đồ tròn");
    m.insert("Dashboard layout", "Bố cục bảng");
    m.insert("Minimal", "Tối thiểu");
    m.insert("Standard", "Tiêu chuẩn");
    m.insert("Detailed", "Chi tiết");
    m.insert("Hide Extra Usage", "Ẩn Extra Usage");
    m
}
