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

//! Set Cluster command
//!
//! Wraps invoking of the `v3/clusters/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::block_storage::v3::cluster::set;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Enable/Disable scheduling for a cluster.
///
#[derive(Args)]
pub struct ClusterCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The binary name of the services in the cluster.
    ///
    #[arg(help_heading = "Body parameters", long)]
    binary: Option<String>,

    /// The reason for disabling a resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    disabled_reason: Option<String>,

    /// The name to identify the service cluster.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: String,
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
        info!("Set Cluster");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.binary data
        if let Some(arg) = &self.binary {
            ep_builder.binary(Some(arg.into()));
        }

        // Set Request.disabled_reason data
        if let Some(arg) = &self.disabled_reason {
            ep_builder.disabled_reason(Some(arg.into()));
        }

        // Set Request.name data
        ep_builder.name(&self.name);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
