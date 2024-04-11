use crate::api::{opt_var, var, CiEnvironment, CiOutput, CiProvider};
use std::{fs, path::PathBuf};

pub const TEAMCITY_OUTPUT: CiOutput = CiOutput {
    close_log_group: "##teamcity[blockClosed name='{name}']",
    open_log_group: "##teamcity[blockOpened name='{name}']",
};

// https://www.jetbrains.com/help/teamcity/predefined-build-parameters.html
pub fn create_environment() -> CiEnvironment {
    let mut env = CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: "".into(),
        head_revision: None,
        id: var("BUILD_NUMBER"),
        provider: CiProvider::TeamCity,
        request_id: None,
        request_url: None,
        revision: var("BUILD_VCS_NUMBER"),
        url: None,
    };

    if let Some(file) = opt_var("TEAMCITY_BUILD_PROPERTIES_FILE") {
        parse_build_file(PathBuf::from(file), &mut env);
    }

    env
}

fn parse_build_file(path: PathBuf, env: &mut CiEnvironment) {
    if !path.exists() {
        return;
    }

    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            if let Some(value) = line.strip_prefix("teamcity.configuration.properties.file=") {
                parse_config_file(PathBuf::from(value), env);
            }
        }
    }
}

fn parse_config_file(path: PathBuf, env: &mut CiEnvironment) {
    if !path.exists() {
        return;
    }

    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            if let Some(value) = line.strip_prefix("teamcity.build.branch=") {
                env.branch = value.to_owned();
            }
        }
    }
}
