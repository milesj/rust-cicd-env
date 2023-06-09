use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://docs.railway.app/develop/variables
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: opt_var("RAILWAY_GIT_BRANCH"),
        provider: CdProvider::Railway,
        revision: var("RAILWAY_GIT_COMMIT_SHA"),
        service_id: opt_var("RAILWAY_SERVICE_ID"),
    }
}
