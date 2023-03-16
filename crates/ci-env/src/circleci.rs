use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://circleci.com/docs/variables/#built-in-environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        branch: var("CIRCLE_BRANCH"),
        id: var("CIRCLE_WORKFLOW_ID"),
        provider: CiProvider::CircleCI,
        request_id: opt_var("CIRCLE_PR_NUMBER"),
        request_url: opt_var("CIRCLE_PULL_REQUEST"),
        revision: var("CIRCLE_SHA1"),
        url: opt_var("CIRCLE_BUILD_URL"),
    }
}
