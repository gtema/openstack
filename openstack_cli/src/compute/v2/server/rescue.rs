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

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::rescue;
use openstack_types::compute::v2::server::response::rescue::ServerResponse;

/// Puts a server in rescue mode and changes its status to `RESCUE`.
///
/// Specify the `rescue` action in the request body.
///
/// If you specify the `rescue_image_ref` extended attribute, the image is used
/// to rescue the instance. If you omit an image reference, the base image
/// reference is used by default.
///
/// **Asynchronous Postconditions**
///
/// After you successfully rescue a server and make a
/// `GET /servers/​{server_id}​` request, its status changes to `RESCUE`.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409), notImplemented(501)
#[derive(Args)]
#[command(about = "Rescue Server (rescue Action)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action to rescue a server.
    #[command(flatten)]
    rescue: Option<Rescue>,
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
/// Rescue Body data
#[derive(Args, Clone)]
struct Rescue {
    #[arg(help_heading = "Body parameters", long)]
    admin_pass: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    rescue_image_ref: Option<String>,
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

        let mut ep_builder = rescue::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.rescue data
        if let Some(lrescue) = &self.rescue {
            let mut rescue_builder = rescue::RescueBuilder::default();
            if let Some(val) = &lrescue.admin_pass {
                rescue_builder.admin_pass(val);
            }
            if let Some(val) = &lrescue.rescue_image_ref {
                rescue_builder.rescue_image_ref(val);
            }
            ep_builder.rescue(rescue_builder.build().expect("A valid object"));
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ServerResponse>(data)?;
        Ok(())
    }
}
