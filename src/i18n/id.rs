use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "Sesi 5 jam");
    m.insert("Weekly (7-day)", "Mingguan (7 hari)");
    m.insert("Opus (7-day)", "Opus (7 hari)");
    m.insert("Sonnet (7-day)", "Sonnet (7 hari)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 hari)");
    m.insert("resets in", "direset dalam");
    m.insert("Plan", "Paket");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code tidak terdeteksi");
    m.insert("credentials_not_found", "Kredensial tidak ditemukan");
    m.insert("connection_error", "Kesalahan koneksi");
    m.insert("token_expired", "Token kedaluwarsa");
    m.insert(
        "token_expired_desc",
        "Token OAuth Anda telah kedaluwarsa. Jalankan `claude login` di terminal untuk memperbaruinya.",
    );
    m.insert("rate_limited", "Batas permintaan");
    m.insert("server_error", "Kesalahan server");
    m.insert(
        "server_error_desc",
        "API Anthropic sementara tidak tersedia. Akan dicoba ulang secara otomatis.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code terinstal tetapi belum login. Jalankan `claude login` di terminal untuk menghubungkan akun.",
    );
    m.insert(
        "install_claude_desc",
        "Instal Claude Code dan jalankan `claude login` untuk mengaktifkan pelacakan penggunaan otomatis.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Instal Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI tidak menyediakan API untuk melacak penggunaan langganan ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Periksa penggunaan Anda secara manual:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Buka Penggunaan ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Segarkan Sekarang");
    m.insert("Open Dashboard", "Buka Dasbor");
    m.insert("Export History (CSV)", "Ekspor Riwayat (CSV)");
    m.insert("Settings", "Pengaturan");
    m.insert("Start with Windows", "Mulai dengan Windows");
    m.insert("About", "Tentang");
    m.insert("Exit", "Keluar");
    m.insert("Last updated:", "Terakhir diperbarui:");
    m.insert("Refresh", "Segarkan");
    m.insert("Status", "Status");
    m.insert("Usage Alert", "Peringatan Penggunaan");
    m.insert("Usage Critical", "Penggunaan Kritis");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Berjalan di system tray. Klik ikon untuk detail.",
    );
    m.insert("Compact mode", "Mode ringkas");
    m.insert("Theme", "Tema");
    m.insert("Language", "Bahasa");
    m.insert("Notifications", "Notifikasi");
    m.insert("Dark", "Gelap");
    m.insert("Light", "Terang");
    m.insert("Auto", "Otomatis");
    m.insert("Show ChatGPT section", "Tampilkan bagian ChatGPT");
    m.insert("Enabled", "Aktif");
    m.insert("Sound", "Suara");
    m.insert("Thresholds", "Ambang batas");
    m.insert("Polling interval", "Interval pembaruan");
    m.insert("seconds", "detik");
    m.insert("Startup", "Mulai");
    m.insert("General", "Umum");
    m.insert("Back", "\u{2190} Kembali");
    m.insert("Open Claude.ai \u{2192}", "Buka Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Riwayat Penggunaan");
    m.insert("Usage History (24h)", "Riwayat Penggunaan (24j)");
    m.insert("Auto (English)", "Otomatis (Bahasa Indonesia)");
    m.insert("at", "pada");
    m.insert("Resets in", "Direset dalam");
    m.insert("Tray icon colors:", "Warna ikon tray:");
    m.insert("< 50% usage", "< 50% penggunaan");
    m.insert("50-79% usage", "50\u{2013}79% penggunaan");
    m.insert(">= 80% usage", "\u{2265} 80% penggunaan");
    m.insert("No data", "Tidak ada data");
    m.insert("exceeded", "terlampaui");
    m.insert("Show widget", "Tampilkan widget");
    m.insert("Check for updates", "Periksa pembaruan");
    m.insert("Accessibility patterns", "Pola aksesibilitas");
    m.insert("Update available", "Pembaruan tersedia");
    m.insert(
        "is available. Click to download.",
        "tersedia. Klik untuk mengunduh.",
    );
    m.insert("Icon style", "Gaya ikon");
    m.insert("Number", "Angka");
    m.insert("Ring", "Cincin");
    m.insert("Bar", "Batang");
    m.insert("Pie", "Lingkaran");
    m.insert("Dashboard layout", "Tata letak dasbor");
    m.insert("Minimal", "Minimal");
    m.insert("Standard", "Standar");
    m.insert("Detailed", "Detail");
    m.insert("Hide Extra Usage", "Sembunyikan Extra Usage");
    m
}
