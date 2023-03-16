use crate::api::{opt_var, CdEnvironment, CdProvider};

// https://cloud.google.com/appengine/docs/standard/cloud-run-for-gae-customers
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        provider: CdProvider::GoogleAppEngine,
        revision: String::new(), // ???
        service_id: opt_var("GAE_APPLICATION"),
    }
}
