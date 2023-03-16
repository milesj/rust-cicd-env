use crate::api::{opt_var, var, CdProvider, DeployEnvironment};

// https://docs.railway.app/develop/variables
pub fn create_environment() -> DeployEnvironment {
    DeployEnvironment {
        branch: opt_var("RAILWAY_GIT_BRANCH"),
        provider: CdProvider::Railway,
        revision: var("RAILWAY_GIT_COMMIT_SHA"),
        service_id: None,
    }
}
