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

//! Create Loadbalancer command
//!
//! Wraps invoking of the `v2/lbaas/loadbalancers` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_json;
use clap::ValueEnum;
use openstack_sdk::api::load_balancer::v2::loadbalancer::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Creates a load balancer.
///
/// This operation provisions a new load balancer by using the configuration
/// that you define in the request object. After the API validates the request
/// and starts the provisioning process, the API returns a response object that
/// contains a unique ID and the status of provisioning the load balancer.
///
/// In the response, the load balancer [provisioning status](#prov-status) is
/// `ACTIVE`, `PENDING_CREATE`, or `ERROR`.
///
/// If the status is `PENDING_CREATE`, issue GET
/// `/v2/lbaas/loadbalancers/{loadbalancer_id}` to view the progress of the
/// provisioning operation. When the load balancer status changes to `ACTIVE`,
/// the load balancer is successfully provisioned and is ready for further
/// configuration.
///
/// If the API cannot fulfill the request due to insufficient data or data that
/// is not valid, the service returns the HTTP `Bad Request (400)` response
/// code with information about the failure in the response body. Validation
/// errors require that you correct the error and submit the request again.
///
/// Administrative users can specify a project ID that is different than their
/// own to create load balancers for other projects.
///
/// An optional `flavor_id` attribute can be used to create the load balancer
/// using a pre-configured octavia flavor. Flavors are created by the operator
/// to allow custom load balancer configurations, such as allocating more
/// memory for the load balancer.
///
/// An optional `vip_qos_policy_id` attribute from Neutron can be used to apply
/// QoS policies on a loadbalancer VIP, also could pass a ‘null’ value to
/// remove QoS policies.
///
/// You can also specify the `provider` attribute when you create a load
/// balancer. The `provider` attribute specifies which backend should be used
/// to create the load balancer. This could be the default provider (`octavia`)
/// or a vendor supplied `provider` if one has been installed. Setting both a
/// flavor_id and a provider will result in a conflict error if the provider
/// does not match the provider of the configured flavor profiles.
///
/// Specifying a Virtual IP (VIP) is mandatory. There are three ways to specify
/// a VIP network for the load balancer:
///
/// Additional VIPs may also be specified in the `additional_vips` field, by
/// providing a list of JSON objects containing a `subnet_id` and optionally an
/// `ip_address`. All additional subnets must be part of the same network as
/// the primary VIP.
///
#[derive(Args)]
#[command(about = "Create a Load Balancer")]
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
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Protocol {
    Http,
    Https,
    Prometheus,
    Sctp,
    Tcp,
    TerminatedHttps,
    Udp,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Type {
    AppCookie,
    HttpCookie,
    SourceIp,
}

/// Loadbalancer Body data
#[derive(Args, Clone)]
struct Loadbalancer {
    /// A list of JSON objects defining “additional VIPs”. The format for these
    /// is `{"subnet_id": <subnet_id>, "ip_address": <ip_address>}`, where the
    /// `subnet_id` field is mandatory and the `ip_address` field is optional.
    /// Additional VIP subnets must all belong to the same network as the
    /// primary VIP.
    ///
    /// **New in version 2.26**
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    additional_vips: Option<Vec<Value>>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    admin_state_up: Option<bool>,

    /// An availability zone name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    availability_zone: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The ID of the flavor.
    ///
    #[arg(help_heading = "Body parameters", long)]
    flavor_id: Option<String>,

    /// The associated listener IDs, if any.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    listeners: Option<Vec<Value>>,

    /// Human-readable name of the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    pools: Option<Vec<Value>>,

    /// The ID of the project owning this resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    /// Provider name for the load balancer. Default is `octavia`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    provider: Option<String>,

    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,

    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,

    /// The IP address of the Virtual IP (VIP).
    ///
    #[arg(help_heading = "Body parameters", long)]
    vip_address: Option<String>,

    /// The ID of the network for the Virtual IP (VIP). One of
    /// `vip_network_id`, `vip_port_id`, or `vip_subnet_id` must be specified.
    ///
    #[arg(help_heading = "Body parameters", long)]
    vip_network_id: Option<String>,

    /// The ID of the Virtual IP (VIP) port. One of `vip_network_id`,
    /// `vip_port_id`, or `vip_subnet_id` must be specified.
    ///
    #[arg(help_heading = "Body parameters", long)]
    vip_port_id: Option<String>,

    /// The ID of the QoS Policy which will apply to the Virtual IP (VIP).
    ///
    #[arg(help_heading = "Body parameters", long)]
    vip_qos_policy_id: Option<String>,

    /// The ID of the subnet for the Virtual IP (VIP). One of `vip_network_id`,
    /// `vip_port_id`, or `vip_subnet_id` must be specified.
    ///
    #[arg(help_heading = "Body parameters", long)]
    vip_subnet_id: Option<String>,
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

    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<bool>,

    /// An availability zone name.
    ///
    #[serde()]
    #[structable(optional)]
    availability_zone: Option<String>,

    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the flavor.
    ///
    #[serde()]
    #[structable(optional)]
    flavor_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The associated listener IDs, if any.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    listeners: Option<Value>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional)]
    operating_status: Option<String>,

    /// The associated pool IDs, if any.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    pools: Option<Value>,

    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// Provider name for the load balancer.
    ///
    #[serde()]
    #[structable(optional)]
    provider: Option<String>,

    #[serde()]
    #[structable(optional)]
    provisioning_status: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

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
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponsePools {
    id: String,
}

