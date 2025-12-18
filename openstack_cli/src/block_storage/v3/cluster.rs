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

//! Block storage Cluster commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod list;
pub mod show;

/// Clusters (clusters)
///
/// Administrator only. Lists all Cinder clusters, show cluster detail, enable or disable a
/// cluster.
///
/// Each cinder service runs on a host computer (possibly multiple services on the same host; it
/// depends how you decide to deploy cinder). In order to support High Availability scenarios,
/// services can be grouped into clusters where the same type of service (for example,
/// cinder-volume) can run on different hosts so that if one host goes down the service is still
/// available on a different host. Since there’s no point having these services sitting around
/// doing nothing while waiting for some other host to go down (which is also known as
/// Active/Passive mode), grouping services into clusters also allows cinder to support
/// Active/Active mode in which all services in a cluster are doing work all the time.
///
///
/// **Note**: Currently the only service that can be grouped into clusters is cinder-volume.
///
/// Clusters are determined by the deployment configuration; that’s why there is no
/// ‘create-cluster’ API call listed below. Once your services are up and running, however, you can
/// use the following API requests to get information about your clusters and to update their
/// status.
#[derive(Parser)]
pub struct ClusterCommand {
    /// subcommand
    #[command(subcommand)]
    command: ClusterCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ClusterCommands {
    List(Box<list::ClustersCommand>),
    Show(Box<show::ClusterCommand>),
}

impl ClusterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ClusterCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ClusterCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
