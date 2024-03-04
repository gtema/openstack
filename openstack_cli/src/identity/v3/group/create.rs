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

//! Create Group command
//!
//! Wraps invoking of the `v3/groups` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::identity::v3::group::create;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Creates a group.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/groups`
///
#[derive(Args)]
#[command(about = "Create group")]
pub struct GroupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    group: Group,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Group Body data
#[derive(Args)]
struct Group {
    /// The description of the group.
    ///
    #[arg(long)]
    description: Option<String>,

    /// The ID of the domain.
    ///
    #[arg(long)]
    domain_id: Option<String>,

    /// The user name. Must be unique within the owning domain.
    ///
    #[arg(long)]
    name: Option<String>,
}

/// Group response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the group.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The description of the group.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the domain.
    ///
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// The user name. Must be unique within the owning domain.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,
}

impl GroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Group");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.group data
        let args = &self.group;
        let mut group_builder = create::GroupBuilder::default();
        if let Some(val) = &args.description {
            group_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.domain_id {
            group_builder.domain_id(val);
        }

        if let Some(val) = &args.name {
            group_builder.name(val);
        }

        ep_builder.group(group_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}