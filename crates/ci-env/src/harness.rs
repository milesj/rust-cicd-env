use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://developer.harness.io/docs/continuous-integration/use-ci/optimize-and-more/ci-env-var/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("DRONE_TARGET_BRANCH"),
        base_revision: None,
        branch: opt_var("DRONE_SOURCE_BRANCH")
            .or_else(|| opt_var("DRONE_BRANCH"))
            .unwrap_or_default(),
        env_prefix: Some("DRONE_".into()),
        head_revision: None,
        id: var("HARNESS_BUILD_ID"),
        provider: CiProvider::Harness,
        request_id: None,
        request_url: None,
        revision: var("DRONE_COMMIT"),
        url: None,
    }
}
