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

//! Set Domain command
//!
//! Wraps invoking of the `v3/domains/{domain_id}` with `PATCH` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::domain::find;
use openstack_sdk::api::identity::v3::domain::set;
use openstack_types::identity::v3::domain::response::set::DomainResponse;

/// Updates a domain.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/domain`
#[derive(Args)]
#[command(about = "Update domain")]
pub struct DomainCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `domain` object
    #[command(flatten)]
    domain: Domain,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// domain_id parameter for /v3/domains/{domain_id} API
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

/// Domain Body data
#[derive(Args, Clone)]
struct Domain {
    /// The new description of the domain.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Set explicit NULL for the description
    #[arg(help_heading = "Body parameters", long, action = clap::ArgAction::SetTrue, conflicts_with = "description")]
    no_description: bool,

    /// If set to `true`, domain is enabled. If set to `false`, domain is
    /// disabled. The default is `true`.
    ///
    /// Users can only authorize against an enabled domain (and any of its
    /// projects). In addition, users can only authenticate if the domain that
    /// owns them is also enabled. Disabling a domain prevents both of these
    /// things. When you disable a domain, all tokens that are authorized for
    /// that domain become invalid. However, if you reenable the domain, these
    /// tokens become valid again, providing that they haven’t expired.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    enabled: Option<bool>,

    /// The new name of the domain.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The resource options for the domain. Available resource options are
    /// `immutable`.
    #[command(flatten)]
    options: Option<Options>,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,
}

impl DomainCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Domain");

        let op = OutputProcessor::from_args(parsed_args, Some("identity.domain"), Some("set"));
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);

        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let mut ep_builder = set::Request::builder();

        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());

        // Set body parameters
        // Set Request.domain data
        let args = &self.domain;
        let mut domain_builder = set::DomainBuilder::default();
        if let Some(val) = &args.description {
            domain_builder.description(Some(val.into()));
        } else if args.no_description {
            domain_builder.description(None);
        }

        if let Some(val) = &args.enabled {
            domain_builder.enabled(*val);
        }

        if let Some(val) = &args.name {
            domain_builder.name(val);
        }

        if let Some(val) = &args.options {
            let mut options_builder = set::OptionsBuilder::default();
            if let Some(val) = &val.immutable {
                options_builder.immutable(*val);
            }
            domain_builder.options(options_builder.build().expect("A valid object"));
        }

        if let Some(val) = &args.tags {
            domain_builder.tags(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        ep_builder.domain(domain_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<DomainResponse>(data)?;
        Ok(())
    }
}
