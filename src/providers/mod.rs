pub mod claude;
pub mod zai;

use crate::credentials::{read_claude_token, read_zai_token};
use claude::{ClaudeClient, UsageResponse};
pub use zai::ZaiClient;

/// The active usage backend. The macOS agent dispatches on this; the Windows
/// app still uses `ClaudeClient` directly (Claude only).
pub enum Provider {
    Claude(ClaudeClient),
    Zai(ZaiClient),
}

impl Provider {
    /// Stable identifier persisted as the history `provider` column. Must match
    /// the allow-list in config.rs validate().
    pub fn name(&self) -> &'static str {
        match self {
            Provider::Claude(_) => "claude",
            Provider::Zai(_) => "zai",
        }
    }

    /// Notification hint shown when the credential itself is missing.
    pub fn login_hint(&self) -> &'static str {
        match self {
            Provider::Claude(_) => "Claude login not found. Run `claude` in Terminal.",
            Provider::Zai(_) => "Z.ai token not found. Set GLM_API_KEY in ~/.hermes/.env.",
        }
    }

    /// Read the provider's credential and fetch usage. Credential errors are
    /// prefixed `[cred]` so the caller can surface the login hint separately
    /// from a transport or API failure.
    pub async fn fetch(&self) -> Result<UsageResponse, String> {
        match self {
            Provider::Claude(client) => {
                let credential = read_claude_token()
                    .map_err(|e| format!("[cred] Claude credentials unavailable: {e}"))?;
                let mut usage = client.fetch_usage(&credential.access_token).await?;
                usage.subscription_type = credential.subscription_type;
                usage.rate_limit_tier = credential.rate_limit_tier;
                Ok(usage)
            }
            Provider::Zai(client) => {
                let credential = read_zai_token().map_err(|_| {
                    "[cred] Z.ai token not found. Set GLM_API_KEY in ~/.hermes/.env.".to_string()
                })?;
                client.fetch_usage(&credential.access_token).await
            }
        }
    }
}
