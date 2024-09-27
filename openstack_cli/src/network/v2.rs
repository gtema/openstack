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

//! Networking v2 commands

use clap::{Parser, Subcommand};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::{Cli, OpenStackCliError};

mod address_group;
mod address_scope;
mod agent;
mod auto_allocated_topology;
mod availability_zone;
mod default_security_group_rule;
mod extension;
mod flavor;
mod floatingip;
mod floatingip_pool;
mod local_ip;
mod log;
mod metering;
mod ndp_proxy;
mod network;
mod network_ip_availability;
mod network_segment_range;
mod port;
mod qos;
mod quota;
mod rbac_policy;
mod router;
mod security_group;
mod security_group_rule;
mod segment;
mod subnet;
mod subnetpool;
mod vpn;

/// Network (Neutron) commands
#[derive(Parser)]
pub struct NetworkCommand {
    /// Network service resource
    #[command(subcommand)]
    command: NetworkCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum NetworkCommands {
    AddressGroup(Box<address_group::AddressGroupCommand>),
    AddressScope(Box<address_scope::AddressScopeCommand>),
    Agent(Box<agent::AgentCommand>),
    AutoAllocatedTopology(Box<auto_allocated_topology::AutoAllocatedTopologyCommand>),
    AvailabilityZone(Box<availability_zone::AvailabilityZoneCommand>),
    DefaultSecurityGroupRule(Box<default_security_group_rule::DefaultSecurityGroupRuleCommand>),
    Extension(Box<extension::ExtensionCommand>),
    Flavor(Box<flavor::FlavorCommand>),
    FloatingIP(Box<floatingip::FloatingIPCommand>),
    FloatingIPPool(Box<floatingip_pool::FloatingIPPoolCommand>),
    LocalIP(Box<local_ip::LocalIPCommand>),
    Log(Box<log::LogCommand>),
    Metering(Box<metering::MeteringCommand>),
    NdpProxy(Box<ndp_proxy::NdpProxyCommand>),
    Network(Box<network::NetworkCommand>),
    NetworkIpAvailability(Box<network_ip_availability::NetworkIpAvailabilityCommand>),
    NetworkSegmentRange(Box<network_segment_range::NetworkSegmentRangeCommand>),
    Port(Box<port::PortCommand>),
    Qos(Box<qos::QosCommand>),
    Quota(Box<quota::QuotaCommand>),
    RbacPolicy(Box<rbac_policy::RbacPolicyCommand>),
    Router(Box<router::RouterCommand>),
    SecurityGroup(Box<security_group::SecurityGroupCommand>),
    SecurityGroupRule(Box<security_group_rule::SecurityGroupRuleCommand>),
    Segment(Box<segment::SegmentCommand>),
    Subnet(Box<subnet::SubnetCommand>),
    #[command(visible_alias = "subnet-pool")]
    Subnetpool(Box<subnetpool::SubnetPoolCommand>),
    Vpn(Box<vpn::VpnCommand>),
}

impl NetworkCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        session
            .discover_service_endpoint(&ServiceType::Network)
            .await?;

        match &self.command {
            NetworkCommands::AddressGroup(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::AddressScope(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Agent(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::AutoAllocatedTopology(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            NetworkCommands::AvailabilityZone(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::DefaultSecurityGroupRule(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            NetworkCommands::Extension(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Flavor(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::FloatingIP(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::FloatingIPPool(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::LocalIP(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Log(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Metering(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::NdpProxy(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Network(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::NetworkIpAvailability(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            NetworkCommands::NetworkSegmentRange(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            NetworkCommands::Port(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Qos(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Quota(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::RbacPolicy(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Router(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::SecurityGroup(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::SecurityGroupRule(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Segment(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Subnet(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Subnetpool(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Vpn(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
