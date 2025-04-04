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

//! List IpsecSiteConnections command
//!
//! Wraps invoking of the `v2.0/vpn/ipsec-site-connections` with `GET` method

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
use openstack_sdk::api::network::v2::vpn::ipsec_site_connection::list;
use openstack_sdk::api::{Pagination, paged};
use openstack_sdk::types::BoolString;
use openstack_sdk::types::IntString;
use serde_json::Value;
use structable_derive::StructTable;

/// Lists all IPsec connections.
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
#[command(about = "List IPsec connections")]
pub struct IpsecSiteConnectionsCommand {
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
/// IpsecSiteConnections response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<BoolString>,

    /// The authentication mode. A valid value is `psk`, which is the default.
    ///
    #[serde()]
    #[structable(optional, wide)]
    auth_mode: Option<String>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// A dictionary with dead peer detection (DPD) protocol controls.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dpd: Option<String>,

    /// The ID of the IPsec site-to-site connection.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the IKE policy.
    ///
    #[serde()]
    #[structable(optional, wide)]
    ikepolicy_id: Option<String>,

    /// Indicates whether this VPN can only respond to connections or both
    /// respond to and initiate connections. A valid value is `response- only`
    /// or `bi-directional`. Default is `bi-directional`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    initiator: Option<String>,

    /// The ID of the IPsec policy.
    ///
    #[serde()]
    #[structable(optional, wide)]
    ipsecpolicy_id: Option<String>,

    /// The ID for the endpoint group that contains private subnets for the
    /// local side of the connection. Yo must specify this parameter with the
    /// `peer_ep_group_id` parameter unless in backward- compatible mode where
    /// `peer_cidrs` is provided with a `subnet_id` for the VPN service.
    ///
    #[serde()]
    #[structable(optional, wide)]
    local_ep_group_id: Option<String>,

    /// An ID to be used instead of the external IP address for a virtual
    /// router used in traffic between instances on different networks in
    /// east-west traffic. Most often, local ID would be domain name, email
    /// address, etc. If this is not configured then the external IP address
    /// will be used as the ID.
    ///
    #[serde()]
    #[structable(optional, wide)]
    local_id: Option<String>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    ///
    #[serde()]
    #[structable(optional, wide)]
    mtu: Option<IntString>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The peer gateway public IPv4 or IPv6 address or FQDN.
    ///
    #[serde()]
    #[structable(optional, wide)]
    peer_address: Option<String>,

    /// (Deprecated) Unique list of valid peer private CIDRs in the form \<
    /// net_address > / < prefix > .
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    peer_cidrs: Option<Value>,

    /// The ID for the endpoint group that contains private CIDRs in the form
    /// \< net_address > / < prefix > for the peer side of the connection. You
    /// must specify this parameter with the `local_ep_group_id` parameter
    /// unless in backward-compatible mode where `peer_cidrs` is provided with
    /// a `subnet_id` for the VPN service.
    ///
    #[serde()]
    #[structable(optional, wide)]
    peer_ep_group_id: Option<String>,

    /// The peer router identity for authentication. A valid value is an IPv4
    /// address, IPv6 address, e-mail address, key ID, or FQDN. Typically, this
    /// value matches the `peer_address` value.
    ///
    #[serde()]
    #[structable(optional, wide)]
    peer_id: Option<String>,

    /// The pre-shared key. A valid value is any string.
    ///
    #[serde()]
    #[structable(optional, wide)]
    psk: Option<String>,

    /// The route mode. A valid value is `static`, which is the default.
    ///
    #[serde()]
    #[structable(optional, wide)]
    route_mode: Option<String>,

    /// Indicates whether the IPsec connection is currently operational. Values
    /// are `ACTIVE`, `DOWN`, `BUILD`, `ERROR`, `PENDING_CREATE`,
    /// `PENDING_UPDATE`, or `PENDING_DELETE`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The ID of the VPN service.
    ///
    #[serde()]
    #[structable(optional, wide)]
    vpnservice_id: Option<String>,
}

impl IpsecSiteConnectionsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List IpsecSiteConnections");

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
