use serde::{Deserialize, Serialize};
use std::env;

/// List of supported CD providers.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum CdProvider {
    Fly,
    Heroku,
    Railway,
    Render,
    Vercel,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeployEnvironment {
    /// Source branch that was deployed.
    pub branch: Option<String>,

    /// Name of the provider.
    pub provider: CdProvider,

    /// Revision (commit, sha, etc) that triggered the deploy.
    pub revision: String,

    /// Unique ID of the deployed service.
    pub service_id: Option<String>,
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
