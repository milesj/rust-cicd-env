use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://render.com/docs/environment-variables
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: opt_var("RENDER_GIT_BRANCH"),
        provider: CdProvider::Render,
        revision: var("RENDER_GIT_COMMIT"),
        service_id: opt_var("RENDER_SERVICE_ID"),
    }
}
