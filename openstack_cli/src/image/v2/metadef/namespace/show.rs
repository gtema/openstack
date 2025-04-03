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

//! Show Namespace command
//!
//! Wraps invoking of the `v2/metadefs/namespaces/{namespace_name}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::image::v2::metadef::namespace::get;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct NamespaceCommand {
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
    /// namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_namespace_name",
        value_name = "NAMESPACE_NAME"
    )]
    namespace_name: String,
}
/// Namespace response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Date and time of namespace creation
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Provides a user friendly description of the namespace.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The user friendly name for the namespace. Used by UI if available.
    ///
    #[serde()]
    #[structable(optional)]
    display_name: Option<String>,

    /// The unique namespace text.
    ///
    #[serde()]
    #[structable()]
    namespace: String,

    #[serde()]
    #[structable(optional, pretty)]
    objects: Option<Value>,

    /// Owner of the namespace.
    ///
    #[serde()]
    #[structable(optional)]
    owner: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    properties: Option<Value>,

    /// If true, namespace will not be deletable.
    ///
    #[serde()]
    #[structable(optional)]
    protected: Option<bool>,

    #[serde()]
    #[structable(optional, pretty)]
    resource_type_associations: Option<Value>,

    #[serde()]
    #[structable(optional)]
    schema: Option<String>,

    #[serde(rename = "self")]
    #[structable(optional, title = "self")]
    _self: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    /// Date and time of the last namespace modification
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// Scope of namespace accessibility.
    ///
    #[serde()]
    #[structable(optional)]
    visibility: Option<String>,
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseProperties {
    additional_items: Option<bool>,
    _default: Option<Value>,
    description: Option<String>,
    _enum: Option<Value>,
    items: Option<Value>,
    maximum: Option<f32>,
    max_items: Option<i32>,
    max_length: Option<i32>,
    minimum: Option<f32>,
    min_items: Option<i32>,
    min_length: Option<i32>,
    name: Option<String>,
    operators: Option<Value>,
    pattern: Option<String>,
    readonly: Option<bool>,
    required: Option<Value>,
    title: String,
    _type: String,
    unique_items: Option<bool>,
}

impl fmt::Display for ResponseProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "additional_items={}",
                self.additional_items
                    .map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "_default={}",
                self._default
                    .clone()
                    .map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "description={}",
                self.description
                    .clone()
                    .map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "_enum={}",
                self._enum.clone().map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "items={}",
                self.items.clone().map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "maximum={}",
                self.maximum.map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "max_items={}",
                self.max_items.map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "max_length={}",
                self.max_length.map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "minimum={}",
                self.minimum.map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "min_items={}",
                self.min_items.map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "min_length={}",
                self.min_length.map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "name={}",
                self.name.clone().map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "operators={}",
                self.operators
                    .clone()
                    .map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "pattern={}",
                self.pattern
                    .clone()
                    .map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "readonly={}",
                self.readonly.map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "required={}",
                self.required
                    .clone()
                    .map_or(String::new(), |v| v.to_string())
            ),
            format!("title={}", self.title),
            format!("_type={}", self._type),
            format!(
                "unique_items={}",
                self.unique_items.map_or(String::new(), |v| v.to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl NamespaceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Namespace");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.namespace_name(&self.path.namespace_name);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
