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

//! Load Balancer v2 commands

use clap::{Parser, Subcommand};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::{Cli, OpenStackCliError};

mod amphorae;
mod availability_zone;
mod availability_zone_profile;
mod flavor;
mod flavor_profile;
mod healthmonitor;
mod l7policy;
mod listener;
mod loadbalancer;
mod pool;
mod provider;
mod quota;
mod version;

/// Load Balancer service operations
#[derive(Parser)]
pub struct LoadBalancerCommand {
    /// subcommand
    #[command(subcommand)]
    command: LoadBalancerCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum LoadBalancerCommands {
    Amphorae(amphorae::AmphoraeCommand),
    AvailabilityZone(availability_zone::AvailabilityZoneCommand),
    AvailabilityZoneProfile(availability_zone_profile::AvailabilityZoneProfileCommand),
    Flavor(flavor::FlavorCommand),
    FlavorProfile(flavor_profile::FlavorProfileCommand),
    Healthmonitor(healthmonitor::HealthmonitorCommand),
    L7policy(l7policy::L7PolicyCommand),
    Listener(listener::ListenerCommand),
    Loadbalancer(loadbalancer::LoadbalancerCommand),
    Pool(pool::PoolCommand),
    Provider(provider::ProviderCommand),
    Quota(quota::QuotaCommand),
    Version(version::VersionCommand),
}

impl LoadBalancerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        session
            .discover_service_endpoint(&ServiceType::LoadBalancer)
            .await?;

        match &self.command {
            LoadBalancerCommands::Amphorae(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::AvailabilityZone(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            LoadBalancerCommands::AvailabilityZoneProfile(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            LoadBalancerCommands::Flavor(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::FlavorProfile(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::Healthmonitor(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::L7policy(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::Listener(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::Loadbalancer(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::Pool(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::Provider(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::Quota(cmd) => cmd.take_action(parsed_args, session).await,
            LoadBalancerCommands::Version(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
