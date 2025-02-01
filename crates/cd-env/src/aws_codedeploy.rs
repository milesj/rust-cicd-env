use crate::api::{var, CdEnvironment, CdProvider};

// https://docs.aws.amazon.com/codedeploy/latest/userguide/reference-appspec-file-structure-hooks.html
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: Some("BUNDLE_".into()),
        provider: CdProvider::AwsCodedeploy,
        revision: var("BUNDLE_COMMIT"),
        service_id: None,
    }
}
