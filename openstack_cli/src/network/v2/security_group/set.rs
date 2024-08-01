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

//! Set SecurityGroup command
//!
//! Wraps invoking of the `v2.0/security-groups/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::BoolString;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::security_group::find;
use openstack_sdk::api::network::v2::security_group::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates a security group.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 403, 404, 412
///
#[derive(Args)]
#[command(about = "Update security group")]
pub struct SecurityGroupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `security_group` object.
    ///
    #[command(flatten)]
    security_group: SecurityGroup,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/security-groups/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// SecurityGroup Body data
#[derive(Args, Clone)]
struct SecurityGroup {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    stateful: Option<bool>,
}

/// SecurityGroup response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the security group.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// A list of `security_group_rule` objects. Refer to
    /// [Security group rules](#security-group-rules) for details.
    ///
    #[serde()]
    #[structable(optional)]
    security_group_rules: Option<String>,

    /// Indicates whether this security group is shared to the requester’s
    /// project.
    ///
    #[serde()]
    #[structable(optional)]
    shared: Option<BoolString>,

    /// Indicates if the security group is stateful or stateless.
    ///
    #[serde()]
    #[structable(optional)]
    stateful: Option<BoolString>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl SecurityGroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set SecurityGroup");

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
        // Set Request.security_group data
        let args = &self.security_group;
        let mut security_group_builder = set::SecurityGroupBuilder::default();
        if let Some(val) = &args.name {
            security_group_builder.name(val);
        }

        if let Some(val) = &args.description {
            security_group_builder.description(val);
        }

        if let Some(val) = &args.stateful {
            security_group_builder.stateful(*val);
        }

        ep_builder.security_group(security_group_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}