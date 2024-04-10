use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://jenkins-x.io/v3/develop/reference/variables/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("PULL_BASE_REF"),
        base_revision: None,
        branch: opt_var("PR_HEAD_REF")
            .or_else(|| opt_var("GIT_BRANCH"))
            .or_else(|| opt_var("BRANCH_NAME"))
            .unwrap_or_default(),
        head_revision: None,
        id: var("BUILD_ID"),
        provider: CiProvider::JenkinsX,
        request_id: opt_var("PULL_NUMBER"),
        request_url: None,
        revision: opt_var("PR_HEAD_SHA")
            .or_else(|| opt_var("PULL_PULL_SHA"))
            .unwrap_or_default(),
        url: opt_var("JENKINS_X_URL"),
    }
}
