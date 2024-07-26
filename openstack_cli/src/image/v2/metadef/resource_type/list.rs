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

//! List ResourceTypes command
//!
//! Wraps invoking of the `v2/metadefs/resource_types` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::image::v2::metadef::resource_type::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists all available resource types.
///
/// Using the other API calls in this section, you can create and maintain
/// *resource type associations* between metadata definition namespaces and the
/// resource types that are returned by this call.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 404
///
#[derive(Args)]
#[command(about = "List resource types")]
pub struct ResourceTypesCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// ResourceTypes response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Resource type creation date
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Resource type name
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// Resource type update date
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl ResourceTypesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List ResourceTypes");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
