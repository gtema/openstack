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

//! Compute API v2 command

use clap::{Parser, Subcommand};

use openstack_sdk::{AsyncOpenStack, types::ServiceType};

use crate::{Cli, OpenStackCliError};

pub mod aggregate;
pub mod assisted_volume_snapshot;
pub mod availability_zone;
pub mod extension;
pub mod flavor;
pub mod hypervisor;
pub mod instance_usage_audit_log;
pub mod keypair;
pub mod limit;
pub mod migration;
pub mod quota_class_set;
pub mod quota_set;
pub mod server;
pub mod server_external_event;
pub mod server_group;
pub mod service;
pub mod simple_tenant_usage;

/// Compute service (Nova) operations
#[derive(Parser)]
pub struct ComputeCommand {
    /// Compute service resource
    #[command(subcommand)]
    command: ComputeCommands,
}

/// Compute resources commands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ComputeCommands {
    #[command(about = "Host Aggregates")]
    Aggregate(Box<aggregate::AggregateCommand>),
    AssistedVolumeSnapshot(Box<assisted_volume_snapshot::AssistedVolumeSnapshotCommand>),
    AvailabilityZone(Box<availability_zone::AvailabilityZoneCommand>),
    Extension(Box<extension::ExtensionCommand>),
    Flavor(Box<flavor::FlavorCommand>),
    Hypervisor(Box<hypervisor::HypervisorCommand>),
    InstanceUsageAuditLog(Box<instance_usage_audit_log::InstanceUsageAuditLogCommand>),
    Keypair(Box<keypair::KeypairCommand>),
    Limit(Box<limit::LimitCommand>),
    Migration(Box<migration::MigrationCommand>),
    QuotaClassSet(Box<quota_class_set::QuotaClassSetCommand>),
    QuotaSet(Box<quota_set::QuotaSetCommand>),
    Server(Box<server::ServerCommand>),
    ServerExternalEvent(Box<server_external_event::ServerExternalEventCommand>),
    ServerGroup(Box<server_group::ServerGroupCommand>),
    Service(Box<service::ServiceCommand>),
    #[command(visible_alias = "usage")]
    SimpleTenantUsage(Box<simple_tenant_usage::SimpleTenantUsageCommand>),
}

impl ComputeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        session
            .discover_service_endpoint(&ServiceType::Compute)
            .await?;

        match &self.command {
            ComputeCommands::Aggregate(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::AssistedVolumeSnapshot(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ComputeCommands::AvailabilityZone(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Extension(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Hypervisor(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::InstanceUsageAuditLog(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ComputeCommands::Flavor(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Keypair(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Limit(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Migration(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::QuotaClassSet(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::QuotaSet(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Server(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::ServerExternalEvent(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ComputeCommands::ServerGroup(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Service(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::SimpleTenantUsage(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
