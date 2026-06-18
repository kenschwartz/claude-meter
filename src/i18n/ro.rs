use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "Sesiune de 5 ore");
    m.insert("Weekly (7-day)", "S\u{0103}pt\u{0103}m\u{00e2}nal (7 zile)");
    m.insert("Opus (7-day)", "Opus (7 zile)");
    m.insert("Sonnet (7-day)", "Sonnet (7 zile)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 zile)");
    m.insert("resets in", "se reseteaz\u{0103} \u{00ee}n");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code nu a fost detectat");
    m.insert("credentials_not_found", "Credentiale neg\u{0103}site");
    m.insert("connection_error", "Eroare de conexiune");
    m.insert("token_expired", "Token expirat");
    m.insert(
        "token_expired_desc",
        "Tokenul OAuth a expirat. Rulați `claude login` în terminal pentru a-l reînnoi.",
    );
    m.insert("rate_limited", "Limită de cereri");
    m.insert("server_error", "Eroare de server");
    m.insert(
        "server_error_desc",
        "API-ul Anthropic este temporar indisponibil. Se va reîncerca automat.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code este instalat dar nu este autentificat. Ruleaz\u{0103} `claude login` \u{00ee}n terminal pentru a conecta contul.",
    );
    m.insert(
        "install_claude_desc",
        "Instaleaz\u{0103} Claude Code \u{0219}i ruleaz\u{0103} `claude login` pentru a activa urm\u{0103}rirea automat\u{0103} a utiliz\u{0103}rii.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Instaleaz\u{0103} Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI nu ofer\u{0103} un API pentru a urm\u{0103}ri utilizarea abonamentului ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Verific\u{0103} utilizarea manual:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Deschide utilizarea ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Actualizeaz\u{0103} acum");
    m.insert("Open Dashboard", "Deschide panoul");
    m.insert("Export History (CSV)", "Export\u{0103} istoricul (CSV)");
    m.insert("Settings", "Set\u{0103}ri");
    m.insert("Start with Windows", "Porne\u{0219}te cu Windows");
    m.insert("About", "Despre");
    m.insert("Exit", "Ie\u{0219}ire");
    m.insert("Last updated:", "Ultima actualizare:");
    m.insert("Refresh", "Actualizeaz\u{0103}");
    m.insert("Status", "Stare");
    m.insert("Usage Alert", "Alert\u{0103} utilizare");
    m.insert("Usage Critical", "Utilizare critic\u{0103}");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Ruleaz\u{0103} \u{00ee}n bara de sistem. Apas\u{0103} pe icon pentru detalii.",
    );
    m.insert("Compact mode", "Mod compact");
    m.insert("Theme", "Tem\u{0103}");
    m.insert("Language", "Limb\u{0103}");
    m.insert("Notifications", "Notific\u{0103}ri");
    m.insert("Dark", "\u{00ce}ntunecat");
    m.insert("Light", "Luminos");
    m.insert("Auto", "Auto");
    m.insert(
        "Show ChatGPT section",
        "Arat\u{0103} sec\u{021b}iunea ChatGPT",
    );
    m.insert("Enabled", "Activat");
    m.insert("Sound", "Sunet");
    m.insert("Thresholds", "Praguri");
    m.insert("Polling interval", "Interval de actualizare");
    m.insert("seconds", "secunde");
    m.insert("Startup", "Pornire");
    m.insert("General", "General");
    m.insert("Back", "\u{2190} \u{00ce}napoi");
    m.insert("Open Claude.ai \u{2192}", "Deschide Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Istoric utilizare");
    m.insert("Usage History (24h)", "Istoric utilizare (24h)");
    m.insert("Auto (English)", "Auto (Rom\u{00e2}n\u{0103})");
    m.insert("at", "la");
    m.insert("Resets in", "Se reseteaz\u{0103} \u{00ee}n");
    m.insert("Tray icon colors:", "Culorile iconului din bar\u{0103}:");
    m.insert("< 50% usage", "< 50% utilizare");
    m.insert("50-79% usage", "50\u{2013}79% utilizare");
    m.insert(">= 80% usage", "\u{2265} 80% utilizare");
    m.insert("No data", "F\u{0103}r\u{0103} date");
    m.insert("exceeded", "dep\u{0103}\u{0219}it");
    m.insert("Show widget", "Arat\u{0103} widget");
    m.insert("Check for updates", "Verific\u{0103} actualiz\u{0103}ri");
    m.insert("Accessibility patterns", "Modele de accesibilitate");
    m.insert("Update available", "Actualizare disponibil\u{0103}");
    m.insert(
        "is available. Click to download.",
        "este disponibil\u{0103}. Apas\u{0103} pentru desc\u{0103}rcare.",
    );
    m.insert("Icon style", "Stil icon");
    m.insert("Number", "Num\u{0103}r");
    m.insert("Ring", "Inel");
    m.insert("Bar", "Bar\u{0103}");
    m.insert("Pie", "Cerc");
    m.insert("Dashboard layout", "Aspectul panoului");
    m.insert("Minimal", "Minim");
    m.insert("Standard", "Standard");
    m.insert("Detailed", "Detaliat");
    m.insert("Hide Extra Usage", "Ascunde Extra Usage");
    m
}
