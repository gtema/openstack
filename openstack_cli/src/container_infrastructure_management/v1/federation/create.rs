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

//! Create Federation command
//!
//! Wraps invoking of the `v1/federations` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use crate::common::parse_json;
use crate::common::parse_key_val;
use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::container_infrastructure_management::v1::federation::create;
use serde_json::Value;
use structable_derive::StructTable;

/// Create a new federation.
///
/// | param federation: | | | --- | --- | | | a federation within the request
/// body. |
///
#[derive(Args)]
pub struct FederationCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long)]
    created_at: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    hostcluster_id: Option<String>,

    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    links: Option<Vec<Value>>,

    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    member_ids: Option<Vec<String>>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    properties: Option<Vec<(String, String)>>,

    #[arg(help_heading = "Body parameters", long)]
    status: Option<Status>,

    #[arg(help_heading = "Body parameters", long)]
    status_reason: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    updated_at: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    uuid: Option<String>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Status {
    CreateComplete,
    CreateFailed,
    CreateInProgress,
    DeleteComplete,
    DeleteFailed,
    DeleteInProgress,
    UpdateComplete,
    UpdateFailed,
    UpdateInProgress,
}

/// Federation response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable()]
    uuid: String,
}

impl FederationCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Federation");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.created_at data
        if let Some(arg) = &self.created_at {
            ep_builder.created_at(arg);
        }

        // Set Request.hostcluster_id data
        if let Some(arg) = &self.hostcluster_id {
            ep_builder.hostcluster_id(arg);
        }

        // Set Request.links data
        if let Some(arg) = &self.links {
            let links_builder: Vec<create::Links> = arg
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Links>(v.to_owned()))
                .collect::<Vec<create::Links>>();
            ep_builder.links(links_builder);
        }

        // Set Request.member_ids data
        if let Some(arg) = &self.member_ids {
            ep_builder.member_ids(arg.iter().map(Into::into).collect::<Vec<_>>());
        }

        // Set Request.name data
        if let Some(arg) = &self.name {
            ep_builder.name(arg);
        }

        // Set Request.properties data
        if let Some(arg) = &self.properties {
            ep_builder.properties(arg.iter().cloned());
        }

        // Set Request.status data
        if let Some(arg) = &self.status {
            let tmp = match arg {
                Status::CreateComplete => create::Status::CreateComplete,
                Status::CreateFailed => create::Status::CreateFailed,
                Status::CreateInProgress => create::Status::CreateInProgress,
                Status::DeleteComplete => create::Status::DeleteComplete,
                Status::DeleteFailed => create::Status::DeleteFailed,
                Status::DeleteInProgress => create::Status::DeleteInProgress,
                Status::UpdateComplete => create::Status::UpdateComplete,
                Status::UpdateFailed => create::Status::UpdateFailed,
                Status::UpdateInProgress => create::Status::UpdateInProgress,
            };
            ep_builder.status(tmp);
        }

        // Set Request.status_reason data
        if let Some(arg) = &self.status_reason {
            ep_builder.status_reason(arg);
        }

        // Set Request.updated_at data
        if let Some(arg) = &self.updated_at {
            ep_builder.updated_at(arg);
        }

        // Set Request.uuid data
        if let Some(arg) = &self.uuid {
            ep_builder.uuid(arg);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
