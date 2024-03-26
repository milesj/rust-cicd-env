use crate::api::{var, CiEnvironment, CiProvider};

pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        branch: "".into(),
        id: var("EAS_BUILD_ID"),
        provider: CiProvider::Eas,
        request_id: None,
        request_url: None,
        revision: var("EAS_BUILD_GIT_COMMIT_HASH"),
        url: None,
    }
}
