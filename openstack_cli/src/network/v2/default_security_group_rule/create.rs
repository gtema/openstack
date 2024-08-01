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

//! Create DefaultSecurityGroupRule command
//!
//! Wraps invoking of the `v2.0/default-security-group-rules` with `POST` method

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
use clap::ValueEnum;
use openstack_sdk::api::network::v2::default_security_group_rule::create;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Creates an Openstack Networking security group rule template.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 404, 409
///
#[derive(Args)]
#[command(about = "Create security group default rule")]
pub struct DefaultSecurityGroupRuleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `default_security_group_rule` object.
    ///
    #[command(flatten)]
    default_security_group_rule: DefaultSecurityGroupRule,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Direction {
    Egress,
    Ingress,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Ethertype {
    Ipv4,
    Ipv6,
}

/// DefaultSecurityGroupRule Body data
#[derive(Args, Clone)]
struct DefaultSecurityGroupRule {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Ingress or egress, which is the direction in which the security group
    /// rule is applied.
    ///
    #[arg(help_heading = "Body parameters", long)]
    direction: Option<Direction>,

    /// Must be IPv4 or IPv6, and addresses represented in CIDR must match the
    /// ingress or egress rules.
    ///
    #[arg(help_heading = "Body parameters", long)]
    ethertype: Option<Ethertype>,

    /// The maximum port number in the range that is matched by the security
    /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
    /// value must be greater than or equal to the `port_range_min` attribute
    /// value. If the protocol is ICMP, this value must be an ICMP code.
    ///
    #[arg(help_heading = "Body parameters", long)]
    port_range_max: Option<Option<i32>>,

    /// The minimum port number in the range that is matched by the security
    /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
    /// value must be less than or equal to the `port_range_max` attribute
    /// value. If the protocol is ICMP, this value must be an ICMP type.
    ///
    #[arg(help_heading = "Body parameters", long)]
    port_range_min: Option<Option<i32>>,

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
    #[arg(help_heading = "Body parameters", long)]
    protocol: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    remote_address_group_id: Option<String>,

    /// The remote group UUID to associate with this security group rule. You
    /// can specify either the `remote_group_id` or `remote_ip_prefix`
    /// attribute in the request body.
    ///
    #[arg(help_heading = "Body parameters", long)]
    remote_group_id: Option<String>,

    /// The remote IP prefix that is matched by this security group rule.
    ///
    #[arg(help_heading = "Body parameters", long)]
    remote_ip_prefix: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,

    /// Whether this security group rule template should be used in default
    /// security group created automatically for each new project. Default
    /// value is `False`.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    used_in_default_sg: Option<bool>,

    /// Whether this security group rule template should be used in custom
    /// security groups created by project user. Default value is `True`.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    used_in_non_default_sg: Option<bool>,
}

/// DefaultSecurityGroupRule response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
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

    /// The ID of the security group default rule.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

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

    /// The remote address group UUID to associate with this security group
    /// rule.
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

    /// Whether this security group rule template should be used in default
    /// security group created automatically for each new project. Default
    /// value is `False`.
    ///
    #[serde()]
    #[structable(optional)]
    used_in_default_sg: Option<BoolString>,

    /// Whether this security group rule template should be used in custom
    /// security groups created by project user. Default value is `True`.
    ///
    #[serde()]
    #[structable(optional)]
    used_in_non_default_sg: Option<BoolString>,
}

impl DefaultSecurityGroupRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create DefaultSecurityGroupRule");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.default_security_group_rule data
        let args = &self.default_security_group_rule;
        let mut default_security_group_rule_builder =
            create::DefaultSecurityGroupRuleBuilder::default();
        if let Some(val) = &args.tenant_id {
            default_security_group_rule_builder.tenant_id(val);
        }

        if let Some(val) = &args.description {
            default_security_group_rule_builder.description(val);
        }

        if let Some(val) = &args.remote_group_id {
            default_security_group_rule_builder.remote_group_id(val);
        }

        if let Some(val) = &args.remote_address_group_id {
            default_security_group_rule_builder.remote_address_group_id(val);
        }

        if let Some(val) = &args.direction {
            let tmp = match val {
                Direction::Egress => create::Direction::Egress,
                Direction::Ingress => create::Direction::Ingress,
            };
            default_security_group_rule_builder.direction(tmp);
        }

        if let Some(val) = &args.protocol {
            default_security_group_rule_builder.protocol(val);
        }

        if let Some(val) = &args.port_range_min {
            default_security_group_rule_builder.port_range_min(*val);
        }

        if let Some(val) = &args.port_range_max {
            default_security_group_rule_builder.port_range_max(*val);
        }

        if let Some(val) = &args.ethertype {
            let tmp = match val {
                Ethertype::Ipv4 => create::Ethertype::Ipv4,
                Ethertype::Ipv6 => create::Ethertype::Ipv6,
            };
            default_security_group_rule_builder.ethertype(tmp);
        }

        if let Some(val) = &args.remote_ip_prefix {
            default_security_group_rule_builder.remote_ip_prefix(val);
        }

        if let Some(val) = &args.used_in_default_sg {
            default_security_group_rule_builder.used_in_default_sg(*val);
        }

        if let Some(val) = &args.used_in_non_default_sg {
            default_security_group_rule_builder.used_in_non_default_sg(*val);
        }

        ep_builder
            .default_security_group_rule(default_security_group_rule_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
