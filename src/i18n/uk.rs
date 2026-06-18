use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5-годинна сесія");
    m.insert("Weekly (7-day)", "Тижневий (7 днів)");
    m.insert("Opus (7-day)", "Opus (7 днів)");
    m.insert("Sonnet (7-day)", "Sonnet (7 днів)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 днів)");
    m.insert("resets in", "скидається через");
    m.insert("Plan", "План");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code не знайдено");
    m.insert("credentials_not_found", "Облікові дані не знайдено");
    m.insert("connection_error", "Помилка з'єднання");
    m.insert("token_expired", "Токен протермінувався");
    m.insert(
        "token_expired_desc",
        "Ваш OAuth токен протермінувався. Виконайте `claude login` у терміналі, щоб оновити його.",
    );
    m.insert("rate_limited", "Ліміт запитів");
    m.insert("server_error", "Помилка сервера");
    m.insert(
        "server_error_desc",
        "API Anthropic тимчасово недоступне. Повторна спроба буде автоматично.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code встановлено, але вхід не виконано. Запустіть `claude login` у терміналі, щоб підключити обліковий запис.",
    );
    m.insert(
        "install_claude_desc",
        "Встановіть Claude Code та виконайте `claude login` для автоматичного відстеження використання.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Встановити Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI не надає API для відстеження використання підписки ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Перевірте використання вручну:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Відкрити використання ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Оновити зараз");
    m.insert("Open Dashboard", "Відкрити панель");
    m.insert("Export History (CSV)", "Експорт історії (CSV)");
    m.insert("Settings", "Налаштування");
    m.insert("Start with Windows", "Запускати з Windows");
    m.insert("About", "Про програму");
    m.insert("Exit", "Вихід");
    m.insert("Last updated:", "Останнє оновлення:");
    m.insert("Refresh", "Оновити");
    m.insert("Status", "Статус");
    m.insert("Usage Alert", "Попередження використання");
    m.insert("Usage Critical", "Критичне використання");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Працює в системному треї. Натисніть на іконку для деталей.",
    );
    m.insert("Compact mode", "Компактний режим");
    m.insert("Theme", "Тема");
    m.insert("Language", "Мова");
    m.insert("Notifications", "Сповіщення");
    m.insert("Dark", "Темна");
    m.insert("Light", "Світла");
    m.insert("Auto", "Авто");
    m.insert("Show ChatGPT section", "Показувати секцію ChatGPT");
    m.insert("Enabled", "Увімкнено");
    m.insert("Sound", "Звук");
    m.insert("Thresholds", "Порогові значення");
    m.insert("Polling interval", "Інтервал оновлення");
    m.insert("seconds", "секунд");
    m.insert("Startup", "Запуск");
    m.insert("General", "Загальне");
    m.insert("Back", "\u{2190} Назад");
    m.insert("Open Claude.ai \u{2192}", "Відкрити Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Історія використання");
    m.insert("Usage History (24h)", "Історія використання (24г)");
    m.insert("Auto (English)", "Авто (Українська)");
    m.insert("at", "на");
    m.insert("Resets in", "Скидається через");
    m.insert("Tray icon colors:", "Кольори іконки в треї:");
    m.insert("< 50% usage", "< 50% використання");
    m.insert("50-79% usage", "50\u{2013}79% використання");
    m.insert(">= 80% usage", "\u{2265} 80% використання");
    m.insert("No data", "Немає даних");
    m.insert("exceeded", "перевищено");
    m.insert("Show widget", "Показати віджет");
    m.insert("Check for updates", "Перевіряти оновлення");
    m.insert("Accessibility patterns", "Патерни доступності");
    m.insert("Update available", "Доступне оновлення");
    m.insert(
        "is available. Click to download.",
        "доступна. Натисніть для завантаження.",
    );
    m.insert("Icon style", "Стиль іконки");
    m.insert("Number", "Число");
    m.insert("Ring", "Кільце");
    m.insert("Bar", "Смужка");
    m.insert("Pie", "Секторна");
    m.insert("Dashboard layout", "Макет панелі");
    m.insert("Minimal", "Мінімальний");
    m.insert("Standard", "Стандартний");
    m.insert("Detailed", "Детальний");
    m.insert("Hide Extra Usage", "Сховати Extra Usage");
    m.insert("Show startup notification", "Сповіщення про запуск");
    m.insert("Show login expiry warning", "Попередження про вхід Claude");
    m
}
