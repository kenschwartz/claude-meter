use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5 tunnin istunto");
    m.insert(
        "Weekly (7-day)",
        "Viikoittainen (7 p\u{00e4}iv\u{00e4}\u{00e4})",
    );
    m.insert("Opus (7-day)", "Opus (7 p\u{00e4}iv\u{00e4}\u{00e4})");
    m.insert("Sonnet (7-day)", "Sonnet (7 p\u{00e4}iv\u{00e4}\u{00e4})");
    m.insert(
        "OAuth Apps (7-day)",
        "OAuth Apps (7 p\u{00e4}iv\u{00e4}\u{00e4})",
    );
    m.insert("resets in", "nollautuu");
    m.insert("Plan", "Tilaus");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Codea ei havaittu");
    m.insert(
        "credentials_not_found",
        "Tunnistetietoja ei l\u{00f6}ytynyt",
    );
    m.insert("connection_error", "Yhteysvirhe");
    m.insert("token_expired", "Token vanhentunut");
    m.insert(
        "token_expired_desc",
        "OAuth-tokenisi on vanhentunut. Suorita `claude login` terminaalissa uusiaksesi sen.",
    );
    m.insert("rate_limited", "Pyyntöraja");
    m.insert("server_error", "Palvelinvirhe");
    m.insert(
        "server_error_desc",
        "Anthropic API ei ole tilapäisesti käytettävissä. Yritetään automaattisesti uudelleen.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code on asennettu, mutta sis\u{00e4}\u{00e4}nkirjautumista ei ole tehty. Suorita `claude login` terminaalissa yhdist\u{00e4}\u{00e4}ksesi tilisi.",
    );
    m.insert(
        "install_claude_desc",
        "Asenna Claude Code ja suorita `claude login` ottaaksesi automaattisen k\u{00e4}yt\u{00f6}n seurannan k\u{00e4}ytt\u{00f6}\u{00f6}n.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Asenna Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI ei tarjoa rajapintaa ChatGPT-tilauksen k\u{00e4}yt\u{00f6}n seurantaan.",
    );
    m.insert(
        "Check your usage manually:",
        "Tarkista k\u{00e4}ytt\u{00f6}si manuaalisesti:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Avaa ChatGPT-k\u{00e4}ytt\u{00f6} \u{2192}",
    );
    m.insert("Refresh Now", "P\u{00e4}ivit\u{00e4} nyt");
    m.insert("Open Dashboard", "Avaa hallintapaneeli");
    m.insert("Export History (CSV)", "Vie historia (CSV)");
    m.insert("Settings", "Asetukset");
    m.insert(
        "Start with Windows",
        "K\u{00e4}ynnist\u{00e4} Windowsin kanssa",
    );
    m.insert("About", "Tietoja");
    m.insert("Exit", "Poistu");
    m.insert("Last updated:", "Viimeksi p\u{00e4}ivitetty:");
    m.insert("Refresh", "P\u{00e4}ivit\u{00e4}");
    m.insert("Status", "Tila");
    m.insert("Usage Alert", "K\u{00e4}ytt\u{00f6}varoitus");
    m.insert("Usage Critical", "Kriittinen k\u{00e4}ytt\u{00f6}");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Toimii ilmaisinalueella. Napsauta kuvaketta n\u{00e4}hd\u{00e4}ksesi tiedot.",
    );
    m.insert("Compact mode", "Kompakti tila");
    m.insert("Theme", "Teema");
    m.insert("Language", "Kieli");
    m.insert("Notifications", "Ilmoitukset");
    m.insert("Dark", "Tumma");
    m.insert("Light", "Vaalea");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "N\u{00e4}yt\u{00e4} ChatGPT-osio");
    m.insert("Enabled", "K\u{00e4}yt\u{00f6}ss\u{00e4}");
    m.insert("Sound", "\u{00c4}\u{00e4}ni");
    m.insert("Thresholds", "Kynnysarvot");
    m.insert("Polling interval", "P\u{00e4}ivitysv\u{00e4}li");
    m.insert("seconds", "sekuntia");
    m.insert("Startup", "K\u{00e4}ynnistys");
    m.insert("General", "Yleiset");
    m.insert("Back", "\u{2190} Takaisin");
    m.insert("Open Claude.ai \u{2192}", "Avaa Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Käyttöhistoria");
    m.insert("Usage History (24h)", "K\u{00e4}ytt\u{00f6}historia (24h)");
    m.insert("Auto (English)", "Auto (Suomi)");
    m.insert("at", "klo");
    m.insert("Resets in", "Nollautuu");
    m.insert(
        "Tray icon colors:",
        "Ilmaisinalueen kuvakkeen v\u{00e4}rit:",
    );
    m.insert("< 50% usage", "< 50% k\u{00e4}ytt\u{00f6}");
    m.insert("50-79% usage", "50\u{2013}79% k\u{00e4}ytt\u{00f6}");
    m.insert(">= 80% usage", "\u{2265} 80% k\u{00e4}ytt\u{00f6}");
    m.insert("No data", "Ei tietoja");
    m.insert("exceeded", "ylitetty");
    m.insert("Show widget", "N\u{00e4}yt\u{00e4} widget");
    m.insert("Check for updates", "Tarkista p\u{00e4}ivitykset");
    m.insert("Accessibility patterns", "Saavutettavuuskuviot");
    m.insert("Update available", "P\u{00e4}ivitys saatavilla");
    m.insert(
        "is available. Click to download.",
        "on saatavilla. Napsauta ladataksesi.",
    );
    m.insert("Icon style", "Kuvaketyyli");
    m.insert("Number", "Numero");
    m.insert("Ring", "Rengas");
    m.insert("Bar", "Palkki");
    m.insert("Pie", "Ympyrä");
    m.insert("Dashboard layout", "Paneelin asettelu");
    m.insert("Minimal", "Minimaalinen");
    m.insert("Standard", "Vakio");
    m.insert("Detailed", "Yksityiskohtainen");
    m.insert("Hide Extra Usage", "Piilota Extra Usage");
    m
}
