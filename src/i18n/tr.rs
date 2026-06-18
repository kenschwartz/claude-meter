use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5 saatlik oturum");
    m.insert("Weekly (7-day)", "Haftal\u{0131}k (7 g\u{00fc}n)");
    m.insert("Opus (7-day)", "Opus (7 g\u{00fc}n)");
    m.insert("Sonnet (7-day)", "Sonnet (7 g\u{00fc}n)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 g\u{00fc}n)");
    m.insert("resets in", "s\u{0131}f\u{0131}rlanma");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code bulunamad\u{0131}");
    m.insert(
        "credentials_not_found",
        "Kimlik bilgileri bulunamad\u{0131}",
    );
    m.insert("connection_error", "Bağlantı hatası");
    m.insert("token_expired", "Token süresi doldu");
    m.insert(
        "token_expired_desc",
        "OAuth tokeninizin süresi doldu. Yenilemek için terminalinizde `claude login` komutunu çalıştırın.",
    );
    m.insert("rate_limited", "İstek sınırı");
    m.insert("server_error", "Sunucu hatası");
    m.insert(
        "server_error_desc",
        "Anthropic API geçici olarak kullanılamıyor. Otomatik olarak tekrar denenecek.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code y\u{00fc}kl\u{00fc} ancak giri\u{015f} yap\u{0131}lmam\u{0131}\u{015f}. Terminalinizde `claude login` komutunu \u{00e7}al\u{0131}\u{015f}t\u{0131}r\u{0131}n.",
    );
    m.insert(
        "install_claude_desc",
        "Claude Code y\u{00fc}kleyin ve otomatik izleme i\u{00e7}in `claude login` komutunu \u{00e7}al\u{0131}\u{015f}t\u{0131}r\u{0131}n.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Claude Code Y\u{00fc}kle \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI, ChatGPT abonelik kullan\u{0131}m\u{0131}n\u{0131} izlemek i\u{00e7}in bir API sa\u{011f}lam\u{0131}yor.",
    );
    m.insert(
        "Check your usage manually:",
        "Kullan\u{0131}m\u{0131}n\u{0131}z\u{0131} manuel olarak kontrol edin:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "ChatGPT Kullan\u{0131}m\u{0131}n\u{0131} A\u{00e7} \u{2192}",
    );
    m.insert("Refresh Now", "\u{015e}imdi Yenile");
    m.insert("Open Dashboard", "Paneli A\u{00e7}");
    m.insert(
        "Export History (CSV)",
        "Ge\u{00e7}mi\u{015f}i D\u{0131}\u{015f}a Aktar (CSV)",
    );
    m.insert("Settings", "Ayarlar");
    m.insert("Start with Windows", "Windows ile ba\u{015f}lat");
    m.insert("About", "Hakk\u{0131}nda");
    m.insert("Exit", "\u{00c7}\u{0131}k\u{0131}\u{015f}");
    m.insert("Last updated:", "Son g\u{00fc}ncelleme:");
    m.insert("Refresh", "Yenile");
    m.insert("Status", "Durum");
    m.insert("Usage Alert", "Kullan\u{0131}m Uyar\u{0131}s\u{0131}");
    m.insert("Usage Critical", "Kritik Kullan\u{0131}m");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Sistem tepsisinde \u{00e7}al\u{0131}\u{015f}\u{0131}yor. Ayr\u{0131}nt\u{0131}lar i\u{00e7}in simgeye t\u{0131}klay\u{0131}n.",
    );
    m.insert("Compact mode", "Kompakt mod");
    m.insert("Theme", "Tema");
    m.insert("Language", "Dil");
    m.insert("Notifications", "Bildirimler");
    m.insert("Dark", "Koyu");
    m.insert("Light", "A\u{00e7}\u{0131}k");
    m.insert("Auto", "Otomatik");
    m.insert(
        "Show ChatGPT section",
        "ChatGPT b\u{00f6}l\u{00fc}m\u{00fc}n\u{00fc} g\u{00f6}ster",
    );
    m.insert("Enabled", "Etkin");
    m.insert("Sound", "Ses");
    m.insert("Thresholds", "E\u{015f}ikler");
    m.insert("Polling interval", "Sorgulama aral\u{0131}\u{011f}\u{0131}");
    m.insert("seconds", "saniye");
    m.insert("Startup", "Ba\u{015f}lang\u{0131}\u{00e7}");
    m.insert("General", "Genel");
    m.insert("Back", "\u{2190} Geri");
    m.insert("Open Claude.ai \u{2192}", "Claude.ai A\u{00e7} \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Kullanım Geçmişi");
    m.insert(
        "Usage History (24h)",
        "Kullan\u{0131}m Ge\u{00e7}mi\u{015f}i (24s)",
    );
    m.insert("Auto (English)", "Otomatik (T\u{00fc}rk\u{00e7}e)");
    m.insert("at", "saat");
    m.insert("Resets in", "S\u{0131}f\u{0131}rlanma");
    m.insert("Tray icon colors:", "Tepsi simgesi renkleri:");
    m.insert("< 50% usage", "< 50% kullan\u{0131}m");
    m.insert("50-79% usage", "50\u{2013}79% kullan\u{0131}m");
    m.insert(">= 80% usage", "\u{2265} 80% kullan\u{0131}m");
    m.insert("No data", "Veri yok");
    m.insert("exceeded", "a\u{015f}\u{0131}ld\u{0131}");
    m.insert("Show widget", "Widget g\u{00f6}ster");
    m.insert("Check for updates", "G\u{00fc}ncellemeleri kontrol et");
    m.insert("Accessibility patterns", "Eri\u{015f}ilebilirlik desenleri");
    m.insert("Update available", "G\u{00fc}ncelleme mevcut");
    m.insert(
        "is available. Click to download.",
        "mevcut. \u{0130}ndirmek i\u{00e7}in t\u{0131}klay\u{0131}n.",
    );
    m.insert("Icon style", "Simge stili");
    m.insert("Number", "Say\u{0131}");
    m.insert("Ring", "Halka");
    m.insert("Bar", "\u{00c7}ubuk");
    m.insert("Pie", "Pasta");
    m.insert("Dashboard layout", "Panel düzeni");
    m.insert("Minimal", "Minimal");
    m.insert("Standard", "Standart");
    m.insert("Detailed", "Detaylı");
    m.insert("Hide Extra Usage", "Extra Usage gizle");
    m
}
