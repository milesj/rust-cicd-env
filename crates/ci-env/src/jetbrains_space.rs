use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://jetbrains.com/help/space/automation-environment-variables.html#automation
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        branch: var("JB_SPACE_GIT_BRANCH"),
        id: var("JB_SPACE_EXECUTION_NUMBER"),
        provider: CiProvider::JetbrainsSpace,
        request_id: None,
        request_url: None,
        revision: var("JB_SPACE_GIT_REVISION"),
        url: opt_var("JB_SPACE_EXECUTION_URL"),
    }
}
