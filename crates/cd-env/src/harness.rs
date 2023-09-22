use crate::api::{opt_var, var, CdEnvironment, CdProvider};

pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        provider: CdProvider::Harness,
        revision: var("DRONE_COMMIT"),
        service_id: opt_var("HARNESS_ACCOUNT_ID"),
    }
}
