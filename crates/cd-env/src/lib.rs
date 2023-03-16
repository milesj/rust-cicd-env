mod api;
mod heroku;
mod render;

use api::*;
use std::env;

/// Detects the CI provider by checking for the existence of environment variables specific to each provider. Returns `Unknown` if no provider is detected.
pub fn detect_cd_provider() -> CdProvider {
    if env::var("HEROKU_APP_ID").is_ok() || env::var("DYNO").is_ok() {
        return CdProvider::Heroku;
    }

    if env::var("RENDER").is_ok() {
        return CdProvider::Render;
    }

    CdProvider::Unknown
}

/// Returns metadata and information about the current deploy environment and CD provider.
pub fn get_deploy_environment() -> Option<DeployEnvironment> {
    let environment = match detect_cd_provider() {
        CdProvider::Heroku => heroku::create_environment(),
        CdProvider::Render => render::create_environment(),
        CdProvider::Unknown => {
            return None;
        }
    };

    Some(environment)
}
