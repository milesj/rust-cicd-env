use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://buddy.works/docs/pipelines/environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BUDDY_EXECUTION_PULL_REQUEST_BASE_BRANCH"),
        base_revision: None,
        branch: opt_var("BUDDY_EXECUTION_PULL_REQUEST_HEAD_BRANCH")
            .or_else(|| opt_var("BUDDY_EXECUTION_BRANCH"))
            .unwrap_or_default(),
        head_revision: None,
        id: var("BUDDY_PIPELINE_ID"),
        provider: CiProvider::Buddy,
        request_id: opt_var("BUDDY_EXECUTION_PULL_REQUEST_NO")
            .or_else(|| opt_var("BUDDY_EXECUTION_PULL_REQUEST_ID")),
        request_url: None,
        revision: var("BUDDY_EXECUTION_REVISION"),
        url: opt_var("BUDDY_PIPELINE_URL"),
    }
}
