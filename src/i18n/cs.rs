use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5hodinová relace");
    m.insert("Weekly (7-day)", "Týdenní (7 dní)");
    m.insert("Opus (7-day)", "Opus (7 dní)");
    m.insert("Sonnet (7-day)", "Sonnet (7 dní)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 dní)");
    m.insert("resets in", "resetuje se za");
    m.insert("Plan", "Plán");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code nebyl nalezen");
    m.insert("credentials_not_found", "Přihlašovací údaje nenalezeny");
    m.insert("connection_error", "Chyba připojení");
    m.insert("token_expired", "Token vypršel");
    m.insert(
        "token_expired_desc",
        "Váš OAuth token vypršel. Spusťte `claude login` v terminálu pro jeho obnovení.",
    );
    m.insert("rate_limited", "Limit požadavků");
    m.insert("server_error", "Chyba serveru");
    m.insert(
        "server_error_desc",
        "Anthropic API je dočasně nedostupné. Automaticky se pokusí znovu.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code je nainstalován, ale není přihlášen. Spusťte `claude login` v terminálu pro připojení účtu.",
    );
    m.insert(
        "install_claude_desc",
        "Nainstalujte Claude Code a spusťte `claude login` pro aktivaci automatického sledování využití.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Nainstalovat Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI neposkytuje API pro sledování využití předplatného ChatGPT.",
    );
    m.insert("Check your usage manually:", "Zkontrolujte využití ručně:");
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Otevřít využití ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Obnovit nyní");
    m.insert("Open Dashboard", "Otevřít panel");
    m.insert("Export History (CSV)", "Exportovat historii (CSV)");
    m.insert("Settings", "Nastavení");
    m.insert("Start with Windows", "Spustit s Windows");
    m.insert("About", "O aplikaci");
    m.insert("Exit", "Ukončit");
    m.insert("Last updated:", "Poslední aktualizace:");
    m.insert("Refresh", "Obnovit");
    m.insert("Status", "Stav");
    m.insert("Usage Alert", "Upozornění na využití");
    m.insert("Usage Critical", "Kritické využití");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Běží v systémové liště. Klikněte na ikonu pro podrobnosti.",
    );
    m.insert("Compact mode", "Kompaktní režim");
    m.insert("Theme", "Motiv");
    m.insert("Language", "Jazyk");
    m.insert("Notifications", "Oznámení");
    m.insert("Dark", "Tmavý");
    m.insert("Light", "Světlý");
    m.insert("Auto", "Automaticky");
    m.insert("Show ChatGPT section", "Zobrazit sekci ChatGPT");
    m.insert("Enabled", "Povoleno");
    m.insert("Sound", "Zvuk");
    m.insert("Thresholds", "Prahové hodnoty");
    m.insert("Polling interval", "Interval aktualizace");
    m.insert("seconds", "sekund");
    m.insert("Startup", "Spuštění");
    m.insert("General", "Obecné");
    m.insert("Back", "\u{2190} Zpět");
    m.insert("Open Claude.ai \u{2192}", "Otevřít Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Historie využití");
    m.insert("Usage History (24h)", "Historie využití (24h)");
    m.insert("Auto (English)", "Automaticky (Čeština)");
    m.insert("at", "na");
    m.insert("Resets in", "Resetuje se za");
    m.insert("Tray icon colors:", "Barvy ikony v liště:");
    m.insert("< 50% usage", "< 50% využití");
    m.insert("50-79% usage", "50\u{2013}79% využití");
    m.insert(">= 80% usage", "\u{2265} 80% využití");
    m.insert("No data", "Žádná data");
    m.insert("exceeded", "překročeno");
    m.insert("Show widget", "Zobrazit widget");
    m.insert("Check for updates", "Kontrolovat aktualizace");
    m.insert("Accessibility patterns", "Vzory přístupnosti");
    m.insert("Update available", "Dostupná aktualizace");
    m.insert(
        "is available. Click to download.",
        "je k dispozici. Klikněte pro stažení.",
    );
    m.insert("Icon style", "Styl ikony");
    m.insert("Number", "Číslo");
    m.insert("Ring", "Kroužek");
    m.insert("Bar", "Sloupec");
    m.insert("Pie", "Koláčový");
    m.insert("Dashboard layout", "Rozložení panelu");
    m.insert("Minimal", "Minimální");
    m.insert("Standard", "Standardní");
    m.insert("Detailed", "Podrobný");
    m.insert("Hide Extra Usage", "Skrýt Extra Usage");
    m
}
