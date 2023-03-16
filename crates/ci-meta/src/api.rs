use serde::{Deserialize, Serialize};
use std::env;

pub struct CiOutput {
    pub close_log_group: &'static str,
    pub open_log_group: &'static str,
}

impl Default for CiOutput {
    fn default() -> Self {
        CiOutput {
            close_log_group: "",
            open_log_group: "▪▪▪▪ ",
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum CiProvider {
    Agola,
    AppCenter,
    Appcircle,
    AppVeyor,
    AwsCodebuild,
    Azure,
    Bamboo,
    Bitbucket,
    Bitrise,
    Buildkite,
    CircleCI,
    Codefresh,
    Codeship,
    Drone,
    GithubActions,
    Gitlab,
    GoogleCloudBuild,
    Semaphore,
    TravisCI,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CiEnvironment {
    /// Base branch of the pull/merge request.
    pub base_branch: Option<String>,

    /// Branch of the triggered pipeline.
    pub branch: String,

    /// Unique ID of the current pipeline.
    pub id: String,

    /// Name of the provider.
    pub provider: CiProvider,

    /// ID of an associated pull/merge request.
    pub request_id: Option<String>,

    /// Link to the pull/merge request.
    pub request_url: Option<String>,

    /// Revision that triggered the pipeline.
    pub revision: String,

    /// Link to the pipeline.
    pub url: Option<String>,
}

pub fn var(key: &str) -> String {
    env::var(key).unwrap_or_default()
}

pub fn opt_var(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(value) => {
            if value == "false" || value.is_empty() {
                None
            } else {
                Some(value)
            }
        }
        Err(_) => None,
    }
}
