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

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::keypair::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use structable_derive::StructTable;

/// Lists keypairs that are associated with the account.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403)
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
    #[arg(long)]
    user_id: Option<String>,

    #[arg(long)]
    limit: Option<i32>,

    #[arg(long)]
    marker: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Keypairs response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The name for the keypair.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The keypair public key.
    #[serde()]
    #[structable(optional)]
    public_key: Option<String>,

    /// The fingerprint for the keypair.
    #[serde()]
    #[structable(optional)]
    fingerprint: Option<String>,

    /// The type of the keypair. Allowed values are `ssh` or `x509`.
    ///
    ///
    /// **New in version 2.2**
    #[serde(rename = "type")]
    #[structable(optional, title = "type", wide)]
    _type: Option<String>,
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
        if let Some(val) = &self.query.user_id {
            ep_builder.user_id(val.clone());
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val.clone());
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
