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

//! Show QuotaSet command
//!
//! Wraps invoking of the `v2.1/os-quota-sets/{id}/defaults` with `GET` method

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
use openstack_sdk::api::compute::v2::quota_set::defaults;
use openstack_sdk::types::IntString;
use structable_derive::StructTable;

/// Lists the default quotas for a project.
///
/// Normal response codes: 200
///
/// Error response codes: badrequest(400), unauthorized(401), forbidden(403)
///
#[derive(Args)]
#[command(about = "List Default Quotas For Tenant")]
pub struct QuotaSetCommand {
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
    /// id parameter for /v2.1/os-quota-sets/{id}/defaults API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// QuotaSet response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    cores: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    fixed_ips: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    floating_ips: Option<IntString>,

    /// The UUID of the tenant/user the quotas listed for.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    injected_file_content_bytes: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    injected_file_path_bytes: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    injected_files: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    instances: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    key_pairs: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    metadata_items: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    networks: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    ram: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    security_group_rules: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    security_groups: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    server_group_members: Option<IntString>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde()]
    #[structable(optional)]
    server_groups: Option<IntString>,
}

impl QuotaSetCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show QuotaSet");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = defaults::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
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
