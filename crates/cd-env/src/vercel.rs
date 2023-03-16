use crate::api::{opt_var, var, CdProvider, DeployEnvironment};

// https://vercel.com/docs/concepts/projects/environment-variables#system-environment-variables
pub fn create_environment() -> DeployEnvironment {
    DeployEnvironment {
        branch: opt_var("VERCEL_GIT_COMMIT_REF"),
        provider: CdProvider::Vercel,
        revision: var("VERCEL_GIT_COMMIT_SHA"),
        service_id: None,
    }
}
