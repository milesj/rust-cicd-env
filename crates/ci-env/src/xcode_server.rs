use crate::api::{var, CiEnvironment, CiProvider};

// https://developer.apple.com/library/archive/documentation/IDEs/Conceptual/xcode_guide-continuous_integration/EnvironmentVariableReference.html
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        branch: var("XCS_PRIMARY_REPO_BRANCH"),
        id: var("XCS_INTEGRATION_ID"),
        provider: CiProvider::XcodeServer,
        request_id: None,
        request_url: None,
        revision: var("XCS_PRIMARY_REPO_REVISION"),
        url: None,
    }
}
