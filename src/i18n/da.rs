use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5-timers session");
    m.insert("Weekly (7-day)", "Ugentlig (7 dage)");
    m.insert("Opus (7-day)", "Opus (7 dage)");
    m.insert("Sonnet (7-day)", "Sonnet (7 dage)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 dage)");
    m.insert("resets in", "nulstilles om");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code ikke fundet");
    m.insert(
        "credentials_not_found",
        "Legitimationsoplysninger ikke fundet",
    );
    m.insert("connection_error", "Forbindelsesfejl");
    m.insert("token_expired", "Token udløbet");
    m.insert(
        "token_expired_desc",
        "Dit OAuth-token er udløbet. Kør `claude login` i terminalen for at forny det.",
    );
    m.insert("rate_limited", "Hastighedsgrænse");
    m.insert("server_error", "Serverfejl");
    m.insert(
        "server_error_desc",
        "Anthropic API er midlertidigt utilgængeligt. Prøver igen automatisk.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code er installeret, men ikke logget ind. K\u{00f8}r `claude login` i din terminal for at forbinde din konto.",
    );
    m.insert(
        "install_claude_desc",
        "Installer Claude Code og k\u{00f8}r `claude login` for at aktivere automatisk forbrugssporing.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Installer Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI tilbyder ikke en API til at spore ChatGPT-abonnementsforbrug.",
    );
    m.insert("Check your usage manually:", "Tjek dit forbrug manuelt:");
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "\u{00c5}bn ChatGPT-forbrug \u{2192}",
    );
    m.insert("Refresh Now", "Opdater nu");
    m.insert("Open Dashboard", "\u{00c5}bn dashboard");
    m.insert("Export History (CSV)", "Eksporter historik (CSV)");
    m.insert("Settings", "Indstillinger");
    m.insert("Start with Windows", "Start med Windows");
    m.insert("About", "Om");
    m.insert("Exit", "Afslut");
    m.insert("Last updated:", "Sidst opdateret:");
    m.insert("Refresh", "Opdater");
    m.insert("Status", "Status");
    m.insert("Usage Alert", "Forbrugsadvarsel");
    m.insert("Usage Critical", "Kritisk forbrug");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "K\u{00f8}rer i systembakken. Klik p\u{00e5} ikonet for detaljer.",
    );
    m.insert("Compact mode", "Kompakt tilstand");
    m.insert("Theme", "Tema");
    m.insert("Language", "Sprog");
    m.insert("Notifications", "Notifikationer");
    m.insert("Dark", "M\u{00f8}rk");
    m.insert("Light", "Lys");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Vis ChatGPT-sektion");
    m.insert("Enabled", "Aktiveret");
    m.insert("Sound", "Lyd");
    m.insert("Thresholds", "Gr\u{00e6}nsev\u{00e6}rdier");
    m.insert("Polling interval", "Opdateringsinterval");
    m.insert("seconds", "sekunder");
    m.insert("Startup", "Opstart");
    m.insert("General", "Generelt");
    m.insert("Back", "\u{2190} Tilbage");
    m.insert("Open Claude.ai \u{2192}", "\u{00c5}bn Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Forbrugshistorik");
    m.insert("Usage History (24h)", "Forbrugshistorik (24t)");
    m.insert("Auto (English)", "Auto (Dansk)");
    m.insert("at", "kl.");
    m.insert("Resets in", "Nulstilles om");
    m.insert("Tray icon colors:", "Bakkeikon-farver:");
    m.insert("< 50% usage", "< 50% forbrug");
    m.insert("50-79% usage", "50\u{2013}79% forbrug");
    m.insert(">= 80% usage", "\u{2265} 80% forbrug");
    m.insert("No data", "Ingen data");
    m.insert("exceeded", "overskredet");
    m.insert("Show widget", "Vis widget");
    m.insert("Check for updates", "S\u{00f8}g efter opdateringer");
    m.insert(
        "Accessibility patterns",
        "Tilg\u{00e6}ngelighedsm\u{00f8}nstre",
    );
    m.insert("Update available", "Opdatering tilg\u{00e6}ngelig");
    m.insert(
        "is available. Click to download.",
        "er tilg\u{00e6}ngelig. Klik for at downloade.",
    );
    m.insert("Icon style", "Ikonstil");
    m.insert("Number", "Tal");
    m.insert("Ring", "Ring");
    m.insert("Bar", "Bjælke");
    m.insert("Pie", "Cirkel");
    m.insert("Dashboard layout", "Panel layout");
    m.insert("Minimal", "Minimal");
    m.insert("Standard", "Standard");
    m.insert("Detailed", "Detaljeret");
    m.insert("Hide Extra Usage", "Skjul Extra Usage");
    m
}
