use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://cloud.google.com/run/docs/container-contract
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: Some("K_".into()),
        provider: CdProvider::GoogleCloudRun,
        revision: var("K_REVISION"),
        service_id: opt_var("K_SERVICE"),
    }
}
