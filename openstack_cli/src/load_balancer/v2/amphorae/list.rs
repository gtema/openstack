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

//! List Amphoraes command
//!
//! Wraps invoking of the `v2/octavia/amphorae` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::load_balancer::v2::amphorae::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists all amphora for the project.
///
/// If you are not an administrative user, the service returns the HTTP
/// `Forbidden (403)` response code.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. Additionally, you can filter results by using query
/// string parameters. For information, see
/// [Filtering and column selection](#filtering).
///
/// The list might be empty.
///
#[derive(Args)]
#[command(about = "List Amphora")]
pub struct AmphoraesCommand {
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
    cached_zone: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    cert_busy: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    cert_expiration: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    compute_flavor: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    compute_id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    created_at: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    ha_ip: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    ha_port_id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    image_id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    lb_network_ip: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    loadbalancer_id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    role: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    status: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    updated_at: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    vrrp_id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    vrrp_interface: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    vrrp_ip: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    vrrp_port_id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    vrrp_priority: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Amphoraes response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The availability zone of a compute instance, cached at create time.
    /// This is not guaranteed to be current. May be an empty-string if the
    /// compute service does not use zones.
    ///
    #[serde()]
    #[structable(optional, wide)]
    cached_zone: Option<String>,

    /// Whether the certificate is in the process of being replaced.
    ///
    #[serde()]
    #[structable(optional, wide)]
    cert_busy: Option<bool>,

    /// The date the certificate for the amphora expires.
    ///
    #[serde()]
    #[structable(optional, wide)]
    cert_expiration: Option<String>,

    /// The ID of the compute flavor used for the amphora.
    ///
    /// **New in version 2.3**
    ///
    #[serde()]
    #[structable(optional, wide)]
    compute_flavor: Option<String>,

    /// The ID of the amphora resource in the compute system.
    ///
    #[serde()]
    #[structable(optional, wide)]
    compute_id: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The IP address of the Virtual IP (VIP).
    ///
    #[serde()]
    #[structable(optional, wide)]
    ha_ip: Option<String>,

    /// The ID of the Virtual IP (VIP) port.
    ///
    #[serde()]
    #[structable(optional, wide)]
    ha_port_id: Option<String>,

    /// The associated amphora ID.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the glance image used for the amphora.
    ///
    /// **New in version 2.1**
    ///
    #[serde()]
    #[structable(optional, wide)]
    image_id: Option<String>,

    /// The management IP of the amphora.
    ///
    #[serde()]
    #[structable(optional, wide)]
    lb_network_ip: Option<String>,

    /// The ID of the load balancer.
    ///
    #[serde()]
    #[structable(optional, wide)]
    loadbalancer_id: Option<String>,

    /// The role of the amphora. One of `STANDALONE`, `MASTER`, `BACKUP`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    role: Option<String>,

    /// The status of the amphora. One of: `BOOTING`, `ALLOCATED`, `READY`,
    /// `PENDING_CREATE`, `PENDING_DELETE`, `DELETED`, `ERROR`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The vrrp group’s ID for the amphora.
    ///
    #[serde()]
    #[structable(optional, wide)]
    vrrp_id: Option<i32>,

    /// The bound interface name of the vrrp port on the amphora.
    ///
    #[serde()]
    #[structable(optional, wide)]
    vrrp_interface: Option<String>,

    /// The address of the vrrp port on the amphora.
    ///
    #[serde()]
    #[structable(optional, wide)]
    vrrp_ip: Option<String>,

    /// The vrrp port’s ID in the networking system.
    ///
    #[serde()]
    #[structable(optional, wide)]
    vrrp_port_id: Option<String>,

    /// The priority of the amphora in the vrrp group.
    ///
    #[serde()]
    #[structable(optional, wide)]
    vrrp_priority: Option<i32>,
}

impl AmphoraesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Amphoraes");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.loadbalancer_id {
            ep_builder.loadbalancer_id(val);
        }
        if let Some(val) = &self.query.compute_id {
            ep_builder.compute_id(val);
        }
        if let Some(val) = &self.query.lb_network_ip {
            ep_builder.lb_network_ip(val);
        }
        if let Some(val) = &self.query.vrrp_ip {
            ep_builder.vrrp_ip(val);
        }
        if let Some(val) = &self.query.ha_ip {
            ep_builder.ha_ip(val);
        }
        if let Some(val) = &self.query.vrrp_port_id {
            ep_builder.vrrp_port_id(val);
        }
        if let Some(val) = &self.query.ha_port_id {
            ep_builder.ha_port_id(val);
        }
        if let Some(val) = &self.query.cert_expiration {
            ep_builder.cert_expiration(val);
        }
        if let Some(val) = &self.query.cert_busy {
            ep_builder.cert_busy(val);
        }
        if let Some(val) = &self.query.role {
            ep_builder.role(val);
        }
        if let Some(val) = &self.query.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.query.vrrp_interface {
            ep_builder.vrrp_interface(val);
        }
        if let Some(val) = &self.query.vrrp_id {
            ep_builder.vrrp_id(val);
        }
        if let Some(val) = &self.query.vrrp_priority {
            ep_builder.vrrp_priority(val);
        }
        if let Some(val) = &self.query.cached_zone {
            ep_builder.cached_zone(val);
        }
        if let Some(val) = &self.query.created_at {
            ep_builder.created_at(val);
        }
        if let Some(val) = &self.query.updated_at {
            ep_builder.updated_at(val);
        }
        if let Some(val) = &self.query.image_id {
            ep_builder.image_id(val);
        }
        if let Some(val) = &self.query.compute_flavor {
            ep_builder.compute_flavor(val);
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
