use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://devcenter.bitrise.io/en/references/available-environment-variables.html
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BITRISEIO_PULL_REQUEST_MERGE_BRANCH"),
        base_revision: None,
        branch: opt_var("BITRISEIO_PULL_REQUEST_HEAD_BRANCH")
            .or_else(|| opt_var("BITRISE_GIT_BRANCH"))
            .unwrap_or_default(),
        env_prefix: Some("BITRISE".into()),
        head_revision: None,
        id: var("BITRISEIO_PIPELINE_ID"),
        provider: CiProvider::Bitrise,
        request_id: opt_var("BITRISE_PULL_REQUEST"),
        request_url: opt_var("BITRISEIO_PULL_REQUEST_REPOSITORY_URL"),
        revision: var("BITRISE_GIT_COMMIT"),
        url: opt_var("BITRISEIO_PIPELINE_BUILD_URL"),
    }
}
