use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://support.atlassian.com/bitbucket-cloud/docs/variables-and-secrets/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BITBUCKET_PR_DESTINATION_BRANCH"),
        base_revision: None,
        branch: var("BITBUCKET_BRANCH"),
        env_prefix: Some("BITBUCKET_".into()),
        head_revision: None,
        id: var("BITBUCKET_PIPELINE_UUID"),
        provider: CiProvider::Bitbucket,
        request_id: opt_var("BITBUCKET_PR_ID"),
        request_url: None,
        revision: var("BITBUCKET_COMMIT"),
        url: None,
    }
}
