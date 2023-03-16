use crate::api::{opt_var, var, CdProvider, DeployEnvironment};

// https://devcenter.heroku.com/articles/dyno-metadata
pub fn create_environment() -> DeployEnvironment {
    DeployEnvironment {
        branch: None,
        provider: CdProvider::Heroku,
        revision: var("HEROKU_SLUG_COMMIT"),
        service_id: opt_var("HEROKU_APP_ID"),
    }
}
