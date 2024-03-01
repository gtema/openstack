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

//! Create Role command
//!
//! Wraps invoking of the `v3/roles` with `POST` method

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

use openstack_sdk::api::identity::v3::role::create;
use openstack_sdk::api::QueryAsync;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Creates a role.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/roles`
///
#[derive(Args)]
#[command(about = "Create role")]
pub struct RoleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    role: Role,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Options Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct Options {
    #[arg(action=clap::ArgAction::Set, long)]
    immutable: Option<bool>,
}

/// Role Body data
#[derive(Args)]
struct Role {
    /// The role name.
    ///
    #[arg(long)]
    name: Option<String>,

    /// The role description.
    ///
    #[arg(long)]
    description: Option<String>,

    /// The resource options for the role. Available resource options are
    /// `immutable`.
    ///
    #[command(flatten)]
    options: Option<Options>,
}

/// Role response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The role ID.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The link to the resources in question.
    ///
    #[serde()]
    #[structable(optional)]
    links: Option<HashMapStringOptionString>,

    /// The role name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The role description.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The resource options for the role. Available resource options are
    /// `immutable`.
    ///
    #[serde()]
    #[structable(optional)]
    options: Option<ResponseOptions>,
}
/// HashMap of `Option<String>` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct HashMapStringOptionString(HashMap<String, Option<String>>);
impl fmt::Display for HashMapStringOptionString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1.clone().unwrap_or("".to_string())))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseOptions {
    immutable: Option<bool>,
}

impl fmt::Display for ResponseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "immutable={}",
            self.immutable
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}

impl RoleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Role");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.role data
        let args = &self.role;
        let mut role_builder = create::RoleBuilder::default();
        if let Some(val) = &args.name {
            role_builder.name(val);
        }

        if let Some(val) = &args.description {
            role_builder.description(val);
        }

        if let Some(val) = &args.options {
            let mut options_builder = create::OptionsBuilder::default();
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
