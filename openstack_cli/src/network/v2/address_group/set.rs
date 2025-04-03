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

//! Set AddressGroup command
//!
//! Wraps invoking of the `v2.0/address-groups/{id}` with `PUT` method

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
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::address_group::find;
use openstack_sdk::api::network::v2::address_group::set;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates an address group.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 403, 404, 412
///
#[derive(Args)]
#[command(about = "Update an address group")]
pub struct AddressGroupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// An `address group` object.
    ///
    #[command(flatten)]
    address_group: AddressGroup,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/address-groups/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// AddressGroup Body data
#[derive(Args, Clone)]
struct AddressGroup {
    /// A human-readable description for the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// AddressGroup response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A list of IP addresses.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    addresses: Option<Value>,

    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the address group.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl AddressGroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set AddressGroup");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.address_group data
        let args = &self.address_group;
        let mut address_group_builder = set::AddressGroupBuilder::default();
        if let Some(val) = &args.name {
            address_group_builder.name(val);
        }

        if let Some(val) = &args.description {
            address_group_builder.description(val);
        }

        ep_builder.address_group(address_group_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
