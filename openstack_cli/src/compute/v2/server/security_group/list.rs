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

//! List SecurityGroups command
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/os-security-groups` with `GET` method

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

use openstack_sdk::api::compute::v2::server::security_group::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Lists security groups for a server.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args)]
#[command(about = "List Security Groups By Server")]
pub struct SecurityGroupsCommand {
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
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[arg(value_name = "SERVER_ID", id = "path_param_server_id")]
    server_id: String,
}
/// SecurityGroups response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ResponseData {
    /// The ID of the security group.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The security group name.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// Security group description.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The UUID of the tenant in a multi-tenancy cloud.
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The list of security group rules.
    #[serde()]
    #[structable(optional, wide)]
    rules: Option<VecResponseRules>,
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
struct ResponseGroup {
    name: Option<String>,
}

impl fmt::Display for ResponseGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "name={}",
            self.name
                .clone()
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseRules {
    id: Option<String>,
    from_port: Option<i32>,
    to_port: Option<i32>,
    ip_protocol: Option<String>,
    ip_range: Option<HashMapStringValue>,
    group: Option<ResponseGroup>,
    parent_group_id: Option<String>,
}

impl fmt::Display for ResponseRules {
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
                "from_port={}",
                self.from_port
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "to_port={}",
                self.to_port
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ip_protocol={}",
                self.ip_protocol
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ip_range={}",
                self.ip_range
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "group={}",
                self.group
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "parent_group_id={}",
                self.parent_group_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// Vector of ResponseRules response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecResponseRules(Vec<ResponseRules>);
impl fmt::Display for VecResponseRules {
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

impl SecurityGroupsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List SecurityGroups");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
