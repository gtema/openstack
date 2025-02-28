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

//! List Keypairs command
//!
//! Wraps invoking of the `v2.1/os-keypairs` with `GET` method

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
use openstack_sdk::api::compute::v2::keypair::list;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::user::find as find_user;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use structable_derive::StructTable;
use tracing::warn;

/// Lists keypairs that are associated with the account.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403)
///
#[derive(Args)]
#[command(about = "List Keypairs")]
pub struct KeypairsCommand {
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
    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// User resource for which the operation should be performed.
    #[command(flatten)]
    user: UserInput,
}

/// User input select group
#[derive(Args)]
#[group(required = false, multiple = false)]
struct UserInput {
    /// User Name.
    #[arg(long, help_heading = "Path parameters", value_name = "USER_NAME")]
    user_name: Option<String>,
    /// User ID.
    #[arg(long, help_heading = "Path parameters", value_name = "USER_ID")]
    user_id: Option<String>,
    /// Current authenticated user.
    #[arg(long, help_heading = "Path parameters", action = clap::ArgAction::SetTrue)]
    current_user: bool,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Keypairs response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The fingerprint for the keypair.
    ///
    #[serde()]
    #[structable()]
    fingerprint: String,

    /// The name for the keypair.
    ///
    #[serde()]
    #[structable()]
    name: String,

    /// The keypair public key.
    ///
    #[serde()]
    #[structable()]
    public_key: String,

    /// The type of the keypair. Allowed values are `ssh` or `x509`.
    ///
    /// **New in version 2.2**
    ///
    #[serde(rename = "type")]
    #[structable(title = "type", wide)]
    _type: String,
}

impl KeypairsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Keypairs");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(id) = &self.query.user.user_id {
            // user_id is passed. No need to lookup
            ep_builder.user_id(id);
        } else if let Some(name) = &self.query.user.user_name {
            // user_name is passed. Need to lookup resource
            let mut sub_find_builder = find_user::Request::builder();
            warn!("Querying user by name (because of `--user-name` parameter passed) may not be definite. This may fail in which case parameter `--user-id` should be used instead.");

            sub_find_builder.id(name);
            let find_ep = sub_find_builder
                .build()
                .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
            let find_data: serde_json::Value = find_by_name(find_ep).query_async(client).await?;
            // Try to extract resource id
            match find_data.get("id") {
                Some(val) => match val.as_str() {
                    Some(id_str) => {
                        ep_builder.user_id(id_str.to_owned());
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
        } else if self.query.user.current_user {
            ep_builder.user_id(
                client
                    .get_auth_info()
                    .ok_or_eyre("Cannot determine current authentication information")?
                    .token
                    .user
                    .id,
            );
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
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
