use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.semaphoreci.com/ci-cd-environment/environment-variables/
pub fn create_environment() -> CiEnvironment {
    let base_branch;
    let branch;

    if let Some(pr_branch) = opt_var("SEMAPHORE_GIT_PR_BRANCH") {
        base_branch = opt_var("SEMAPHORE_GIT_BRANCH");
        branch = pr_branch;
    } else {
        base_branch = None;
        branch = var("SEMAPHORE_GIT_BRANCH");
    }

    CiEnvironment {
        base_branch,
        base_revision: None,
        branch,
        env_prefix: Some("SEMAPHORE_".into()),
        head_revision: None,
        id: var("SEMAPHORE_WORKFLOW_ID"),
        provider: CiProvider::Semaphore,
        request_id: opt_var("SEMAPHORE_GIT_PR_NUMBER"),
        request_url: None,
        revision: opt_var("SEMAPHORE_GIT_PR_SHA")
            .or_else(|| opt_var("SEMAPHORE_GIT_SHA"))
            .unwrap_or_default(),
        url: None,
    }
}
