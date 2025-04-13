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
//! Response type for the get lbaas/listeners/{listener_id} operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Listener response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ListenerResponse {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[structable(optional)]
    pub admin_state_up: Option<bool>,

    /// A list of IPv4, IPv6 or mix of both CIDRs.
    ///
    /// **New in version 2.12**
    ///
    #[structable(optional, serialize)]
    pub allowed_cidrs: Option<Vec<String>>,

    /// A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2
    ///
    /// **New in version 2.20**
    ///
    #[structable(optional, serialize)]
    pub alpn_protocols: Option<Vec<String>>,

    /// The TLS client authentication mode. One of the options `NONE`,
    /// `OPTIONAL` or `MANDATORY`.
    ///
    /// **New in version 2.8**
    ///
    #[structable(optional)]
    pub client_authentication: Option<String>,

    /// The ref of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format client CA certificate bundle for
    /// `TERMINATED_HTTPS` listeners.
    ///
    /// **New in version 2.8**
    ///
    #[structable(optional)]
    pub client_ca_tls_container_ref: Option<String>,

    /// The URI of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA revocation list file for
    /// `TERMINATED_HTTPS` listeners.
    ///
    /// **New in version 2.8**
    ///
    #[structable(optional)]
    pub client_crl_container_ref: Option<String>,

    /// The maximum number of connections permitted for this listener. Default
    /// value is -1 which represents infinite connections or a default value
    /// defined by the provider driver.
    ///
    #[structable(optional)]
    pub connection_limit: Option<i32>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// The ID of the pool used by the listener if no L7 policies match. The
    /// pool has some restrictions. See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    ///
    #[structable(optional)]
    pub default_pool_id: Option<String>,

    /// The URI of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PKCS12 format certificate/key bundle for
    /// `TERMINATED_HTTPS` listeners. DEPRECATED: A secret container of type
    /// “certificate” containing the certificate and key for `TERMINATED_HTTPS`
    /// listeners.
    ///
    #[structable(optional)]
    pub default_tls_container_ref: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// Defines whether the `includeSubDomains` directive should be added to
    /// the Strict-Transport-Security HTTP response header.
    ///
    /// **New in version 2.27**
    ///
    #[structable(optional)]
    pub hsts_include_subdomains: Option<bool>,

    /// The value of the `max_age` directive for the Strict-Transport-Security
    /// HTTP response header.
    ///
    /// **New in version 2.27**
    ///
    #[structable(optional)]
    pub hsts_max_age: Option<i32>,

    /// Defines whether the `preload` directive should be added to the
    /// Strict-Transport-Security HTTP response header.
    ///
    /// **New in version 2.27**
    ///
    #[structable(optional)]
    pub hsts_preload: Option<bool>,

    /// The ID of the listener.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// A dictionary of optional headers to insert into the request before it
    /// is sent to the backend `member`. See
    /// [Supported HTTP Header Insertions](#header-insertions). Both keys and
    /// values are always specified as strings.
    ///
    #[structable(optional, serialize)]
    pub insert_headers: Option<HashMap<String, String>>,

    /// A list of L7 policy IDs.
    ///
    #[structable(optional, serialize)]
    pub l7policies: Option<Vec<L7policies>>,

    /// A list of load balancer IDs.
    ///
    #[structable(optional, serialize)]
    pub loadbalancers: Option<Vec<Loadbalancers>>,

    /// Human-readable name of the resource.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[structable(optional)]
    pub operating_status: Option<String>,

    /// The ID of the project owning this resource.
    ///
    #[structable(optional)]
    pub project_id: Option<String>,

    /// The protocol for the resource. One of `HTTP`, `HTTPS`, `SCTP`,
    /// `PROMETHEUS`, `TCP`, `TERMINATED_HTTPS`, or `UDP`.
    ///
    #[structable(optional)]
    pub protocol: Option<String>,

    /// The protocol port number for the resource.
    ///
    #[structable(optional)]
    pub protocol_port: Option<i32>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[structable(optional)]
    pub provisioning_status: Option<String>,

    /// A list of URIs to the
    /// [key manager service](https://docs.openstack.org/barbican/latest/)
    /// secrets containing PKCS12 format certificate/key bundles for
    /// `TERMINATED_HTTPS` listeners. (DEPRECATED) Secret containers of type
    /// “certificate” containing the certificates and keys for
    /// `TERMINATED_HTTPS` listeners.
    ///
    #[structable(optional, serialize)]
    pub sni_container_refs: Option<Vec<String>>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// Frontend client inactivity timeout in milliseconds. Default: 50000.
    ///
    /// **New in version 2.1**
    ///
    #[structable(optional)]
    pub timeout_client_data: Option<i32>,

    /// Backend member connection timeout in milliseconds. Default: 5000.
    ///
    /// **New in version 2.1**
    ///
    #[structable(optional)]
    pub timeout_member_connect: Option<i32>,

    /// Backend member inactivity timeout in milliseconds. Default: 50000.
    ///
    /// **New in version 2.1**
    ///
    #[structable(optional)]
    pub timeout_member_data: Option<i32>,

    /// Time, in milliseconds, to wait for additional TCP packets for content
    /// inspection. Default: 0.
    ///
    /// **New in version 2.1**
    ///
    #[structable(optional)]
    pub timeout_tcp_inspect: Option<i32>,

    /// List of ciphers in OpenSSL format (colon-separated). See
    /// <https://www.openssl.org/docs/man1.1.1/man1/ciphers.html>
    ///
    /// **New in version 2.15**
    ///
    #[structable(optional)]
    pub tls_ciphers: Option<String>,

    /// A list of TLS protocol versions. Available versions: SSLv3, TLSv1,
    /// TLSv1.1, TLSv1.2, TLSv1.3
    ///
    /// **New in version 2.17**
    ///
    #[structable(optional, serialize)]
    pub tls_versions: Option<Vec<String>>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,
}

/// Base type for complex types
///
/// `L7policies` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct L7policies {
    pub id: String,
}

/// Base type for complex types
///
/// `Loadbalancers` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Loadbalancers {
    pub id: String,
}
