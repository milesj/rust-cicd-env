use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://scrutinizer-ci.com/docs/build/environment-variables
pub fn create_environment() -> CiEnvironment {
    let request_id = opt_var("SCRUTINIZER_PR_NUMBER");

    CiEnvironment {
        base_branch: if request_id.is_some() {
            opt_var("SCRUTINIZER_BRANCH")
        } else {
            None
        },
        base_revision: None,
        branch: opt_var("SCRUTINIZER_PR_SOURCE_BRANCH")
            .unwrap_or_else(|| var("SCRUTINIZER_BRANCH")),
        head_revision: None,
        id: var("SCRUTINIZER_INSPECTION_UUID"),
        provider: CiProvider::Scrutinizer,
        request_id,
        request_url: None,
        revision: var("SCRUTINIZER_SHA1"),
        url: None,
    }
}
