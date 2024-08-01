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

//! Show SecurityGroupRule command
//!
//! Wraps invoking of the `v2.0/security-group-rules/{id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::BoolString;
use openstack_sdk::api::network::v2::security_group_rule::get;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Shows detailed information for a security group rule.
///
/// The response body contains the following information about the security
/// group rule:
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. For more information, see [Fields](#fields).
///
/// Normal response codes: 200
///
/// Error response codes: 401, 404
///
#[derive(Args)]
#[command(about = "Show security group rule")]
pub struct SecurityGroupRuleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/security-group-rules/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// SecurityGroupRule response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Indicates if the security group rule belongs to the default security
    /// group of the project or not.
    ///
    #[serde()]
    #[structable(optional)]
    belongs_to_default_sg: Option<BoolString>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// Ingress or egress, which is the direction in which the security group
    /// rule is applied.
    ///
    #[serde()]
    #[structable(optional)]
    direction: Option<String>,

    /// Must be IPv4 or IPv6, and addresses represented in CIDR must match the
    /// ingress or egress rules.
    ///
    #[serde()]
    #[structable(optional)]
    ethertype: Option<String>,

    /// The ID of the security group rule.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    normalized_cidr: Option<String>,

    /// The maximum port number in the range that is matched by the security
    /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
    /// value must be greater than or equal to the `port_range_min` attribute
    /// value. If the protocol is ICMP, this value must be an ICMP code.
    ///
    #[serde()]
    #[structable(optional)]
    port_range_max: Option<i32>,

    /// The minimum port number in the range that is matched by the security
    /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
    /// value must be less than or equal to the `port_range_max` attribute
    /// value. If the protocol is ICMP, this value must be an ICMP type.
    ///
    #[serde()]
    #[structable(optional)]
    port_range_min: Option<i32>,

    /// The IP protocol can be represented by a string, an integer, or `null`.
    /// Valid string or integer values are `any` or `0`, `ah` or `51`, `dccp`
    /// or `33`, `egp` or `8`, `esp` or `50`, `gre` or `47`, `icmp` or `1`,
    /// `icmpv6` or `58`, `igmp` or `2`, `ipip` or `4`, `ipv6-encap` or `41`,
    /// `ipv6-frag` or `44`, `ipv6-icmp` or `58`, `ipv6-nonxt` or `59`,
    /// `ipv6-opts` or `60`, `ipv6-route` or `43`, `ospf` or `89`, `pgm` or
    /// `113`, `rsvp` or `46`, `sctp` or `132`, `tcp` or `6`, `udp` or `17`,
    /// `udplite` or `136`, `vrrp` or `112`. Additionally, any integer value
    /// between \[0-255\] is also valid. The string `any` (or integer `0`)
    /// means `all` IP protocols. See the constants in `neutron_lib.constants`
    /// for the most up-to-date list of supported strings.
    ///
    #[serde()]
    #[structable(optional)]
    protocol: Option<String>,

    /// The remote address group UUID that is associated with this security
    /// group rule.
    ///
    #[serde()]
    #[structable(optional)]
    remote_address_group_id: Option<String>,

    /// The remote group UUID to associate with this security group rule. You
    /// can specify either the `remote_group_id` or `remote_ip_prefix`
    /// attribute in the request body.
    ///
    #[serde()]
    #[structable(optional)]
    remote_group_id: Option<String>,

    /// The remote IP prefix that is matched by this security group rule.
    ///
    #[serde()]
    #[structable(optional)]
    remote_ip_prefix: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The security group ID to associate with this security group rule.
    ///
    #[serde()]
    #[structable(optional)]
    security_group_id: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl SecurityGroupRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show SecurityGroupRule");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
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
