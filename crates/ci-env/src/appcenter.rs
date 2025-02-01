use crate::api::{var, CiEnvironment, CiProvider};

// https://learn.microsoft.com/en-us/appcenter/build/custom/variables/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("APPCENTER_BRANCH"),
        env_prefix: Some("APPCENTER_".into()),
        head_revision: None,
        id: var("APPCENTER_BUILD_ID"),
        provider: CiProvider::AppCenter,
        request_id: None,
        request_url: None,
        revision: String::new(),
        url: None,
    }
}
