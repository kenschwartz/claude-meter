use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5-timmarssession");
    m.insert("Weekly (7-day)", "Veckovis (7 dagar)");
    m.insert("Opus (7-day)", "Opus (7 dagar)");
    m.insert("Sonnet (7-day)", "Sonnet (7 dagar)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 dagar)");
    m.insert("resets in", "återställs om");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code hittades inte");
    m.insert(
        "credentials_not_found",
        "Inloggningsuppgifter hittades inte",
    );
    m.insert("connection_error", "Anslutningsfel");
    m.insert("token_expired", "Token har utgått");
    m.insert(
        "token_expired_desc",
        "Din OAuth-token har utgått. Kör `claude login` i terminalen för att förnya den.",
    );
    m.insert("rate_limited", "Hastighetsbegränsning");
    m.insert("server_error", "Serverfel");
    m.insert(
        "server_error_desc",
        "Anthropic API är tillfälligt otillgängligt. Försöker igen automatiskt.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code är installerat men inte inloggat. Kör `claude login` i terminalen för att ansluta ditt konto.",
    );
    m.insert(
        "install_claude_desc",
        "Installera Claude Code och kör `claude login` för att aktivera automatisk användningsspårning.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Installera Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI tillhandahåller inget API för att spåra ChatGPT-prenumerationsanvändning.",
    );
    m.insert(
        "Check your usage manually:",
        "Kontrollera din användning manuellt:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Öppna ChatGPT-användning \u{2192}",
    );
    m.insert("Refresh Now", "Uppdatera nu");
    m.insert("Open Dashboard", "Öppna instrumentpanel");
    m.insert("Export History (CSV)", "Exportera historik (CSV)");
    m.insert("Settings", "Inställningar");
    m.insert("Start with Windows", "Starta med Windows");
    m.insert("About", "Om");
    m.insert("Exit", "Avsluta");
    m.insert("Last updated:", "Senast uppdaterad:");
    m.insert("Refresh", "Uppdatera");
    m.insert("Status", "Status");
    m.insert("Usage Alert", "Användningsvarning");
    m.insert("Usage Critical", "Kritisk användning");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Körs i systemfältet. Klicka på ikonen för detaljer.",
    );
    m.insert("Compact mode", "Kompakt läge");
    m.insert("Theme", "Tema");
    m.insert("Language", "Språk");
    m.insert("Notifications", "Aviseringar");
    m.insert("Dark", "Mörkt");
    m.insert("Light", "Ljust");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Visa ChatGPT-sektion");
    m.insert("Enabled", "Aktiverad");
    m.insert("Sound", "Ljud");
    m.insert("Thresholds", "Tröskelvärden");
    m.insert("Polling interval", "Uppdateringsintervall");
    m.insert("seconds", "sekunder");
    m.insert("Startup", "Start");
    m.insert("General", "Allmänt");
    m.insert("Back", "\u{2190} Tillbaka");
    m.insert("Open Claude.ai \u{2192}", "Öppna Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Användningshistorik");
    m.insert("Usage History (24h)", "Användningshistorik (24t)");
    m.insert("Auto (English)", "Auto (Svenska)");
    m.insert("at", "vid");
    m.insert("Resets in", "Återställs om");
    m.insert("Tray icon colors:", "Ikonfärger i systemfältet:");
    m.insert("< 50% usage", "< 50% användning");
    m.insert("50-79% usage", "50\u{2013}79% användning");
    m.insert(">= 80% usage", "\u{2265} 80% användning");
    m.insert("No data", "Ingen data");
    m.insert("exceeded", "överskridet");
    m.insert("Show widget", "Visa widget");
    m.insert("Check for updates", "Sök efter uppdateringar");
    m.insert("Accessibility patterns", "Tillgänglighetsmönster");
    m.insert("Update available", "Uppdatering tillgänglig");
    m.insert(
        "is available. Click to download.",
        "är tillgänglig. Klicka för att ladda ner.",
    );
    m.insert("Icon style", "Ikonstil");
    m.insert("Number", "Nummer");
    m.insert("Ring", "Ring");
    m.insert("Bar", "Stapel");
    m.insert("Pie", "Cirkel");
    m.insert("Dashboard layout", "Panellayout");
    m.insert("Minimal", "Minimal");
    m.insert("Standard", "Standard");
    m.insert("Detailed", "Detaljerad");
    m.insert("Hide Extra Usage", "Dölj Extra Usage");
    m
}
