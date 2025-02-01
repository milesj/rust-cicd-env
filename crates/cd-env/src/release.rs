use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://docs.release.com/reference-documentation/application-settings/default-environment-variables#releases-environment-variables
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: opt_var("RELEASE_BRANCH_NAME"),
        env_prefix: Some("RELEASE_".into()),
        provider: CdProvider::Release,
        revision: var("RELEASE_COMMIT_SHA"),
        service_id: opt_var("RELEASE_ACCOUNT_ID"),
    }
}
