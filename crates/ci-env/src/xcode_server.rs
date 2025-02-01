use crate::api::{var, CiEnvironment, CiProvider};

// https://developer.apple.com/library/archive/documentation/IDEs/Conceptual/xcode_guide-continuous_integration/EnvironmentVariableReference.html
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("XCS_PRIMARY_REPO_BRANCH"),
        env_prefix: Some("XCS_".into()),
        head_revision: None,
        id: var("XCS_INTEGRATION_ID"),
        provider: CiProvider::XcodeServer,
        request_id: None,
        request_url: None,
        revision: var("XCS_PRIMARY_REPO_REVISION"),
        url: None,
    }
}
