use crate::api::{opt_var, var, CiEnvironment, CiOutput, CiProvider};

pub const AZURE_OUTPUT: CiOutput = CiOutput {
    close_log_group: "##[endgroup]",
    open_log_group: "##[group]{name}",
};

// https://learn.microsoft.com/en-us/azure/devops/pipelines/build/variables?view=azure-devops&tabs=yaml
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("SYSTEM_PULLREQUEST_TARGETBRANCH")
            .or_else(|| opt_var("BUILD_SOURCEBRANCH")),
        base_revision: None,
        branch: opt_var("SYSTEM_PULLREQUEST_SOURCEBRANCH")
            .or_else(|| opt_var("BUILD_SOURCEBRANCHNAME"))
            .unwrap_or_default(),
        env_prefix: Some("BUILD_".into()),
        head_revision: opt_var("SYSTEM_PULLREQUEST_SOURCECOMMITID"),
        id: var("BUILD_BUILDNUMBER"),
        provider: CiProvider::Azure,
        request_id: opt_var("SYSTEM_PULLREQUEST_PULLREQUESTNUMBER")
            .or_else(|| opt_var("SYSTEM_PULLREQUEST_PULLREQUESTID")),
        request_url: None,
        revision: var("BUILD_SOURCEVERSION"),
        url: opt_var("BUILD_BUILDURI"),
    }
}
