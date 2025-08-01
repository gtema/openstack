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

//! Action Server command
//!
//! Wraps invoking of the `v2.1/servers/{id}/action` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::resize;

/// Resizes a server.
///
/// Specify the `resize` action in the request body.
///
/// **Preconditions**
///
/// You can only resize a server when its status is `ACTIVE` or `SHUTOFF`.
///
/// If the server is locked, you must have administrator privileges to resize
/// the server.
///
/// **Asynchronous Postconditions**
///
/// A successfully resized server shows a `VERIFY_RESIZE` status and `finished`
/// migration status. If the cloud has configured the
/// [resize_confirm_window](https://docs.openstack.org/nova/latest/configuration/config.html#DEFAULT.resize_confirm_window)
/// option of the Compute service to a positive value, the Compute service
/// automatically confirms the resize operation after the configured interval.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args)]
#[command(about = "Resize Server (resize Action)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action to resize a server.
    #[command(flatten)]
    resize: Resize,
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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum OsDcfDiskConfig {
    Auto,
    Manual,
}

/// Resize Body data
#[derive(Args, Clone)]
struct Resize {
    /// The flavor ID for resizing the server. The size of the disk in the
    /// flavor being resized to must be greater than or equal to the size of
    /// the disk in the current flavor.
    ///
    /// If a specified flavor ID is the same as the current one of the server,
    /// the request returns a `Bad Request (400)` response code.
    #[arg(help_heading = "Body parameters", long)]
    flavor_ref: String,

    /// Controls how the API partitions the disk when you create, rebuild, or
    /// resize servers. A server inherits the `OS-DCF:diskConfig` value from
    /// the image from which it was created, and an image inherits the
    /// `OS-DCF:diskConfig` value from the server from which it was created. To
    /// override the inherited setting, you can include this attribute in the
    /// request body of a server create, rebuild, or resize request. If the
    /// `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a
    /// server from that image and set its `OS-DCF:diskConfig` value to `AUTO`.
    /// A valid value is:
    ///
    /// - `AUTO`. The API builds the server with a single partition the size of
    ///   the target flavor disk. The API automatically adjusts the file system
    ///   to fit the entire partition.
    /// - `MANUAL`. The API builds the server by using whatever partition
    ///   scheme and file system is in the source image. If the target flavor
    ///   disk is larger, the API does not partition the remaining disk space.
    #[arg(help_heading = "Body parameters", long)]
    os_dcf_disk_config: Option<OsDcfDiskConfig>,
}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server");

        let op = OutputProcessor::from_args(parsed_args, Some("compute.server"), Some("resize"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = resize::Request::builder();

        ep_builder.id(&self.path.id);

        // Set body parameters
        // Set Request.resize data
        let args = &self.resize;
        let mut resize_builder = resize::ResizeBuilder::default();
        if let Some(val) = &args.os_dcf_disk_config {
            let tmp = match val {
                OsDcfDiskConfig::Auto => resize::OsDcfDiskConfig::Auto,
                OsDcfDiskConfig::Manual => resize::OsDcfDiskConfig::Manual,
            };
            resize_builder.os_dcf_disk_config(tmp);
        }

        resize_builder.flavor_ref(&args.flavor_ref);

        ep_builder.resize(resize_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
