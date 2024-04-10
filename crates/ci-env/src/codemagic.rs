use crate::api::{opt_var, var, CiEnvironment, CiProvider};

pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("CM_BRANCH"),
        head_revision: None,
        id: var("CM_BUILD_ID"),
        provider: CiProvider::Codemagic,
        request_id: opt_var("CM_PULL_REQUEST_NUMBER").or_else(|| opt_var("CM_PULL_REQUEST")),
        request_url: None,
        revision: var("CM_COMMIT"),
        url: None,
    }
}
