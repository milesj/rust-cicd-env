use crate::api::{opt_var, var, CiEnvironment, CiProvider};

pub fn create_environment() -> CiEnvironment {
    let base_branch;
    let branch;

    if let Some(pr_branch) = opt_var("APPVEYOR_PULL_REQUEST_HEAD_REPO_BRANCH") {
        base_branch = opt_var("APPVEYOR_REPO_BRANCH");
        branch = pr_branch;
    } else {
        base_branch = None;
        branch = var("APPVEYOR_REPO_BRANCH");
    }

    CiEnvironment {
        base_branch,
        branch,
        id: var("APPVEYOR_BUILD_ID"),
        provider: CiProvider::AppVeyor,
        request_id: opt_var("APPVEYOR_PULL_REQUEST_NUMBER"),
        request_url: None,
        revision: opt_var("APPVEYOR_PULL_REQUEST_HEAD_COMMIT")
            .or_else(|| opt_var("APPVEYOR_REPO_COMMIT"))
            .unwrap_or_default(),
        url: None,
    }
}
