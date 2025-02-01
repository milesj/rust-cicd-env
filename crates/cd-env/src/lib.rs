mod api;
mod aws_codedeploy;
mod digital_ocean;
mod fly;
mod go_cd;
mod google_appengine;
mod google_cloud_run;
mod harness;
mod heroku;
mod netlify;
mod octopus;
mod railway;
mod release;
mod render;
mod seed;
mod vercel;

pub use api::*;
use std::env;
use std::sync::OnceLock;

/// Returns true if in a CD environment by checking for the existence of a deploy provider environment variable.
pub fn is_cd() -> bool {
    !matches!(detect_provider(), CdProvider::Unknown)
}

static PROVIDER: OnceLock<CdProvider> = OnceLock::new();

/// Detects the CD provider by checking for the existence of environment variables specific to each provider. Returns `Unknown` if no provider is detected.
pub fn detect_provider() -> CdProvider {
    *PROVIDER.get_or_init(|| {
        for (key, value) in env::vars() {
            if value.is_empty() {
                continue;
            }

            return match key.as_str() {
                "DEPLOYMENT_GROUP_NAME" => CdProvider::AwsCodedeploy,
                "FLY_APP_NAME" => CdProvider::Fly,
                "GAE_SERVICE" => CdProvider::GoogleAppEngine,
                "GO_PIPELINE_NAME" | "GO_PIPELINE_LABEL" => CdProvider::GoCD,
                "HARNESS_BUILD_ID" => CdProvider::Harness,
                "HEROKU_APP_ID" | "DYNO" => CdProvider::Heroku,
                "K_SERVICE" | "CLOUD_RUN_JOB" => CdProvider::GoogleCloudRun,
                "NETLIFY" => CdProvider::Netlify,
                "OCTOPUS_RELEASE_ID" => CdProvider::Octopus,
                "RAILWAY_STATIC_URL" => CdProvider::Railway,
                "RELEASE_BUILD_ID" => CdProvider::Release,
                "RENDER" => CdProvider::Render,
                "SEED_APP_NAME" => CdProvider::Seed,
                "VERCEL" => CdProvider::Vercel,
                _ => {
                    continue;
                }
            };
        }

        // Not sure if correct...
        if env::var("COMMIT_HASH").is_ok() && env::var("PUBLIC_URL").is_ok() {
            return CdProvider::DigitalOceanAppPlatform;
        }

        CdProvider::Unknown
    })
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
        CdProvider::Harness => harness::create_environment(),
        CdProvider::Heroku => heroku::create_environment(),
        CdProvider::Netlify => netlify::create_environment(),
        CdProvider::Octopus => octopus::create_environment(),
        CdProvider::Railway => railway::create_environment(),
        CdProvider::Release => release::create_environment(),
        CdProvider::Render => render::create_environment(),
        CdProvider::Seed => seed::create_environment(),
        CdProvider::Vercel => vercel::create_environment(),
        CdProvider::Unknown => {
            return None;
        }
    };

    Some(environment)
}
