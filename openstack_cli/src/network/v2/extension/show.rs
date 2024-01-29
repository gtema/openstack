//! Shows details for an extension, by alias.
//! The response shows the extension name and its alias. To show
//! details for an extension, you specify the alias.
//!
//! Normal response codes: 200
//!
//! Error response codes: 401, 404
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
use crate::{OSCCommand, OpenStackCliError};
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::network::v2::extension::get;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ExtensionArgs {
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
    /// id parameter for /v2.0/extensions/{id} API
    #[arg()]
    id: String,
}

/// Extension show command
pub struct ExtensionCmd {
    pub args: ExtensionArgs,
}
/// Extension response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The alias for the extension. For example “quotas” or
    /// “security-group”.
    #[serde()]
    #[structable(optional)]
    alias: Option<String>,

    /// The human-readable description for the resource.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// Human-readable name of the resource.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A URL pointing to the namespace for this extension.
    #[serde()]
    #[structable(optional)]
    namespace: Option<String>,

    /// The date and timestamp when the extension was
    /// last updated.
    #[serde()]
    #[structable(optional)]
    updated: Option<String>,
}

#[async_trait]
impl OSCCommand for ExtensionCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Extension with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
