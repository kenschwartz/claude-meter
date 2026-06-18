//! Auto-update checker: queries GitHub Releases API for newer versions.

const GITHUB_RELEASES_URL: &str = "https://api.github.com/repos/klivak/zaimeter/releases/latest";

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check if a newer version is available on GitHub.
/// Returns `Some((tag, html_url))` if newer, `None` otherwise.
pub async fn check_for_update() -> Option<(String, String)> {
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent(format!("ZaiMeter/{}", CURRENT_VERSION))
        .build()
        .ok()?;

    let resp = client.get(GITHUB_RELEASES_URL).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }

    let json: serde_json::Value = resp.json().await.ok()?;
    let tag = json.get("tag_name")?.as_str()?;
    let html_url = json.get("html_url")?.as_str()?;

    let remote_ver = tag.trim_start_matches('v');
    if is_newer(remote_ver, CURRENT_VERSION) {
        Some((tag.to_string(), html_url.to_string()))
    } else {
        None
    }
}

/// Simple semver comparison: returns true if `remote` > `current`.
fn is_newer(remote: &str, current: &str) -> bool {
    let parse =
        |s: &str| -> Vec<u32> { s.split('.').filter_map(|p| p.parse::<u32>().ok()).collect() };
    let r = parse(remote);
    let c = parse(current);
    for i in 0..3 {
        let rv = r.get(i).copied().unwrap_or(0);
        let cv = c.get(i).copied().unwrap_or(0);
        if rv > cv {
            return true;
        }
        if rv < cv {
            return false;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_newer() {
        assert!(is_newer("1.4.0", "1.3.6"));
        assert!(is_newer("2.0.0", "1.9.9"));
        assert!(!is_newer("1.3.6", "1.3.6"));
        assert!(!is_newer("1.3.5", "1.3.6"));
        assert!(is_newer("1.3.7", "1.3.6"));
    }
}
