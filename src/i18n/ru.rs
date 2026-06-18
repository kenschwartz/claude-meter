use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "5-часовая сессия");
    m.insert("Weekly (7-day)", "Недельный (7 дней)");
    m.insert("Opus (7-day)", "Opus (7 дней)");
    m.insert("Sonnet (7-day)", "Sonnet (7 дней)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 дней)");
    m.insert("resets in", "сбросится через");
    m.insert("Plan", "План");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code не обнаружен");
    m.insert("credentials_not_found", "Учётные данные не найдены");
    m.insert("connection_error", "Ошибка соединения");
    m.insert("token_expired", "Токен истёк");
    m.insert(
        "token_expired_desc",
        "Ваш OAuth токен истёк. Выполните `claude login` в терминале, чтобы обновить его.",
    );
    m.insert("rate_limited", "Лимит запросов");
    m.insert("server_error", "Ошибка сервера");
    m.insert(
        "server_error_desc",
        "API Anthropic временно недоступно. Повторная попытка будет автоматически.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code установлен, но вход не выполнен. Запустите `claude login` в терминале для подключения аккаунта.",
    );
    m.insert(
        "install_claude_desc",
        "Установите Claude Code и выполните `claude login` для автоматического отслеживания использования.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Установить Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI не предоставляет API для отслеживания использования подписки ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Проверьте использование вручную:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Открыть использование ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Обновить сейчас");
    m.insert("Open Dashboard", "Открыть панель");
    m.insert("Export History (CSV)", "Экспорт истории (CSV)");
    m.insert("Settings", "Настройки");
    m.insert("Start with Windows", "Запускать с Windows");
    m.insert("About", "О программе");
    m.insert("Exit", "Выход");
    m.insert("Last updated:", "Последнее обновление:");
    m.insert("Refresh", "Обновить");
    m.insert("Status", "Статус");
    m.insert("Usage Alert", "Предупреждение об использовании");
    m.insert("Usage Critical", "Критическое использование");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Работает в системном трее. Нажмите на иконку для подробностей.",
    );
    m.insert("Compact mode", "Компактный режим");
    m.insert("Theme", "Тема");
    m.insert("Language", "Язык");
    m.insert("Notifications", "Уведомления");
    m.insert("Dark", "Тёмная");
    m.insert("Light", "Светлая");
    m.insert("Auto", "Авто");
    m.insert("Show ChatGPT section", "Показывать секцию ChatGPT");
    m.insert("Enabled", "Включено");
    m.insert("Sound", "Звук");
    m.insert("Thresholds", "Пороговые значения");
    m.insert("Polling interval", "Интервал обновления");
    m.insert("seconds", "секунд");
    m.insert("Startup", "Запуск");
    m.insert("General", "Общее");
    m.insert("Back", "\u{2190} Назад");
    m.insert("Open Claude.ai \u{2192}", "Открыть Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "История использования");
    m.insert("Usage History (24h)", "История использования (24ч)");
    m.insert("Auto (English)", "Авто (Русский)");
    m.insert("at", "на");
    m.insert("Resets in", "Сбросится через");
    m.insert("Tray icon colors:", "Цвета иконки в трее:");
    m.insert("< 50% usage", "< 50% использования");
    m.insert("50-79% usage", "50\u{2013}79% использования");
    m.insert(">= 80% usage", "\u{2265} 80% использования");
    m.insert("No data", "Нет данных");
    m.insert("exceeded", "превышено");
    m.insert("Show widget", "Показать виджет");
    m.insert("Check for updates", "Проверять обновления");
    m.insert("Accessibility patterns", "Паттерны доступности");
    m.insert("Update available", "Доступно обновление");
    m.insert(
        "is available. Click to download.",
        "доступна. Нажмите для загрузки.",
    );
    m.insert("Icon style", "Стиль иконки");
    m.insert("Number", "Число");
    m.insert("Ring", "Кольцо");
    m.insert("Bar", "Полоса");
    m.insert("Pie", "Круговая");
    m.insert("Dashboard layout", "Макет панели");
    m.insert("Minimal", "Минимальный");
    m.insert("Standard", "Стандартный");
    m.insert("Detailed", "Детальный");
    m.insert("Hide Extra Usage", "Скрыть Extra Usage");
    m
}
