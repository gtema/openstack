//! Lists all availability zones.
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
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

use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::network::v2::availability_zone::list;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct AvailabilityZonesArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {
    /// name query parameter for /v2.0/availability_zones API
    #[arg(long)]
    name: Option<String>,

    /// resource query parameter for /v2.0/availability_zones API
    #[arg(long)]
    resource: Option<String>,

    /// state query parameter for /v2.0/availability_zones API
    #[arg(long)]
    state: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// AvailabilityZones list command
pub struct AvailabilityZonesCmd {
    pub args: AvailabilityZonesArgs,
}
/// AvailabilityZones response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// Human-readable name of the resource.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The resource type of the availability zone. The supported resource
    /// types
    /// are `network` and `router`.
    #[serde()]
    #[structable(optional)]
    resource: Option<String>,

    /// The state of the availability zone, which is either `available` or
    /// `unavailable`.
    #[serde()]
    #[structable(optional)]
    state: Option<String>,
}

#[async_trait]
impl OSCCommand for AvailabilityZonesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List AvailabilityZones with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.query.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &self.args.query.resource {
            ep_builder.resource(val.clone());
        }
        if let Some(val) = &self.args.query.state {
            ep_builder.state(val.clone());
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
