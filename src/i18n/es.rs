use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "Sesión de 5 horas");
    m.insert("Weekly (7-day)", "Semanal (7 días)");
    m.insert("Opus (7-day)", "Opus (7 días)");
    m.insert("Sonnet (7-day)", "Sonnet (7 días)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 días)");
    m.insert("resets in", "se restablece en");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code no detectado");
    m.insert("credentials_not_found", "Credenciales no encontradas");
    m.insert("connection_error", "Error de conexión");
    m.insert("token_expired", "Token expirado");
    m.insert(
        "token_expired_desc",
        "Su token OAuth ha expirado. Ejecute `claude login` en su terminal para renovarlo.",
    );
    m.insert("rate_limited", "Límite de solicitudes");
    m.insert("server_error", "Error del servidor");
    m.insert(
        "server_error_desc",
        "La API de Anthropic no está disponible temporalmente. Se reintentará automáticamente.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code está instalado pero no ha iniciado sesión. Ejecute `claude login` en su terminal para conectar su cuenta.",
    );
    m.insert(
        "install_claude_desc",
        "Instale Claude Code y ejecute `claude login` para habilitar el seguimiento automático.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Instalar Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI no proporciona una API para rastrear el uso de la suscripción de ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Compruebe su uso manualmente:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Abrir uso de ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Actualizar ahora");
    m.insert("Open Dashboard", "Abrir panel");
    m.insert("Export History (CSV)", "Exportar historial (CSV)");
    m.insert("Settings", "Configuración");
    m.insert("Start with Windows", "Iniciar con Windows");
    m.insert("About", "Acerca de");
    m.insert("Exit", "Salir");
    m.insert("Last updated:", "Última actualización:");
    m.insert("Refresh", "Actualizar");
    m.insert("Status", "Estado");
    m.insert("Usage Alert", "Alerta de uso");
    m.insert("Usage Critical", "Uso crítico");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Ejecutándose en la bandeja del sistema. Haz clic en el icono para más detalles.",
    );
    m.insert("Compact mode", "Modo compacto");
    m.insert("Theme", "Tema");
    m.insert("Language", "Idioma");
    m.insert("Notifications", "Notificaciones");
    m.insert("Dark", "Oscuro");
    m.insert("Light", "Claro");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Mostrar sección ChatGPT");
    m.insert("Enabled", "Activado");
    m.insert("Sound", "Sonido");
    m.insert("Thresholds", "Umbrales");
    m.insert("Polling interval", "Intervalo de actualización");
    m.insert("seconds", "segundos");
    m.insert("Startup", "Inicio");
    m.insert("General", "General");
    m.insert("Back", "\u{2190} Volver");
    m.insert("Open Claude.ai \u{2192}", "Abrir Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Historial de uso");
    m.insert("Usage History (24h)", "Historial de uso (24h)");
    m.insert("Auto (English)", "Auto (Español)");
    m.insert("at", "a las");
    m.insert("Resets in", "Se restablece en");
    m.insert("Tray icon colors:", "Colores del icono:");
    m.insert("< 50% usage", "< 50% uso");
    m.insert("50-79% usage", "50\u{2013}79% uso");
    m.insert(">= 80% usage", "\u{2265} 80% uso");
    m.insert("No data", "Sin datos");
    m.insert("exceeded", "superado");
    m.insert("Show widget", "Mostrar widget");
    m.insert("Check for updates", "Buscar actualizaciones");
    m.insert("Accessibility patterns", "Patrones de accesibilidad");
    m.insert("Update available", "Actualizaci\u{00f3}n disponible");
    m.insert(
        "is available. Click to download.",
        "est\u{00e1} disponible. Haz clic para descargar.",
    );
    m.insert("Icon style", "Estilo de icono");
    m.insert("Number", "N\u{00fa}mero");
    m.insert("Ring", "Anillo");
    m.insert("Bar", "Barra");
    m.insert("Pie", "Circular");
    m.insert("Dashboard layout", "Diseño del panel");
    m.insert("Minimal", "Mínimo");
    m.insert("Standard", "Estándar");
    m.insert("Detailed", "Detallado");
    m.insert("Hide Extra Usage", "Ocultar uso extra");
    m
}
