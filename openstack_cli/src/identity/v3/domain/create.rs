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

//! Create Domain command
//!
//! Wraps invoking of the `v3/domains` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_key_val;
use openstack_sdk::api::identity::v3::domain::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a domain.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/domains`
///
#[derive(Args)]
#[command(about = "Create domain")]
pub struct DomainCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `domain` object
    ///
    #[command(flatten)]
    domain: Domain,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Domain Body data
#[derive(Args, Clone)]
struct Domain {
    /// The description of the domain.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// If set to `true`, domain is enabled. If set to `false`, domain is
    /// disabled.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    enabled: Option<bool>,

    /// The name of the domain.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The resource options for the role. Available resource options are
    /// `immutable`.
    ///
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    options: Option<Vec<(String, Value)>>,

    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,
}

/// Domain response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The description of the domain.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// If set to `true`, domain is enabled. If set to `false`, domain is
    /// disabled.
    ///
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// The ID of the domain.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The name of the domain.
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

    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,
}

impl DomainCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Domain");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.domain data
        let args = &self.domain;
        let mut domain_builder = create::DomainBuilder::default();
        if let Some(val) = &args.name {
            domain_builder.name(val);
        }

        if let Some(val) = &args.description {
            domain_builder.description(val);
        }

        if let Some(val) = &args.enabled {
            domain_builder.enabled(*val);
        }

        if let Some(val) = &args.tags {
            domain_builder.tags(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.options {
            domain_builder.options(val.iter().cloned());
        }

        ep_builder.domain(domain_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
