#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use windows::Win32::Security::Credentials::{CredFree, CredReadW, CREDENTIALW, CRED_TYPE_GENERIC};

#[derive(Debug, Clone)]
pub struct CredentialInfo {
    pub access_token: String,
    /// e.g. "max", "pro" — from credentials file, if available
    pub subscription_type: Option<String>,
    /// e.g. "default_claude_max_5x" — rate limit tier from credentials
    pub rate_limit_tier: Option<String>,
    /// Token expiry time in milliseconds since epoch
    #[cfg_attr(not(windows), allow(dead_code))] // read by the Windows app
    pub expires_at: Option<u64>,
}

#[derive(Debug)]
pub enum CredentialError {
    NotFound,
    #[cfg_attr(not(windows), allow(dead_code))] // constructed by the Windows credential path
    WindowsError(String),
    ParseError(String),
}

impl std::fmt::Display for CredentialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(
                f,
                "Claude Code credentials not found in ~/.claude/.credentials.json"
            ),
            Self::WindowsError(e) => write!(f, "Platform credential store error: {e}"),
            Self::ParseError(e) => write!(f, "Failed to parse credential JSON: {e}"),
        }
    }
}

/// Read the Claude OAuth access token.
///
/// Checks two locations in order:
/// 1. File-based: `~/.claude/.credentials.json` (Claude Code v2.x)
/// 2. Windows Credential Manager: target "Claude Code-credentials" (legacy, Windows only)
pub fn read_claude_token() -> Result<CredentialInfo, CredentialError> {
    // Try file-based credentials first (Claude Code v2.x stores here)
    if let Ok(info) = read_token_from_file() {
        return Ok(info);
    }

    // Fall back to Windows Credential Manager
    read_token_from_credential_manager()
}

/// Read the z.ai (GLM Coding Plan) bearer token.
///
/// Resolution order matches the `zai` shell launcher (~/.zshrc):
/// 1. `GLM_API_KEY` in `~/.hermes/.env` (canonical fleet secrets file, 0600).
/// 2. `ANTHROPIC_AUTH_TOKEN` env var (how the launcher injects it at runtime).
/// 3. `GLM_API_KEY` env var.
///
/// The token is a fleet secret (Permanent Constraint #2): read it, never log
/// or persist it elsewhere.
pub fn read_zai_token() -> Result<CredentialInfo, CredentialError> {
    if let Some(key) = read_glm_api_key_from_env_file() {
        return Ok(bare_token(key));
    }
    for var in ["ANTHROPIC_AUTH_TOKEN", "GLM_API_KEY"] {
        if let Ok(val) = std::env::var(var) {
            if !val.is_empty() {
                return Ok(bare_token(val));
            }
        }
    }
    Err(CredentialError::NotFound)
}

fn bare_token(token: String) -> CredentialInfo {
    CredentialInfo {
        access_token: token,
        subscription_type: None,
        rate_limit_tier: None,
        expires_at: None,
    }
}

/// Pull `GLM_API_KEY=` out of ~/.hermes/.env. None if the file or key is
/// absent. See [`glm_api_key_from_contents`] for the parsing rules.
fn read_glm_api_key_from_env_file() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let env_path = std::path::Path::new(&home).join(".hermes").join(".env");
    let contents = std::fs::read_to_string(env_path).ok()?;
    glm_api_key_from_contents(&contents)
}

/// Extract the last non-empty `GLM_API_KEY` value from .env-style contents.
/// Tolerates optional whitespace around `=` and surrounding quotes, and ignores
/// look-alike keys (`GLM_API_KEY_BACKUP`). A strict superset of the `KEY=value`
/// shape the `zai` shell function expects. None if absent.
fn glm_api_key_from_contents(contents: &str) -> Option<String> {
    let mut last: Option<String> = None;
    for line in contents.lines() {
        // Split on the first `=` so a value may contain `=`, and so a look-alike
        // key (`GLM_API_KEY_BACKUP`) cannot match.
        let Some((key, val)) = line.split_once('=') else {
            continue;
        };
        if key.trim() != "GLM_API_KEY" {
            continue;
        }
        let val = val.trim().trim_matches(['"', '\'']);
        if !val.is_empty() {
            last = Some(val.to_string());
        }
    }
    last
}

/// Read token from ~/.claude/.credentials.json
fn read_token_from_file() -> Result<CredentialInfo, CredentialError> {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map_err(|_| CredentialError::NotFound)?;

    let cred_path = std::path::Path::new(&home)
        .join(".claude")
        .join(".credentials.json");

    let contents = std::fs::read_to_string(&cred_path).map_err(|_| CredentialError::NotFound)?;

    extract_credential_info(&contents)
}

