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

//! List VolumeAttachments command
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/os-volume_attachments` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::volume_attachment::list;
use openstack_sdk::api::{Pagination, paged};
use openstack_types::compute::v2::server::volume_attachment::response::list::VolumeAttachmentResponse;

/// List volume attachments for an instance.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args)]
#[command(about = "List volume attachments for an instance")]
pub struct VolumeAttachmentsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[arg(
        help_heading = "Query parameters",
        long("page-size"),
        visible_alias("limit")
    )]
    limit: Option<i32>,

    #[arg(help_heading = "Query parameters", long)]
    offset: Option<i32>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// server_id parameter for
    /// /v2.1/servers/{server_id}/os-volume_attachments/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_server_id",
        value_name = "SERVER_ID"
    )]
    server_id: String,
}

impl VolumeAttachmentsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List VolumeAttachments");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("compute.server/volume_attachment"),
            Some("list"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.offset {
            ep_builder.offset(*val);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;
        op.output_list::<VolumeAttachmentResponse>(data)?;
        Ok(())
    }
}
