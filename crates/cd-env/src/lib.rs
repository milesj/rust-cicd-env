mod api;
mod fly;
mod heroku;
mod railway;
mod render;

use api::*;
use std::env;

/// Detects the CD provider by checking for the existence of environment variables specific to each provider. Returns `Unknown` if no provider is detected.
pub fn detect_cd_provider() -> CdProvider {
    if env::var("FLY_APP_NAME").is_ok() {
        return CdProvider::Fly;
    }

    if env::var("HEROKU_APP_ID").is_ok() || env::var("DYNO").is_ok() {
        return CdProvider::Heroku;
    }

    if env::var("RAILWAY_STATIC_URL").is_ok() {
        return CdProvider::Railway;
    }

    if env::var("RENDER").is_ok() {
        return CdProvider::Render;
    }

    CdProvider::Unknown
}

/// Returns metadata and information about the current deploy environment and CD provider.
pub fn get_deploy_environment() -> Option<DeployEnvironment> {
    let environment = match detect_cd_provider() {
        CdProvider::Fly => fly::create_environment(),
        CdProvider::Heroku => heroku::create_environment(),
        CdProvider::Railway => railway::create_environment(),
        CdProvider::Render => render::create_environment(),
        CdProvider::Unknown => {
            return None;
        }
    };

    Some(environment)
}
