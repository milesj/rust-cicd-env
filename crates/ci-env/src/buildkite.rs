use crate::api::{opt_var, var, CiEnvironment, CiOutput, CiProvider};

// https://buildkite.com/docs/pipelines/managing-log-output
pub const BUILDKITE_OUTPUT: CiOutput = CiOutput {
    close_log_group: "",
    open_log_group: "--- ",
};

// https://buildkite.com/docs/pipelines/environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BUILDKITE_PULL_REQUEST_BASE_BRANCH"),
        base_revision: None,
        branch: var("BUILDKITE_BRANCH"),
        head_revision: None,
        id: var("BUILDKITE_BUILD_ID"),
        provider: CiProvider::Buildkite,
        request_id: opt_var("BUILDKITE_PULL_REQUEST"),
        request_url: None,
        revision: var("BUILDKITE_COMMIT"),
        url: opt_var("BUILDKITE_BUILD_URL"),
    }
}