/// Read token from Windows Credential Manager (legacy path)
#[cfg(windows)]
fn read_token_from_credential_manager() -> Result<CredentialInfo, CredentialError> {
    const TARGET: &str = "Claude Code-credentials";

    let target_wide: Vec<u16> = TARGET.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let mut pcredential: *mut CREDENTIALW = std::ptr::null_mut();
        let result = CredReadW(
            PCWSTR(target_wide.as_ptr()),
            CRED_TYPE_GENERIC,
            0,
            &mut pcredential,
        );

        if result.is_err() {
            let err = windows::core::Error::from_win32();
            if err.code().0 == 0x80070490u32 as i32 {
                return Err(CredentialError::NotFound);
            }
            return Err(CredentialError::WindowsError(err.to_string()));
        }

        if pcredential.is_null() {
            return Err(CredentialError::NotFound);
        }

        let cred = &*pcredential;
        let blob_size = cred.CredentialBlobSize as usize;
        let blob_ptr = cred.CredentialBlob;

        if blob_ptr.is_null() || blob_size == 0 {
            CredFree(pcredential as *mut _);
            return Err(CredentialError::ParseError(
                "Empty credential blob".to_string(),
            ));
        }

        let json_string = if blob_size % 2 == 0 {
            let wide_len = blob_size / 2;
            let wide_slice = std::slice::from_raw_parts(blob_ptr as *const u16, wide_len);
            let wide_slice = if wide_slice.last() == Some(&0) {
                &wide_slice[..wide_len - 1]
            } else {
                wide_slice
            };
            match String::from_utf16(wide_slice) {
                Ok(s) if s.starts_with('{') => s,
                _ => {
                    let bytes = std::slice::from_raw_parts(blob_ptr, blob_size);
                    String::from_utf8_lossy(bytes)
                        .trim_end_matches('\0')
                        .to_string()
                }
            }
        } else {
            let bytes = std::slice::from_raw_parts(blob_ptr, blob_size);
            String::from_utf8_lossy(bytes)
                .trim_end_matches('\0')
                .to_string()
        };

        CredFree(pcredential as *mut _);

        extract_credential_info(&json_string)
    }
}

#[cfg(not(windows))]
fn read_token_from_credential_manager() -> Result<CredentialInfo, CredentialError> {
    Err(CredentialError::NotFound)
}

fn extract_credential_info(json: &str) -> Result<CredentialInfo, CredentialError> {
    let v: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| CredentialError::ParseError(format!("Invalid JSON: {e}")))?;

    let oauth = v.get("claudeAiOauth");

    // Try nested: {"claudeAiOauth": {"accessToken": "..."}}
    if let Some(token) = oauth
        .and_then(|o| o.get("accessToken"))
        .and_then(|t| t.as_str())
    {
        let subscription_type = oauth
            .and_then(|o| o.get("subscriptionType"))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());
        let rate_limit_tier = oauth
            .and_then(|o| o.get("rateLimitTier"))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());
        let expires_at = oauth
            .and_then(|o| o.get("expiresAt"))
            .and_then(|v| v.as_u64());
        return Ok(CredentialInfo {
            access_token: token.to_string(),
            subscription_type,
            rate_limit_tier,
            expires_at,
        });
    }

    // Try flat: {"accessToken": "..."}
    if let Some(token) = v.get("accessToken").and_then(|t| t.as_str()) {
        let subscription_type = v
            .get("subscriptionType")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());
        let rate_limit_tier = v
            .get("rateLimitTier")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());
        let expires_at = v.get("expiresAt").and_then(|v| v.as_u64());
        return Ok(CredentialInfo {
            access_token: token.to_string(),
            subscription_type,
            rate_limit_tier,
            expires_at,
        });
    }

    Err(CredentialError::ParseError(
        "accessToken field not found in credential JSON".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glm_api_key_from_contents_variants() {
        // Standard form.
        assert_eq!(
            glm_api_key_from_contents("GLM_API_KEY=sk-abc\n"),
            Some("sk-abc".to_string())
        );
        // Whitespace around `=` and the value is tolerated.
        assert_eq!(
            glm_api_key_from_contents("GLM_API_KEY =  sk-abc  \n"),
            Some("sk-abc".to_string())
        );
        // Surrounding quotes are stripped.
        assert_eq!(
            glm_api_key_from_contents("GLM_API_KEY=\"sk-abc\"\n"),
            Some("sk-abc".to_string())
        );
        // A look-alike key must not match.
        assert_eq!(
            glm_api_key_from_contents("GLM_API_KEY_BACKUP=sk-abc\n"),
            None
        );
        // Last non-empty match wins.
        assert_eq!(
            glm_api_key_from_contents("GLM_API_KEY=first\nOTHER=x\nGLM_API_KEY=second\n"),
            Some("second".to_string())
        );
        // Absent.
        assert_eq!(glm_api_key_from_contents("OTHER=x\n"), None);
    }

    #[test]
    fn test_extract_nested() {
        let json = r#"{"claudeAiOauth": {"accessToken": "test-token-nested"}}"#;
        let info = extract_credential_info(json).unwrap();
        assert_eq!(info.access_token, "test-token-nested");
        assert_eq!(info.subscription_type, None);
    }

    #[test]
    fn test_extract_nested_with_subscription() {
        let json = r#"{"claudeAiOauth": {"accessToken": "tok", "subscriptionType": "max"}}"#;
        let info = extract_credential_info(json).unwrap();
        assert_eq!(info.access_token, "tok");
        assert_eq!(info.subscription_type, Some("max".to_string()));
    }

    #[test]
    fn test_extract_flat() {
        let json = r#"{"accessToken": "test-token-flat"}"#;
        let info = extract_credential_info(json).unwrap();
        assert_eq!(info.access_token, "test-token-flat");
    }

    #[test]
    fn test_extract_missing() {
        let json = r#"{"other": "data"}"#;
        assert!(matches!(
            extract_credential_info(json),
            Err(CredentialError::ParseError(_))
        ));
    }
}
