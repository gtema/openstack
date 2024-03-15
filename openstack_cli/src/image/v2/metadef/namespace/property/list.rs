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

//! List_from_struct Property command
//!
//! Wraps invoking of the `v2/metadefs/namespaces/{namespace_name}/properties` with `GET` method

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

use openstack_sdk::api::image::v2::metadef::namespace::property::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Lists property definitions in a namespace.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 403, 404
///
#[derive(Args)]
#[command(about = "List properties")]
pub struct PropertyCommand {
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
struct PathParameters {
    /// namespace_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_namespace_name",
        value_name = "NAMESPACE_NAME"
    )]
    namespace_name: String,
}
/// Property response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde(rename = "additionalItems")]
    #[structable(optional, title = "additionalItems", wide)]
    additional_items: Option<bool>,

    #[serde(rename = "default")]
    #[structable(optional, pretty, title = "default", wide)]
    _default: Option<Value>,

    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    #[serde(rename = "enum")]
    #[structable(optional, pretty, title = "enum", wide)]
    _enum: Option<Value>,

    #[serde()]
    #[structable(optional, pretty)]
    items: Option<Value>,

    #[serde()]
    #[structable(optional, wide)]
    maximum: Option<f32>,

    #[serde(rename = "maxItems")]
    #[structable(optional, title = "maxItems", wide)]
    max_items: Option<i32>,

    #[serde(rename = "maxLength")]
    #[structable(optional, title = "maxLength", wide)]
    max_length: Option<i32>,

    #[serde()]
    #[structable(optional, wide)]
    minimum: Option<f32>,

    #[serde(rename = "minItems")]
    #[structable(optional, title = "minItems", wide)]
    min_items: Option<i32>,

    #[serde(rename = "minLength")]
    #[structable(optional, title = "minLength", wide)]
    min_length: Option<i32>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional, pretty, wide)]
    operators: Option<Value>,

    #[serde()]
    #[structable(optional)]
    pattern: Option<String>,

    #[serde()]
    #[structable(optional)]
    readonly: Option<bool>,

    #[serde()]
    #[structable(optional, pretty, wide)]
    required: Option<Value>,

    #[serde()]
    #[structable()]
    title: String,

    #[serde(rename = "type")]
    #[structable(title = "type", wide)]
    _type: String,

    #[serde(rename = "uniqueItems")]
    #[structable(optional, title = "uniqueItems", wide)]
    unique_items: Option<bool>,
}

impl PropertyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List_from_struct Property");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.namespace_name(&self.path.namespace_name);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: serde_json::Value = ep.query_async(client).await?;
        let split: Vec<Value> = data
            .as_object()
            .expect("API response is not an object")
            .iter()
            .map(|(k, v)| {
                let mut new = v.clone();
                new.as_object_mut()
                    .expect("Object item is an object")
                    .entry("name".to_string())
                    .or_insert(serde_json::json!(k));
                new
            })
            .collect();

        op.output_list::<ResponseData>(split)?;
        Ok(())
    }
}