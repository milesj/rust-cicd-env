use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.screwdriver.cd/user-guide/environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("PR_BASE_BRANCH_NAME"),
        base_revision: None,
        branch: opt_var("PR_BRANCH_NAME").unwrap_or_else(|| var("GIT_BRANCH")),
        head_revision: None,
        id: var("SD_BUILD_ID"),
        provider: CiProvider::Screwdriver,
        request_id: opt_var("SD_PULL_REQUEST"),
        request_url: None,
        revision: var("SD_BUILD_SHA"),
        url: opt_var("SD_UI_BUILD_URL"),
    }
}
