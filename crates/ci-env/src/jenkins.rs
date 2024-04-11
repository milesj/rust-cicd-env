use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://www.jenkins.io/doc/book/pipeline/jenkinsfile/#using-environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: opt_var("GIT_BRANCH")
            .or_else(|| opt_var("BRANCH_NAME"))
            .unwrap_or_default(),
        head_revision: None,
        id: var("BUILD_NUMBER"),
        provider: CiProvider::Jenkins,
        request_id: opt_var("CHANGE_ID"),
        request_url: None,
        revision: var("GIT_COMMIT"),
        url: opt_var("JENKINS_URL"),
    }
}
