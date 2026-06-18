use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5-timers\u{00f8}kt");
    m.insert("Weekly (7-day)", "Ukentlig (7 dager)");
    m.insert("Opus (7-day)", "Opus (7 dager)");
    m.insert("Sonnet (7-day)", "Sonnet (7 dager)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 dager)");
    m.insert("resets in", "nullstilles om");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code ikke funnet");
    m.insert("credentials_not_found", "Legitimasjon ikke funnet");
    m.insert("connection_error", "Tilkoblingsfeil");
    m.insert("token_expired", "Token utl\u{00f8}pt");
    m.insert(
        "token_expired_desc",
        "OAuth-tokenet ditt har utl\u{00f8}pt. Kj\u{00f8}r `claude login` i terminalen.",
    );
    m.insert("rate_limited", "Hastighetsbegrenset");
    m.insert("server_error", "Serverfeil");
    m.insert(
        "server_error_desc",
        "Anthropic API er midlertidig utilgjengelig. Pr\u{00f8}ver igjen automatisk.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code er installert men ikke p\u{00e5}logget. Kj\u{00f8}r `claude login`.",
    );
    m.insert(
        "install_claude_desc",
        "Installer Claude Code og kj\u{00f8}r `claude login`.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Installer Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI tilbyr ikke et API for \u{00e5} spore ChatGPT-abonnementsbruk.",
    );
    m.insert("Check your usage manually:", "Sjekk bruken manuelt:");
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "\u{00c5}pne ChatGPT Usage \u{2192}",
    );
    m.insert("Refresh Now", "Oppdater n\u{00e5}");
    m.insert("Open Dashboard", "\u{00c5}pne dashbord");
    m.insert("Export History (CSV)", "Eksporter historikk (CSV)");
    m.insert("Settings", "Innstillinger");
    m.insert("Start with Windows", "Start med Windows");
    m.insert("About", "Om");
    m.insert("Exit", "Avslutt");
    m.insert("Last updated:", "Sist oppdatert:");
    m.insert("Refresh", "Oppdater");
    m.insert("Status", "Status");
    m.insert("Usage Alert", "Bruksvarsel");
    m.insert("Usage Critical", "Kritisk bruk");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Kj\u{00f8}rer i systemstatusfeltet. Klikk p\u{00e5} ikonet for detaljer.",
    );
    m.insert("Compact mode", "Kompakt modus");
    m.insert("Theme", "Tema");
    m.insert("Language", "Spr\u{00e5}k");
    m.insert("Notifications", "Varsler");
    m.insert("Dark", "M\u{00f8}rk");
    m.insert("Light", "Lys");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Vis ChatGPT-seksjon");
    m.insert("Enabled", "Aktivert");
    m.insert("Sound", "Lyd");
    m.insert("Thresholds", "Terskelverdier");
    m.insert("Polling interval", "Avsp\u{00f8}rringsintervall");
    m.insert("seconds", "sekunder");
    m.insert("Startup", "Oppstart");
    m.insert("General", "Generelt");
    m.insert("Back", "\u{2190} Tilbake");
    m.insert("Open Claude.ai \u{2192}", "\u{00c5}pne Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Brukshistorikk");
    m.insert("Usage History (24h)", "Brukshistorikk (24t)");
    m.insert("Auto (English)", "Auto (English)");
    m.insert("at", "kl.");
    m.insert("Resets in", "Nullstilles om");
    m.insert("Tray icon colors:", "Ikonfarger:");
    m.insert("< 50% usage", "< 50%");
    m.insert("50-79% usage", "50\u{2013}79%");
    m.insert(">= 80% usage", "\u{2265} 80%");
    m.insert("No data", "Ingen data");
    m.insert("exceeded", "overskredet");
    m.insert("Show widget", "Vis widget");
    m.insert("Check for updates", "Se etter oppdateringer");
    m.insert("Accessibility patterns", "Tilgjengelighetstm\u{00f8}nstre");
    m.insert("Update available", "Oppdatering tilgjengelig");
    m.insert(
        "is available. Click to download.",
        "er tilgjengelig. Klikk for \u{00e5} laste ned.",
    );
    m.insert("Icon style", "Ikonstil");
    m.insert("Number", "Nummer");
    m.insert("Ring", "Ring");
    m.insert("Bar", "Stolpe");
    m.insert("Pie", "Kake");
    m.insert("Dashboard layout", "Dashbordoppsett");
    m.insert("Minimal", "Minimal");
    m.insert("Standard", "Standard");
    m.insert("Detailed", "Detaljert");
    m.insert("Hide Extra Usage", "Skjul Extra Usage");
    m
}
