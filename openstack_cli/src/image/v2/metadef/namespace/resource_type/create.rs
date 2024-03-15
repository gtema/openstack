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

//! Create ResourceType command
//!
//! Wraps invoking of the `v2/metadefs/namespaces/{namespace_name}/resource_types` with `POST` method

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

use openstack_sdk::api::image::v2::metadef::namespace::resource_type::create;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Creates a resource type association between a namespace and the resource
/// type specified in the body of the request.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 403, 404, 409
///
#[derive(Args)]
#[command(about = "Create resource type association")]
pub struct ResourceTypeCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Resource type names should be aligned with Heat resource types whenever
    /// possible:
    /// https://docs.openstack.org/heat/latest/template_guide/openstack.html
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: String,

    /// Prefix for any properties in the namespace that you want to apply to
    /// the resource type. If you specify a prefix, you must append a prefix
    /// separator, such as the colon (`:`) character.
    ///
    #[arg(help_heading = "Body parameters", long)]
    prefix: Option<String>,

    /// Some resource types allow more than one key and value pair for each
    /// instance. For example, the Image service allows both user and image
    /// metadata on volumes. The `properties_target` parameter enables a
    /// namespace target to remove the ambiguity.
    ///
    #[arg(help_heading = "Body parameters", long)]
    properties_target: Option<String>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// namespace_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/resource_types/{resource_type}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_namespace_name",
        value_name = "NAMESPACE_NAME"
    )]
    namespace_name: String,
}
/// ResourceType response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The date and time when the resource was created.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Resource type names should be aligned with Heat resource types whenever
    /// possible:
    /// https://docs.openstack.org/heat/latest/template_guide/openstack.html
    ///
    #[serde()]
    #[structable()]
    name: String,

    /// Prefix for any properties in the namespace that you want to apply to
    /// the resource type. If you specify a prefix, you must append a prefix
    /// separator, such as the colon (`:`) character.
    ///
    #[serde()]
    #[structable(optional)]
    prefix: Option<String>,

    /// Some resource types allow more than one key and value pair for each
    /// instance. For example, the Image service allows both user and image
    /// metadata on volumes. The `properties_target` parameter enables a
    /// namespace target to remove the ambiguity.
    ///
    #[serde()]
    #[structable(optional)]
    properties_target: Option<String>,

    /// The date and time when the resource was last updated.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl ResourceTypeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ResourceType");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.namespace_name(&self.path.namespace_name);
        // Set query parameters
        // Set body parameters
        // Set Request.name data
        ep_builder.name(&self.name);

        // Set Request.prefix data
        if let Some(arg) = &self.prefix {
            ep_builder.prefix(arg);
        }

        // Set Request.properties_target data
        if let Some(arg) = &self.properties_target {
            ep_builder.properties_target(arg);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}