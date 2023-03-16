use crate::api::{opt_var, CdEnvironment, CdProvider};

// https://fly.io/docs/reference/runtime-environment/
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        provider: CdProvider::Fly,
        revision: String::new(), // ???
        service_id: opt_var("FLY_APP_NAME"),
    }
}
