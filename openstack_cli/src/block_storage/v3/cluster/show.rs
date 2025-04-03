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

//! Show Cluster command
//!
//! Wraps invoking of the `v3/clusters/{id}` with `GET` method

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
use openstack_sdk::api::block_storage::v3::cluster::get;
use structable_derive::StructTable;

/// Return data for a given cluster name with optional binary.
///
#[derive(Args)]
pub struct ClusterCommand {
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
    /// id parameter for /v3/clusters/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Cluster response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of active storage backend. Only in cinder-volume service.
    ///
    #[serde()]
    #[structable(optional)]
    active_backend_id: Option<String>,

    /// The binary name of the services in the cluster.
    ///
    #[serde()]
    #[structable(optional)]
    binary: Option<String>,

    /// The date and time when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The reason for disabling a resource.
    ///
    #[serde()]
    #[structable(optional)]
    disabled_reason: Option<String>,

    /// Whether the cluster is frozen or not.
    ///
    #[serde()]
    #[structable(optional)]
    frozen: Option<bool>,

    /// The last periodic heartbeat received.
    ///
    #[serde()]
    #[structable(optional)]
    last_heartbeat: Option<String>,

    /// The name of the service cluster.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The number of down hosts in the cluster.
    ///
    #[serde()]
    #[structable(optional)]
    num_down_hosts: Option<i32>,

    /// The number of hosts in the cluster.
    ///
    #[serde()]
    #[structable(optional)]
    num_hosts: Option<i32>,

    /// The cluster replication status. Only included in responses if
    /// configured.
    ///
    #[serde()]
    #[structable(optional)]
    replication_status: Option<String>,

    /// The state of the cluster.
    ///
    #[serde()]
    #[structable(optional)]
    state: Option<String>,

    /// The status of the cluster.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The date and time when the resource was updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl ClusterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Cluster");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

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
