use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5-uurs sessie");
    m.insert("Weekly (7-day)", "Wekelijks (7 dagen)");
    m.insert("Opus (7-day)", "Opus (7 dagen)");
    m.insert("Sonnet (7-day)", "Sonnet (7 dagen)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 dagen)");
    m.insert("resets in", "reset over");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code niet gevonden");
    m.insert("credentials_not_found", "Inloggegevens niet gevonden");
    m.insert("connection_error", "Verbindingsfout");
    m.insert("token_expired", "Token verlopen");
    m.insert(
        "token_expired_desc",
        "Uw OAuth-token is verlopen. Voer `claude login` uit in uw terminal om het te vernieuwen.",
    );
    m.insert("rate_limited", "Snelheidslimiet bereikt");
    m.insert("server_error", "Serverfout");
    m.insert(
        "server_error_desc",
        "Anthropic API is tijdelijk niet beschikbaar. Wordt automatisch opnieuw geprobeerd.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code is ge\u{00ef}nstalleerd maar niet ingelogd. Voer `claude login` uit in uw terminal.",
    );
    m.insert(
        "install_claude_desc",
        "Installeer Claude Code en voer `claude login` uit voor automatische tracking.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Claude Code Installeren \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI biedt geen API om ChatGPT-abonnementsgebruik bij te houden.",
    );
    m.insert(
        "Check your usage manually:",
        "Controleer uw gebruik handmatig:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "ChatGPT Gebruik Openen \u{2192}",
    );
    m.insert("Refresh Now", "Nu vernieuwen");
    m.insert("Open Dashboard", "Dashboard openen");
    m.insert("Export History (CSV)", "Geschiedenis exporteren (CSV)");
    m.insert("Settings", "Instellingen");
    m.insert("Start with Windows", "Starten met Windows");
    m.insert("About", "Over");
    m.insert("Exit", "Afsluiten");
    m.insert("Last updated:", "Laatst bijgewerkt:");
    m.insert("Refresh", "Vernieuwen");
    m.insert("Status", "Status");
    m.insert("Usage Alert", "Gebruikswaarschuwing");
    m.insert("Usage Critical", "Kritiek gebruik");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Actief in systeemvak. Klik op het pictogram voor details.",
    );
    m.insert("Compact mode", "Compacte modus");
    m.insert("Theme", "Thema");
    m.insert("Language", "Taal");
    m.insert("Notifications", "Meldingen");
    m.insert("Dark", "Donker");
    m.insert("Light", "Licht");
    m.insert("Auto", "Automatisch");
    m.insert("Show ChatGPT section", "ChatGPT-sectie tonen");
    m.insert("Enabled", "Ingeschakeld");
    m.insert("Sound", "Geluid");
    m.insert("Thresholds", "Drempelwaarden");
    m.insert("Polling interval", "Polling-interval");
    m.insert("seconds", "seconden");
    m.insert("Startup", "Opstarten");
    m.insert("General", "Algemeen");
    m.insert("Back", "\u{2190} Terug");
    m.insert("Open Claude.ai \u{2192}", "Claude.ai Openen \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Gebruiksgeschiedenis");
    m.insert("Usage History (24h)", "Gebruiksgeschiedenis (24u)");
    m.insert("Auto (English)", "Automatisch (Nederlands)");
    m.insert("at", "om");
    m.insert("Resets in", "Reset over");
    m.insert("Tray icon colors:", "Systeemvakpictogram kleuren:");
    m.insert("< 50% usage", "< 50% gebruik");
    m.insert("50-79% usage", "50\u{2013}79% gebruik");
    m.insert(">= 80% usage", "\u{2265} 80% gebruik");
    m.insert("No data", "Geen gegevens");
    m.insert("exceeded", "overschreden");
    m.insert("Show widget", "Widget tonen");
    m.insert("Check for updates", "Controleren op updates");
    m.insert("Accessibility patterns", "Toegankelijkheidspatronen");
    m.insert("Update available", "Update beschikbaar");
    m.insert(
        "is available. Click to download.",
        "is beschikbaar. Klik om te downloaden.",
    );
    m.insert("Icon style", "Pictogramstijl");
    m.insert("Number", "Nummer");
    m.insert("Ring", "Ring");
    m.insert("Bar", "Balk");
    m.insert("Pie", "Taart");
    m.insert("Dashboard layout", "Dashboard-indeling");
    m.insert("Minimal", "Minimaal");
    m.insert("Standard", "Standaard");
    m.insert("Detailed", "Gedetailleerd");
    m.insert("Hide Extra Usage", "Extra Usage verbergen");
    m
}
