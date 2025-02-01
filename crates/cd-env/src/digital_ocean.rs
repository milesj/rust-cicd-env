use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://docs.digitalocean.com/products/app-platform/how-to/use-environment-variables/
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: None,
        provider: CdProvider::DigitalOceanAppPlatform,
        revision: var("COMMIT_HASH"),
        service_id: opt_var("APP_ID"),
    }
}
