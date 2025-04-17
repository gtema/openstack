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

//! Create Object command
//!
//! Wraps invoking of the `v2/metadefs/namespaces/{namespace_name}/objects` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::image::v2::metadef::namespace::object::create;
use openstack_types::image::v2::metadef::namespace::object::response::create::ObjectResponse;
use serde_json::Value;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct ObjectCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    name: String,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    properties: Option<Vec<(String, Value)>>,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    required: Option<Vec<String>>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// namespace_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_namespace_name",
        value_name = "NAMESPACE_NAME"
    )]
    namespace_name: String,
}

impl ObjectCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Object");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.namespace_name(&self.path.namespace_name);
        // Set query parameters
        // Set body parameters
        // Set Request.description data
        if let Some(arg) = &self.description {
            ep_builder.description(arg);
        }

        // Set Request.name data
        ep_builder.name(&self.name);

        // Set Request.properties data
        if let Some(arg) = &self.properties {
            ep_builder.properties(
                arg.iter()
                    .map(|(k, v)| {
                        serde_json::from_value(v.to_owned()).map(|v: create::Properties| (k, v))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter(),
            );
        }

        // Set Request.required data
        if let Some(arg) = &self.required {
            ep_builder.required(arg.iter().map(Into::into).collect::<Vec<_>>());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ObjectResponse>(data)?;
        Ok(())
    }
}
