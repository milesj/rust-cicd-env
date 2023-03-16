use crate::api::{opt_var, var, CiEnvironment, CiProvider};

pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("DRONE_TARGET_BRANCH"),
        branch: opt_var("DRONE_SOURCE_BRANCH")
            .or_else(|| opt_var("DRONE_BRANCH"))
            .unwrap_or_default(),
        id: var("DRONE_BUILD_NUMBER"),
        provider: CiProvider::Drone,
        request_id: opt_var("DRONE_PULL_REQUEST"),
        request_url: None,
        revision: var("DRONE_COMMIT"),
        url: opt_var("DRONE_BUILD_LINK"),
    }
}
