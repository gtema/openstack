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

//! List EndpointGroups command
//!
//! Wraps invoking of the `v3/OS-EP-FILTER/endpoint_groups` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::os_ep_filter::endpoint_group::list;
use openstack_types::identity::v3::os_ep_filter::endpoint_group::response::list::EndpointGroupResponse;

/// List all endpoint groups.
///
/// GET /v3/OS-EP-FILTER/endpoint_groups
#[derive(Args)]
pub struct EndpointGroupsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// The name of the endpoint group.
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

impl EndpointGroupsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List EndpointGroups");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("identity.OS_EP_FILTER/endpoint_group"),
            Some("list"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set query parameters
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<EndpointGroupResponse>(data)?;
        Ok(())
    }
}
