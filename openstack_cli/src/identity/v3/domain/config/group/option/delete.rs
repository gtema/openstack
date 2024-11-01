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

//! Delete Option command
//!
//! Wraps invoking of the `v3/domains/{domain_id}/config/{group}/{option}` with `DELETE` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use bytes::Bytes;
use eyre::eyre;
use eyre::OptionExt;
use http::Response;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::domain::config::group::option::delete;
use openstack_sdk::api::identity::v3::domain::find as find_domain;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;
use tracing::warn;

/// Deletes a domain group option configuration.
///
/// The API supports only the `identity` and `ldap` groups. For the `ldap`
/// group, a valid value is `url` or `user_tree_dn`. For the `identity` group,
/// a valid value is `driver`.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`
///
#[derive(Args)]
#[command(about = "Delete domain group option configuration")]
pub struct OptionCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// Domain resource for which the operation should be performed.
    #[command(flatten)]
    domain: DomainInput,

    /// group parameter for /v3/domains/{domain_id}/config/{group}/{option} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_group",
        value_name = "GROUP"
    )]
    group: String,

    /// option parameter for /v3/domains/{domain_id}/config/{group}/{option}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_option",
        value_name = "OPTION"
    )]
    option: String,
}

/// Domain input select group
#[derive(Args)]
#[group(required = true, multiple = false)]
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
/// Option response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl OptionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Option");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = delete::Request::builder();

        // Set path parameters

        // Process path parameter `domain_id`
        if let Some(id) = &self.path.domain.domain_id {
            // domain_id is passed. No need to lookup
            ep_builder.domain_id(id);
        } else if let Some(name) = &self.path.domain.domain_name {
            // domain_name is passed. Need to lookup resource
            let mut sub_find_builder = find_domain::Request::builder();
            warn!("Querying domain by name (because of `--domain-name` parameter passed) may not be definite. This may fail in which case parameter `--domain-id` should be used instead.");

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
                        ))
                    }
                },
                None => {
                    return Err(OpenStackCliError::ResourceAttributeMissing(
                        "id".to_string(),
                    ))
                }
            };
        } else if self.path.domain.current_domain {
            let token = client
                .get_auth_info()
                .ok_or_eyre("Cannot determine current authentication information")?
                .token;
            if let Some(domain) = token.domain {
                ep_builder.domain_id(domain.id.ok_or_eyre("Domain ID is missing in the auth")?);
            } else if let Some(project) = token.project {
                ep_builder.domain_id(
                    project
                        .domain
                        .ok_or_eyre("Domain information is missing in the project auth info")?
                        .id
                        .ok_or_eyre("Domain ID is missing in the project.domain auth info")?,
                );
            } else {
                return Err(eyre!("Current domain information can not be identified").into());
            }
        }
        ep_builder.group(&self.path.group);
        ep_builder.option(&self.path.option);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
