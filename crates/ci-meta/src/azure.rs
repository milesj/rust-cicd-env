use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://learn.microsoft.com/en-us/azure/devops/pipelines/build/variables?view=azure-devops&tabs=yaml
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("SYSTEM_PULLREQUEST_TARGETBRANCH"),
        branch: opt_var("SYSTEM_PULLREQUEST_SOURCEBRANCH")
            .or_else(|| opt_var("BUILD_SOURCEBRANCHNAME"))
            .unwrap_or_default(),
        id: var("BUILD_BUILDID"),
        provider: CiProvider::Azure,
        request_id: opt_var("SYSTEM_PULLREQUEST_PULLREQUESTNUMBER")
            .or_else(|| opt_var("SYSTEM_PULLREQUEST_PULLREQUESTID")),
        request_url: None,
        revision: var("BUILD_SOURCEVERSION"),
        url: opt_var("BUILD_BUILDURI"),
    }
}
