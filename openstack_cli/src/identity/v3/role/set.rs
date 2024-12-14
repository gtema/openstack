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

//! Set Role command
//!
//! Wraps invoking of the `v3/roles/{role_id}` with `PATCH` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::role::find;
use openstack_sdk::api::identity::v3::role::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates a role.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/role`
///
#[derive(Args)]
#[command(about = "Update role")]
pub struct RoleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `role` object
    ///
    #[command(flatten)]
    role: Role,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// role_id parameter for /v3/roles/{role_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Options Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct Options {
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    immutable: Option<bool>,
}

/// Role Body data
#[derive(Args, Clone)]
struct Role {
    /// The new role description.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The ID of the domain.
    ///
    #[arg(help_heading = "Body parameters", long)]
    domain_id: Option<String>,

    /// The new role name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The resource options for the role. Available resource options are
    /// `immutable`.
    ///
    #[command(flatten)]
    options: Option<Options>,
}

/// Role response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The role description.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the domain.
    ///
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// The role ID.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The link to the resources in question.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    /// The resource name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The resource options for the role. Available resource options are
    /// `immutable`.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    options: Option<Value>,
}

impl RoleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Role");

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
        // Set Request.role data
        let args = &self.role;
        let mut role_builder = set::RoleBuilder::default();
        if let Some(val) = &args.name {
            role_builder.name(val);
        }

        if let Some(val) = &args.description {
            role_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.domain_id {
            role_builder.domain_id(Some(val.into()));
        }

        if let Some(val) = &args.options {
            let mut options_builder = set::OptionsBuilder::default();
            if let Some(val) = &val.immutable {
                options_builder.immutable(*val);
            }
            role_builder.options(options_builder.build().expect("A valid object"));
        }

        ep_builder.role(role_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
