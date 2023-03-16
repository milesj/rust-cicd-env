use crate::api::{opt_var, var, CdProvider, DeployEnvironment};

// https://seed.run/docs/adding-a-build-spec.html#build-environment-variables
pub fn create_environment() -> DeployEnvironment {
    DeployEnvironment {
        branch: opt_var("SEED_BRANCH"),
        provider: CdProvider::Seed,
        revision: var("SEED_BUILD_SERVICE_SHA"),
        service_id: opt_var("SEED_SERVICE_NAME"),
    }
}
