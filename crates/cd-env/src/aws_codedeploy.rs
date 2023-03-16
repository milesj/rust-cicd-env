use crate::api::{var, CdProvider, DeployEnvironment};

// https://docs.aws.amazon.com/codedeploy/latest/userguide/reference-appspec-file-structure-hooks.html
pub fn create_environment() -> DeployEnvironment {
    DeployEnvironment {
        branch: None,
        provider: CdProvider::AwsCodedeploy,
        revision: var("BUNDLE_COMMIT"),
        service_id: None,
    }
}
