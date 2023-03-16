mod api;
mod fly;
mod heroku;
mod railway;
mod render;
mod seed;
mod vercel;

use api::*;
use std::env;

/// Returns true if in a CD environment by checking for the existence of a deploy provider environment variable.
pub fn is_cd() -> bool {
    !matches!(detect_cd_provider(), CdProvider::Unknown)
}

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

    if env::var("SEED_APP_NAME").is_ok() {
        return CdProvider::Seed;
    }

    if env::var("VERCEL").is_ok() {
        return CdProvider::Vercel;
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
        CdProvider::Seed => seed::create_environment(),
        CdProvider::Vercel => vercel::create_environment(),
        CdProvider::Unknown => {
            return None;
        }
    };

    Some(environment)
}
