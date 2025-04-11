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
//! Response type for the put vpn/ipsecpolicies/{id} operation

use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// Ipsecpolicy response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct IpsecpolicyResponse {
    /// The authentication hash algorithm. Valid values are `sha1`, `sha256`,
    /// `sha384`, `sha512`, `aes-xcbc`, `aes-cmac`. The default is `sha1`.
    ///
    #[structable(optional, serialize)]
    pub auth_algorithm: Option<AuthAlgorithm>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// The encapsulation mode. A valid value is `tunnel` or `transport`.
    /// Default is `tunnel`.
    ///
    #[structable(optional, serialize)]
    pub encapsulation_mode: Option<EncapsulationMode>,

    /// The encryption algorithm. A valid value is `3des`, `aes-128`,
    /// `aes-192`, `aes-256`, `aes-128-ctr`, `aes-192-ctr`, `aes-256-ctr`.
    /// Additional values for AES CCM and GCM modes are defined (e.g.
    /// `aes-256-ccm-16`, `aes-256-gcm-16`) for all combinations of key length
    /// 128, 192, 256 bits and ICV length 8, 12, 16 octets. Default is
    /// `aes-128`.
    ///
    #[structable(optional, serialize)]
    pub encryption_algorithm: Option<EncryptionAlgorithm>,

    /// The ID of the IPsec policy.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The lifetime of the security association. The lifetime consists of a
    /// unit and integer value. You can omit either the unit or value portion
    /// of the lifetime. Default unit is seconds and default value is 3600.
    ///
    #[structable(optional)]
    pub lifetime: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// Perfect forward secrecy (PFS). A valid value is `Group2`, `Group5`,
    /// `Group14` to `Group31`. Default is `Group5`.
    ///
    #[structable(optional, serialize)]
    pub pfs: Option<Pfs>,

    /// The ID of the project.
    ///
    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// The transform protocol. A valid value is `ESP`, `AH`, or `AH- ESP`.
    /// Default is `ESP`.
    ///
    #[structable(optional, serialize)]
    pub transform_protocol: Option<TransformProtocol>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum EncryptionAlgorithm {
    // _3des
    #[serde(rename = "3des")]
    _3des,

    // Aes128
    #[serde(rename = "aes-128")]
    Aes128,

    // Aes128Ccm12
    #[serde(rename = "aes-128-ccm-12")]
    Aes128Ccm12,

    // Aes128Ccm16
    #[serde(rename = "aes-128-ccm-16")]
    Aes128Ccm16,

    // Aes128Ccm8
    #[serde(rename = "aes-128-ccm-8")]
    Aes128Ccm8,

    // Aes128Ctr
    #[serde(rename = "aes-128-ctr")]
    Aes128Ctr,

    // Aes128Gcm12
    #[serde(rename = "aes-128-gcm-12")]
    Aes128Gcm12,

    // Aes128Gcm16
    #[serde(rename = "aes-128-gcm-16")]
    Aes128Gcm16,

    // Aes128Gcm8
    #[serde(rename = "aes-128-gcm-8")]
    Aes128Gcm8,

    // Aes192
    #[serde(rename = "aes-192")]
    Aes192,

    // Aes192Ccm12
    #[serde(rename = "aes-192-ccm-12")]
    Aes192Ccm12,

    // Aes192Ccm16
    #[serde(rename = "aes-192-ccm-16")]
    Aes192Ccm16,

    // Aes192Ccm8
    #[serde(rename = "aes-192-ccm-8")]
    Aes192Ccm8,

    // Aes192Ctr
    #[serde(rename = "aes-192-ctr")]
    Aes192Ctr,

    // Aes192Gcm12
    #[serde(rename = "aes-192-gcm-12")]
    Aes192Gcm12,

    // Aes192Gcm16
    #[serde(rename = "aes-192-gcm-16")]
    Aes192Gcm16,

    // Aes192Gcm8
    #[serde(rename = "aes-192-gcm-8")]
    Aes192Gcm8,

    // Aes256
    #[serde(rename = "aes-256")]
    Aes256,

    // Aes256Ccm12
    #[serde(rename = "aes-256-ccm-12")]
    Aes256Ccm12,

    // Aes256Ccm16
    #[serde(rename = "aes-256-ccm-16")]
    Aes256Ccm16,

    // Aes256Ccm8
    #[serde(rename = "aes-256-ccm-8")]
    Aes256Ccm8,

    // Aes256Ctr
    #[serde(rename = "aes-256-ctr")]
    Aes256Ctr,

    // Aes256Gcm12
    #[serde(rename = "aes-256-gcm-12")]
    Aes256Gcm12,

    // Aes256Gcm16
    #[serde(rename = "aes-256-gcm-16")]
    Aes256Gcm16,

    // Aes256Gcm8
    #[serde(rename = "aes-256-gcm-8")]
    Aes256Gcm8,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum TransformProtocol {
    // Ah
    #[serde(rename = "ah")]
    Ah,

    // AhEsp
    #[serde(rename = "ah-esp")]
    AhEsp,

    // Esp
    #[serde(rename = "esp")]
    Esp,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum AuthAlgorithm {
    // AesCmac
    #[serde(rename = "aes-cmac")]
    AesCmac,

    // AesXcbc
    #[serde(rename = "aes-xcbc")]
    AesXcbc,

    // Sha1
    #[serde(rename = "sha1")]
    Sha1,

    // Sha256
    #[serde(rename = "sha256")]
    Sha256,

    // Sha384
    #[serde(rename = "sha384")]
    Sha384,

    // Sha512
    #[serde(rename = "sha512")]
    Sha512,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum EncapsulationMode {
    // Transport
    #[serde(rename = "transport")]
    Transport,

    // Tunnel
    #[serde(rename = "tunnel")]
    Tunnel,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Pfs {
    // Group14
    #[serde(rename = "group14")]
    Group14,

    // Group15
    #[serde(rename = "group15")]
    Group15,

    // Group16
    #[serde(rename = "group16")]
    Group16,

    // Group17
    #[serde(rename = "group17")]
    Group17,

    // Group18
    #[serde(rename = "group18")]
    Group18,

    // Group19
    #[serde(rename = "group19")]
    Group19,

    // Group2
    #[serde(rename = "group2")]
    Group2,

    // Group20
    #[serde(rename = "group20")]
    Group20,

    // Group21
    #[serde(rename = "group21")]
    Group21,

    // Group22
    #[serde(rename = "group22")]
    Group22,

    // Group23
    #[serde(rename = "group23")]
    Group23,

    // Group24
    #[serde(rename = "group24")]
    Group24,

    // Group25
    #[serde(rename = "group25")]
    Group25,

    // Group26
    #[serde(rename = "group26")]
    Group26,

    // Group27
    #[serde(rename = "group27")]
    Group27,

    // Group28
    #[serde(rename = "group28")]
    Group28,

    // Group29
    #[serde(rename = "group29")]
    Group29,

    // Group30
    #[serde(rename = "group30")]
    Group30,

    // Group31
    #[serde(rename = "group31")]
    Group31,

    // Group5
    #[serde(rename = "group5")]
    Group5,
}
