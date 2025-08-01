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

//! Set Tag command
//!
//! Wraps invoking of the `v2/metadefs/namespaces/{namespace_name}/tags/{tag_name}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::image::v2::metadef::namespace::tag::set;
use openstack_types::image::v2::metadef::namespace::tag::response::set::TagResponse;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct TagCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long)]
    name: String,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// namespace_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_namespace_name",
        value_name = "NAMESPACE_NAME"
    )]
    namespace_name: String,

    /// tag_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_tag_name",
        value_name = "TAG_NAME"
    )]
    tag_name: String,
}

impl TagCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Tag");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("image.metadef/namespace/tag"),
            Some("set"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        ep_builder.namespace_name(&self.path.namespace_name);
        ep_builder.tag_name(&self.path.tag_name);

        // Set body parameters
        // Set Request.name data
        ep_builder.name(&self.name);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<TagResponse>(data)?;
        Ok(())
    }
}
