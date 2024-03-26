use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://developer.apple.com/documentation/xcode/environment-variable-reference
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("CI_PULL_REQUEST_TARGET_BRANCH"),
        branch: opt_var("CI_PULL_REQUEST_SOURCE_BRANCH").unwrap_or_else(|| var("CI_BRANCH")),
        id: var("CI_BUILD_ID"),
        provider: CiProvider::XcodeCloud,
        request_id: opt_var("CI_PULL_REQUEST_NUMBER"),
        request_url: opt_var("CI_PULL_REQUEST_HTML_URL"),
        revision: var("CI_COMMIT"),
        url: opt_var("CI_BUILD_URL"),
    }
}
