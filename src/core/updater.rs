//! GitHub release checking and SHA-256 verification.

use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ReleaseAsset {
    pub name: String,
    #[serde(default, rename = "browser_download_url")]
    pub download_url: String,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_sha256")]
    pub sha256_hash: Option<String>,
}

fn deserialize_sha256<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value.map(|digest| digest.strip_prefix("sha256:").unwrap_or(&digest).to_ascii_lowercase()))
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ReleaseInfo {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub tag_name: String,
    #[serde(default, rename = "name")]
    pub title: String,
    #[serde(default, rename = "html_url")]
    pub url: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone)]
pub struct UpdateOptions {
    pub owner: String,
    pub repository: String,
    pub check_interval_seconds: i64,
}

impl Default for UpdateOptions {
    fn default() -> Self {
        Self {
            owner: "Stevehunn".to_string(),
            repository: "soundswaper".to_string(),
            check_interval_seconds: 43_200,
        }
    }
}

impl UpdateOptions {
    pub fn latest_release_api_url(&self) -> String {
        format!("https://api.github.com/repos/{}/{}/releases/latest", self.owner, self.repository)
    }

    pub fn releases_page_url(&self) -> String {
        format!("https://github.com/{}/{}/releases", self.owner, self.repository)
    }
}

/// Fetch and deserialize the latest published GitHub release.
pub async fn check_for_updates(
    options: &UpdateOptions,
    client: Client,
) -> Result<ReleaseInfo, reqwest::Error> {
    let response = client
        .get(options.latest_release_api_url())
        .header(reqwest::header::USER_AGENT, "SoundSwaper")
        .send()
        .await?
        .error_for_status()?;

    let mut release: ReleaseInfo = response.json().await?;
    if release.version.is_empty() {
        release.version = release.tag_name.trim_start_matches('v').to_string();
    }
    Ok(release)
}

/// Returns true when the latest release is newer than the installed version.
pub fn is_outdated(current_version: &str, release_tag_name: &str) -> bool {
    compare_versions(current_version, release_tag_name).is_lt()
}

/// Compare numeric dot-separated version components.
fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |value: &str| {
        value.trim_start_matches('v')
            .split('.')
            .map(|part| part.parse::<u64>().unwrap_or(0))
            .collect::<Vec<_>>()
    };
    let left = parse(a);
    let right = parse(b);
    for index in 0..left.len().max(right.len()) {
        let ordering = left.get(index).copied().unwrap_or(0)
            .cmp(&right.get(index).copied().unwrap_or(0));
        if !ordering.is_eq() {
            return ordering;
        }
    }
    std::cmp::Ordering::Equal
}

/// Verify a downloaded file against a hexadecimal SHA-256 digest.
pub fn verify_sha256(file_data: &[u8], expected_hash: &str) -> bool {
    let expected = expected_hash.trim().strip_prefix("sha256:").unwrap_or(expected_hash.trim());
    if expected.len() != 64 || !expected.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return false;
    }
    let actual = format!("{:x}", Sha256::digest(file_data));
    actual.eq_ignore_ascii_case(expected)
}

/// Check whether a release newer than the installed version is available.
pub async fn check_updates(
    options: &UpdateOptions,
    client: Client,
    current_version: &str,
) -> Option<String> {
    match check_for_updates(options, client).await {
        Ok(release) if is_outdated(current_version, &release.tag_name) => Some(release.tag_name),
        Ok(_) => None,
        Err(error) => {
            eprintln!("Failed to check for updates: {error}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(is_outdated("v1.0.0", "v2.0.0"));
        assert!(!is_outdated("v2.0.0", "v1.0.0"));
        assert!(!is_outdated("v1.0.0", "v1.0.0"));
        assert!(is_outdated("1.9.9", "v1.10.0"));
    }

    #[test]
    fn test_sha256_verification() {
        let file_data = b"hello world";
        let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert!(verify_sha256(file_data, expected));
        assert!(!verify_sha256(file_data, "abc123"));
    }

    #[test]
    fn test_default_options() {
        let options = UpdateOptions::default();
        assert_eq!(options.owner, "Stevehunn");
        assert_eq!(options.repository, "soundswaper");
        assert_eq!(options.check_interval_seconds, 43_200);
        assert!(options.releases_page_url().contains("Stevehunn/soundswaper"));
    }

    #[test]
    fn test_release_default_is_not_recursive() {
        assert!(ReleaseInfo::default().assets.is_empty());
    }
}
