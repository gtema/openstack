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

//! List Vpnservices command
//!
//! Wraps invoking of the `v2.0/vpn/vpnservices` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::BoolString;
use openstack_sdk::api::network::v2::vpn::vpnservice::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use structable_derive::StructTable;

/// Lists all VPN services.
///
/// The list might be empty.
///
/// Standard query parameters are supported on the URI. For more information,
/// see [Filtering and Column Selection](#filtering).
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. For more information, see [Fields](#fields).
///
/// Pagination query parameters are supported if Neutron configuration supports
/// it by overriding `allow_pagination=false`. For more information, see
/// [Pagination](#pagination).
///
/// Sorting query parameters are supported if Neutron configuration supports it
/// with `allow_sorting=true`. For more information, see [Sorting](#sorting).
///
/// Normal response codes: 200
///
/// Error response codes: 401, 403
///
#[derive(Args)]
#[command(about = "List VPN services")]
pub struct VpnservicesCommand {
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
    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    ///
    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    ///
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// Reverse the page direction
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_dir: Option<Vec<String>>,

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_key: Option<Vec<String>>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Vpnservices response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<BoolString>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// Read-only external (public) IPv4 address that is used for the VPN
    /// service. The VPN plugin sets this address if an IPv4 interface is
    /// available.
    ///
    #[serde()]
    #[structable(optional, wide)]
    external_v4_ip: Option<String>,

    /// Read-only external (public) IPv6 address that is used for the VPN
    /// service. The VPN plugin sets this address if an IPv6 interface is
    /// available.
    ///
    #[serde()]
    #[structable(optional, wide)]
    external_v6_ip: Option<String>,

    /// The ID of the flavor.
    ///
    #[serde()]
    #[structable(optional, wide)]
    flavor_id: Option<String>,

    /// The ID of the VPN service.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    router_id: Option<String>,

    /// Indicates whether IPsec VPN service is currently operational. Values
    /// are `ACTIVE`, `DOWN`, `BUILD`, `ERROR`, `PENDING_CREATE`,
    /// `PENDING_UPDATE`, or `PENDING_DELETE`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// If you specify only a subnet UUID, OpenStack Networking allocates an
    /// available IP from that subnet to the port. If you specify both a subnet
    /// UUID and an IP address, OpenStack Networking tries to allocate the
    /// address to the port.
    ///
    #[serde()]
    #[structable(optional, wide)]
    subnet_id: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,
}

impl VpnservicesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Vpnservices");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val.iter());
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val.iter());
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.page_reverse {
            ep_builder.page_reverse(*val);
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
