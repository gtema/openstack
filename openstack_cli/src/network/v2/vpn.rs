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

//! VPN-as-a-service commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod endpoint_group;
pub mod ikepolicy;
pub mod ipsec_site_connection;
pub mod ipsecpolicy;
pub mod vpnservice;

/// VPNaaS 2.0 (vpn, vpnservices, ikepolicies, ipsecpolicies, endpoint-groups,
/// ipsec-site-connections)
///
/// The Virtual-Private-Network-as-a-Service (VPNaaS) extension enables OpenStack projects to
/// extend private networks across the public telecommunication infrastructure.
///
/// This initial implementation of the VPNaaS extension provides:
///
///   - Site-to-site VPN that connects two private networks.
///
///   - Multiple VPN connections per project.
///
///   - IKEv1 policy support with 3des, aes-128, aes-256, or aes-192 encryption.
///
///   - IPsec policy support with 3des, aes-128, aes-192, or aes-256 encryption, sha1
///     authentication, ESP, AH, or AH-ESP transform protocol, and tunnel or transport mode
///     encapsulation.
///
///   - Dead Peer Detection (DPD) with hold, clear, restart, disabled, or restart-by-peer actions.
///
/// This extension introduces these resources:
///
///   - service. A parent object that associates VPN with a specific subnet and router.
///
///   - ikepolicy. The Internet Key Exchange (IKE) policy that identifies the authentication and
///     encryption algorithm to use during phase one and two negotiation of a VPN connection.
///
///   - ipsecpolicy. The IP security policy that specifies the authentication and encryption
///     algorithm and encapsulation mode to use for the established VPN connection.
///
///   - ipsec-site-connection. Details for the site-to-site IPsec connection, including the peer
///     CIDRs, MTU, authentication mode, peer address, DPD settings, and status.
///
/// VPN Endpoint Groups
///
/// The endpoint-groups extension adds support for defining one or more endpoints of a specific
/// type, and can be used to specify both local and peer endpoints for IPsec connections.
///
/// VPN Flavors
///
/// The vpn-flavors extension adds the flavor_id attribute to vpnservices resources. During
/// vpnservice creation, if a flavor_id is passed, it is used to find the provider for the driver
/// which would handle the newly created vpnservice.
#[derive(Parser)]
pub struct VpnCommand {
    /// subcommand
    #[command(subcommand)]
    command: VpnCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum VpnCommands {
    EndpointGroup(Box<endpoint_group::EndpointGroupCommand>),
    Ikepolicy(ikepolicy::IkepolicyCommand),
    IpsecSiteConnection(Box<ipsec_site_connection::IpsecSiteConnectionCommand>),
    Ipsecpolicy(Box<ipsecpolicy::IpsecpolicyCommand>),
    Vpnservice(Box<vpnservice::VpnserviceCommand>),
}

impl VpnCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            VpnCommands::EndpointGroup(cmd) => cmd.take_action(parsed_args, session).await,
            VpnCommands::Ikepolicy(cmd) => cmd.take_action(parsed_args, session).await,
            VpnCommands::IpsecSiteConnection(cmd) => cmd.take_action(parsed_args, session).await,
            VpnCommands::Ipsecpolicy(cmd) => cmd.take_action(parsed_args, session).await,
            VpnCommands::Vpnservice(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
