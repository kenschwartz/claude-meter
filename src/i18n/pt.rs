use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "Sess\u{00e3}o de 5 horas");
    m.insert("Weekly (7-day)", "Semanal (7 dias)");
    m.insert("Opus (7-day)", "Opus (7 dias)");
    m.insert("Sonnet (7-day)", "Sonnet (7 dias)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 dias)");
    m.insert("resets in", "reinicia em");
    m.insert("Plan", "Plano");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert(
        "Claude Code not detected",
        "Claude Code n\u{00e3}o detectado",
    );
    m.insert(
        "credentials_not_found",
        "Credenciais n\u{00e3}o encontradas",
    );
    m.insert("connection_error", "Erro de conexão");
    m.insert("token_expired", "Token expirado");
    m.insert(
        "token_expired_desc",
        "Seu token OAuth expirou. Execute `claude login` no terminal para renová-lo.",
    );
    m.insert("rate_limited", "Limite de requisições");
    m.insert("server_error", "Erro do servidor");
    m.insert(
        "server_error_desc",
        "A API da Anthropic está temporariamente indisponível. Tentará novamente automaticamente.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code est\u{00e1} instalado mas n\u{00e3}o conectado. Execute `claude login` no terminal para conectar sua conta.",
    );
    m.insert(
        "install_claude_desc",
        "Instale o Claude Code e execute `claude login` para ativar o monitoramento autom\u{00e1}tico de uso.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Instalar Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "A OpenAI n\u{00e3}o oferece uma API para rastrear o uso da assinatura do ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Verifique seu uso manualmente:",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Abrir uso do ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Atualizar agora");
    m.insert("Open Dashboard", "Abrir painel");
    m.insert("Export History (CSV)", "Exportar hist\u{00f3}rico (CSV)");
    m.insert("Settings", "Configura\u{00e7}\u{00f5}es");
    m.insert("Start with Windows", "Iniciar com o Windows");
    m.insert("About", "Sobre");
    m.insert("Exit", "Sair");
    m.insert("Last updated:", "\u{00da}ltima atualiza\u{00e7}\u{00e3}o:");
    m.insert("Refresh", "Atualizar");
    m.insert("Status", "Status");
    m.insert("Usage Alert", "Alerta de uso");
    m.insert("Usage Critical", "Uso cr\u{00ed}tico");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Executando na bandeja do sistema. Clique no \u{00ed}cone para detalhes.",
    );
    m.insert("Compact mode", "Modo compacto");
    m.insert("Theme", "Tema");
    m.insert("Language", "Idioma");
    m.insert("Notifications", "Notifica\u{00e7}\u{00f5}es");
    m.insert("Dark", "Escuro");
    m.insert("Light", "Claro");
    m.insert("Auto", "Auto");
    m.insert(
        "Show ChatGPT section",
        "Mostrar se\u{00e7}\u{00e3}o ChatGPT",
    );
    m.insert("Enabled", "Ativado");
    m.insert("Sound", "Som");
    m.insert("Thresholds", "Limites");
    m.insert("Polling interval", "Intervalo de consulta");
    m.insert("seconds", "segundos");
    m.insert("Startup", "Inicializa\u{00e7}\u{00e3}o");
    m.insert("General", "Geral");
    m.insert("Back", "\u{2190} Voltar");
    m.insert("Open Claude.ai \u{2192}", "Abrir Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Histórico de uso");
    m.insert("Usage History (24h)", "Hist\u{00f3}rico de uso (24h)");
    m.insert("Auto (English)", "Auto (Portugu\u{00ea}s)");
    m.insert("at", "em");
    m.insert("Resets in", "Reinicia em");
    m.insert("Tray icon colors:", "Cores do \u{00ed}cone na bandeja:");
    m.insert("< 50% usage", "< 50% uso");
    m.insert("50-79% usage", "50\u{2013}79% uso");
    m.insert(">= 80% usage", "\u{2265} 80% uso");
    m.insert("No data", "Sem dados");
    m.insert("exceeded", "excedido");
    m.insert("Show widget", "Mostrar widget");
    m.insert("Check for updates", "Verificar atualiza\u{00e7}\u{00f5}es");
    m.insert("Accessibility patterns", "Padr\u{00f5}es de acessibilidade");
    m.insert(
        "Update available",
        "Atualiza\u{00e7}\u{00e3}o dispon\u{00ed}vel",
    );
    m.insert(
        "is available. Click to download.",
        "est\u{00e1} dispon\u{00ed}vel. Clique para baixar.",
    );
    m.insert("Icon style", "Estilo do \u{00ed}cone");
    m.insert("Number", "N\u{00fa}mero");
    m.insert("Ring", "Anel");
    m.insert("Bar", "Barra");
    m.insert("Pie", "Pizza");
    m.insert("Dashboard layout", "Layout do painel");
    m.insert("Minimal", "Mínimo");
    m.insert("Standard", "Padrão");
    m.insert("Detailed", "Detalhado");
    m.insert("Hide Extra Usage", "Ocultar uso extra");
    m
}
