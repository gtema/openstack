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

//! Set Project command
//!
//! Wraps invoking of the `v3/projects/{project_id}` with `PATCH` method

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
use openstack_sdk::api::identity::v3::project::find;
use openstack_sdk::api::identity::v3::project::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates a project.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/project`
///
#[derive(Args)]
#[command(about = "Update project")]
pub struct ProjectCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `project` object
    ///
    #[command(flatten)]
    project: Project,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// project_id parameter for /v3/projects/{project_id} API
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

/// Project Body data
#[derive(Args, Clone)]
struct Project {
    /// The description of the project.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// If set to `true`, project is enabled. If set to `false`, project is
    /// disabled.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    enabled: Option<bool>,

    /// The name of the project, which must be unique within the owning domain.
    /// A project can have the same name as its domain.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    ///
    #[command(flatten)]
    options: Option<Options>,

    /// A list of simple strings assigned to a project. Tags can be used to
    /// classify projects into groups.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,
}

/// Project response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The description of the project.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the domain for the project.
    ///
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// If set to `true`, project is enabled. If set to `false`, project is
    /// disabled.
    ///
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// The ID for the project.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// If set to `true`, project is enabled. If set to `false`, project is
    /// disabled.
    ///
    #[serde()]
    #[structable(optional)]
    is_domain: Option<bool>,

    /// The link to the resources in question.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    /// The name of the project.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    options: Option<Value>,

    /// The ID of the parent for the project.
    ///
    /// **New in version 3.4**
    ///
    #[serde()]
    #[structable(optional)]
    parent_id: Option<String>,

    /// A list of simple strings assigned to a project.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,
}

impl ProjectCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Project");

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
        // Set Request.project data
        let args = &self.project;
        let mut project_builder = set::ProjectBuilder::default();
        if let Some(val) = &args.description {
            project_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.enabled {
            project_builder.enabled(*val);
        }

        if let Some(val) = &args.name {
            project_builder.name(val);
        }

        if let Some(val) = &args.options {
            let mut options_builder = set::OptionsBuilder::default();
            if let Some(val) = &val.immutable {
                options_builder.immutable(*val);
            }
            project_builder.options(options_builder.build().expect("A valid object"));
        }

        if let Some(val) = &args.tags {
            project_builder.tags(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        ep_builder.project(project_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
