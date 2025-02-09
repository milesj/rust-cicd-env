use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://codefresh.io/docs/docs/pipelines/variables/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("CF_PULL_REQUEST_TARGET").or_else(|| opt_var("CF_BASE_BRANCH")),
        base_revision: None,
        branch: var("CF_BRANCH"),
        env_prefix: Some("CF_".into()),
        head_revision: None,
        id: var("CF_BUILD_ID"),
        provider: CiProvider::Codefresh,
        request_id: opt_var("CF_PULL_REQUEST_NUMBER").or_else(|| opt_var("CF_PULL_REQUEST_ID")),
        request_url: None,
        revision: var("CF_REVISION"),
        url: opt_var("CF_BUILD_URL"),
    }
}
