use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://devcenter.heroku.com/articles/dyno-metadata
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: Some("HEROKU_".into()),
        provider: CdProvider::Heroku,
        revision: var("HEROKU_SLUG_COMMIT"),
        service_id: opt_var("HEROKU_APP_ID"),
    }
}
