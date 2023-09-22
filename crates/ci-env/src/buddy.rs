use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://devcenter.bitrise.io/en/references/available-environment-variables.html
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BUDDY_EXECUTION_PULL_REQUEST_BASE_BRANCH"),
        branch: opt_var("BUDDY_EXECUTION_PULL_REQUEST_HEAD_BRANCH")
            .or_else(|| opt_var("BUDDY_EXECUTION_BRANCH"))
            .unwrap_or_default(),
        id: var("BUDDY_PIPELINE_ID"),
        provider: CiProvider::Buddy,
        request_id: opt_var("BUDDY_EXECUTION_PULL_REQUEST_NO")
            .or_else(|| opt_var("BUDDY_EXECUTION_PULL_REQUEST_ID")),
        request_url: None,
        revision: var("BUDDY_EXECUTION_REVISION"),
        url: opt_var("BUDDY_PIPELINE_URL"),
    }
}
