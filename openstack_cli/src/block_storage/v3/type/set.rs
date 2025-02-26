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

//! Set Type command
//!
//! Wraps invoking of the `v3/types/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::block_storage::v3::r#type::find;
use openstack_sdk::api::block_storage::v3::r#type::set;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct TypeCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    volume_type: VolumeType,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v3/types/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// VolumeType Body data
#[derive(Args, Clone)]
struct VolumeType {
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    is_public: Option<bool>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// Type response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The volume type description.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// A key and value pair that contains additional specifications that are
    /// associated with the volume type. Examples include capabilities,
    /// capacity, compression, and so on, depending on the storage driver in
    /// use.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    extra_specs: Option<Value>,

    /// The UUID of the volume type.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// Whether the volume type is publicly visible.
    ///
    #[serde()]
    #[structable(optional)]
    is_public: Option<bool>,

    /// The name of the volume type.
    ///
    #[serde()]
    #[structable()]
    name: String,

    /// Whether the volume type is publicly visible.
    ///
    #[serde(rename = "os-volume-type-access:is_public")]
    #[structable(optional, title = "os-volume-type-access:is_public")]
    os_volume_type_access_is_public: Option<bool>,

    /// The QoS specifications ID.
    ///
    #[serde()]
    #[structable(optional)]
    qos_specs_id: Option<String>,
}

impl TypeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Type");

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
        // Set Request.volume_type data
        let args = &self.volume_type;
        let mut volume_type_builder = set::VolumeTypeBuilder::default();
        if let Some(val) = &args.name {
            volume_type_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.description {
            volume_type_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.is_public {
            volume_type_builder.is_public(*val);
        }

        ep_builder.volume_type(volume_type_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
