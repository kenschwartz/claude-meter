use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "Sesi 5 jam");
    m.insert("Weekly (7-day)", "Mingguan (7 hari)");
    m.insert("Opus (7-day)", "Opus (7 hari)");
    m.insert("Sonnet (7-day)", "Sonnet (7 hari)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 hari)");
    m.insert("resets in", "set semula dalam");
    m.insert("Plan", "Pelan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code tidak dikesan");
    m.insert("credentials_not_found", "Kelayakan tidak dijumpai");
    m.insert("connection_error", "Ralat sambungan");
    m.insert("token_expired", "Token tamat tempoh");
    m.insert(
        "token_expired_desc",
        "Token OAuth anda telah tamat tempoh. Jalankan `claude login` di terminal.",
    );
    m.insert("rate_limited", "Had kadar");
    m.insert("server_error", "Ralat pelayan");
    m.insert(
        "server_error_desc",
        "Anthropic API tidak tersedia buat sementara. Akan cuba semula secara automatik.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code dipasang tetapi tidak disambungkan. Jalankan `claude login`.",
    );
    m.insert(
        "install_claude_desc",
        "Pasang Claude Code dan jalankan `claude login`.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Pasang Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI tidak menyediakan API untuk menjejak penggunaan langganan ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Semak penggunaan secara manual:",
    );
    m.insert("Open ChatGPT Usage \u{2192}", "Buka ChatGPT Usage \u{2192}");
    m.insert("Refresh Now", "Muat semula sekarang");
    m.insert("Open Dashboard", "Buka papan pemuka");
    m.insert("Export History (CSV)", "Eksport sejarah (CSV)");
    m.insert("Settings", "Tetapan");
    m.insert("Start with Windows", "Mulakan dengan Windows");
    m.insert("About", "Perihal");
    m.insert("Exit", "Keluar");
    m.insert("Last updated:", "Kemas kini terakhir:");
    m.insert("Refresh", "Muat semula");
    m.insert("Status", "Status");
    m.insert("Usage Alert", "Amaran penggunaan");
    m.insert("Usage Critical", "Penggunaan kritikal");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Berjalan di system tray. Klik ikon untuk butiran.",
    );
    m.insert("Compact mode", "Mod padat");
    m.insert("Theme", "Tema");
    m.insert("Language", "Bahasa");
    m.insert("Notifications", "Pemberitahuan");
    m.insert("Dark", "Gelap");
    m.insert("Light", "Cerah");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Tunjuk bahagian ChatGPT");
    m.insert("Enabled", "Diaktifkan");
    m.insert("Sound", "Bunyi");
    m.insert("Thresholds", "Ambang");
    m.insert("Polling interval", "Selang tinjauan");
    m.insert("seconds", "saat");
    m.insert("Startup", "Permulaan");
    m.insert("General", "Umum");
    m.insert("Back", "\u{2190} Kembali");
    m.insert("Open Claude.ai \u{2192}", "Buka Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Sejarah penggunaan");
    m.insert("Usage History (24h)", "Sejarah penggunaan (24j)");
    m.insert("Auto (English)", "Auto (English)");
    m.insert("at", "pada");
    m.insert("Resets in", "Set semula dalam");
    m.insert("Tray icon colors:", "Warna ikon tray:");
    m.insert("< 50% usage", "< 50%");
    m.insert("50-79% usage", "50\u{2013}79%");
    m.insert(">= 80% usage", "\u{2265} 80%");
    m.insert("No data", "Tiada data");
    m.insert("exceeded", "melebihi");
    m.insert("Show widget", "Tunjuk widget");
    m.insert("Check for updates", "Semak kemas kini");
    m.insert("Accessibility patterns", "Corak kebolehcapaian");
    m.insert("Update available", "Kemas kini tersedia");
    m.insert(
        "is available. Click to download.",
        "tersedia. Klik untuk muat turun.",
    );
    m.insert("Icon style", "Gaya ikon");
    m.insert("Number", "Nombor");
    m.insert("Ring", "Cincin");
    m.insert("Bar", "Bar");
    m.insert("Pie", "Pai");
    m.insert("Dashboard layout", "Susun atur papan pemuka");
    m.insert("Minimal", "Minimal");
    m.insert("Standard", "Standard");
    m.insert("Detailed", "Terperinci");
    m.insert("Hide Extra Usage", "Sembunyikan Extra Usage");
    m
}
