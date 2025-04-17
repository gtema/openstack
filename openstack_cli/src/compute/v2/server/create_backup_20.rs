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

//! Action Server command [microversion = 2.0]
//!
//! Wraps invoking of the `v2.1/servers/{id}/action` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::create_backup_20;
use openstack_types::compute::v2::server::response::create_backup::ServerResponse;

/// Command without description in OpenAPI
#[derive(Args)]
#[command(about = "Create Server Back Up (createBackup Action) (microversion = 2.0)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action.
    #[command(flatten)]
    create_backup: CreateBackup,
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
/// CreateBackup Body data
#[derive(Args, Clone)]
struct CreateBackup {
    /// The type of the backup, for example, `daily`.
    #[arg(help_heading = "Body parameters", long)]
    backup_type: String,

    /// Metadata key and value pairs. The maximum size of the metadata key and
    /// value is 255 bytes each.
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    /// The name of the image to be backed up.
    #[arg(help_heading = "Body parameters", long)]
    name: String,

    /// The rotation of the back up image, the oldest image will be removed
    /// when image count exceed the rotation count.
    #[arg(help_heading = "Body parameters", long)]
    rotation: i32,
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

        let mut ep_builder = create_backup_20::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.0");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.create_backup data
        let args = &self.create_backup;
        let mut create_backup_builder = create_backup_20::CreateBackupBuilder::default();

        create_backup_builder.name(&args.name);

        create_backup_builder.backup_type(&args.backup_type);

        create_backup_builder.rotation(args.rotation);

        if let Some(val) = &args.metadata {
            create_backup_builder.metadata(val.iter().cloned());
        }

        ep_builder.create_backup(create_backup_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ServerResponse>(data)?;
        Ok(())
    }
}
