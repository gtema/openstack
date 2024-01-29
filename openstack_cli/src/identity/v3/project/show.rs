//! Shows details for a project.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/project`
//!
use async_trait::async_trait;

use clap::Args;

use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, OSCCommand};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::project::find;

use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ProjectArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// project_id parameter for
    /// /v3/projects/{project_id}/groups/{group_id}/roles API
    #[arg()]
    id: String,
}

/// Project show command
pub struct ProjectCmd {
    pub args: ProjectArgs,
}
/// Project response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID for the project.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The description of the project.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the domain for the project.
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional)]
    is_domain: Option<bool>,

    /// The ID of the parent for the project.
    ///
    ///
    /// **New in version 3.4**
    #[serde()]
    #[structable(optional)]
    parent_id: Option<String>,

    /// The name of the project.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A list of simple strings assigned to a project.
    #[serde()]
    #[structable(optional)]
    tags: Option<VecString>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    #[serde()]
    #[structable(optional)]
    options: Option<ResponseOptions>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseOptions {
    immutable: Option<bool>,
}

impl fmt::Display for ResponseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "immutable={}",
            self.immutable
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}

#[async_trait]
impl OSCCommand for ProjectCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Project with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.args.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
