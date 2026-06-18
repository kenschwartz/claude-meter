use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "Sesja 5-godzinna");
    m.insert("Weekly (7-day)", "Tygodniowy (7 dni)");
    m.insert("Opus (7-day)", "Opus (7 dni)");
    m.insert("Sonnet (7-day)", "Sonnet (7 dni)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 dni)");
    m.insert("resets in", "reset za");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code nie wykryto");
    m.insert("credentials_not_found", "Nie znaleziono danych logowania");
    m.insert("connection_error", "Błąd połączenia");
    m.insert("token_expired", "Token wygasł");
    m.insert(
        "token_expired_desc",
        "Twój token OAuth wygasł. Uruchom `claude login` w terminalu, aby go odnowić.",
    );
    m.insert("rate_limited", "Limit zapytań");
    m.insert("server_error", "Błąd serwera");
    m.insert(
        "server_error_desc",
        "API Anthropic jest tymczasowo niedostępne. Ponowna próba nastąpi automatycznie.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code jest zainstalowany, ale nie zalogowany. Uruchom `claude login` w terminalu.",
    );
    m.insert(
        "install_claude_desc",
        "Zainstaluj Claude Code i uruchom `claude login`, aby w\u{0142}\u{0105}czy\u{0107} automatyczne \u{015b}ledzenie.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Zainstaluj Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI nie udost\u{0119}pnia API do \u{015b}ledzenia u\u{017c}ycia subskrypcji ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Sprawd\u{017a} u\u{017c}ycie r\u{0119}cznie:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Otw\u{00f3}rz u\u{017c}ycie ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Od\u{015b}wie\u{017c} teraz");
    m.insert("Open Dashboard", "Otw\u{00f3}rz panel");
    m.insert("Export History (CSV)", "Eksportuj histori\u{0119} (CSV)");
    m.insert("Settings", "Ustawienia");
    m.insert("Start with Windows", "Uruchom z Windows");
    m.insert("About", "O programie");
    m.insert("Exit", "Wyj\u{015b}cie");
    m.insert("Last updated:", "Ostatnia aktualizacja:");
    m.insert("Refresh", "Od\u{015b}wie\u{017c}");
    m.insert("Status", "Status");
    m.insert("Usage Alert", "Alert u\u{017c}ycia");
    m.insert("Usage Critical", "Krytyczne u\u{017c}ycie");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Dzia\u{0142}a w zasobniku systemowym. Kliknij ikon\u{0119}, aby zobaczy\u{0107} szczeg\u{00f3}\u{0142}y.",
    );
    m.insert("Compact mode", "Tryb kompaktowy");
    m.insert("Theme", "Motyw");
    m.insert("Language", "J\u{0119}zyk");
    m.insert("Notifications", "Powiadomienia");
    m.insert("Dark", "Ciemny");
    m.insert("Light", "Jasny");
    m.insert("Auto", "Automatyczny");
    m.insert("Show ChatGPT section", "Poka\u{017c} sekcj\u{0119} ChatGPT");
    m.insert("Enabled", "W\u{0142}\u{0105}czony");
    m.insert("Sound", "D\u{017a}wi\u{0119}k");
    m.insert("Thresholds", "Progi");
    m.insert("Polling interval", "Interwa\u{0142} odpytywania");
    m.insert("seconds", "sekund");
    m.insert("Startup", "Uruchamianie");
    m.insert("General", "Og\u{00f3}lne");
    m.insert("Back", "\u{2190} Wstecz");
    m.insert(
        "Open Claude.ai \u{2192}",
        "Otw\u{00f3}rz Claude.ai \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Historia użycia");
    m.insert("Usage History (24h)", "Historia u\u{017c}ycia (24h)");
    m.insert("Auto (English)", "Automatyczny (Polski)");
    m.insert("at", "o");
    m.insert("Resets in", "Reset za");
    m.insert("Tray icon colors:", "Kolory ikony zasobnika:");
    m.insert("< 50% usage", "< 50% u\u{017c}ycia");
    m.insert("50-79% usage", "50\u{2013}79% u\u{017c}ycia");
    m.insert(">= 80% usage", "\u{2265} 80% u\u{017c}ycia");
    m.insert("No data", "Brak danych");
    m.insert("exceeded", "przekroczono");
    m.insert("Show widget", "Poka\u{017c} widget");
    m.insert("Check for updates", "Sprawd\u{017a} aktualizacje");
    m.insert("Accessibility patterns", "Wzorce dost\u{0119}pno\u{015b}ci");
    m.insert("Update available", "Dost\u{0119}pna aktualizacja");
    m.insert(
        "is available. Click to download.",
        "jest dost\u{0119}pna. Kliknij, aby pobra\u{0107}.",
    );
    m.insert("Icon style", "Styl ikony");
    m.insert("Number", "Liczba");
    m.insert("Ring", "Pier\u{015b}cie\u{0144}");
    m.insert("Bar", "Pasek");
    m.insert("Pie", "Kołowy");
    m.insert("Dashboard layout", "Układ panelu");
    m.insert("Minimal", "Minimalny");
    m.insert("Standard", "Standardowy");
    m.insert("Detailed", "Szczegółowy");
    m.insert("Hide Extra Usage", "Ukryj Extra Usage");
    m
}
