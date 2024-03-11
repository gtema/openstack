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

//! Action Image command [microversion = 2.81]
//!
//! Wraps invoking of the `v2.1/os-aggregates/{id}/images` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::aggregate::image::cache_281;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Requests that a set of images be pre-cached on compute nodes within the
/// referenced aggregate.
///
/// This API is available starting with microversion 2.81.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404)
///
#[derive(Args)]
#[command(about = "Request Image Pre-caching for Aggregate (microversion = 2.81)")]
pub struct ImageCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A list of image objects to cache.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    cache: Vec<String>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/os-aggregates/{id}/images API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Image response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ImageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Image");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = cache_281::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.81");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.cache data
        let args = &self.cache;

        let cache_builder: Vec<cache_281::Cache> = args
            .iter()
            .flat_map(|v| cache_281::CacheBuilder::default().id(v).build())
            .collect();
        ep_builder.cache(cache_builder);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
