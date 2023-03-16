use crate::api::{opt_var, var, CiEnvironment, CiProvider};

pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("_BASE_BRANCH"),
        branch: opt_var("_HEAD_BRANCH")
            .or_else(|| opt_var("BRANCH_NAME"))
            .unwrap_or_default(),
        id: var("BUILD_ID"),
        provider: CiProvider::GoogleCloudBuild,
        request_id: opt_var("_PR_NUMBER"),
        request_url: opt_var("_HEAD_REPO_URL"),
        revision: var("REVISION_ID"),
        url: None,
    }
}
