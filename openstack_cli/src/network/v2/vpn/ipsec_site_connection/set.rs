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

//! Set IpsecSiteConnection command
//!
//! Wraps invoking of the `v2.0/vpn/ipsec-site-connections/{id}` with `PUT` method

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
use crate::common::IntString;
use clap::ValueEnum;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::vpn::ipsec_site_connection::find;
use openstack_sdk::api::network::v2::vpn::ipsec_site_connection::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates connection settings for an IPsec connection.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 404
///
#[derive(Args)]
#[command(about = "Update IPsec connection")]
pub struct IpsecSiteConnectionCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// An `ipsec_site_connection` object.
    ///
    #[command(flatten)]
    ipsec_site_connection: IpsecSiteConnection,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/vpn/ipsec-site-connections/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Initiator {
    BiDirectional,
    ResponseOnly,
}

/// IpsecSiteConnection Body data
#[derive(Args, Clone)]
struct IpsecSiteConnection {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    admin_state_up: Option<bool>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// A dictionary with dead peer detection (DPD) protocol controls.
    ///
    #[arg(help_heading = "Body parameters", long)]
    dpd: Option<String>,

    /// Indicates whether this VPN can only respond to connections or both
    /// respond to and initiate connections. A valid value is `response- only`
    /// or `bi-directional`. Default is `bi-directional`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    initiator: Option<Initiator>,

    /// The ID for the endpoint group that contains private subnets for the
    /// local side of the connection. Yo must specify this parameter with the
    /// `peer_ep_group_id` parameter unless in backward- compatible mode where
    /// `peer_cidrs` is provided with a `subnet_id` for the VPN service.
    ///
    #[arg(help_heading = "Body parameters", long)]
    local_ep_group_id: Option<String>,

    /// An ID to be used instead of the external IP address for a virtual
    /// router used in traffic between instances on different networks in
    /// east-west traffic. Most often, local ID would be domain name, email
    /// address, etc. If this is not configured then the external IP address
    /// will be used as the ID.
    ///
    #[arg(help_heading = "Body parameters", long)]
    local_id: Option<String>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    ///
    #[arg(help_heading = "Body parameters", long)]
    mtu: Option<i32>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The peer gateway public IPv4 or IPv6 address or FQDN.
    ///
    #[arg(help_heading = "Body parameters", long)]
    peer_address: Option<String>,

    /// (Deprecated) Unique list of valid peer private CIDRs in the form \<
    /// net_address > / < prefix > .
    ///
    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    peer_cidrs: Option<Vec<String>>,

    /// The ID for the endpoint group that contains private CIDRs in the form
    /// \< net_address > / < prefix > for the peer side of the connection. You
    /// must specify this parameter with the `local_ep_group_id` parameter
    /// unless in backward-compatible mode where `peer_cidrs` is provided with
    /// a `subnet_id` for the VPN service.
    ///
    #[arg(help_heading = "Body parameters", long)]
    peer_ep_group_id: Option<String>,

    /// The peer router identity for authentication. A valid value is an IPv4
    /// address, IPv6 address, e-mail address, key ID, or FQDN. Typically, this
    /// value matches the `peer_address` value.
    ///
    #[arg(help_heading = "Body parameters", long)]
    peer_id: Option<String>,

    /// The pre-shared key. A valid value is any string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    psk: Option<String>,
}

