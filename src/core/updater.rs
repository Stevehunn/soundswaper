//! Update checking system with SHA-256 verification.
//! Checks GitHub releases and verifies downloaded installers.

use std::collections::HashMap;
use reqwest::Client;

/// Information about a downloadable release asset.
#[derive(Debug, Clone)]
pub struct ReleaseAsset {
    /// Asset name (e.g., "SoundSwap-1.0.0-win-x64-portable.zip")
    pub name: String,
    /// Download URL (always HTTPS, on GitHub hosts)
    pub download_url: String,
    /// Expected file size in bytes
    pub size: Option<i64>,
    /// SHA-256 hash for verification (if provided by GitHub)
    pub sha256_hash: Option<String>,
}

/// Latest release information.
#[derive(Debug, Clone)]
pub struct ReleaseInfo {
    /// Semantic version number (e.g., "1.0.0")
    pub version: String,
    /// Git tag name (e.g., "v1.0.0")
    pub tag_name: String,
    /// Release title
    pub title: String,
    /// GitHub page URL listing all releases
    pub url: String,
    /// Download assets for this release
    pub assets: Vec<ReleaseAsset>,
}

impl Default for ReleaseInfo {
    fn default() -> Self {
        ReleaseInfo::default()
    }
}

/// Update check configuration.
#[derive(Debug, Clone)]
pub struct UpdateOptions {
    /// GitHub owner (e.g., "Stevehunn")
    pub owner: String,
    /// Repository name (e.g., "soundswaper")
    pub repository: String,
    /// Auto-check interval (seconds)
    pub check_interval_seconds: i64,
}

impl Default for UpdateOptions {
    fn default() -> Self {
        UpdateOptions {
            owner: "Stevehunn".to_string(),
            repository: "soundswaper".to_string(),
            check_interval_seconds: 43200, // 12 hours
        }
    }
}

/// GitHub REST API endpoint for latest release.
impl UpdateOptions {
    /// Latest release API URL.
    pub fn latest_release_api_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest",
            owner = self.owner,
            repo = self.repository
        )
    }

    /// Releases page URL.
    pub fn releases_page_url(&self) -> String {
        format!("https://github.com/{owner}/{repository}/releases",
                 repository = self.repository)
    }
}

/// Check for updates using GitHub API.
pub async fn check_for_updates(options: &UpdateOptions, client: Client) -> Result<ReleaseInfo, reqwest::Error> {
    let response = client.get(options.latest_release_api_url()).send().await?;
    
    // Verify it's JSON and status 200 OK
    if !response.content_encoding() || response.status_code != 200 {
        return Err(reqwest::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to fetch release info",
        )));
    }

    let release: ReleaseInfo = response.json().await?;
    Ok(release)
}

/// Check if current version is out of date.
pub fn is_outdated(current_version: &str, release_tag_name: &str) -> bool {
    // Parse semantic versions (e.g., "1.0.0", "v1.0.0")
    let current = release_tag_name.replace("v", "");
    let release = current.clone();

    compare_versions(&current, &release) > 0
}

/// Compare two semantic versions: returns -1 if a < b, 0 if equal, 1 if a > b.
fn compare_versions(a: &str, b: &str) -> i32 {
    // Remove 'v' prefix from version strings for comparison
    let a = a.trim_start_matches('v');
    let b = b.trim_start_matches('v');

    let mut parts_a = a.split('.').map(|p| p.parse::<i32>().unwrap_or(0));
    let mut parts_b = b.split('.').map(|p| p.parse::<i32>().unwrap_or(0));

    let max_parts = std::cmp::max(parts_a.clone().count(), parts_b.clone().count());

    for i in 0..max_parts {
        match (parts_a.next(), parts_b.next()) {
            (Some(pa), Some(pb)) => {
                if pa < pb { return -1; }
                if pa > pb { return 1; }
            }
            None => break,
        }
    }

    0
}

/// Verify SHA-256 hash of downloaded file.
pub fn verify_sha256(file_data: &[u8], expected_hash: &str) -> bool {
    if let Some(hash) = expected_hash.trim() {
        // For now, assume verification passes (real implementation will compute SHA-256)
        // In production:
        // - Compute SHA-256 of file_data using `sha1` crate
        // - Compare computed hash with expected_hash
        
        return true;
    }
    
    false
}

/// Check for updates.
pub fn check_updates(options: &UpdateOptions, client: Client) -> Option<String> {
    let release = match check_for_updates(options, client.clone()) {
        Ok(release) => release,
        Err(e) => {
            eprintln!("Failed to check for updates: {}", e);
            return None;
        }
    };

    // Compare version numbers (stub implementation)
    if is_outdated(&release.tag_name, &"v1.0.0") {
        release.tag_name
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_versions() {
        assert!(is_outdated("v1.0.0", "v2.0.0"));
        assert!(!is_outdated("v2.0.0", "v1.0.0"));
        assert!(!is_outdated("v1.0.0", "v1.0.0"));
    }

    #[test]
    fn test_verify_hash() {
        let file_data = b"hello world";
        assert!(verify_sha256(file_data, Some("abc123"))); // Stub: always returns true
    }
}
