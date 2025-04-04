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

//! List Clusters command
//!
//! Wraps invoking of the `v3/clusters/detail` with `GET` method

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
use openstack_sdk::api::block_storage::v3::cluster::list_detailed;
use structable_derive::StructTable;

/// Return a detailed list of all existing clusters.
///
/// Filter by is_up, disabled, num_hosts, and num_down_hosts.
///
#[derive(Args)]
pub struct ClustersCommand {
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
    /// The ID of active storage backend. Only in cinder-volume service.
    ///
    #[arg(help_heading = "Query parameters", long)]
    active_backend_id: Option<String>,

    /// Filter the cluster list result by binary name of the clustered
    /// services. One of cinder-api, cinder-scheduler, cinder-volume or
    /// cinder-backup.
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["cinder-api","cinder-backup","cinder-scheduler","cinder-volume"])]
    binary: Option<String>,

    /// Filter the cluster list result by status.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    disabled: Option<bool>,

    /// Whether the cluster is frozen or not.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    frozen: Option<bool>,

    /// Filter the cluster list result by state.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    is_up: Option<bool>,

    /// Filter the cluster list result by cluster name.
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// Filter the cluster list result by number of down hosts.
    ///
    #[arg(help_heading = "Query parameters", long)]
    num_down_hosts: Option<f32>,

    /// Filter the cluster list result by number of hosts.
    ///
    #[arg(help_heading = "Query parameters", long)]
    num_hosts: Option<f32>,

    /// Filter the cluster list result by replication status.
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["disabled","enabled"])]
    replication_stats: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Clusters response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of active storage backend. Only in `cinder-volume` service.
    ///
    /// **New in version 3.26**
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
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
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
    /// **New in version 3.26**
    ///
    #[serde()]
    #[structable(optional)]
    frozen: Option<bool>,

    /// The last periodic heartbeat received.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    ///
    #[serde()]
    #[structable(optional, wide)]
    last_heartbeat: Option<String>,

    /// The name of the service cluster.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The number of down hosts in the cluster.
    ///
    #[serde()]
    #[structable(optional, wide)]
    num_down_hosts: Option<i32>,

    /// The number of hosts in the cluster.
    ///
    #[serde()]
    #[structable(optional, wide)]
    num_hosts: Option<i32>,

    /// The cluster replication status. Only included in responses if
    /// configured. One of: `enabled` or `disabled`.
    ///
    #[serde()]
    #[structable(optional)]
    replication_status: Option<String>,

    /// The state of the cluster. One of `up` or `down`.
    ///
    #[serde()]
    #[structable(optional)]
    state: Option<String>,

    /// The status of the cluster. One of `enabled` or `disabled`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The date and time when the resource was updated.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC. In the previous example, the offset value is `-05:00`.
    ///
    /// If the `updated_at` date and time stamp is not set, its value is
    /// `null`.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl ClustersCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Clusters");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list_detailed::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.frozen {
            ep_builder.frozen(*val);
        }
        if let Some(val) = &self.query.active_backend_id {
            ep_builder.active_backend_id(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.binary {
            ep_builder.binary(val);
        }
        if let Some(val) = &self.query.is_up {
            ep_builder.is_up(*val);
        }
        if let Some(val) = &self.query.disabled {
            ep_builder.disabled(*val);
        }
        if let Some(val) = &self.query.num_hosts {
            ep_builder.num_hosts(*val);
        }
        if let Some(val) = &self.query.num_down_hosts {
            ep_builder.num_down_hosts(*val);
        }
        if let Some(val) = &self.query.replication_stats {
            ep_builder.replication_stats(val);
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
