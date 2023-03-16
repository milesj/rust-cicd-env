use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://support.atlassian.com/bitbucket-cloud/docs/variables-and-secrets/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BITBUCKET_PR_DESTINATION_BRANCH"),
        branch: var("BITBUCKET_BRANCH"),
        id: var("BITBUCKET_PIPELINE_UUID"),
        provider: CiProvider::Bitbucket,
        request_id: opt_var("BITBUCKET_PR_ID"),
        request_url: None,
        revision: var("BITBUCKET_COMMIT"),
        url: None,
    }
}
