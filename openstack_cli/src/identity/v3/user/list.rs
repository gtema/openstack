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

//! List Users command
//!
//! Wraps invoking of the `v3/users` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use eyre::OptionExt;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::domain::find as find_domain;
use openstack_sdk::api::identity::v3::user::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use structable_derive::StructTable;
use tracing::warn;

/// Lists users.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/users`
///
#[derive(Args)]
#[command(about = "List users")]
pub struct UsersCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// Domain resource for which the operation should be performed.
    #[command(flatten)]
    domain: DomainInput,

    /// Whether the identity provider is enabled or not
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    enabled: Option<bool>,

    /// Filters the response by an identity provider ID.
    ///
    #[arg(help_heading = "Query parameters", long)]
    idp_id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    /// ID of the last fetched entry
    ///
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// The resource name.
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// Filter results based on which user passwords have expired. The query
    /// should include an operator and a timestamp with a colon (:) separating
    /// the two, for example: `password_expires_at={operator}:{timestamp}`
    /// Valid operators are: lt, lte, gt, gte, eq, and neq
    ///
    /// - lt: expiration time lower than the timestamp
    /// - lte: expiration time lower than or equal to the timestamp
    /// - gt: expiration time higher than the timestamp
    /// - gte: expiration time higher than or equal to the timestamp
    /// - eq: expiration time equal to the timestamp
    /// - neq: expiration time not equal to the timestamp
    ///
    /// Valid timestamps are of the form: `YYYY-MM-DDTHH:mm:ssZ`.For
    /// example:`/v3/users?password_expires_at=lt:2016-12-08T22:02:00Z` The
    /// example would return a list of users whose password expired before the
    /// timestamp `(2016-12-08T22:02:00Z).`
    ///
    #[arg(help_heading = "Query parameters", long)]
    password_expires_at: Option<String>,

    /// Filters the response by a protocol ID.
    ///
    #[arg(help_heading = "Query parameters", long)]
    protocol_id: Option<String>,

    /// Sort direction. A valid value is asc (ascending) or desc (descending).
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["asc","desc"])]
    sort_dir: Option<String>,

    /// Sorts resources by attribute.
    ///
    #[arg(help_heading = "Query parameters", long)]
    sort_key: Option<String>,

    /// Filters the response by a unique ID.
    ///
    #[arg(help_heading = "Query parameters", long)]
    unique_id: Option<String>,
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
/// Users response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the default project for the user.
    ///
    #[serde()]
    #[structable(optional, wide)]
    default_project_id: Option<String>,

    /// The resource description.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the domain.
    ///
    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    /// List of federated objects associated with a user. Each object in the
    /// list contains the `idp_id` and `protocols`. `protocols` is a list of
    /// objects, each of which contains `protocol_id` and `unique_id` of the
    /// protocol and user respectively. For example:
    ///
    /// ```text
    /// "federated": [
    ///   {
    ///     "idp_id": "efbab5a6acad4d108fec6c63d9609d83",
    ///     "protocols": [
    ///       {"protocol_id": "mapped", "unique_id": "test@example.com"}
    ///     ]
    ///   }
    /// ]
    ///
    /// ```
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    federated: Option<Value>,

    /// The user ID.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The user name. Must be unique within the owning domain.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The resource options for the user. Available resource options are
    /// `ignore_change_password_upon_first_use`, `ignore_password_expiry`,
    /// `ignore_lockout_failure_attempts`, `lock_password`,
    /// `multi_factor_auth_enabled`, and `multi_factor_auth_rules`
    /// `ignore_user_inactivity`.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    options: Option<Value>,

    /// The date and time when the password expires. The time zone is UTC.
    ///
    /// This is a response object attribute; not valid for requests. A `null`
    /// value indicates that the password never expires.
    ///
    /// **New in version 3.7**
    ///
    #[serde()]
    #[structable(optional, wide)]
    password_expires_at: Option<String>,
}

impl UsersCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Users");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(id) = &self.query.domain.domain_id {
            // domain_id is passed. No need to lookup
            ep_builder.domain_id(id);
        } else if let Some(name) = &self.query.domain.domain_name {
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
        if let Some(val) = &self.query.enabled {
            ep_builder.enabled(*val);
        }
        if let Some(val) = &self.query.idp_id {
            ep_builder.idp_id(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.password_expires_at {
            ep_builder.password_expires_at(val);
        }
        if let Some(val) = &self.query.protocol_id {
            ep_builder.protocol_id(val);
        }
        if let Some(val) = &self.query.unique_id {
            ep_builder.unique_id(val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val);
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
