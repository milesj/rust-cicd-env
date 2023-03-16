use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://www.jenkins.io/doc/book/pipeline/jenkinsfile/#using-environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        branch: opt_var("GIT_BRANCH")
            .or_else(|| opt_var("BRANCH_NAME"))
            .unwrap_or_default(),
        id: var("BUILD_NUMBER"),
        provider: CiProvider::Jenkins,
        request_id: opt_var("CHANGE_ID"),
        request_url: None,
        revision: var("GIT_COMMIT"),
        url: opt_var("BUILD_URL"),
    }
}
