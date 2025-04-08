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

//! Get Token command
//!
//! Wraps invoking of the `v3/auth/tokens` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::auth::token::get;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Validates and shows information for a token, including its expiration date
/// and authorization scope.
///
/// Pass your own token in the `X-Auth-Token` request header.
///
/// Pass the token that you want to validate in the `X-Subject-Token` request
/// header.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/auth_tokens`
///
#[derive(Args)]
#[command(about = "Validate and show information for token")]
pub struct TokenCommand {
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
struct PathParameters {}
/// Token response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A list of one or two audit IDs. An audit ID is a unique, randomly
    /// generated, URL-safe string that you can use to track a token. The first
    /// audit ID is the current audit ID for the token. The second audit ID is
    /// present for only re-scoped tokens and is the audit ID from the token
    /// before it was re-scoped. A re- scoped token is one that was exchanged
    /// for another token of the same or different scope. You can use these
    /// audit IDs to track the use of a token or chain of tokens across
    /// multiple requests and endpoints without exposing the token ID to
    /// non-privileged users.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    audit_ids: Option<Value>,

    /// A `catalog` object.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    catalog: Option<Value>,

    /// A domain object including the id and name representing the domain the
    /// token is scoped to. This is only included in tokens that are scoped to
    /// a domain.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    domain: Option<Value>,

    /// The date and time when the token expires.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss.sssZ
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58.000000Z`.
    ///
    /// A `null` value indicates that the token never expires.
    ///
    #[serde()]
    #[structable(optional)]
    expires_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    is_domain: Option<bool>,

    /// The date and time when the token was issued.
    ///
    #[serde()]
    #[structable(optional)]
    issues_at: Option<String>,

    /// The authentication methods, which are commonly `password`, `token`, or
    /// other methods. Indicates the accumulated set of authentication methods
    /// that were used to obtain the token. For example, if the token was
    /// obtained by password authentication, it contains `password`. Later, if
    /// the token is exchanged by using the token authentication method one or
    /// more times, the subsequently created tokens contain both `password` and
    /// `token` in their `methods` attribute. Unlike multi-factor
    /// authentication, the `methods` attribute merely indicates the methods
    /// that were used to authenticate the user in exchange for a token. The
    /// client is responsible for determining the total number of
    /// authentication factors.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    methods: Option<Value>,

    /// A `project` object including the `id`, `name` and `domain` object
    /// representing the project the token is scoped to. This is only included
    /// in tokens that are scoped to a project.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    project: Option<Value>,

    /// A list of `role` objects
    ///
    #[serde()]
    #[structable(optional, pretty)]
    roles: Option<Value>,

    /// A `system` object containing information about which parts of the
    /// system the token is scoped to. If the token is scoped to the entire
    /// deployment system, the `system` object will consist of `{"all": true}`.
    /// This is only included in tokens that are scoped to the system.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    system: Option<Value>,

    /// A `user` object.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    user: Option<Value>,
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseDomain {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id.clone().map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "name={}",
                self.name.clone().map_or(String::new(), |v| v.to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl TokenCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Token");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let ep_builder = get::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
