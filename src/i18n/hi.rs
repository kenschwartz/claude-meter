use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert(
        "5-hour session",
        "5-\u{0918}\u{0902}\u{091f}\u{0947} \u{0915}\u{093e} \u{0938}\u{0924}\u{094d}\u{0930}",
    );
    m.insert("Weekly (7-day)", "\u{0938}\u{093e}\u{092a}\u{094d}\u{0924}\u{093e}\u{0939}\u{093f}\u{0915} (7-\u{0926}\u{093f}\u{0928})");
    m.insert("Opus (7-day)", "Opus (7-\u{0926}\u{093f}\u{0928})");
    m.insert("Sonnet (7-day)", "Sonnet (7-\u{0926}\u{093f}\u{0928})");
    m.insert(
        "OAuth Apps (7-day)",
        "OAuth Apps (7-\u{0926}\u{093f}\u{0928})",
    );
    m.insert(
        "resets in",
        "\u{0930}\u{0940}\u{0938}\u{0947}\u{091f} \u{0939}\u{094b}\u{0917}\u{093e}",
    );
    m.insert("Plan", "\u{092a}\u{094d}\u{0932}\u{093e}\u{0928}");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert(
        "Claude Code not detected",
        "Claude Code \u{0928}\u{0939}\u{0940}\u{0902} \u{092e}\u{093f}\u{0932}\u{093e}",
    );
    m.insert("credentials_not_found", "\u{0915}\u{094d}\u{0930}\u{0947}\u{0921}\u{0947}\u{0902}\u{0936}\u{093f}\u{092f}\u{0932} \u{0928}\u{0939}\u{0940}\u{0902} \u{092e}\u{093f}\u{0932}\u{0947}");
    m.insert("connection_error", "\u{0915}\u{0928}\u{0947}\u{0915}\u{094d}\u{0936}\u{0928} \u{0924}\u{094d}\u{0930}\u{0941}\u{091f}\u{093f}");
    m.insert("token_expired", "टोकन समाप्त");
    m.insert(
        "token_expired_desc",
        "आपका OAuth टोकन समाप्त हो गया है। इसे नवीनीकृत करने के लिए टर्मिनल में `claude login` चलाएँ।",
    );
    m.insert("rate_limited", "अनुरोध सीमा");
    m.insert("server_error", "सर्वर त्रुटि");
    m.insert(
        "server_error_desc",
        "Anthropic API अस्थायी रूप से अनुपलब्ध है। स्वचालित रूप से पुनः प्रयास होगा।",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code \u{0907}\u{0902}\u{0938}\u{094d}\u{091f}\u{0949}\u{0932} \u{0939}\u{0948} \u{0932}\u{0947}\u{0915}\u{093f}\u{0928} \u{0932}\u{0949}\u{0917}\u{093f}\u{0928} \u{0928}\u{0939}\u{0940}\u{0902} \u{0939}\u{0948}\u{0964} \u{0905}\u{092a}\u{0928}\u{0947} \u{091f}\u{0930}\u{094d}\u{092e}\u{093f}\u{0928}\u{0932} \u{092e}\u{0947}\u{0902} `claude login` \u{091a}\u{0932}\u{093e}\u{090f}\u{0902}\u{0964}",
    );
    m.insert(
        "install_claude_desc",
        "Claude Code \u{0907}\u{0902}\u{0938}\u{094d}\u{091f}\u{0949}\u{0932} \u{0915}\u{0930}\u{0947}\u{0902} \u{0914}\u{0930} \u{0911}\u{091f}\u{094b}\u{092e}\u{0948}\u{091f}\u{093f}\u{0915} \u{091f}\u{094d}\u{0930}\u{0948}\u{0915}\u{093f}\u{0902}\u{0917} \u{0915}\u{0947} \u{0932}\u{093f}\u{090f} `claude login` \u{091a}\u{0932}\u{093e}\u{090f}\u{0902}\u{0964}",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Claude Code \u{0907}\u{0902}\u{0938}\u{094d}\u{091f}\u{0949}\u{0932} \u{0915}\u{0930}\u{0947}\u{0902} \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI ChatGPT \u{0938}\u{092c}\u{094d}\u{0938}\u{0915}\u{094d}\u{0930}\u{093f}\u{092a}\u{094d}\u{0936}\u{0928} \u{0909}\u{092a}\u{092f}\u{094b}\u{0917} \u{091f}\u{094d}\u{0930}\u{0948}\u{0915} \u{0915}\u{0930}\u{0928}\u{0947} \u{0915}\u{0947} \u{0932}\u{093f}\u{090f} API \u{092a}\u{094d}\u{0930}\u{0926}\u{093e}\u{0928} \u{0928}\u{0939}\u{0940}\u{0902} \u{0915}\u{0930}\u{0924}\u{093e}\u{0964}",
    );
    m.insert(
        "Check your usage manually:",
        "\u{0905}\u{092a}\u{0928}\u{093e} \u{0909}\u{092a}\u{092f}\u{094b}\u{0917} \u{092e}\u{0948}\u{0928}\u{094d}\u{092f}\u{0941}\u{0905}\u{0932} \u{0930}\u{0942}\u{092a} \u{0938}\u{0947} \u{091c}\u{093e}\u{0901}\u{091a}\u{0947}\u{0902}:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "ChatGPT \u{0909}\u{092a}\u{092f}\u{094b}\u{0917} \u{0916}\u{094b}\u{0932}\u{0947}\u{0902} \u{2192}",
    );
    m.insert("Refresh Now", "\u{0905}\u{092d}\u{0940} \u{0930}\u{093f}\u{092b}\u{094d}\u{0930}\u{0947}\u{0936} \u{0915}\u{0930}\u{0947}\u{0902}");
    m.insert("Open Dashboard", "\u{0921}\u{0948}\u{0936}\u{092c}\u{094b}\u{0930}\u{094d}\u{0921} \u{0916}\u{094b}\u{0932}\u{0947}\u{0902}");
    m.insert("Export History (CSV)", "\u{0907}\u{0924}\u{093f}\u{0939}\u{093e}\u{0938} \u{0928}\u{093f}\u{0930}\u{094d}\u{092f}\u{093e}\u{0924} \u{0915}\u{0930}\u{0947}\u{0902} (CSV)");
    m.insert(
        "Settings",
        "\u{0938}\u{0947}\u{091f}\u{093f}\u{0902}\u{0917}\u{094d}\u{0938}",
    );
    m.insert("Start with Windows", "Windows \u{0915}\u{0947} \u{0938}\u{093e}\u{0925} \u{0936}\u{0941}\u{0930}\u{0942} \u{0915}\u{0930}\u{0947}\u{0902}");
    m.insert(
        "About",
        "\u{092c}\u{093e}\u{0930}\u{0947} \u{092e}\u{0947}\u{0902}",
    );
    m.insert(
        "Exit",
        "\u{092c}\u{093e}\u{0939}\u{0930} \u{0928}\u{093f}\u{0915}\u{0932}\u{0947}\u{0902}",
    );
    m.insert(
        "Last updated:",
        "\u{0905}\u{0902}\u{0924}\u{093f}\u{092e} \u{0905}\u{092a}\u{0921}\u{0947}\u{091f}:",
    );
    m.insert(
        "Refresh",
        "\u{0930}\u{093f}\u{092b}\u{094d}\u{0930}\u{0947}\u{0936}",
    );
    m.insert("Status", "\u{0938}\u{094d}\u{0925}\u{093f}\u{0924}\u{093f}");
    m.insert(
        "Usage Alert",
        "\u{0909}\u{092a}\u{092f}\u{094b}\u{0917} \u{0905}\u{0932}\u{0930}\u{094d}\u{091f}",
    );
    m.insert("Usage Critical", "\u{0909}\u{092a}\u{092f}\u{094b}\u{0917} \u{0915}\u{094d}\u{0930}\u{093f}\u{091f}\u{093f}\u{0915}\u{0932}");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "\u{0938}\u{093f}\u{0938}\u{094d}\u{091f}\u{092e} \u{091f}\u{094d}\u{0930}\u{0947} \u{092e}\u{0947}\u{0902} \u{091a}\u{0932} \u{0930}\u{0939}\u{093e} \u{0939}\u{0948}\u{0964} \u{0935}\u{093f}\u{0935}\u{0930}\u{0923} \u{0915}\u{0947} \u{0932}\u{093f}\u{090f} \u{0906}\u{0907}\u{0915}\u{0928} \u{092a}\u{0930} \u{0915}\u{094d}\u{0932}\u{093f}\u{0915} \u{0915}\u{0930}\u{0947}\u{0902}\u{0964}",
    );
    m.insert("Compact mode", "\u{0915}\u{0949}\u{092e}\u{094d}\u{092a}\u{0948}\u{0915}\u{094d}\u{091f} \u{092e}\u{094b}\u{0921}");
    m.insert("Theme", "\u{0925}\u{0940}\u{092e}");
    m.insert("Language", "\u{092d}\u{093e}\u{0937}\u{093e}");
    m.insert(
        "Notifications",
        "\u{0938}\u{0942}\u{091a}\u{0928}\u{093e}\u{090f}\u{0902}",
    );
    m.insert("Dark", "\u{0921}\u{093e}\u{0930}\u{094d}\u{0915}");
    m.insert("Light", "\u{0932}\u{093e}\u{0907}\u{091f}");
    m.insert("Auto", "\u{0911}\u{091f}\u{094b}");
    m.insert("Show ChatGPT section", "ChatGPT \u{0905}\u{0928}\u{0941}\u{092d}\u{093e}\u{0917} \u{0926}\u{093f}\u{0916}\u{093e}\u{090f}\u{0902}");
    m.insert(
        "Enabled",
        "\u{0938}\u{0915}\u{094d}\u{0930}\u{093f}\u{092f}",
    );
    m.insert("Sound", "\u{0927}\u{094d}\u{0935}\u{0928}\u{093f}");
    m.insert(
        "Thresholds",
        "\u{0938}\u{0940}\u{092e}\u{093e}\u{090f}\u{0902}",
    );
    m.insert("Polling interval", "\u{092a}\u{094b}\u{0932}\u{093f}\u{0902}\u{0917} \u{0905}\u{0902}\u{0924}\u{0930}\u{093e}\u{0932}");
    m.insert("seconds", "\u{0938}\u{0947}\u{0915}\u{0902}\u{0921}");
    m.insert(
        "Startup",
        "\u{0938}\u{094d}\u{091f}\u{093e}\u{0930}\u{094d}\u{091f}\u{0905}\u{092a}",
    );
    m.insert(
        "General",
        "\u{0938}\u{093e}\u{092e}\u{093e}\u{0928}\u{094d}\u{092f}",
    );
    m.insert("Back", "\u{2190} \u{0935}\u{093e}\u{092a}\u{0938}");
    m.insert(
        "Open Claude.ai \u{2192}",
        "Claude.ai \u{0916}\u{094b}\u{0932}\u{0947}\u{0902} \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "उपयोग इतिहास");
    m.insert("Usage History (24h)", "\u{0909}\u{092a}\u{092f}\u{094b}\u{0917} \u{0907}\u{0924}\u{093f}\u{0939}\u{093e}\u{0938} (24\u{0918}\u{0902})");
    m.insert(
        "Auto (English)",
        "\u{0911}\u{091f}\u{094b} (\u{0939}\u{093f}\u{0928}\u{094d}\u{0926}\u{0940})",
    );
    m.insert("at", "\u{092a}\u{0930}");
    m.insert(
        "Resets in",
        "\u{0930}\u{0940}\u{0938}\u{0947}\u{091f} \u{0939}\u{094b}\u{0917}\u{093e}",
    );
    m.insert("Tray icon colors:", "\u{091f}\u{094d}\u{0930}\u{0947} \u{0906}\u{0907}\u{0915}\u{0928} \u{0930}\u{0902}\u{0917}:");
    m.insert(
        "< 50% usage",
        "< 50% \u{0909}\u{092a}\u{092f}\u{094b}\u{0917}",
    );
    m.insert(
        "50-79% usage",
        "50\u{2013}79% \u{0909}\u{092a}\u{092f}\u{094b}\u{0917}",
    );
    m.insert(
        ">= 80% usage",
        "\u{2265} 80% \u{0909}\u{092a}\u{092f}\u{094b}\u{0917}",
    );
    m.insert("No data", "\u{0915}\u{094b}\u{0908} \u{0921}\u{0947}\u{091f}\u{093e} \u{0928}\u{0939}\u{0940}\u{0902}");
    m.insert("exceeded", "\u{092a}\u{093e}\u{0930}");
    m.insert(
        "Show widget",
        "\u{0935}\u{093f}\u{091c}\u{0947}\u{091f} \u{0926}\u{093f}\u{0916}\u{093e}\u{090f}\u{0902}",
    );
    m.insert(
        "Check for updates",
        "\u{0905}\u{092a}\u{0921}\u{0947}\u{091f} \u{091c}\u{093e}\u{0901}\u{091a}\u{0947}\u{0902}",
    );
    m.insert("Accessibility patterns", "\u{0905}\u{092d}\u{093f}\u{0917}\u{092e}\u{094d}\u{092f}\u{0924}\u{093e} \u{092a}\u{0948}\u{091f}\u{0930}\u{094d}\u{0928}");
    m.insert(
        "Update available",
        "\u{0905}\u{092a}\u{0921}\u{0947}\u{091f} \u{0909}\u{092a}\u{0932}\u{092c}\u{094d}\u{0927}",
    );
    m.insert(
        "is available. Click to download.",
        "\u{0909}\u{092a}\u{0932}\u{092c}\u{094d}\u{0927} \u{0939}\u{0948}\u{0964} \u{0921}\u{093e}\u{0909}\u{0928}\u{0932}\u{094b}\u{0921} \u{0915}\u{0947} \u{0932}\u{093f}\u{090f} \u{0915}\u{094d}\u{0932}\u{093f}\u{0915} \u{0915}\u{0930}\u{0947}\u{0902}\u{0964}",
    );
    m.insert(
        "Icon style",
        "\u{0906}\u{0907}\u{0915}\u{0928} \u{0936}\u{0948}\u{0932}\u{0940}",
    );
    m.insert("Number", "\u{0938}\u{0902}\u{0916}\u{094d}\u{092f}\u{093e}");
    m.insert("Ring", "\u{0930}\u{093f}\u{0902}\u{0917}");
    m.insert("Bar", "\u{092c}\u{093e}\u{0930}");
    m.insert("Pie", "पाई");
    m.insert("Dashboard layout", "डैशबोर्ड लेआउट");
    m.insert("Minimal", "न्यूनतम");
    m.insert("Standard", "मानक");
    m.insert("Detailed", "विस्तृत");
    m.insert("Hide Extra Usage", "Extra Usage छिपाएँ");
    m
}
