mod api;
mod aws_codedeploy;
mod digital_ocean;
mod fly;
mod go_cd;
mod google_appengine;
mod google_cloud_run;
mod heroku;
mod octopus;
mod railway;
mod render;
mod seed;
mod vercel;

pub use api::*;
use std::env;

/// Returns true if in a CD environment by checking for the existence of a deploy provider environment variable.
pub fn is_cd() -> bool {
    !matches!(detect_provider(), CdProvider::Unknown)
}

/// Detects the CD provider by checking for the existence of environment variables specific to each provider. Returns `Unknown` if no provider is detected.
pub fn detect_provider() -> CdProvider {
    if env::var("DEPLOYMENT_GROUP_NAME").is_ok() {
        return CdProvider::AwsCodedeploy;
    }

    // Not sure if correct...
    if env::var("COMMIT_HASH").is_ok() && env::var("PUBLIC_URL").is_ok() {
        return CdProvider::DigitalOceanAppPlatform;
    }

    if env::var("FLY_APP_NAME").is_ok() {
        return CdProvider::Fly;
    }

    if env::var("GO_PIPELINE_NAME").is_ok() {
        return CdProvider::GoCD;
    }

    if env::var("GAE_SERVICE").is_ok() {
        return CdProvider::GoogleAppEngine;
    }

    if env::var("K_SERVICE").is_ok() || env::var("CLOUD_RUN_JOB").is_ok() {
        return CdProvider::GoogleCloudRun;
    }

    if env::var("HEROKU_APP_ID").is_ok() || env::var("DYNO").is_ok() {
        return CdProvider::Heroku;
    }

    if env::var("OCTOPUS_RELEASE_ID").is_ok() {
        return CdProvider::Octopus;
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
pub fn get_environment() -> Option<CdEnvironment> {
    let environment = match detect_provider() {
        CdProvider::AwsCodedeploy => aws_codedeploy::create_environment(),
        CdProvider::DigitalOceanAppPlatform => digital_ocean::create_environment(),
        CdProvider::Fly => fly::create_environment(),
        CdProvider::GoCD => go_cd::create_environment(),
        CdProvider::GoogleAppEngine => google_appengine::create_environment(),
        CdProvider::GoogleCloudRun => google_cloud_run::create_environment(),
        CdProvider::Heroku => heroku::create_environment(),
        CdProvider::Octopus => octopus::create_environment(),
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
