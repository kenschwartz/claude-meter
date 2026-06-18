use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5 \u{00f3}r\u{00e1}s munkamenet");
    m.insert("Weekly (7-day)", "Heti (7 nap)");
    m.insert("Opus (7-day)", "Opus (7 nap)");
    m.insert("Sonnet (7-day)", "Sonnet (7 nap)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 nap)");
    m.insert("resets in", "vissza\u{00e1}ll");
    m.insert("Plan", "Csomag");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert(
        "Claude Code not detected",
        "Claude Code nem tal\u{00e1}lhat\u{00f3}",
    );
    m.insert(
        "credentials_not_found",
        "Hiteles\u{00ed}t\u{0151} adatok nem tal\u{00e1}lhat\u{00f3}k",
    );
    m.insert("connection_error", "Csatlakoz\u{00e1}si hiba");
    m.insert("token_expired", "Token lejárt");
    m.insert(
        "token_expired_desc",
        "Az OAuth tokenje lejárt. Futtassa a `claude login` parancsot a terminálban a megújításhoz.",
    );
    m.insert("rate_limited", "Kéréskorlát");
    m.insert("server_error", "Szerverhiba");
    m.insert(
        "server_error_desc",
        "Az Anthropic API átmenetileg nem érhető el. Automatikusan újrapróbálkozik.",
    );
    m.insert(
        "run_claude_login_desc",
        "A Claude Code telep\u{00ed}tve van, de nincs bejelentkezve. Futtasd a `claude login` parancsot a termin\u{00e1}lban a fi\u{00f3}kod \u{00f6}sszekapcsol\u{00e1}s\u{00e1}hoz.",
    );
    m.insert(
        "install_claude_desc",
        "Telep\u{00ed}tsd a Claude Code-ot \u{00e9}s futtasd a `claude login` parancsot az automatikus haszn\u{00e1}latk\u{00f6}vet\u{00e9}s enged\u{00e9}lyez\u{00e9}s\u{00e9}hez.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Claude Code telep\u{00ed}t\u{00e9}se \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "Az OpenAI nem biztos\u{00ed}t API-t a ChatGPT el\u{0151}fizet\u{00e9}si haszn\u{00e1}lat k\u{00f6}vet\u{00e9}s\u{00e9}hez.",
    );
    m.insert(
        "Check your usage manually:",
        "Ellen\u{0151}rizd a haszn\u{00e1}latod manu\u{00e1}lisan:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "ChatGPT haszn\u{00e1}lat megnyit\u{00e1}sa \u{2192}",
    );
    m.insert("Refresh Now", "Friss\u{00ed}t\u{00e9}s most");
    m.insert(
        "Open Dashboard",
        "Vez\u{00e9}rl\u{0151}pult megnyit\u{00e1}sa",
    );
    m.insert(
        "Export History (CSV)",
        "El\u{0151}zm\u{00e9}nyek export\u{00e1}l\u{00e1}sa (CSV)",
    );
    m.insert("Settings", "Be\u{00e1}ll\u{00ed}t\u{00e1}sok");
    m.insert("Start with Windows", "Ind\u{00ed}t\u{00e1}s a Windows-szal");
    m.insert("About", "N\u{00e9}vjegy");
    m.insert("Exit", "Kil\u{00e9}p\u{00e9}s");
    m.insert("Last updated:", "Utols\u{00f3} friss\u{00ed}t\u{00e9}s:");
    m.insert("Refresh", "Friss\u{00ed}t\u{00e9}s");
    m.insert("Status", "\u{00c1}llapot");
    m.insert("Usage Alert", "Haszn\u{00e1}lati figyelmeztet\u{00e9}s");
    m.insert("Usage Critical", "Kritikus haszn\u{00e1}lat");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Fut a t\u{00e1}lc\u{00e1}n. Kattints az ikonra a r\u{00e9}szletekhez.",
    );
    m.insert("Compact mode", "Kompakt m\u{00f3}d");
    m.insert("Theme", "T\u{00e9}ma");
    m.insert("Language", "Nyelv");
    m.insert("Notifications", "\u{00c9}rtes\u{00ed}t\u{00e9}sek");
    m.insert("Dark", "S\u{00f6}t\u{00e9}t");
    m.insert("Light", "Vil\u{00e1}gos");
    m.insert("Auto", "Auto");
    m.insert(
        "Show ChatGPT section",
        "ChatGPT szekci\u{00f3} megjelen\u{00ed}t\u{00e9}se",
    );
    m.insert("Enabled", "Enged\u{00e9}lyezve");
    m.insert("Sound", "Hang");
    m.insert("Thresholds", "K\u{00fc}sz\u{00f6}b\u{00e9}rt\u{00e9}kek");
    m.insert(
        "Polling interval",
        "Friss\u{00ed}t\u{00e9}si id\u{0151}k\u{00f6}z",
    );
    m.insert("seconds", "m\u{00e1}sodperc");
    m.insert("Startup", "Ind\u{00ed}t\u{00e1}s");
    m.insert("General", "\u{00c1}ltal\u{00e1}nos");
    m.insert("Back", "\u{2190} Vissza");
    m.insert(
        "Open Claude.ai \u{2192}",
        "Claude.ai megnyit\u{00e1}sa \u{2192}",
    );
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Használati előzmények");
    m.insert(
        "Usage History (24h)",
        "Haszn\u{00e1}lati el\u{0151}zm\u{00e9}nyek (24\u{00f3})",
    );
    m.insert("Auto (English)", "Auto (Magyar)");
    m.insert("at", "-kor");
    m.insert("Resets in", "Vissza\u{00e1}ll");
    m.insert("Tray icon colors:", "T\u{00e1}lca ikon sz\u{00ed}nei:");
    m.insert("< 50% usage", "< 50% haszn\u{00e1}lat");
    m.insert("50-79% usage", "50\u{2013}79% haszn\u{00e1}lat");
    m.insert(">= 80% usage", "\u{2265} 80% haszn\u{00e1}lat");
    m.insert("No data", "Nincs adat");
    m.insert("exceeded", "t\u{00fa}ll\u{00e9}pve");
    m.insert("Show widget", "Widget megjelen\u{00ed}t\u{00e9}se");
    m.insert(
        "Check for updates",
        "Friss\u{00ed}t\u{00e9}sek keres\u{00e9}se",
    );
    m.insert(
        "Accessibility patterns",
        "Akad\u{00e1}lymentes\u{00ed}t\u{00e9}si mint\u{00e1}k",
    );
    m.insert(
        "Update available",
        "Friss\u{00ed}t\u{00e9}s el\u{00e9}rhet\u{0151}",
    );
    m.insert(
        "is available. Click to download.",
        "el\u{00e9}rhet\u{0151}. Kattints a let\u{00f6}lt\u{00e9}shez.",
    );
    m.insert("Icon style", "Ikon st\u{00ed}lus");
    m.insert("Number", "Sz\u{00e1}m");
    m.insert("Ring", "Gy\u{0171}r\u{0171}");
    m.insert("Bar", "S\u{00e1}v");
    m.insert("Pie", "Kör");
    m.insert("Dashboard layout", "Panel elrendezés");
    m.insert("Minimal", "Minimális");
    m.insert("Standard", "Alapértelmezett");
    m.insert("Detailed", "Részletes");
    m.insert("Hide Extra Usage", "Extra Usage elrejtése");
    m
}
