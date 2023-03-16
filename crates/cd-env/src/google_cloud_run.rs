use crate::api::{opt_var, var, CdProvider, DeployEnvironment};

// https://cloud.google.com/run/docs/container-contract
pub fn create_environment() -> DeployEnvironment {
    DeployEnvironment {
        branch: None,
        provider: CdProvider::GoogleCloudRun,
        revision: var("K_REVISION"),
        service_id: opt_var("K_SERVICE"),
    }
}
