use crate::api::{opt_var, var, CiEnvironment, CiOutput, CiProvider};

// https://buildkite.com/docs/pipelines/managing-log-output
pub const BUILDKITE_OUTPUT: CiOutput = CiOutput {
    close_log_group: "",
    open_log_group: "--- ",
};

pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BUILDKITE_PULL_REQUEST_BASE_BRANCH"),
        branch: var("BUILDKITE_BRANCH"),
        id: var("BUILDKITE_BUILD_ID"),
        provider: CiProvider::Buildkite,
        request_id: opt_var("BUILDKITE_PULL_REQUEST"),
        request_url: None,
        revision: var("BUILDKITE_COMMIT"),
        url: opt_var("BUILDKITE_BUILD_URL"),
    }
}
