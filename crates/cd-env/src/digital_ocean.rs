use crate::api::{opt_var, var, CdProvider, DeployEnvironment};

// https://docs.digitalocean.com/products/app-platform/how-to/use-environment-variables/
pub fn create_environment() -> DeployEnvironment {
    DeployEnvironment {
        branch: None,
        provider: CdProvider::DigitalOceanAppPlatform,
        revision: var("COMMIT_HASH"),
        service_id: opt_var("APP_ID"),
    }
}
