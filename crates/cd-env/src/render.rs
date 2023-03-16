use crate::api::{opt_var, var, CdProvider, DeployEnvironment};

// https://render.com/docs/environment-variables
pub fn create_environment() -> DeployEnvironment {
    DeployEnvironment {
        branch: var("RENDER_GIT_BRANCH"),
        provider: CdProvider::Render,
        revision: var("RENDER_GIT_COMMIT"),
        service_id: opt_var("RENDER_SERVICE_ID"),
    }
}