impl fmt::Display for ResponsePools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!("id={}", self.id)]);
        write!(f, "{}", data.join(";"))
    }
}

impl LoadbalancerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Loadbalancer");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.loadbalancer data
        let args = &self.loadbalancer;
        let mut loadbalancer_builder = create::LoadbalancerBuilder::default();
        if let Some(val) = &args.name {
            loadbalancer_builder.name(val);
        }

        if let Some(val) = &args.description {
            loadbalancer_builder.description(val);
        }

        if let Some(val) = &args.admin_state_up {
            loadbalancer_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.vip_address {
            loadbalancer_builder.vip_address(val);
        }

        if let Some(val) = &args.vip_port_id {
            loadbalancer_builder.vip_port_id(val);
        }

        if let Some(val) = &args.vip_subnet_id {
            loadbalancer_builder.vip_subnet_id(val);
        }

        if let Some(val) = &args.vip_network_id {
            loadbalancer_builder.vip_network_id(val);
        }

        if let Some(val) = &args.vip_qos_policy_id {
            loadbalancer_builder.vip_qos_policy_id(val);
        }

        if let Some(val) = &args.additional_vips {
            let additional_vips_builder: Vec<create::AdditionalVips> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::AdditionalVips>(v.to_owned()))
                .collect::<Vec<create::AdditionalVips>>();
            loadbalancer_builder.additional_vips(additional_vips_builder);
        }

        if let Some(val) = &args.project_id {
            loadbalancer_builder.project_id(val);
        }

        if let Some(val) = &args.listeners {
            let listeners_builder: Vec<create::Listeners> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Listeners>(v.to_owned()))
                .collect::<Vec<create::Listeners>>();
            loadbalancer_builder.listeners(listeners_builder);
        }

        if let Some(val) = &args.pools {
            let pools_builder: Vec<create::Pools> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Pools>(v.to_owned()))
                .collect::<Vec<create::Pools>>();
            loadbalancer_builder.pools(pools_builder);
        }

        if let Some(val) = &args.provider {
            loadbalancer_builder.provider(val);
        }

        if let Some(val) = &args.tags {
            loadbalancer_builder.tags(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.flavor_id {
            loadbalancer_builder.flavor_id(val);
        }

        if let Some(val) = &args.availability_zone {
            loadbalancer_builder.availability_zone(val);
        }

        if let Some(val) = &args.tenant_id {
            loadbalancer_builder.tenant_id(val);
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
