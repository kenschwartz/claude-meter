use std::collections::HashMap;

pub fn strings() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("5-hour session", "Session de 5 heures");
    m.insert("Weekly (7-day)", "Hebdomadaire (7 jours)");
    m.insert("Opus (7-day)", "Opus (7 jours)");
    m.insert("Sonnet (7-day)", "Sonnet (7 jours)");
    m.insert("OAuth Apps (7-day)", "OAuth Apps (7 jours)");
    m.insert("resets in", "réinitialisation dans");
    m.insert("Plan", "Plan");
    m.insert("Pro", "Pro");
    m.insert("Max", "Max");
    m.insert("Claude Code not detected", "Claude Code non détecté");
    m.insert("credentials_not_found", "Identifiants non trouvés");
    m.insert("connection_error", "Erreur de connexion");
    m.insert("token_expired", "Token expiré");
    m.insert(
        "token_expired_desc",
        "Votre token OAuth a expiré. Exécutez `claude login` dans votre terminal pour le renouveler.",
    );
    m.insert("rate_limited", "Limite de requêtes");
    m.insert("server_error", "Erreur serveur");
    m.insert(
        "server_error_desc",
        "L'API Anthropic est temporairement indisponible. Nouvelle tentative automatique.",
    );
    m.insert(
        "run_claude_login_desc",
        "Claude Code est installé mais non connecté. Exécutez `claude login` dans votre terminal pour connecter votre compte.",
    );
    m.insert(
        "install_claude_desc",
        "Installez Claude Code et exécutez `claude login` pour activer le suivi automatique.",
    );
    m.insert(
        "Install Claude Code \u{2192}",
        "Installer Claude Code \u{2192}",
    );
    m.insert(
        "openai_no_api",
        "OpenAI ne fournit pas d'API pour suivre l'utilisation de l'abonnement ChatGPT.",
    );
    m.insert(
        "Check your usage manually:",
        "Vérifiez votre utilisation manuellement :",
    );
    m.insert(
        "Open ChatGPT Usage \u{2192}",
        "Ouvrir l'utilisation ChatGPT \u{2192}",
    );
    m.insert("Refresh Now", "Actualiser maintenant");
    m.insert("Open Dashboard", "Ouvrir le tableau de bord");
    m.insert("Export History (CSV)", "Exporter l'historique (CSV)");
    m.insert("Settings", "Paramètres");
    m.insert("Start with Windows", "Démarrer avec Windows");
    m.insert("About", "À propos");
    m.insert("Exit", "Quitter");
    m.insert("Last updated:", "Dernière mise à jour :");
    m.insert("Refresh", "Actualiser");
    m.insert("Status", "Statut");
    m.insert("Usage Alert", "Alerte d'utilisation");
    m.insert("Usage Critical", "Utilisation critique");
    m.insert(
        "Running in system tray. Click the icon for details.",
        "Fonctionne dans la barre des tâches. Cliquez sur l'icône pour les détails.",
    );
    m.insert("Compact mode", "Mode compact");
    m.insert("Theme", "Thème");
    m.insert("Language", "Langue");
    m.insert("Notifications", "Notifications");
    m.insert("Dark", "Sombre");
    m.insert("Light", "Clair");
    m.insert("Auto", "Auto");
    m.insert("Show ChatGPT section", "Afficher la section ChatGPT");
    m.insert("Enabled", "Activé");
    m.insert("Sound", "Son");
    m.insert("Thresholds", "Seuils");
    m.insert("Polling interval", "Intervalle de mise à jour");
    m.insert("seconds", "secondes");
    m.insert("Startup", "Démarrage");
    m.insert("General", "Général");
    m.insert("Back", "\u{2190} Retour");
    m.insert("Open Claude.ai \u{2192}", "Ouvrir Claude.ai \u{2192}");
    m.insert("ZaiMeter", "ZaiMeter");
    m.insert("CLAUDE", "CLAUDE");
    m.insert("CHATGPT / CODEX", "CHATGPT / CODEX");
    m.insert("Usage History", "Historique d'utilisation");
    m.insert("Usage History (24h)", "Historique d'utilisation (24h)");
    m.insert("Auto (English)", "Auto (Français)");
    m.insert("at", "à");
    m.insert("Resets in", "Réinitialisation dans");
    m.insert("Tray icon colors:", "Couleurs de l'icône :");
    m.insert("< 50% usage", "< 50% utilisation");
    m.insert("50-79% usage", "50\u{2013}79% utilisation");
    m.insert(">= 80% usage", "\u{2265} 80% utilisation");
    m.insert("No data", "Pas de données");
    m.insert("exceeded", "d\u{00e9}pass\u{00e9}");
    m.insert("Show widget", "Afficher le widget");
    m.insert(
        "Check for updates",
        "V\u{00e9}rifier les mises \u{00e0} jour",
    );
    m.insert("Accessibility patterns", "Motifs d'accessibilit\u{00e9}");
    m.insert("Update available", "Mise \u{00e0} jour disponible");
    m.insert(
        "is available. Click to download.",
        "est disponible. Cliquez pour t\u{00e9}l\u{00e9}charger.",
    );
    m.insert("Icon style", "Style d'ic\u{00f4}ne");
    m.insert("Number", "Nombre");
    m.insert("Ring", "Anneau");
    m.insert("Bar", "Barre");
    m.insert("Pie", "Camembert");
    m.insert("Dashboard layout", "Disposition");
    m.insert("Minimal", "Minimal");
    m.insert("Standard", "Standard");
    m.insert("Detailed", "Détaillé");
    m.insert("Hide Extra Usage", "Masquer Extra Usage");
    m
}
