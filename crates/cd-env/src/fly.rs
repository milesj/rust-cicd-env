use crate::api::{opt_var, CdProvider, DeployEnvironment};

// https://fly.io/docs/reference/runtime-environment/
pub fn create_environment() -> DeployEnvironment {
    DeployEnvironment {
        branch: None,
        provider: CdProvider::Fly,
        revision: String::new(), // ???
        service_id: opt_var("FLY_APP_NAME"),
    }
}
