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

//! Create Backup command [microversion = 3.0]
//!
//! Wraps invoking of the `v3/backups` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::block_storage::v3::backup::create_30;
use openstack_types::block_storage::v3::backup::response::create::BackupResponse;

/// Create a new backup.
#[derive(Args)]
pub struct BackupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `backup` object.
    #[command(flatten)]
    backup: Backup,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Backup Body data
#[derive(Args, Clone)]
struct Backup {
    /// The container name or null.
    #[arg(help_heading = "Body parameters", long)]
    container: Option<String>,

    /// The backup description or null.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Indicates whether to backup, even if the volume is attached. Default is
    /// `false`. See [valid boolean values](#valid-boolean-values)
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    force: Option<bool>,

    /// Indicates whether to backup, even if the volume is attached. Default is
    /// `false`. See [valid boolean values](#valid-boolean-values)
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    incremental: Option<bool>,

    /// The name of the Volume Backup.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The UUID of the source snapshot that you want to back up.
    #[arg(help_heading = "Body parameters", long)]
    snapshot_id: Option<String>,

    /// The UUID of the volume that you want to back up.
    #[arg(help_heading = "Body parameters", long)]
    volume_id: String,
}

impl BackupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Backup");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_30::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.0");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.backup data
        let args = &self.backup;
        let mut backup_builder = create_30::BackupBuilder::default();

        backup_builder.volume_id(&args.volume_id);

        if let Some(val) = &args.container {
            backup_builder.container(Some(val.into()));
        }

        if let Some(val) = &args.description {
            backup_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.incremental {
            backup_builder.incremental(*val);
        }

        if let Some(val) = &args.force {
            backup_builder.force(*val);
        }

        if let Some(val) = &args.name {
            backup_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.snapshot_id {
            backup_builder.snapshot_id(Some(val.into()));
        }

        ep_builder.backup(backup_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<BackupResponse>(data)?;
        Ok(())
    }
}
