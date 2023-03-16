mod api;
mod render;

use api::*;
use std::env;

/// Detects the CI provider by checking for the existence of environment variables specific to each provider. Returns `Unknown` if no provider is detected.
pub fn detect_cd_provider() -> CdProvider {
    if env::var("RENDER").is_ok() {
        return CdProvider::Render;
    }

    CdProvider::Unknown
}

/// Returns metadata and information about the current deploy environment and CD provider.
pub fn get_deploy_environment() -> Option<DeployEnvironment> {
    let environment = match detect_cd_provider() {
        CdProvider::Render => render::create_environment(),
        CdProvider::Unknown => {
            return None;
        }
    };

    Some(environment)
}
