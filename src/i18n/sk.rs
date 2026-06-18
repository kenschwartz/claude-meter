use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5-hodinov\u{00e1} rel\u{00e1}cia");
    m.insert("Weekly (7-day)", "T\u{00fd}\u{017e}dennne (7 dn\u{00ed})");
    m.insert("Opus (7-day)", "Opus (7 dn\u{00ed})");
    m.insert("Sonnet (7-day)", "Sonnet (7 dn\u{00ed})");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 dn\u{00ed})");
    m.insert("resets in", "reset za");
    m.insert("Plan", "Pl\u{00e1}n");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert(
        "Claude Code not detected",
        "Claude Code nebol zisten\u{00fd}",
    );
    m.insert(
        "credentials_not_found",
        "Prihl\u{00e1}sovacie \u{00fa}daje neboli n\u{00e1}jden\u{00e9}",
    );
    m.insert("connection_error", "Chyba pripojenia");
    m.insert("token_expired", "Token vypr\u{0161}al");
    m.insert(
        "token_expired_desc",
        "V\u{00e1}\u{0161} OAuth token vypr\u{0161}al. Spustite `claude login` v termin\u{00e1}li.",
    );
    m.insert("rate_limited", "Obmedzen\u{00e1} frekvencia");
    m.insert("server_error", "Chyba servera");
    m.insert(
        "server_error_desc",
        "Anthropic API je do\u{010d}asne nedostupn\u{00e9}. Automaticky sa pokus\u{00ed} znova.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code je nain\u{0161}talovan\u{00fd}, ale nie ste prihl\u{00e1}sen\u{00ed}. Spustite `claude login`.",
    );
    m.insert(
        "install_claude_desc",
        "Nain\u{0161}talujte Claude Code a spustite `claude login`.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "In\u{0161}talova\u{0165} Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI neposkytuje API na sledovanie pou\u{017e}\u{00ed}vania predplatn\u{00e9}ho ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Skontrolujte pou\u{017e}\u{00ed}vanie manu\u{00e1}lne:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Otvori\u{0165} ChatGPT Usage \u{2192}",
    );
    m.insert("Refresh Now", "Obnovi\u{0165} teraz");
    m.insert("Open Dashboard", "Otvori\u{0165} dashboard");
    m.insert(
        "Export History (CSV)",
        "Exportova\u{0165} hist\u{00f3}riu (CSV)",
    );
    m.insert("Settings", "Nastavenia");
    m.insert("Start with Windows", "Spusti\u{0165} s Windows");
    m.insert("About", "O aplik\u{00e1}cii");
    m.insert("Exit", "Ukon\u{010d}i\u{0165}");
    m.insert("Last updated:", "Naposledy aktualizovan\u{00e9}:");
    m.insert("Refresh", "Obnovi\u{0165}");
    m.insert("Status", "Stav");
    m.insert("Usage Alert", "Upozornenie na pou\u{017e}\u{00ed}vanie");
    m.insert("Usage Critical", "Kritick\u{00e9} pou\u{017e}\u{00ed}vanie");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Be\u{017e}\u{00ed} v syst\u{00e9}movej li\u{0161}te. Kliknite na ikonu pre detaily.",
    );
    m.insert("Compact mode", "Kompaktn\u{00fd} re\u{017e}im");
    m.insert("Theme", "T\u{00e9}ma");
    m.insert("Language", "Jazyk");
    m.insert("Notifications", "Notifik\u{00e1}cie");
    m.insert("Dark", "Tmav\u{00e1}");
    m.insert("Light", "Svetl\u{00e1}");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Zobrazi\u{0165} sekciu ChatGPT");
    m.insert("Enabled", "Povolen\u{00e9}");
    m.insert("Sound", "Zvuk");
    m.insert("Thresholds", "Prahy");
    m.insert("Polling interval", "Interval dotazovania");
    m.insert("seconds", "sek\u{00fa}nd");
    m.insert("Startup", "Spustenie");
    m.insert("General", "V\u{0161}eobecn\u{00e9}");
    m.insert("Back", "\u{2190} Sp\u{00e4}\u{0165}");
    m.insert(
        "Open Claude.ai \u{2192}",
        "Otvori\u{0165} Claude.ai \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Hist\u{00f3}ria pou\u{017e}\u{00ed}vania");
    m.insert(
        "Usage History (24h)",
        "Hist\u{00f3}ria pou\u{017e}\u{00ed}vania (24h)",
    );
    m.insert("Auto (English)", "Auto (English)");
    m.insert("at", "o");
    m.insert("Resets in", "Reset za");
    m.insert("Tray icon colors:", "Farby ikony v li\u{0161}te:");
    m.insert("< 50% usage", "< 50% pou\u{017e}itie");
    m.insert("50-79% usage", "50\u{2013}79% pou\u{017e}itie");
    m.insert(">= 80% usage", "\u{2265} 80% pou\u{017e}itie");
    m.insert("No data", "\u{017d}iadne d\u{00e1}ta");
    m.insert("exceeded", "prekro\u{010d}en\u{00e9}");
    m.insert("Show widget", "Zobrazi\u{0165} widget");
    m.insert(
        "Check for updates",
        "Skontrolova\u{0165} aktualiz\u{00e1}cie",
    );
    m.insert("Accessibility patterns", "Vzory pr\u{00ed}stupnosti");
    m.insert("Update available", "Dostupn\u{00e1} aktualiz\u{00e1}cia");
    m.insert(
        "is available. Click to download.",
        "je dostupn\u{00e1}. Kliknite pre stiahnutie.",
    );
    m.insert("Icon style", "\u{0160}t\u{00fd}l ikony");
    m.insert("Number", "\u{010c}\u{00ed}slo");
    m.insert("Ring", "Kruh");
    m.insert("Bar", "St\u{013a}pec");
    m.insert("Pie", "Kol\u{00e1}\u{010d}");
    m.insert("Dashboard layout", "Rozlo\u{017e}enie dashboardu");
    m.insert("Minimal", "Minim\u{00e1}lne");
    m.insert("Standard", "\u{0160}tandardn\u{00e9}");
    m.insert("Detailed", "Podrobn\u{00e9}");
    m.insert("Hide Extra Usage", "Skry\u{0165} Extra Usage");
    m
}
