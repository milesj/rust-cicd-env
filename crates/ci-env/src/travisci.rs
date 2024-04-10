use crate::api::{opt_var, var, CiEnvironment, CiOutput, CiProvider};

pub const TRAVISCI_OUTPUT: CiOutput = CiOutput {
    close_log_group: "travis_fold:end:{name}",
    open_log_group: "travis_fold:start:{name}",
};

// https://docs.travis-ci.com/user/environment-variables/#default-environment-variables
pub fn create_environment() -> CiEnvironment {
    let base_branch;
    let branch;

    if let Some(pr_branch) = opt_var("TRAVIS_PULL_REQUEST_BRANCH") {
        base_branch = opt_var("TRAVIS_BRANCH");
        branch = pr_branch;
    } else {
        base_branch = None;
        branch = var("TRAVIS_BRANCH");
    }

    CiEnvironment {
        base_branch,
        base_revision: None,
        branch,
        head_revision: opt_var("TRAVIS_PULL_REQUEST_SHA"),
        id: var("TRAVIS_BUILD_ID"),
        provider: CiProvider::TravisCI,
        request_id: opt_var("TRAVIS_PULL_REQUEST"),
        request_url: None,
        revision: var("TRAVIS_COMMIT"),
        url: opt_var("TRAVIS_BUILD_WEB_URL"),
    }
}
