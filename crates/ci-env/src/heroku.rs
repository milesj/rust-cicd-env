use crate::api::{var, CiEnvironment, CiProvider};

// https://devcenter.heroku.com/articles/heroku-ci#immutable-environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("HEROKU_TEST_RUN_BRANCH"),
        head_revision: None,
        id: var("HEROKU_TEST_RUN_ID"),
        provider: CiProvider::Heroku,
        request_id: None,
        request_url: None,
        revision: var("HEROKU_TEST_RUN_COMMIT_VERSION"),
        url: None,
    }
}
