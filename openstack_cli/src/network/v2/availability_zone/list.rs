//! List Availability zones
use async_trait::async_trait;
use clap::Args;
use http::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::network::v2::availability_zones::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// List Availability zones
#[derive(Args, Clone, Debug)]
pub struct AvailabilityZonesArgs {
    /// Filter the list result by the state of the availability zone, which is
    /// either available or unavailable.
    #[arg(long)]
    state: Option<String>,

    /// Filter the list result by the resource type of the availability zone.
    /// The supported resource types are network and router.
    #[arg(long)]
    resource: Option<String>,

    /// Filter the list result by the human-readable name of the resource.
    #[arg(long)]
    name: Option<String>,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

pub struct AvailabilityZonesCmd {
    pub args: AvailabilityZonesArgs,
}

/// AvailabilityZones
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct AvailabilityZones {
    /// The state of the availability zone, which is either available or
    /// unavailable.
    #[structable(optional, wide)]
    state: Option<String>,

    /// The resource type of the availability zone. The supported resource
    /// types are network and router.
    #[structable(optional, wide)]
    resource: Option<String>,

    /// Human-readable name of the resource.
    #[structable(optional)]
    name: Option<String>,
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
        let mut ep_builder = get::AvailabilityZone::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.state {
            ep_builder.state(val);
        }
        if let Some(val) = &self.args.resource {
            ep_builder.resource(val);
        }
        if let Some(val) = &self.args.name {
            ep_builder.name(val);
        }
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<AvailabilityZones>(data)?;
        Ok(())
    }
}
