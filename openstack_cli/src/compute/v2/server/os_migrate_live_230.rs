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

//! Action Server command [microversion = 2.30]
//!
//! Wraps invoking of the `v2.1/servers/{id}/action` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::os_migrate_live_230;

/// Command without description in OpenAPI
#[derive(Args)]
#[command(about = "Live-Migrate Server (os-migrateLive Action) (microversion = 2.30)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action.
    #[command(flatten)]
    os_migrate_live: OsMigrateLive,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// OsMigrateLive Body data
#[derive(Args, Clone)]
struct OsMigrateLive {
    /// Migrates local disks by using block migration. Set to `auto` which
    /// means nova will detect whether source and destination hosts on shared
    /// storage. if they are on shared storage, the live-migration won’t be
    /// block migration. Otherwise the block migration will be executed. Set to
    /// `True`, means the request will fail when the source or destination host
    /// uses shared storage. Set to `False` means the request will fail when
    /// the source and destination hosts are not on the shared storage.
    ///
    /// **New in version 2.25**
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    block_migration: bool,

    /// Force a live-migration by not verifying the provided destination host
    /// by the scheduler.
    ///
    /// Warning
    ///
    /// This could result in failures to actually live migrate the instance to
    /// the specified host. It is recommended to either not specify a host so
    /// that the scheduler will pick one, or specify a host without
    /// `force=True` set.
    ///
    /// **New in version 2.30**
    ///
    /// **Available until version 2.67**
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    force: Option<bool>,

    /// The host to which to migrate the server. If this parameter is `None`,
    /// the scheduler chooses a host.
    ///
    /// Warning
    ///
    /// Prior to microversion 2.30, specifying a host will bypass validation by
    /// the scheduler, which could result in failures to actually migrate the
    /// instance to the specified host, or over-subscription of the host. It is
    /// recommended to either not specify a host so that the scheduler will
    /// pick one, or specify a host with microversion >= 2.30 and without
    /// `force=True` set.
    #[arg(help_heading = "Body parameters", long)]
    host: String,
}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = os_migrate_live_230::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.30");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_migrate_live data
        let args = &self.os_migrate_live;
        let mut os_migrate_live_builder = os_migrate_live_230::OsMigrateLiveBuilder::default();

        os_migrate_live_builder.block_migration(args.block_migration);

        os_migrate_live_builder.host(args.host.clone());

        if let Some(val) = &args.force {
            os_migrate_live_builder.force(*val);
        }

        ep_builder.os_migrate_live(os_migrate_live_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
