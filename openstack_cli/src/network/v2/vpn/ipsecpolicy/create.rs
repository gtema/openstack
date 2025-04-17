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

//! Create Ipsecpolicy command
//!
//! Wraps invoking of the `v2.0/vpn/ipsecpolicies` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::vpn::ipsecpolicy::create;
use openstack_types::network::v2::vpn::ipsecpolicy::response::create::IpsecpolicyResponse;

/// Creates an IP security (IPsec) policy.
///
/// The IPsec policy specifies the authentication and encryption algorithms and
/// encapsulation mode to use for the established VPN connection.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401
#[derive(Args)]
#[command(about = "Create IPsec policy")]
pub struct IpsecpolicyCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// An `ipsecpolicy` object.
    #[command(flatten)]
    ipsecpolicy: Ipsecpolicy,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum EncryptionAlgorithm {
    _3des,
    Aes128,
    Aes128Ccm12,
    Aes128Ccm16,
    Aes128Ccm8,
    Aes128Ctr,
    Aes128Gcm12,
    Aes128Gcm16,
    Aes128Gcm8,
    Aes192,
    Aes192Ccm12,
    Aes192Ccm16,
    Aes192Ccm8,
    Aes192Ctr,
    Aes192Gcm12,
    Aes192Gcm16,
    Aes192Gcm8,
    Aes256,
    Aes256Ccm12,
    Aes256Ccm16,
    Aes256Ccm8,
    Aes256Ctr,
    Aes256Gcm12,
    Aes256Gcm16,
    Aes256Gcm8,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum TransformProtocol {
    Ah,
    AhEsp,
    Esp,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum AuthAlgorithm {
    AesCmac,
    AesXcbc,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum EncapsulationMode {
    Transport,
    Tunnel,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Pfs {
    Group14,
    Group15,
    Group16,
    Group17,
    Group18,
    Group19,
    Group2,
    Group20,
    Group21,
    Group22,
    Group23,
    Group24,
    Group25,
    Group26,
    Group27,
    Group28,
    Group29,
    Group30,
    Group31,
    Group5,
}

/// Ipsecpolicy Body data
#[derive(Args, Clone)]
struct Ipsecpolicy {
    /// The authentication hash algorithm. Valid values are `sha1`, `sha256`,
    /// `sha384`, `sha512`, `aes-xcbc`, `aes-cmac`. The default is `sha1`.
    #[arg(help_heading = "Body parameters", long)]
    auth_algorithm: Option<AuthAlgorithm>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The encapsulation mode. A valid value is `tunnel` or `transport`.
    /// Default is `tunnel`.
    #[arg(help_heading = "Body parameters", long)]
    encapsulation_mode: Option<EncapsulationMode>,

    /// The encryption algorithm. A valid value is `3des`, `aes-128`,
    /// `aes-192`, `aes-256`, `aes-128-ctr`, `aes-192-ctr`, `aes-256-ctr`.
    /// Additional values for AES CCM and GCM modes are defined (e.g.
    /// `aes-256-ccm-16`, `aes-256-gcm-16`) for all combinations of key length
    /// 128, 192, 256 bits and ICV length 8, 12, 16 octets. Default is
    /// `aes-128`.
    #[arg(help_heading = "Body parameters", long)]
    encryption_algorithm: Option<EncryptionAlgorithm>,

    /// The lifetime of the security association. The lifetime consists of a
    /// unit and integer value. You can omit either the unit or value portion
    /// of the lifetime. Default unit is seconds and default value is 3600.
    #[arg(help_heading = "Body parameters", long)]
    lifetime: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// Perfect forward secrecy (PFS). A valid value is `Group2`, `Group5`,
    /// `Group14` to `Group31`. Default is `Group5`.
    #[arg(help_heading = "Body parameters", long)]
    pfs: Option<Pfs>,

    /// The ID of the project.
    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,

    /// The transform protocol. A valid value is `ESP`, `AH`, or `AH- ESP`.
    /// Default is `ESP`.
    #[arg(help_heading = "Body parameters", long)]
    transform_protocol: Option<TransformProtocol>,
}

impl IpsecpolicyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Ipsecpolicy");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.ipsecpolicy data
        let args = &self.ipsecpolicy;
        let mut ipsecpolicy_builder = create::IpsecpolicyBuilder::default();
        if let Some(val) = &args.encryption_algorithm {
            let tmp = match val {
                EncryptionAlgorithm::_3des => create::EncryptionAlgorithm::_3des,
                EncryptionAlgorithm::Aes128 => create::EncryptionAlgorithm::Aes128,
                EncryptionAlgorithm::Aes128Ccm12 => create::EncryptionAlgorithm::Aes128Ccm12,
                EncryptionAlgorithm::Aes128Ccm16 => create::EncryptionAlgorithm::Aes128Ccm16,
                EncryptionAlgorithm::Aes128Ccm8 => create::EncryptionAlgorithm::Aes128Ccm8,
                EncryptionAlgorithm::Aes128Ctr => create::EncryptionAlgorithm::Aes128Ctr,
                EncryptionAlgorithm::Aes128Gcm12 => create::EncryptionAlgorithm::Aes128Gcm12,
                EncryptionAlgorithm::Aes128Gcm16 => create::EncryptionAlgorithm::Aes128Gcm16,
                EncryptionAlgorithm::Aes128Gcm8 => create::EncryptionAlgorithm::Aes128Gcm8,
                EncryptionAlgorithm::Aes192 => create::EncryptionAlgorithm::Aes192,
                EncryptionAlgorithm::Aes192Ccm12 => create::EncryptionAlgorithm::Aes192Ccm12,
                EncryptionAlgorithm::Aes192Ccm16 => create::EncryptionAlgorithm::Aes192Ccm16,
                EncryptionAlgorithm::Aes192Ccm8 => create::EncryptionAlgorithm::Aes192Ccm8,
                EncryptionAlgorithm::Aes192Ctr => create::EncryptionAlgorithm::Aes192Ctr,
                EncryptionAlgorithm::Aes192Gcm12 => create::EncryptionAlgorithm::Aes192Gcm12,
                EncryptionAlgorithm::Aes192Gcm16 => create::EncryptionAlgorithm::Aes192Gcm16,
                EncryptionAlgorithm::Aes192Gcm8 => create::EncryptionAlgorithm::Aes192Gcm8,
                EncryptionAlgorithm::Aes256 => create::EncryptionAlgorithm::Aes256,
                EncryptionAlgorithm::Aes256Ccm12 => create::EncryptionAlgorithm::Aes256Ccm12,
                EncryptionAlgorithm::Aes256Ccm16 => create::EncryptionAlgorithm::Aes256Ccm16,
                EncryptionAlgorithm::Aes256Ccm8 => create::EncryptionAlgorithm::Aes256Ccm8,
                EncryptionAlgorithm::Aes256Ctr => create::EncryptionAlgorithm::Aes256Ctr,
                EncryptionAlgorithm::Aes256Gcm12 => create::EncryptionAlgorithm::Aes256Gcm12,
                EncryptionAlgorithm::Aes256Gcm16 => create::EncryptionAlgorithm::Aes256Gcm16,
                EncryptionAlgorithm::Aes256Gcm8 => create::EncryptionAlgorithm::Aes256Gcm8,
            };
            ipsecpolicy_builder.encryption_algorithm(tmp);
        }

        if let Some(val) = &args.tenant_id {
            ipsecpolicy_builder.tenant_id(val);
        }

        if let Some(val) = &args.name {
            ipsecpolicy_builder.name(val);
        }

        if let Some(val) = &args.description {
            ipsecpolicy_builder.description(val);
        }

        if let Some(val) = &args.transform_protocol {
            let tmp = match val {
                TransformProtocol::Ah => create::TransformProtocol::Ah,
                TransformProtocol::AhEsp => create::TransformProtocol::AhEsp,
                TransformProtocol::Esp => create::TransformProtocol::Esp,
            };
            ipsecpolicy_builder.transform_protocol(tmp);
        }

        if let Some(val) = &args.auth_algorithm {
            let tmp = match val {
                AuthAlgorithm::AesCmac => create::AuthAlgorithm::AesCmac,
                AuthAlgorithm::AesXcbc => create::AuthAlgorithm::AesXcbc,
                AuthAlgorithm::Sha1 => create::AuthAlgorithm::Sha1,
                AuthAlgorithm::Sha256 => create::AuthAlgorithm::Sha256,
                AuthAlgorithm::Sha384 => create::AuthAlgorithm::Sha384,
                AuthAlgorithm::Sha512 => create::AuthAlgorithm::Sha512,
            };
            ipsecpolicy_builder.auth_algorithm(tmp);
        }

        if let Some(val) = &args.encapsulation_mode {
            let tmp = match val {
                EncapsulationMode::Transport => create::EncapsulationMode::Transport,
                EncapsulationMode::Tunnel => create::EncapsulationMode::Tunnel,
            };
            ipsecpolicy_builder.encapsulation_mode(tmp);
        }

        if let Some(val) = &args.lifetime {
            ipsecpolicy_builder.lifetime(val);
        }

        if let Some(val) = &args.pfs {
            let tmp = match val {
                Pfs::Group14 => create::Pfs::Group14,
                Pfs::Group15 => create::Pfs::Group15,
                Pfs::Group16 => create::Pfs::Group16,
                Pfs::Group17 => create::Pfs::Group17,
                Pfs::Group18 => create::Pfs::Group18,
                Pfs::Group19 => create::Pfs::Group19,
                Pfs::Group2 => create::Pfs::Group2,
                Pfs::Group20 => create::Pfs::Group20,
                Pfs::Group21 => create::Pfs::Group21,
                Pfs::Group22 => create::Pfs::Group22,
                Pfs::Group23 => create::Pfs::Group23,
                Pfs::Group24 => create::Pfs::Group24,
                Pfs::Group25 => create::Pfs::Group25,
                Pfs::Group26 => create::Pfs::Group26,
                Pfs::Group27 => create::Pfs::Group27,
                Pfs::Group28 => create::Pfs::Group28,
                Pfs::Group29 => create::Pfs::Group29,
                Pfs::Group30 => create::Pfs::Group30,
                Pfs::Group31 => create::Pfs::Group31,
                Pfs::Group5 => create::Pfs::Group5,
            };
            ipsecpolicy_builder.pfs(tmp);
        }

        ep_builder.ipsecpolicy(ipsecpolicy_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<IpsecpolicyResponse>(data)?;
        Ok(())
    }
}
