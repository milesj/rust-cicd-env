mod agola;
mod api;
mod appcenter;
mod appcircle;
mod appveyor;
mod aws_codebuild;
mod azure;
mod bamboo;
mod bitbucket;
mod bitrise;
mod buddy;
mod buildkite;
mod circleci;
mod cirrus;
mod codefresh;
mod codemagic;
mod codeship;
mod drone;
mod github;
mod gitlab;
mod google_cloud_build;
mod heroku;
mod jenkins;
mod jenkins_x;
mod netlify;
mod semaphore;
mod travisci;
mod vela;
mod woodpecker;

pub use api::{CiEnvironment, CiOutput, CiProvider};
use std::env;

/// Returns true if in a CI environment by checking for the existence of the `CI` environment variable. It does not validate the variable value.
pub fn is_ci() -> bool {
    env::var("CI").is_ok()
}

/// Detects the CI provider by checking for the existence of environment variables specific to each provider. Returns `Unknown` if no provider is detected.
pub fn detect_ci_provider() -> CiProvider {
    if env::var("AGOLA_REPOSITORY_URL").is_ok() {
        return CiProvider::Agola;
    }

    if env::var("APPCENTER_BUILD_ID").is_ok() {
        return CiProvider::AppCenter;
    }

    if env::var("AC_APPCIRCLE").is_ok() {
        return CiProvider::Appcircle;
    }

    if env::var("APPVEYOR").is_ok() {
        return CiProvider::AppVeyor;
    }

    if env::var("CODEBUILD_BUILD_ARN").is_ok() {
        return CiProvider::AwsCodebuild;
    }

    if env::var("BUILD_BUILDID").is_ok() {
        return CiProvider::Azure;
    }

    if env::var("bamboo_planKey").is_ok() {
        return CiProvider::Bamboo;
    }

    if env::var("BITBUCKET_WORKSPACE").is_ok() {
        return CiProvider::Bitbucket;
    }

    if env::var("BITRISE_IO").is_ok() {
        return CiProvider::Bitrise;
    }

    if env::var("BUDDY").is_ok() {
        return CiProvider::Buddy;
    }

    if env::var("BUILDKITE").is_ok() {
        return CiProvider::Buildkite;
    }

    if env::var("CIRCLECI").is_ok() {
        return CiProvider::CircleCI;
    }

    if env::var("CIRRUS_CI").is_ok() {
        return CiProvider::Cirrus;
    }

    if env::var("CF_ACCOUNT").is_ok() {
        return CiProvider::Codefresh;
    }

    if env::var("CM_BUILD_ID").is_ok() {
        return CiProvider::Codemagic;
    }

    if let Ok(var) = env::var("CI") {
        if var == "woodpecker" {
            return CiProvider::Woodpecker;
        }
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

    if env::var("HEROKU_TEST_RUN_ID").is_ok() {
        return CiProvider::Heroku;
    }

    if env::var("JENKINS_URL").is_ok() {
        return CiProvider::Jenkins;
    }

    if env::var("JENKINS_X_URL").is_ok() {
        return CiProvider::JenkinsX;
    }

    if env::var("NETLIFY").is_ok() {
        return CiProvider::Netlify;
    }

    if env::var("SEMAPHORE").is_ok() {
        return CiProvider::Semaphore;
    }

    if env::var("TEAMCITY_VERSION").is_ok() {
        return CiProvider::TeamCity;
    }

    if env::var("TRAVIS").is_ok() {
        return CiProvider::TravisCI;
    }

    if env::var("VELA").is_ok() {
        return CiProvider::Vela;
    }

    CiProvider::Unknown
}

/// Returns metadata and information about the current CI environmen and CI provider.
pub fn get_ci_environment() -> Option<CiEnvironment> {
    if !is_ci() {
        return None;
    }

    let environment = match detect_ci_provider() {
        CiProvider::Agola => agola::create_environment(),
        CiProvider::AppCenter => appcenter::create_environment(),
        CiProvider::Appcircle => appcircle::create_environment(),
        CiProvider::AppVeyor => appveyor::create_environment(),
        CiProvider::AwsCodebuild => aws_codebuild::create_environment(),
        CiProvider::Azure => azure::create_environment(),
        CiProvider::Bamboo => bamboo::create_environment(),
        CiProvider::Bitbucket => bitbucket::create_environment(),
        CiProvider::Bitrise => bitrise::create_environment(),
        CiProvider::Buddy => buddy::create_environment(),
        CiProvider::Buildkite => buildkite::create_environment(),
        CiProvider::CircleCI => circleci::create_environment(),
        CiProvider::Cirrus => cirrus::create_environment(),
        CiProvider::Codefresh => codefresh::create_environment(),
        CiProvider::Codemagic => codemagic::create_environment(),
        CiProvider::Codeship => codeship::create_environment(),
        CiProvider::Drone => drone::create_environment(),
        CiProvider::GithubActions => github::create_environment(),
        CiProvider::Gitlab => gitlab::create_environment(),
        CiProvider::GoogleCloudBuild => google_cloud_build::create_environment(),
        CiProvider::Heroku => heroku::create_environment(),
        CiProvider::Jenkins => jenkins::create_environment(),
        CiProvider::JenkinsX => jenkins_x::create_environment(),
        CiProvider::Netlify => netlify::create_environment(),
        CiProvider::Semaphore => semaphore::create_environment(),
        CiProvider::TeamCity => {
            // No env vars to use
            return None;
        }
        CiProvider::TravisCI => travisci::create_environment(),
        CiProvider::Vela => vela::create_environment(),
        CiProvider::Woodpecker => woodpecker::create_environment(),
        CiProvider::Unknown => {
            return None;
        }
    };

    Some(environment)
}

/// Returns the output format for the current CI provider.
pub fn get_ci_output() -> Option<CiOutput> {
    match detect_ci_provider() {
        CiProvider::Buildkite => Some(buildkite::BUILDKITE_OUTPUT),
        CiProvider::GithubActions => Some(github::GITHUB_OUTPUT),
        _ => None,
    }
}
