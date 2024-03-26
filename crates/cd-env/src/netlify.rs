use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://docs.netlify.com/configure-builds/environment-variables/#read-only-variables
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: opt_var("BRANCH"),
        provider: CdProvider::Netlify,
        revision: var("COMMIT_REF"),
        service_id: opt_var("SITE_ID"),
    }
}
