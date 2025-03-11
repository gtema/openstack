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

//! Container Infrastructure Management API v1 command

use clap::{Parser, Subcommand};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::{Cli, OpenStackCliError};

pub mod certificate;
pub mod cluster;
pub mod clustertemplate;
pub mod federation;
pub mod mservice;
pub mod quota;
pub mod stat;
pub mod version;

/// Container Infra service (Magnum) operations
#[derive(Parser)]
pub struct ContainerInfrastructureCommand {
    /// Container Infra service resource
    #[command(subcommand)]
    command: ContainerInfrastructureCommands,
}

/// Container infrastructure management resources commands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ContainerInfrastructureCommands {
    Certificate(Box<certificate::CertificateCommand>),
    Cluster(Box<cluster::ClusterCommand>),
    Clustertemplate(Box<clustertemplate::ClustertemplateCommand>),
    Federation(Box<federation::FederationCommand>),
    #[command(alias = "mservice")]
    Service(Box<mservice::ServiceCommand>),
    Quota(Box<quota::QuotaCommand>),
    Stat(Box<stat::StatCommand>),
    Version(Box<version::VersionCommand>),
}

impl ContainerInfrastructureCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        session
            .discover_service_endpoint(&ServiceType::ContainerInfrastructureManagement)
            .await?;

        match &self.command {
            ContainerInfrastructureCommands::Certificate(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ContainerInfrastructureCommands::Cluster(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ContainerInfrastructureCommands::Clustertemplate(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ContainerInfrastructureCommands::Federation(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ContainerInfrastructureCommands::Service(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ContainerInfrastructureCommands::Quota(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ContainerInfrastructureCommands::Stat(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ContainerInfrastructureCommands::Version(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
        }
    }
}