/// IpsecSiteConnection response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<BoolString>,

    /// The authentication mode. A valid value is `psk`, which is the default.
    ///
    #[serde()]
    #[structable(optional)]
    auth_mode: Option<String>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// A dictionary with dead peer detection (DPD) protocol controls.
    ///
    #[serde()]
    #[structable(optional)]
    dpd: Option<String>,

    /// The ID of the IPsec site-to-site connection.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the IKE policy.
    ///
    #[serde()]
    #[structable(optional)]
    ikepolicy_id: Option<String>,

    /// Indicates whether this VPN can only respond to connections or both
    /// respond to and initiate connections. A valid value is `response- only`
    /// or `bi-directional`. Default is `bi-directional`.
    ///
    #[serde()]
    #[structable(optional)]
    initiator: Option<String>,

    /// The ID of the IPsec policy.
    ///
    #[serde()]
    #[structable(optional)]
    ipsecpolicy_id: Option<String>,

    /// The ID for the endpoint group that contains private subnets for the
    /// local side of the connection. Yo must specify this parameter with the
    /// `peer_ep_group_id` parameter unless in backward- compatible mode where
    /// `peer_cidrs` is provided with a `subnet_id` for the VPN service.
    ///
    #[serde()]
    #[structable(optional)]
    local_ep_group_id: Option<String>,

    /// An ID to be used instead of the external IP address for a virtual
    /// router used in traffic between instances on different networks in
    /// east-west traffic. Most often, local ID would be domain name, email
    /// address, etc. If this is not configured then the external IP address
    /// will be used as the ID.
    ///
    #[serde()]
    #[structable(optional)]
    local_id: Option<String>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    ///
    #[serde()]
    #[structable(optional)]
    mtu: Option<IntString>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The peer gateway public IPv4 or IPv6 address or FQDN.
    ///
    #[serde()]
    #[structable(optional)]
    peer_address: Option<String>,

    /// (Deprecated) Unique list of valid peer private CIDRs in the form \<
    /// net_address > / < prefix > .
    ///
    #[serde()]
    #[structable(optional, pretty)]
    peer_cidrs: Option<Value>,

    /// The ID for the endpoint group that contains private CIDRs in the form
    /// \< net_address > / < prefix > for the peer side of the connection. You
    /// must specify this parameter with the `local_ep_group_id` parameter
    /// unless in backward-compatible mode where `peer_cidrs` is provided with
    /// a `subnet_id` for the VPN service.
    ///
    #[serde()]
    #[structable(optional)]
    peer_ep_group_id: Option<String>,

    /// The peer router identity for authentication. A valid value is an IPv4
    /// address, IPv6 address, e-mail address, key ID, or FQDN. Typically, this
    /// value matches the `peer_address` value.
    ///
    #[serde()]
    #[structable(optional)]
    peer_id: Option<String>,

    /// The pre-shared key. A valid value is any string.
    ///
    #[serde()]
    #[structable(optional)]
    psk: Option<String>,

    /// The route mode. A valid value is `static`, which is the default.
    ///
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The ID of the VPN service.
    ///
    #[serde()]
    #[structable(optional)]
    vpnservice_id: Option<String>,
}

impl IpsecSiteConnectionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set IpsecSiteConnection");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.ipsec_site_connection data
        let args = &self.ipsec_site_connection;
        let mut ipsec_site_connection_builder = set::IpsecSiteConnectionBuilder::default();
        if let Some(val) = &args.name {
            ipsec_site_connection_builder.name(val);
        }

        if let Some(val) = &args.description {
            ipsec_site_connection_builder.description(val);
        }

        if let Some(val) = &args.local_id {
            ipsec_site_connection_builder.local_id(val);
        }

        if let Some(val) = &args.peer_address {
            ipsec_site_connection_builder.peer_address(val);
        }

        if let Some(val) = &args.peer_id {
            ipsec_site_connection_builder.peer_id(val);
        }

        if let Some(val) = &args.peer_cidrs {
            ipsec_site_connection_builder
                .peer_cidrs(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.local_ep_group_id {
            ipsec_site_connection_builder.local_ep_group_id(Some(val.into()));
        }

        if let Some(val) = &args.peer_ep_group_id {
            ipsec_site_connection_builder.peer_ep_group_id(Some(val.into()));
        }

        if let Some(val) = &args.mtu {
            ipsec_site_connection_builder.mtu(*val);
        }

        if let Some(val) = &args.initiator {
            let tmp = match val {
                Initiator::BiDirectional => set::Initiator::BiDirectional,
                Initiator::ResponseOnly => set::Initiator::ResponseOnly,
            };
            ipsec_site_connection_builder.initiator(tmp);
        }

        if let Some(val) = &args.psk {
            ipsec_site_connection_builder.psk(val);
        }

        if let Some(val) = &args.dpd {
            ipsec_site_connection_builder.dpd(val);
        }

        if let Some(val) = &args.admin_state_up {
            ipsec_site_connection_builder.admin_state_up(*val);
        }

        ep_builder.ipsec_site_connection(ipsec_site_connection_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
