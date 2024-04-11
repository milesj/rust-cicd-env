use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://cirrus-ci.org/guide/writing-tasks/#environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("CIRRUS_BASE_BRANCH"),
        base_revision: opt_var("CIRRUS_BASE_SHA"),
        branch: var("CIRRUS_BRANCH"),
        head_revision: None,
        id: var("CIRRUS_BUILD_ID"),
        provider: CiProvider::Cirrus,
        request_id: opt_var("CIRRUS_PR"),
        request_url: None,
        revision: opt_var("CIRRUS_CHANGE_IN_REPO")
            .or_else(|| opt_var("CIRRUS_BASE_SHA"))
            .unwrap_or_default(),
        url: None,
    }
}
