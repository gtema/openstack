// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Set Loadbalancer command
//!
//! Wraps invoking of the `v2/lbaas/loadbalancers/{loadbalancer_id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::find;
use openstack_sdk::api::load_balancer::v2::loadbalancer::find;
use openstack_sdk::api::load_balancer::v2::loadbalancer::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates a load balancer.
///
/// If the request is valid, the service returns the `Accepted (202)` response
/// code. To confirm the update, check that the load balancer provisioning
/// status is `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation
/// to poll the load balancer object for changes.
///
/// This operation returns the updated load balancer object with the `ACTIVE`,
/// `PENDING_UPDATE`, or `ERROR` provisioning status.
///
#[derive(Args)]
#[command(about = "Update a Load Balancer")]
pub struct LoadbalancerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A load balancer object.
    ///
    #[command(flatten)]
    loadbalancer: Loadbalancer,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// loadbalancer_id parameter for /v2/lbaas/loadbalancers/{loadbalancer_id}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Loadbalancer Body data
#[derive(Args, Clone)]
struct Loadbalancer {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    admin_state_up: Option<bool>,

    /// A human-readable description for the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,

    /// The ID of the QoS Policy which will apply to the Virtual IP (VIP).
    ///
    #[arg(help_heading = "Body parameters", long)]
    vip_qos_policy_id: Option<String>,
}

/// Loadbalancer response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A list of JSON objects defining “additional VIPs”. The format for these
    /// is `{"subnet_id": <subnet_id>, "ip_address": <ip_address>}`, where the
    /// `subnet_id` field is mandatory and the `ip_address` field is optional.
    /// Additional VIP subnets must all belong to the same network as the
    /// primary VIP.
    ///
    /// **New in version 2.26**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    additional_vips: Option<Value>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<bool>,

    #[serde()]
    #[structable(optional)]
    availability_zone: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the flavor.
    ///
    #[serde()]
    #[structable(optional)]
    flavor_id: Option<String>,

    /// The ID of the load balancer.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The associated listener IDs, if any.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    listeners: Option<Value>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde()]
    #[structable(optional)]
    operating_status: Option<String>,

    /// The associated pool IDs, if any.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    pools: Option<Value>,

    /// The ID of the project owning this resource.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// Provider name for the load balancer.
    ///
    #[serde()]
    #[structable(optional)]
    provider: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[serde()]
    #[structable(optional)]
    provisioning_status: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The IP address of the Virtual IP (VIP).
    ///
    #[serde()]
    #[structable(optional)]
    vip_address: Option<String>,

    /// The ID of the network for the Virtual IP (VIP).
    ///
    #[serde()]
    #[structable(optional)]
    vip_network_id: Option<String>,

    /// The ID of the Virtual IP (VIP) port.
    ///
    #[serde()]
    #[structable(optional)]
    vip_port_id: Option<String>,

    /// The ID of the QoS Policy which will apply to the Virtual IP (VIP).
    ///
    #[serde()]
    #[structable(optional)]
    vip_qos_policy_id: Option<String>,

    /// The ID of the subnet for the Virtual IP (VIP).
    ///
    #[serde()]
    #[structable(optional)]
    vip_subnet_id: Option<String>,

    /// The VIP vNIC type used for the load balancer. One of `normal` or
    /// `direct`.
    ///
    /// **New in version 2.28**
    ///
    #[serde()]
    #[structable(optional)]
    vip_vnic_type: Option<String>,
}

impl LoadbalancerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Loadbalancer");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.loadbalancer data
        let args = &self.loadbalancer;
        let mut loadbalancer_builder = set::LoadbalancerBuilder::default();
        if let Some(val) = &args.name {
            loadbalancer_builder.name(val);
        }

        if let Some(val) = &args.description {
            loadbalancer_builder.description(val);
        }

        if let Some(val) = &args.vip_qos_policy_id {
            loadbalancer_builder.vip_qos_policy_id(val);
        }

        if let Some(val) = &args.admin_state_up {
            loadbalancer_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.tags {
            loadbalancer_builder.tags(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        ep_builder.loadbalancer(loadbalancer_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
