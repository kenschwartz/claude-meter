use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "Sessione di 5 ore");
    m.insert("Weekly (7-day)", "Settimanale (7 giorni)");
    m.insert("Opus (7-day)", "Opus (7 giorni)");
    m.insert("Sonnet (7-day)", "Sonnet (7 giorni)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 giorni)");
    m.insert("resets in", "si resetta tra");
    m.insert("Plan", "Piano");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code non rilevato");
    m.insert("credentials_not_found", "Credenziali non trovate");
    m.insert("connection_error", "Errore di connessione");
    m.insert("token_expired", "Token scaduto");
    m.insert(
        "token_expired_desc",
        "Il tuo token OAuth è scaduto. Esegui `claude login` nel terminale per rinnovarlo.",
    );
    m.insert("rate_limited", "Limite di richieste");
    m.insert("server_error", "Errore del server");
    m.insert(
        "server_error_desc",
        "L'API Anthropic è temporaneamente non disponibile. Riproverà automaticamente.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code \u{00e8} installato ma non connesso. Esegui `claude login` nel terminale per collegare il tuo account.",
    );
    m.insert(
        "install_claude_desc",
        "Installa Claude Code ed esegui `claude login` per attivare il monitoraggio automatico dell'utilizzo.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Installa Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI non fornisce un'API per monitorare l'utilizzo dell'abbonamento ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Controlla il tuo utilizzo manualmente:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Apri utilizzo ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Aggiorna ora");
    m.insert("Open Dashboard", "Apri pannello");
    m.insert("Export History (CSV)", "Esporta cronologia (CSV)");
    m.insert("Settings", "Impostazioni");
    m.insert("Start with Windows", "Avvia con Windows");
    m.insert("About", "Informazioni");
    m.insert("Exit", "Esci");
    m.insert("Last updated:", "Ultimo aggiornamento:");
    m.insert("Refresh", "Aggiorna");
    m.insert("Status", "Stato");
    m.insert("Usage Alert", "Avviso di utilizzo");
    m.insert("Usage Critical", "Utilizzo critico");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "In esecuzione nella barra delle applicazioni. Clicca sull'icona per i dettagli.",
    );
    m.insert("Compact mode", "Modalit\u{00e0} compatta");
    m.insert("Theme", "Tema");
    m.insert("Language", "Lingua");
    m.insert("Notifications", "Notifiche");
    m.insert("Dark", "Scuro");
    m.insert("Light", "Chiaro");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Mostra sezione ChatGPT");
    m.insert("Enabled", "Attivato");
    m.insert("Sound", "Suono");
    m.insert("Thresholds", "Soglie");
    m.insert("Polling interval", "Intervallo di polling");
    m.insert("seconds", "secondi");
    m.insert("Startup", "Avvio");
    m.insert("General", "Generale");
    m.insert("Back", "\u{2190} Indietro");
    m.insert("Open Claude.ai \u{2192}", "Apri Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Cronologia utilizzo");
    m.insert("Usage History (24h)", "Cronologia utilizzo (24h)");
    m.insert("Auto (English)", "Auto (Italiano)");
    m.insert("at", "alle");
    m.insert("Resets in", "Si resetta tra");
    m.insert("Tray icon colors:", "Colori icona nella barra:");
    m.insert("< 50% usage", "< 50% utilizzo");
    m.insert("50-79% usage", "50\u{2013}79% utilizzo");
    m.insert(">= 80% usage", "\u{2265} 80% utilizzo");
    m.insert("No data", "Nessun dato");
    m.insert("exceeded", "superato");
    m.insert("Show widget", "Mostra widget");
    m.insert("Check for updates", "Controlla aggiornamenti");
    m.insert("Accessibility patterns", "Pattern di accessibilit\u{00e0}");
    m.insert("Update available", "Aggiornamento disponibile");
    m.insert(
        "is available. Click to download.",
        "\u{00e8} disponibile. Clicca per scaricare.",
    );
    m.insert("Icon style", "Stile icona");
    m.insert("Number", "Numero");
    m.insert("Ring", "Anello");
    m.insert("Bar", "Barra");
    m.insert("Pie", "Torta");
    m.insert("Dashboard layout", "Layout pannello");
    m.insert("Minimal", "Minimale");
    m.insert("Standard", "Standard");
    m.insert("Detailed", "Dettagliato");
    m.insert("Hide Extra Usage", "Nascondi Extra Usage");
    m
}
