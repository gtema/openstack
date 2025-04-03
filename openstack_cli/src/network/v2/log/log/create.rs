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

//! Create Log command
//!
//! Wraps invoking of the `v2.0/log/logs` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::log::log::create;
use openstack_sdk::types::BoolString;
use structable_derive::StructTable;

/// Creates a log resource.
///
/// Creates a log resource by using the configuration that you define in the
/// request object. A response object is returned. The object contains a unique
/// ID.
///
/// If the caller is not an administrative user, this call returns the HTTP
/// `Forbidden (403)` response code.
///
/// Users with an administrative role can create policies on behalf of other
/// projects by specifying a project ID that is different than their own.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 403, 409
///
#[derive(Args)]
#[command(about = "Create log")]
pub struct LogCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `log` object.
    ///
    #[command(flatten)]
    log: Log,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Event {
    Accept,
    All,
    Drop,
}

/// Log Body data
#[derive(Args, Clone)]
struct Log {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Indicates whether this log object is enabled or disabled. Default is
    /// true.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    enabled: Option<bool>,

    /// Type of security events to log. `ACCEPT`, `DROP`, or `ALL`. Default is
    /// `ALL`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    event: Option<Event>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The ID of the project that owns the resource. Only administrative and
    /// users with advsvc role can specify a project ID other than their own.
    /// You cannot change this value through authorization policies.
    ///
    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    /// The ID of resource log (e.g security group ID).
    ///
    #[arg(help_heading = "Body parameters", long)]
    resource_id: Option<String>,

    /// The resource log type such as ‘security_group’.
    ///
    #[arg(help_heading = "Body parameters", long)]
    resource_type: Option<String>,

    /// The ID of resource target log such as port ID.
    ///
    #[arg(help_heading = "Body parameters", long)]
    target_id: Option<String>,
}

/// Log response representation
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

    /// Indicates whether this log object is enabled or disabled.
    ///
    #[serde()]
    #[structable(optional)]
    enabled: Option<BoolString>,

    /// Type of security events to log. `ACCEPT`, `DROP`, or `ALL`.
    ///
    #[serde()]
    #[structable(optional)]
    event: Option<String>,

    /// The ID of the log object.
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

    /// The ID of resource log (e.g security group ID).
    ///
    #[serde()]
    #[structable(optional)]
    resource_id: Option<String>,

    /// The resource log type such as ‘security_group’.
    ///
    #[serde()]
    #[structable(optional)]
    resource_type: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The ID of resource target log such as port ID.
    ///
    #[serde()]
    #[structable(optional)]
    target_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl LogCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Log");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.log data
        let args = &self.log;
        let mut log_builder = create::LogBuilder::default();
        if let Some(val) = &args.project_id {
            log_builder.project_id(val);
        }

        if let Some(val) = &args.name {
            log_builder.name(val);
        }

        if let Some(val) = &args.resource_type {
            log_builder.resource_type(val);
        }

        if let Some(val) = &args.resource_id {
            log_builder.resource_id(Some(val.into()));
        }

        if let Some(val) = &args.event {
            let tmp = match val {
                Event::Accept => create::Event::Accept,
                Event::All => create::Event::All,
                Event::Drop => create::Event::Drop,
            };
            log_builder.event(tmp);
        }

        if let Some(val) = &args.target_id {
            log_builder.target_id(Some(val.into()));
        }

        if let Some(val) = &args.enabled {
            log_builder.enabled(*val);
        }

        if let Some(val) = &args.description {
            log_builder.description(val);
        }

        ep_builder.log(log_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
