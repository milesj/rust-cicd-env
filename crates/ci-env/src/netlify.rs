use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.netlify.com/configure-builds/environment-variables/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BRANCH"),
        base_revision: None,
        branch: opt_var("HEAD")
            .or_else(|| opt_var("BRANCH"))
            .unwrap_or_default(),
        head_revision: None,
        id: var("BUILD_ID"),
        provider: CiProvider::Netlify,
        request_id: opt_var("PULL_REQUEST").map(|_| var("REVIEW_ID")),
        request_url: None,
        revision: var("COMMIT_REF"),
        url: None,
    }
}
