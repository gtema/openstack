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

//! List QuotaSets command
//!
//! Wraps invoking of the `v2.1/os-quota-sets/{id}/detail` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::quota_set::list_detailed;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Show the detail of quota for a project or a project and a user.
///
/// To show a quota for a project and a user, specify the `user_id` query
/// parameter.
///
/// Normal response codes: 200
///
/// Error response codes: badrequest(400), unauthorized(401), forbidden(403)
///
#[derive(Args)]
#[command(about = "Show The Detail of Quota")]
pub struct QuotaSetsCommand {
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
    #[arg(help_heading = "Query parameters", long)]
    user_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/os-quota-sets/{id}/detail API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// QuotaSets response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The object of detailed cores quota, including in_use, limit and
    /// reserved number of cores.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    cores: Option<Value>,

    /// The object of detailed fixed ips quota, including in_use, limit and
    /// reserved number of fixed ips.
    ///
    /// **Available until version 2.35**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    fixed_ips: Option<Value>,

    /// The object of detailed floating ips quota, including in_use, limit and
    /// reserved number of floating ips.
    ///
    /// **Available until version 2.35**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    floating_ips: Option<Value>,

    /// The UUID of the tenant/user the quotas listed for.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The object of detailed injected files quota, including in_use, limit
    /// and reserved number of injected files.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    injected_files: Option<Value>,

    /// The object of detailed injected file content bytes quota, including
    /// in_use, limit and reserved number of injected file content bytes.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    injected_files_content_bytes: Option<Value>,

    /// The object of detailed injected file path bytes quota, including
    /// in_use, limit and reserved number of injected file path bytes.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    injected_files_path_bytes: Option<Value>,

    /// The object of detailed servers quota, including in_use, limit and
    /// reserved number of instances.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    instances: Option<Value>,

    /// The object of detailed key pairs quota, including in_use, limit and
    /// reserved number of key pairs.
    ///
    /// Note
    ///
    /// `in_use` field value for keypair quota details is always zero. In Nova,
    /// key_pairs are a user-level resource, not a project- level resource, so
    /// for legacy reasons, the keypair in-use information is not counted.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    key_pairs: Option<Value>,

    /// The object of detailed key metadata items quota, including in_use,
    /// limit and reserved number of metadata items.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    metadata_items: Option<Value>,

    /// The number of private networks that can be created per project.
    ///
    /// **Available until version 2.35**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    networks: Option<Value>,

    /// The object of detailed key ram quota, including in_use, limit and
    /// reserved number of ram.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    ram: Option<Value>,

    /// The object of detailed security group rules quota, including in_use,
    /// limit and reserved number of security group rules.
    ///
    /// **Available until version 2.35**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    security_group_rules: Option<Value>,

    /// The object of detailed security groups, including in_use, limit and
    /// reserved number of security groups.
    ///
    /// **Available until version 2.35**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    security_groups: Option<Value>,

    /// The object of detailed server group members, including in_use, limit
    /// and reserved number of server group members.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    server_group_members: Option<Value>,

    /// The object of detailed server groups, including in_use, limit and
    /// reserved number of server groups.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    server_groups: Option<Value>,
}

impl QuotaSetsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List QuotaSets");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list_detailed::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        if let Some(val) = &self.query.user_id {
            ep_builder.user_id(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
