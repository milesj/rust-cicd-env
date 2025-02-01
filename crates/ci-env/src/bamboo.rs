use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://confluence.atlassian.com/bamboo/bamboo-variables-289277087.html
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("bamboo_repository_pr_targetBranch"),
        base_revision: None,
        branch: opt_var("bamboo_repository_pr_sourceBranch")
            .or_else(|| opt_var("bamboo_planRepository_branchName"))
            .or_else(|| opt_var("bamboo_planRepository_branch"))
            .unwrap_or_default(),
        env_prefix: Some("bamboo_".into()),
        head_revision: None,
        id: var("bamboo_buildNumber"),
        provider: CiProvider::Bamboo,
        request_id: opt_var("bamboo_repository_pr_key"),
        request_url: None,
        revision: var("bamboo_planRepository_revision"),
        url: opt_var("bamboo_buildResultsUrl"),
    }
}
