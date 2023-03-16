use crate::api::{opt_var, var, CiEnvironment, CiOutput, CiProvider};

// https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#grouping-log-lines
pub const GITHUB_OUTPUT: CiOutput = CiOutput {
    close_log_group: "::endgroup::",
    open_log_group: "::group::",
};

// https://docs.github.com/en/actions/learn-github-actions/variables#default-environment-variables
pub fn create_environment() -> CiEnvironment {
    let ref_path = var("GITHUB_REF");

    // GITHUB_HEAD_SHA and GITHUB_PULL_REQUEST are non-standard
    CiEnvironment {
        base_branch: opt_var("GITHUB_BASE_REF"),
        branch: opt_var("GITHUB_HEAD_REF")
            .or_else(|| opt_var("GITHUB_REF_NAME"))
            .unwrap_or_default(),
        id: var("GITHUB_RUN_ID"),
        provider: CiProvider::GithubActions,
        request_id: opt_var("GITHUB_PULL_REQUEST").or_else(|| {
            if ref_path.starts_with("refs/pull") {
                Some(ref_path.replace("refs/pull/", "").replace("/merge", ""))
            } else {
                None
            }
        }),
        request_url: None,
        revision: opt_var("GITHUB_HEAD_SHA")
            .or_else(|| opt_var("GITHUB_SHA"))
            .unwrap_or_default(),
        url: None,
    }
}
