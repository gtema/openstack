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

//! Create Policy command
//!
//! Wraps invoking of the `v2.0/qos/policies` with `POST` method

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
use openstack_sdk::api::network::v2::qos::policy::create;
use openstack_sdk::types::BoolString;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a QoS policy.
///
/// Creates a QoS policy by using the configuration that you define in the
/// request object. A response object is returned. The object contains a unique
/// ID.
///
/// By the default policy configuration, if the caller is not an administrative
/// user, this call returns the HTTP `Forbidden (403)` response code.
///
/// Users with an administrative role can create policies on behalf of other
/// projects by specifying a project ID that is different than their own.
///
/// Normal response codes: 201
///
/// Error response codes: 401, 403, 404, 409
///
#[derive(Args)]
#[command(about = "Create QoS policy")]
pub struct PolicyCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A QoS `policy` object.
    ///
    #[command(flatten)]
    policy: Policy,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Policy Body data
#[derive(Args, Clone)]
struct Policy {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// If `true`, the QoS `policy` is the default policy.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    is_default: Option<bool>,

    /// Human-readable name of the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// Set to `true` to share this policy with other projects. Default is
    /// `false`.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    shared: Option<bool>,

    /// The ID of the project that owns the resource. Only administrative and
    /// users with advsvc role can specify a project ID other than their own.
    /// You cannot change this value through authorization policies.
    ///
    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,
}

/// Policy response representation
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

    /// The ID of the QoS policy.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// If `true`, the QoS `policy` is the default policy.
    ///
    #[serde()]
    #[structable(optional)]
    is_default: Option<BoolString>,

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

    /// A set of zero or more policy rules.
    ///
    #[serde()]
    #[structable(optional)]
    rules: Option<String>,

    /// Indicates whether this policy is shared across all projects.
    ///
    #[serde()]
    #[structable(optional)]
    shared: Option<BoolString>,

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

impl PolicyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Policy");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.policy data
        let args = &self.policy;
        let mut policy_builder = create::PolicyBuilder::default();
        if let Some(val) = &args.name {
            policy_builder.name(val);
        }

        if let Some(val) = &args.shared {
            policy_builder.shared(*val);
        }

        if let Some(val) = &args.tenant_id {
            policy_builder.tenant_id(val);
        }

        if let Some(val) = &args.is_default {
            policy_builder.is_default(*val);
        }

        if let Some(val) = &args.description {
            policy_builder.description(val);
        }

        ep_builder.policy(policy_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
