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

//! List Roles command
//!
//! Wraps invoking of the `v3/roles` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use eyre::OptionExt;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::domain::find as find_domain;
use openstack_sdk::api::identity::v3::role::list;
use openstack_types::identity::v3::role::response::list::RoleResponse;
use tracing::warn;

/// Lists roles.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/roles`
#[derive(Args)]
#[command(about = "List roles")]
pub struct RolesCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// Domain resource for which the operation should be performed.
    #[command(flatten)]
    domain: DomainInput,

    /// The resource name.
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,
}

/// Domain input select group
#[derive(Args)]
#[group(required = false, multiple = false)]
struct DomainInput {
    /// Domain Name.
    #[arg(long, help_heading = "Path parameters", value_name = "DOMAIN_NAME")]
    domain_name: Option<String>,
    /// Domain ID.
    #[arg(long, help_heading = "Path parameters", value_name = "DOMAIN_ID")]
    domain_id: Option<String>,
    /// Current domain.
    #[arg(long, help_heading = "Path parameters", action = clap::ArgAction::SetTrue)]
    current_domain: bool,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

impl RolesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Roles");

        let op = OutputProcessor::from_args(parsed_args, Some("identity.role"), Some("list"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set query parameters
        if let Some(id) = &self.query.domain.domain_id {
            // domain_id is passed. No need to lookup
            ep_builder.domain_id(id);
        } else if let Some(name) = &self.query.domain.domain_name {
            // domain_name is passed. Need to lookup resource
            let mut sub_find_builder = find_domain::Request::builder();
            warn!(
                "Querying domain by name (because of `--domain-name` parameter passed) may not be definite. This may fail in which case parameter `--domain-id` should be used instead."
            );

            sub_find_builder.id(name);
            let find_ep = sub_find_builder
                .build()
                .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
            let find_data: serde_json::Value = find_by_name(find_ep).query_async(client).await?;
            // Try to extract resource id
            match find_data.get("id") {
                Some(val) => match val.as_str() {
                    Some(id_str) => {
                        ep_builder.domain_id(id_str.to_owned());
                    }
                    None => {
                        return Err(OpenStackCliError::ResourceAttributeNotString(
                            serde_json::to_string(&val)?,
                        ));
                    }
                },
                None => {
                    return Err(OpenStackCliError::ResourceAttributeMissing(
                        "id".to_string(),
                    ));
                }
            };
        } else if self.query.domain.current_domain {
            ep_builder.domain_id(
                client
                    .get_auth_info()
                    .ok_or_eyre("Cannot determine current authentication information")?
                    .token
                    .user
                    .id,
            );
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<RoleResponse>(data)?;
        Ok(())
    }
}
