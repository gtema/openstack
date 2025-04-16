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

//! Create Network command
//!
//! Wraps invoking of the `v2.0/networks` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::network::create;
use openstack_types::network::v2::network::response::create::NetworkResponse;
use serde_json::Value;

/// Creates a network.
///
/// A request body is optional. An administrative user can specify another
/// project ID, which is the project that owns the network, in the request
/// body.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401
#[derive(Args)]
#[command(about = "Create network")]
pub struct NetworkCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `network` object.
    #[command(flatten)]
    network: Network,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Network Body data
#[derive(Args, Clone)]
struct Network {
    /// The administrative state of the network, which is up (`true`) or down
    /// (`false`).
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    admin_state_up: Option<bool>,

    /// The availability zone candidate for the network.
    ///
    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    availability_zone_hints: Option<Vec<String>>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// A valid DNS domain.
    #[arg(help_heading = "Body parameters", long)]
    dns_domain: Option<String>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    ha: Option<bool>,

    /// The network is default or not.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    is_default: Option<bool>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    #[arg(help_heading = "Body parameters", long)]
    mtu: Option<i32>,

    /// Human-readable name of the network.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The port security status of the network. Valid values are enabled
    /// (`true`) and disabled (`false`). This value is used as the default
    /// value of `port_security_enabled` field of a newly created port.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    port_security_enabled: Option<bool>,

    #[arg(help_heading = "Body parameters", long)]
    provider_network_type: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    provider_physical_network: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    provider_segmentation_id: Option<String>,

    /// The ID of the QoS policy associated with the network.
    #[arg(help_heading = "Body parameters", long)]
    qos_policy_id: Option<String>,

    /// Indicates whether the network has an external routing facility that’s
    /// not managed by the networking service.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    router_external: Option<bool>,

    /// A list of provider `segment` objects.
    ///
    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=crate::common::parse_json)]
    segments: Option<Vec<Value>>,

    /// Indicates whether this resource is shared across all projects. By
    /// default, only administrative users can change this value.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    shared: Option<bool>,

    /// The ID of the project that owns the resource. Only administrative and
    /// users with advsvc role can specify a project ID other than their own.
    /// You cannot change this value through authorization policies.
    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,
}

impl NetworkCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Network");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.network data
        let args = &self.network;
        let mut network_builder = create::NetworkBuilder::default();
        if let Some(val) = &args.name {
            network_builder.name(val);
        }

        if let Some(val) = &args.admin_state_up {
            network_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.tenant_id {
            network_builder.tenant_id(val);
        }

        if let Some(val) = &args.shared {
            network_builder.shared(*val);
        }

        if let Some(val) = &args.router_external {
            network_builder.router_external(*val);
        }

        if let Some(val) = &args.segments {
            let segments_builder: Vec<create::Segments> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Segments>(v.to_owned()))
                .collect::<Vec<create::Segments>>();
            network_builder.segments(segments_builder);
        }

        if let Some(val) = &args.mtu {
            network_builder.mtu(*val);
        }

        if let Some(val) = &args.availability_zone_hints {
            network_builder.availability_zone_hints(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.ha {
            network_builder.ha(*val);
        }

        if let Some(val) = &args.port_security_enabled {
            network_builder.port_security_enabled(*val);
        }

        if let Some(val) = &args.provider_network_type {
            network_builder.provider_network_type(val);
        }

        if let Some(val) = &args.provider_physical_network {
            network_builder.provider_physical_network(val);
        }

        if let Some(val) = &args.provider_segmentation_id {
            network_builder.provider_segmentation_id(val);
        }

        if let Some(val) = &args.qos_policy_id {
            network_builder.qos_policy_id(Some(val.into()));
        }

        if let Some(val) = &args.is_default {
            network_builder.is_default(*val);
        }

        if let Some(val) = &args.dns_domain {
            network_builder.dns_domain(val);
        }

        if let Some(val) = &args.description {
            network_builder.description(val);
        }

        ep_builder.network(network_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<NetworkResponse>(data)?;
        Ok(())
    }
}
