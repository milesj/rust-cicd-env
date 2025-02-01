use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://agola.io/doc/config/reference.html#agola-provided-environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("AGOLA_GIT_BRANCH"),
        env_prefix: Some("AGOLA_".into()),
        head_revision: None,
        id: String::new(),
        provider: CiProvider::Agola,
        request_id: opt_var("AGOLA_PULL_REQUEST_ID"),
        request_url: None,
        revision: var("AGOLA_GIT_COMMITSHA"),
        url: None,
    }
}
