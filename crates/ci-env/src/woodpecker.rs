use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://woodpecker-ci.org/docs/usage/environment
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("CI_COMMIT_TARGET_BRANCH"),
        base_revision: None,
        branch: opt_var("CI_COMMIT_SOURCE_BRANCH")
            .or_else(|| opt_var("CI_COMMIT_BRANCH"))
            .unwrap_or_default(),
        head_revision: None,
        id: var("CI_BUILD_NUMBER"),
        provider: CiProvider::Woodpecker,
        request_id: opt_var("CI_COMMIT_PULL_REQUEST"),
        request_url: None,
        revision: var("CI_COMMIT_SHA"),
        url: opt_var("CI_BUILD_LINK"),
    }
}
