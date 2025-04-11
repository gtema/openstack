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
//! Response type for the put lbaas/pools/{pool_id} operation

use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// Pool response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct PoolResponse {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[structable(optional)]
    pub admin_state_up: Option<bool>,

    /// A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2
    ///
    /// **New in version 2.24**
    ///
    #[structable(optional, serialize)]
    pub alpn_protocols: Option<Vec<String>>,

    /// The reference of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA certificate bundle for `tls_enabled`
    /// pools.
    ///
    /// **New in version 2.8**
    ///
    #[structable(optional)]
    pub ca_tls_container_ref: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// The reference of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA revocation list file for
    /// `tls_enabled` pools.
    ///
    #[structable(optional)]
    pub crl_container_ref: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// The associated health monitor ID.
    ///
    #[structable(optional)]
    pub healthmonitor_id: Option<String>,

    /// The ID of the pool.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The load balancing algorithm for the pool. One of `LEAST_CONNECTIONS`,
    /// `ROUND_ROBIN`, `SOURCE_IP`, or `SOURCE_IP_PORT`.
    ///
    #[structable(optional)]
    pub lb_algorithm: Option<String>,

    /// A list of listener IDs.
    ///
    #[structable(optional, serialize)]
    pub listeners: Option<Vec<Listeners>>,

    /// A list of load balancer IDs.
    ///
    #[structable(optional, serialize)]
    pub loadbalancers: Option<Vec<Loadbalancers>>,

    /// A list of member IDs.
    ///
    #[structable(optional, serialize)]
    pub members: Option<Vec<Members>>,

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

    /// The protocol for the resource. One of `HTTP`, `HTTPS`, `PROXY`,
    /// `PROXYV2`, `SCTP`, `TCP`, or `UDP`.
    ///
    #[structable(optional)]
    pub protocol: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[structable(optional)]
    pub provisioning_status: Option<String>,

    /// A JSON object specifying the session persistence for the pool or `null`
    /// for no session persistence. See
    /// [Pool Session Persistence](#session-persistence). Default is `null`.
    ///
    #[structable(optional, serialize)]
    pub session_persistence: Option<SessionPersistence>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// List of ciphers in OpenSSL format (colon-separated). See
    /// <https://www.openssl.org/docs/man1.1.1/man1/ciphers.html>
    ///
    /// **New in version 2.15**
    ///
    #[structable(optional)]
    pub tls_ciphers: Option<String>,

    /// The reference to the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PKCS12 format certificate/key bundle for
    /// `tls_enabled` pools for TLS client authentication to the member
    /// servers.
    ///
    /// **New in version 2.8**
    ///
    #[structable(optional)]
    pub tls_container_ref: Option<String>,

    /// When `true` connections to backend member servers will use TLS
    /// encryption. Default is `false`.
    ///
    /// **New in version 2.8**
    ///
    #[structable(optional)]
    pub tls_enabled: Option<bool>,

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

/// A JSON object specifying the session persistence for the pool or `null` for
/// no session persistence. See
/// [Pool Session Persistence](#session-persistence). Default is `null`.
///
/// `SessionPersistence` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionPersistence {
    pub cookie_name: Option<String>,
    pub persistence_granularity: Option<String>,
    pub persistence_timeout: Option<i32>,
    pub _type: Option<String>,
}

/// Base type for complex types
///
/// `Loadbalancers` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Loadbalancers {
    pub id: String,
}

/// Base type for complex types
///
/// `Listeners` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Listeners {
    pub id: String,
}

/// Base type for complex types
///
/// `Members` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Members {
    pub id: String,
}
