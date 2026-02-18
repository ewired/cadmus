use secrecy::SecretString;
use serde::Deserialize;
use std::path::PathBuf;

// ── GitHub REST API response types ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub(crate) struct PullRequest {
    pub head: PrHead,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PrHead {
    pub sha: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WorkflowRunsResponse {
    pub workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WorkflowRun {
    pub name: String,
    pub id: u64,
    #[serde(default)]
    pub head_sha: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Repository {
    pub default_branch: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ArtifactsResponse {
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Artifact {
    pub name: String,
    pub id: u64,
    pub size_in_bytes: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Release {
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

// ── Device flow types ─────────────────────────────────────────────────────────

/// Response from the GitHub device code endpoint.
///
/// Contains the codes to display to the user and the polling parameters.
#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    /// Opaque code passed back to the poll endpoint — do not display to the user.
    pub device_code: String,
    /// Short code the user types at `verification_uri` (e.g. `WDJB-MJHT`).
    pub user_code: String,
    /// URL the user must visit to authorize (always `https://github.com/login/device`).
    pub verification_uri: String,
    /// Seconds until both codes expire (default 900 = 15 minutes).
    pub expires_in: u64,
    /// Minimum seconds to wait between poll attempts.
    pub interval: u64,
}

/// Result of a single poll attempt during device flow.
#[derive(Debug)]
pub enum TokenPollResult {
    /// User has not yet authorized — keep polling after `interval` seconds.
    Pending,
    /// GitHub is being polled too fast — caller must add 5 s to the poll interval.
    SlowDown,
    /// User authorized successfully; token is ready to use.
    Complete(SecretString),
    /// The device code expired before the user authorized.
    Expired,
    /// User explicitly cancelled the authorization on GitHub.
    Cancelled,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AccessTokenResponse {
    pub access_token: Option<String>,
    pub error: Option<String>,
}

// ── OTA progress ──────────────────────────────────────────────────────────────

/// Progress states during an OTA download operation.
#[derive(Debug, Clone)]
pub enum OtaProgress {
    /// Verifying the pull request exists and fetching its metadata.
    CheckingPr,
    /// Searching for the latest successful build on the default branch.
    FindingLatestBuild,
    /// Searching for the associated GitHub Actions workflow run.
    FindingWorkflow,
    /// Actively downloading the artifact with optional progress tracking.
    DownloadingArtifact { downloaded: u64, total: u64 },
    /// Download completed successfully, artifact saved to disk.
    Complete { path: PathBuf },
}
