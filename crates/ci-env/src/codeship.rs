use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.cloudbees.com/docs/cloudbees-codeship/latest/pro-builds-and-configuration/environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("CI_BRANCH"),
        env_prefix: Some("CI_".into()),
        head_revision: None,
        id: var("CI_BUILD_ID"),
        provider: CiProvider::Codeship,
        request_id: opt_var("CI_PR_NUMBER"),
        request_url: opt_var("CI_PULL_REQUEST"),
        revision: var("CI_COMMIT_ID"),
        url: None,
    }
}
