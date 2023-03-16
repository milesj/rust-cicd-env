mod agola;
mod api;
mod appcenter;
mod appveyor;
mod aws_codebuild;
mod bitbucket;
mod buildkite;
mod circleci;
mod codefresh;
mod codeship;
mod drone;
mod github;
mod gitlab;
mod google_cloud_build;
mod semaphore;
mod travisci;

pub use api::{CiEnvironment, CiOutput, CiProvider};
use std::env;

pub fn is_ci() -> bool {
    env::var("CI").is_ok()
}

pub fn detect_ci_provider() -> CiProvider {
    if env::var("AGOLA_REPOSITORY_URL").is_ok() {
        return CiProvider::Agola;
    }

    if env::var("APPCENTER_BUILD_ID").is_ok() {
        return CiProvider::AppCenter;
    }

    if env::var("APPVEYOR").is_ok() {
        return CiProvider::AppVeyor;
    }

    if env::var("CODEBUILD_BUILD_ARN").is_ok() {
        return CiProvider::AwsCodebuild;
    }

    if env::var("BITBUCKET_WORKSPACE").is_ok() {
        return CiProvider::Bitbucket;
    }

    if env::var("BUILDKITE").is_ok() {
        return CiProvider::Buildkite;
    }

    if env::var("CIRCLECI").is_ok() {
        return CiProvider::CircleCI;
    }

    if env::var("CF_ACCOUNT").is_ok() {
        return CiProvider::Codefresh;
    }

    if let Ok(var) = env::var("CI_NAME") {
        if var == "codeship" {
            return CiProvider::Codeship;
        }
    }

    if env::var("DRONE").is_ok() {
        return CiProvider::Drone;
    }

    if env::var("GITHUB_ACTIONS").is_ok() {
        return CiProvider::GithubActions;
    }

    if env::var("GITLAB_CI").is_ok() {
        return CiProvider::Gitlab;
    }

    if env::var("GOOGLE_CLOUD_BUILD").is_ok() || env::var("BUILD_OUTPUT").is_ok() {
        return CiProvider::GoogleCloudBuild;
    }

    if env::var("SEMAPHORE").is_ok() {
        return CiProvider::Semaphore;
    }

    if env::var("TRAVIS").is_ok() {
        return CiProvider::TravisCI;
    }

    CiProvider::Unknown
}

pub fn get_ci_environment() -> Option<CiEnvironment> {
    if !is_ci() {
        return None;
    }

    let environment = match detect_ci_provider() {
        CiProvider::Agola => agola::create_environment(),
        CiProvider::AppCenter => appcenter::create_environment(),
        CiProvider::AppVeyor => appveyor::create_environment(),
        CiProvider::AwsCodebuild => aws_codebuild::create_environment(),
        CiProvider::Bitbucket => bitbucket::create_environment(),
        CiProvider::Buildkite => buildkite::create_environment(),
        CiProvider::CircleCI => circleci::create_environment(),
        CiProvider::Codefresh => codefresh::create_environment(),
        CiProvider::Codeship => codeship::create_environment(),
        CiProvider::Drone => drone::create_environment(),
        CiProvider::GithubActions => github::create_environment(),
        CiProvider::Gitlab => gitlab::create_environment(),
        CiProvider::GoogleCloudBuild => google_cloud_build::create_environment(),
        CiProvider::Semaphore => semaphore::create_environment(),
        CiProvider::TravisCI => travisci::create_environment(),
        CiProvider::Unknown => {
            return None;
        }
    };

    Some(environment)
}

pub fn get_ci_output() -> CiOutput {
    match detect_ci_provider() {
        CiProvider::Buildkite => buildkite::BUILDKITE_OUTPUT,
        CiProvider::GithubActions => github::GITHUB_OUTPUT,
        _ => CiOutput::default(),
    }
}