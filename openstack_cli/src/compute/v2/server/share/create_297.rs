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

//! Create Share command [microversion = 2.97]
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/shares` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::server::share::create_297;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Attach a share to an instance.
///
/// Normal response codes: 201
///
/// Error response codes: badRequest(400), forbidden(403), itemNotFound(404),
/// conflict(409)
///
#[derive(Args)]
#[command(about = "Attach a share to an instance (microversion = 2.97)")]
pub struct ShareCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    share: Share,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/shares/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_server_id",
        value_name = "SERVER_ID"
    )]
    server_id: String,
}
/// Share Body data
#[derive(Args, Clone)]
struct Share {
    /// The UUID of the attached share.
    ///
    #[arg(help_heading = "Body parameters", long)]
    share_id: String,

    /// The device tag to be used by users to mount the share within the
    /// instance, if not provided then the share UUID will be used
    /// automatically.
    ///
    #[arg(help_heading = "Body parameters", long)]
    tag: Option<String>,
}

/// Share response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The export location used to attach the share to the underlying host.
    ///
    #[serde()]
    #[structable(optional)]
    export_location: Option<String>,

    /// The UUID of the attached share.
    ///
    #[serde()]
    #[structable()]
    share_id: String,

    /// Status of the Share:
    ///
    /// - attaching: The share is being attached to the VM by the compute node.
    /// - detaching: The share is being detached from the VM by the compute
    ///   node.
    /// - inactive: The share is attached but inactive because the VM is
    ///   stopped.
    /// - active: The share is attached, and the VM is running.
    /// - error: The share is in an error state.
    ///
    #[serde()]
    #[structable()]
    status: String,

    /// The device tag to be used by users to mount the share within the
    /// instance, if not provided then the share UUID will be used
    /// automatically.
    ///
    #[serde()]
    #[structable()]
    tag: String,

    /// The UUID of the attached share.
    ///
    #[serde()]
    #[structable(optional)]
    uuid: Option<String>,
}

impl ShareCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Share");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_297::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.97");

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        // Set body parameters
        // Set Request.share data
        let args = &self.share;
        let mut share_builder = create_297::ShareBuilder::default();

        share_builder.share_id(&args.share_id);

        if let Some(val) = &args.tag {
            share_builder.tag(val);
        }

        ep_builder.share(share_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}