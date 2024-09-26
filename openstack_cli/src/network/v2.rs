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
mod extension;
mod floatingip;
mod network;
mod port;
mod rbac_policy;
mod router;
mod security_group;
mod security_group_rule;
mod subnet;
mod subnetpool;

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
    Extension(Box<extension::ExtensionCommand>),
    FloatingIP(Box<floatingip::FloatingIPCommand>),
    Network(Box<network::NetworkCommand>),
    Port(Box<port::PortCommand>),
    RbacPolicy(Box<rbac_policy::RbacPolicyCommand>),
    Router(Box<router::RouterCommand>),
    SecurityGroup(Box<security_group::SecurityGroupCommand>),
    SecurityGroupRule(Box<security_group_rule::SecurityGroupRuleCommand>),
    Subnet(Box<subnet::SubnetCommand>),
    #[command(visible_alias = "subnet-pool")]
    Subnetpool(Box<subnetpool::SubnetPoolCommand>),
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
            NetworkCommands::Extension(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::FloatingIP(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Network(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Port(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::RbacPolicy(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Router(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::SecurityGroup(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::SecurityGroupRule(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Subnet(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Subnetpool(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
