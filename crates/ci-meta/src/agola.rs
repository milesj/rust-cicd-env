use crate::api::{opt_var, var, CiEnvironment, CiProvider};

pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        branch: var("AGOLA_GIT_BRANCH"),
        id: String::new(),
        provider: CiProvider::Agola,
        request_id: opt_var("AGOLA_PULL_REQUEST_ID"),
        request_url: None,
        revision: var("AGOLA_GIT_COMMITSHA"),
        url: None,
    }
}
