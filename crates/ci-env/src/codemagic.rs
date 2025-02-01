use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.codemagic.io/yaml-basic-configuration/environment-variables/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("CM_BRANCH"),
        env_prefix: Some("CM_".into()),
        head_revision: None,
        id: var("CM_BUILD_ID"),
        provider: CiProvider::Codemagic,
        request_id: opt_var("CM_PULL_REQUEST_NUMBER").or_else(|| opt_var("CM_PULL_REQUEST")),
        request_url: None,
        revision: var("CM_COMMIT"),
        url: None,
    }
}
