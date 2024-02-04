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

//! Create Websso command
//!
//! Wraps invoking of the `v3/auth/OS-FEDERATION/websso/{protocol_id}` with `POST` method

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

use openstack_sdk::api::identity::v3::auth::os_federation::websso::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// POST operation on /v3/auth/OS-FEDERATION/websso/{protocol_id}
#[derive(Args)]
pub struct WebssoCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args)]
pub struct PathParameters {
    /// protocol_id parameter for /v3/auth/OS-FEDERATION/websso/{protocol_id}
    /// API
    #[arg(value_name = "PROTOCOL_ID", id = "path_param_protocol_id")]
    protocol_id: String,
}
/// Websso response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ResponseData {
    /// A list of one or two audit IDs. An audit ID is a unique, randomly
    /// generated, URL-safe string that you can use to track a token. The first
    /// audit ID is the current audit ID for the token. The second audit ID is
    /// present for only re-scoped tokens and is the audit ID from the token
    /// before it was re-scoped. A re- scoped token is one that was exchanged
    /// for another token of the same or different scope. You can use these
    /// audit IDs to track the use of a token or chain of tokens across
    /// multiple requests and endpoints without exposing the token ID to non-
    /// privileged users.
    #[serde()]
    #[structable(optional)]
    audit_ids: Option<VecString>,

    /// A catalog object.
    #[serde()]
    #[structable(optional)]
    catalog: Option<VecResponseCatalog>,

    /// The date and time when the token expires.
    #[serde()]
    #[structable(optional)]
    expires_at: Option<String>,

    /// The date and time when the token was issued.
    #[serde()]
    #[structable(optional)]
    issues_at: Option<String>,

    /// The authentication methods, which are commonly password, token, or
    /// other methods. Indicates the accumulated set of authentication methods
    /// that were used to obtain the token. For example, if the token was
    /// obtained by password authentication, it contains password. Later, if
    /// the token is exchanged by using the token authentication method one or
    /// more times, the subsequently created tokens contain both password and
    /// token in their methods attribute. Unlike multi-factor authentication,
    /// the methods attribute merely indicates the methods that were used to
    /// authenticate the user in exchange for a token. The client is
    /// responsible for determining the total number of authentication factors.
    #[serde()]
    #[structable(optional)]
    methods: Option<VecString>,

    /// A user object
    #[serde()]
    #[structable(optional)]
    user: Option<ResponseUser>,
}
/// Vector of String response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseEndpoints {
    id: Option<String>,
    interface: Option<String>,
    region: Option<String>,
    url: Option<String>,
}

impl fmt::Display for ResponseEndpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "interface={}",
                self.interface
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "region={}",
                self.region
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "url={}",
                self.url
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// Vector of ResponseEndpoints response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecResponseEndpoints(Vec<ResponseEndpoints>);
impl fmt::Display for VecResponseEndpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseCatalog {
    endpoints: Option<VecResponseEndpoints>,
    id: Option<String>,
    _type: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "endpoints={}",
                self.endpoints
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "_type={}",
                self._type
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// Vector of ResponseCatalog response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecResponseCatalog(Vec<ResponseCatalog>);
impl fmt::Display for VecResponseCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
/// struct response type
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
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// HashMap of Value response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseUser {
    id: Option<String>,
    name: Option<String>,
    domain: Option<ResponseDomain>,
    password_expires_at: Option<String>,
    os_federation: Option<HashMapStringValue>,
}

impl fmt::Display for ResponseUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "domain={}",
                self.domain
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "password_expires_at={}",
                self.password_expires_at
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "os_federation={}",
                self.os_federation
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl WebssoCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Websso");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.protocol_id(&self.path.protocol_id);
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
