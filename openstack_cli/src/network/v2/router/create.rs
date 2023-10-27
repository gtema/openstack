//! Create Router
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

use crate::common::parse_json;
use crate::common::VecString;
use crate::common::VecValue;
use openstack_sdk::api::network::v2::routers::post;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Create Router
#[derive(Args, Clone, Debug)]
pub struct RouterArgs {
    /// The administrative state of the router, which is up ``True`` or down
    /// ``False``.
    #[arg(long)]
    is_admin_state_up: Option<bool>,

    /// Availability zone hints to use when scheduling the router.
    #[arg(long, action=clap::ArgAction::Append)]
    availability_zone_hints: Option<Vec<String>>,

    /// The router description.
    #[arg(long)]
    description: Option<String>,

    /// The distributed state of the router, which is distributed ``True`` or
    /// not ``False``.
    #[arg(long)]
    is_distributed: Option<bool>,

    /// The ndp proxy state of the router
    #[arg(long)]
    enable_ndp_proxy: Option<bool>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with network_id, enable_snat,
    /// external_fixed_ips and qos_policy_id. Otherwise, this would be null.
    #[arg(long, value_parser=parse_json, value_name="JSON_VALUE")]
    external_gateway_info: Option<Value>,

    /// The ID of the flavor.
    #[arg(long)]
    flavor_id: Option<String>,

    /// The highly-available state of the router, which is highly available
    /// ``True`` or not ``False``.
    #[arg(long)]
    is_ha: Option<bool>,

    /// The router name.
    #[arg(long)]
    name: Option<String>,

    /// The ID of the project this router is associated with.
    #[arg(long)]
    project_id: Option<String>,

    /// Tenant_id (deprecated attribute).
    #[arg(long)]
    tenant_id: Option<String>,
}

pub struct RouterCmd {
    pub args: RouterArgs,
}

/// Router
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Router {
    /// The administrative state of the router, which is up ``True`` or down
    /// ``False``.
    #[serde(rename = "admin_state_up")]
    #[structable(optional)]
    is_admin_state_up: Option<bool>,

    /// Availability zone hints to use when scheduling the router.
    #[structable(optional)]
    availability_zone_hints: Option<VecString>,

    /// Availability zones for the router.
    #[structable(optional)]
    availability_zones: Option<VecString>,

    /// Timestamp when the router was created.
    #[structable(optional)]
    created_at: Option<String>,

    /// The router description.
    #[structable(optional)]
    description: Option<String>,

    /// The distributed state of the router, which is distributed ``True`` or
    /// not ``False``.
    #[serde(rename = "distributed")]
    #[structable(optional)]
    is_distributed: Option<bool>,

    /// The ndp proxy state of the router
    #[structable(optional)]
    enable_ndp_proxy: Option<bool>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with network_id, enable_snat,
    /// external_fixed_ips and qos_policy_id. Otherwise, this would be null.
    #[structable(optional)]
    external_gateway_info: Option<Value>,

    /// The ID of the flavor.
    #[structable(optional)]
    flavor_id: Option<String>,

    /// The highly-available state of the router, which is highly available
    /// ``True`` or not ``False``.
    #[serde(rename = "ha")]
    #[structable(optional)]
    is_ha: Option<bool>,

    /// Id of the resource
    #[structable(optional)]
    id: Option<String>,

    /// The router name.
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the project this router is associated with.
    #[structable(optional)]
    project_id: Option<String>,

    /// Revision number of the router.
    #[serde(rename = "revision")]
    #[structable(optional)]
    revision_number: Option<u32>,

    /// The extra routes configuration for the router.
    #[structable(optional)]
    routes: Option<VecValue>,

    /// The router status.
    #[structable(optional)]
    status: Option<String>,

    /// Router Tags.
    #[structable(optional)]
    tags: Option<VecString>,

    /// Tenant_id (deprecated attribute).
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Timestamp when the router was created.
    #[structable(optional)]
    updated_at: Option<String>,
}

#[async_trait]
impl Command for RouterCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Router with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = post::Router::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        if let Some(val) = &self.args.is_admin_state_up {
            ep_builder.is_admin_state_up(*val);
        }
        if let Some(val) = &self.args.availability_zone_hints {
            ep_builder.availability_zone_hints(val.iter().cloned());
        }
        if let Some(val) = &self.args.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.args.is_distributed {
            ep_builder.is_distributed(*val);
        }
        if let Some(val) = &self.args.enable_ndp_proxy {
            ep_builder.enable_ndp_proxy(*val);
        }
        if let Some(val) = &self.args.external_gateway_info {
            let sub: post::ExternalGatewayInfo = serde_json::from_value(val.clone())?;
            ep_builder.external_gateway_info(sub);
        }
        if let Some(val) = &self.args.flavor_id {
            ep_builder.flavor_id(val);
        }
        if let Some(val) = &self.args.is_ha {
            ep_builder.is_ha(*val);
        }
        if let Some(val) = &self.args.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.args.tenant_id {
            ep_builder.tenant_id(val);
        }
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data = ep.query_async(client).await?;
        op.output_single::<Router>(data)?;
        Ok(())
    }
}
