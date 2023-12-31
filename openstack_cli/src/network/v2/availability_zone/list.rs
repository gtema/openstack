//! Lists all availability zones.
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
//!
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::network::v2::availability_zone::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Lists all availability zones.
///
/// Normal response codes: 200
///
/// Error response codes: 401
#[derive(Args, Clone, Debug)]
pub struct AvailabilityZonesArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}
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
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

pub struct AvailabilityZonesCmd {
    pub args: AvailabilityZonesArgs,
}
/// AvailabilityZones
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
    #[structable(optional, wide)]
    resource: Option<String>,

    /// The state of the availability zone, which is either `available` or
    /// `unavailable`.
    #[serde()]
    #[structable(optional, wide)]
    state: Option<String>,
}

#[async_trait]
impl Command for AvailabilityZonesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get AvailabilityZones with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = list::Request::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.query.resource {
            ep_builder.resource(val);
        }
        if let Some(val) = &self.args.query.state {
            ep_builder.state(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
