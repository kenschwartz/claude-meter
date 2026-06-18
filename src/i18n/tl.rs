use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5-oras na sesyon");
    m.insert("Weekly (7-day)", "Lingguhan (7-araw)");
    m.insert("Opus (7-day)", "Opus (7-araw)");
    m.insert("Sonnet (7-day)", "Sonnet (7-araw)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7-araw)");
    m.insert("resets in", "mare-reset sa");
    m.insert("Plan", "Plano");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Hindi nakita ang Claude Code");
    m.insert("credentials_not_found", "Hindi nahanap ang kredensyal");
    m.insert("connection_error", "Error sa koneksyon");
    m.insert("token_expired", "Nag-expire na ang token");
    m.insert(
        "token_expired_desc",
        "Nag-expire na ang iyong OAuth token. Patakbuhin ang `claude login` sa terminal.",
    );
    m.insert("rate_limited", "Na-rate limit");
    m.insert("server_error", "Error sa server");
    m.insert(
        "server_error_desc",
        "Pansamantalang hindi available ang Anthropic API. Awtomatikong susubukan muli.",
    );
    m.insert(
        "run_claude_login_desc",
        "Naka-install ang Claude Code pero hindi naka-login. Patakbuhin ang `claude login`.",
    );
    m.insert(
        "install_claude_desc",
        "I-install ang Claude Code at patakbuhin ang `claude login`.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "I-install ang Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "Walang API ang OpenAI para subaybayan ang paggamit ng ChatGPT subscription.",
    );
    m.insert(
        "Check your usage manually:",
        "Manu-manong suriin ang paggamit:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Buksan ang ChatGPT Usage \u{2192}",
    );
    m.insert("Refresh Now", "I-refresh Ngayon");
    m.insert("Open Dashboard", "Buksan ang Dashboard");
    m.insert("Export History (CSV)", "I-export ang Kasaysayan (CSV)");
    m.insert("Settings", "Mga Setting");
    m.insert("Start with Windows", "Magsimula sa Windows");
    m.insert("About", "Tungkol sa");
    m.insert("Exit", "Lumabas");
    m.insert("Last updated:", "Huling na-update:");
    m.insert("Refresh", "I-refresh");
    m.insert("Status", "Katayuan");
    m.insert("Usage Alert", "Alerto sa Paggamit");
    m.insert("Usage Critical", "Kritikal na Paggamit");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Tumatakbo sa system tray. I-click ang icon para sa detalye.",
    );
    m.insert("Compact mode", "Compact mode");
    m.insert("Theme", "Tema");
    m.insert("Language", "Wika");
    m.insert("Notifications", "Mga Notipikasyon");
    m.insert("Dark", "Madilim");
    m.insert("Light", "Maliwanag");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Ipakita ang seksyon ng ChatGPT");
    m.insert("Enabled", "Naka-enable");
    m.insert("Sound", "Tunog");
    m.insert("Thresholds", "Mga Threshold");
    m.insert("Polling interval", "Agwat ng pag-poll");
    m.insert("seconds", "segundo");
    m.insert("Startup", "Pagsisimula");
    m.insert("General", "Pangkalahatan");
    m.insert("Back", "\u{2190} Bumalik");
    m.insert("Open Claude.ai \u{2192}", "Buksan ang Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Kasaysayan ng Paggamit");
    m.insert("Usage History (24h)", "Kasaysayan ng Paggamit (24h)");
    m.insert("Auto (English)", "Auto (English)");
    m.insert("at", "sa");
    m.insert("Resets in", "Mare-reset sa");
    m.insert("Tray icon colors:", "Mga kulay ng tray icon:");
    m.insert("< 50% usage", "< 50% paggamit");
    m.insert("50-79% usage", "50\u{2013}79% paggamit");
    m.insert(">= 80% usage", "\u{2265} 80% paggamit");
    m.insert("No data", "Walang data");
    m.insert("exceeded", "lumampas");
    m.insert("Show widget", "Ipakita ang widget");
    m.insert("Check for updates", "Suriin ang mga update");
    m.insert("Accessibility patterns", "Mga pattern ng accessibility");
    m.insert("Update available", "May available na update");
    m.insert(
        "is available. Click to download.",
        "ay available. I-click para i-download.",
    );
    m.insert("Icon style", "Estilo ng icon");
    m.insert("Number", "Numero");
    m.insert("Ring", "Bilog");
    m.insert("Bar", "Bar");
    m.insert("Pie", "Pie");
    m.insert("Dashboard layout", "Layout ng dashboard");
    m.insert("Minimal", "Minimal");
    m.insert("Standard", "Standard");
    m.insert("Detailed", "Detalyado");
    m.insert("Hide Extra Usage", "Itago ang Extra Usage");
    m
}
