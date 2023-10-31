//! List Routers
use async_trait::async_trait;
use clap::Args;
use http::Response;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use crate::common::VecString;
use crate::common::VecValue;
use openstack_sdk::api::network::v2::routers::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;

/// List Routers
#[derive(Args, Clone, Debug)]
pub struct RoutersArgs {
    /// limit filter parameter
    #[arg(long)]
    limit: Option<String>,

    /// marker filter parameter
    #[arg(long)]
    marker: Option<String>,

    /// description filter parameter
    #[arg(long)]
    description: Option<String>,

    /// flavor_id filter parameter
    #[arg(long)]
    flavor_id: Option<String>,

    /// name filter parameter
    #[arg(long)]
    name: Option<String>,

    /// status filter parameter
    #[arg(long)]
    status: Option<String>,

    /// project_id filter parameter
    #[arg(long)]
    project_id: Option<String>,

    /// is_admin_state_up filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_admin_state_up: Option<bool>,

    /// is_distributed filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_distributed: Option<bool>,

    /// is_ha filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_ha: Option<bool>,

    /// tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    tags: Option<Vec<String>>,

    /// any_tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    any_tags: Option<Vec<String>>,

    /// not_tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    not_tags: Option<Vec<String>>,

    /// not_any_tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    not_any_tags: Option<Vec<String>>,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

pub struct RoutersCmd {
    pub args: RoutersArgs,
}

/// Routers
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Routers {
    /// The administrative state of the router, which is up ``True`` or down
    /// ``False``.
    #[serde(rename = "admin_state_up")]
    #[structable(optional, wide)]
    is_admin_state_up: Option<bool>,

    /// Availability zone hints to use when scheduling the router.
    #[structable(optional, wide)]
    availability_zone_hints: Option<VecString>,

    /// Availability zones for the router.
    #[structable(optional, wide)]
    availability_zones: Option<VecString>,

    /// Timestamp when the router was created.
    #[structable(optional)]
    created_at: Option<String>,

    /// The router description.
    #[structable(optional, wide)]
    description: Option<String>,

    /// The distributed state of the router, which is distributed ``True`` or
    /// not ``False``.
    #[serde(rename = "distributed")]
    #[structable(optional, wide)]
    is_distributed: Option<bool>,

    /// The ndp proxy state of the router
    #[structable(optional, wide)]
    enable_ndp_proxy: Option<bool>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with network_id, enable_snat,
    /// external_fixed_ips and qos_policy_id. Otherwise, this would be null.
    #[structable(optional, wide)]
    external_gateway_info: Option<Value>,

    /// The ID of the flavor.
    #[structable(optional, wide)]
    flavor_id: Option<String>,

    /// The highly-available state of the router, which is highly available
    /// ``True`` or not ``False``.
    #[serde(rename = "ha")]
    #[structable(optional, wide)]
    is_ha: Option<bool>,

    /// Id of the resource
    #[structable(optional)]
    id: Option<String>,

    /// The router name.
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the project this router is associated with.
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// Revision number of the router.
    #[serde(rename = "revision")]
    #[structable(optional, wide)]
    revision_number: Option<u32>,

    /// The extra routes configuration for the router.
    #[structable(optional, wide)]
    routes: Option<VecValue>,

    /// The router status.
    #[structable(optional, wide)]
    status: Option<String>,

    /// Router Tags.
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// Tenant_id (deprecated attribute).
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// Timestamp when the router was created.
    #[structable(optional)]
    updated_at: Option<String>,
}

#[async_trait]
impl Command for RoutersCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Routers with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Routers::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.limit {
            ep_builder.limit(val);
        }
        if let Some(val) = &self.args.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.args.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.args.flavor_id {
            ep_builder.flavor_id(val);
        }
        if let Some(val) = &self.args.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.args.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.args.is_admin_state_up {
            ep_builder.is_admin_state_up(*val);
        }
        if let Some(val) = &self.args.is_distributed {
            ep_builder.is_distributed(*val);
        }
        if let Some(val) = &self.args.is_ha {
            ep_builder.is_ha(*val);
        }
        if let Some(val) = &self.args.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.args.any_tags {
            ep_builder.any_tags(val.iter());
        }
        if let Some(val) = &self.args.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.args.not_any_tags {
            ep_builder.not_any_tags(val.iter());
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

        op.output_list::<Routers>(data)?;
        Ok(())
    }
}
