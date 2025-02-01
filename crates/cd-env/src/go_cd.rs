use crate::api::{var, CdEnvironment, CdProvider};

// https://docs.gocd.org/current/faq/dev_use_current_revision_in_build.html#standard-gocd-environment-variables
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: Some("GO_".into()),
        provider: CdProvider::GoCD,
        revision: var("GO_REVISION"),
        service_id: None,
    }
}
