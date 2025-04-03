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
use derive_builder::Builder;
use eyre::{Report, Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

use crate::utils::OutputConfig;
use crate::utils::StructTable;
use openstack_sdk::api::load_balancer::v2::listener::list::RequestBuilder;
use openstack_sdk::api::{Pagination, paged};
use openstack_sdk::{AsyncOpenStack, api::QueryAsync};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct LoadBalancerListenerList {
    #[builder(default)]
    pub admin_state_up: Option<bool>,
    #[builder(default)]
    pub alpn_protocols: Option<String>,
    #[builder(default)]
    pub connection_limit: Option<String>,
    #[builder(default)]
    pub created_at: Option<String>,
    #[builder(default)]
    pub default_pool_id: Option<String>,
    #[builder(default)]
    pub default_pool_name: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub hsts_include_subdomains: Option<bool>,
    #[builder(default)]
    pub hsts_max_age: Option<i32>,
    #[builder(default)]
    pub hsts_preload: Option<bool>,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub load_balancer_id: Option<String>,
    #[builder(default)]
    pub load_balancer_name: Option<String>,
    #[builder(default)]
    pub marker: Option<String>,
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub not_tags: Option<String>,
    #[builder(default)]
    pub not_tags_any: Option<String>,
    #[builder(default)]
    pub operating_status: Option<String>,
    #[builder(default)]
    pub page_reverse: Option<bool>,
    #[builder(default)]
    pub project_id: Option<String>,
    #[builder(default)]
    pub project_name: Option<String>,
    #[builder(default)]
    pub protocol: Option<String>,
    #[builder(default)]
    pub protocol_port: Option<i32>,
    #[builder(default)]
    pub provisioning_status: Option<String>,
    #[builder(default)]
    pub tags: Option<String>,
    #[builder(default)]
    pub tags_any: Option<String>,
    #[builder(default)]
    pub timeout_client_data: Option<i32>,
    #[builder(default)]
    pub timeout_member_connect: Option<i32>,
    #[builder(default)]
    pub timeout_member_data: Option<i32>,
    #[builder(default)]
    pub timeout_tcp_inspect: Option<i32>,
    #[builder(default)]
    pub tls_ciphers: Option<String>,
    #[builder(default)]
    pub tls_versions: Option<String>,
    #[builder(default)]
    pub updated_at: Option<String>,
}

impl fmt::Display for LoadBalancerListenerList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.default_pool_id.is_some() || self.default_pool_name.is_some() {
            parts.push(format!(
                "default_pool: {}",
                self.default_pool_name
                    .as_ref()
                    .or(self.default_pool_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.id.is_some() || self.name.is_some() {
            parts.push(format!(
                "name/id: {}",
                self.name
                    .as_ref()
                    .or(self.id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.load_balancer_id.is_some() || self.load_balancer_name.is_some() {
            parts.push(format!(
                "load_balancer: {}",
                self.load_balancer_name
                    .as_ref()
                    .or(self.load_balancer_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.project_id.is_some() || self.project_name.is_some() {
            parts.push(format!(
                "project: {}",
                self.project_name
                    .as_ref()
                    .or(self.project_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&LoadBalancerListenerList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &LoadBalancerListenerList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &value.alpn_protocols {
            ep_builder.alpn_protocols(val.clone());
        }
        if let Some(val) = &value.connection_limit {
            ep_builder.connection_limit(val.clone());
        }
        if let Some(val) = &value.created_at {
            ep_builder.created_at(val.clone());
        }
        if let Some(val) = &value.default_pool_id {
            ep_builder.default_pool_id(val.clone());
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &value.hsts_include_subdomains {
            ep_builder.hsts_include_subdomains(*val);
        }
        if let Some(val) = &value.hsts_max_age {
            ep_builder.hsts_max_age(*val);
        }
        if let Some(val) = &value.hsts_preload {
            ep_builder.hsts_preload(*val);
        }
        if let Some(val) = &value.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.load_balancer_id {
            ep_builder.load_balancer_id(val.clone());
        }
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &value.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &value.page_reverse {
            ep_builder.page_reverse(*val);
        }
        if let Some(val) = &value.project_id {
            ep_builder.project_id(val.clone());
        }
        if let Some(val) = &value.protocol {
            ep_builder.protocol(val.clone());
        }
        if let Some(val) = &value.protocol_port {
            ep_builder.protocol_port(*val);
        }
        if let Some(val) = &value.timeout_client_data {
            ep_builder.timeout_client_data(*val);
        }
        if let Some(val) = &value.timeout_member_connect {
            ep_builder.timeout_member_connect(*val);
        }
        if let Some(val) = &value.timeout_member_data {
            ep_builder.timeout_member_data(*val);
        }
        if let Some(val) = &value.timeout_tcp_inspect {
            ep_builder.timeout_tcp_inspect(*val);
        }
        if let Some(val) = &value.tls_ciphers {
            ep_builder.tls_ciphers(val.clone());
        }
        if let Some(val) = &value.tls_versions {
            ep_builder.tls_versions(val.clone());
        }
        if let Some(val) = &value.updated_at {
            ep_builder.updated_at(val.clone());
        }
        if let Some(val) = &value.provisioning_status {
            ep_builder.provisioning_status(val.clone());
        }
        if let Some(val) = &value.operating_status {
            ep_builder.operating_status(val.clone());
        }
        if let Some(val) = &value.tags {
            ep_builder.tags(val.clone());
        }
        if let Some(val) = &value.tags_any {
            ep_builder.tags_any(val.clone());
        }
        if let Some(val) = &value.not_tags {
            ep_builder.not_tags(val.clone());
        }
        if let Some(val) = &value.not_tags_any {
            ep_builder.not_tags_any(val.clone());
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for LoadBalancerListenerList {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = TryInto::<RequestBuilder>::try_into(self)?
            .build()
            .wrap_err("Cannot prepare request")?;
        app_tx.send(Action::ApiResponsesData {
            request: request.clone(),
            data: paged(ep, Pagination::All).query_async(session).await?,
        })?;
        Ok(())
    }
}
/// LoadBalancerListener response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct LoadBalancerListener {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde(default)]
    #[structable(optional, title = "ADMIN_STATE_UP", wide)]
    pub admin_state_up: Option<bool>,

    /// A list of IPv4, IPv6 or mix of both CIDRs.
    ///
    /// **New in version 2.12**
    ///
    #[serde(default)]
    #[structable(optional, title = "ALLOWED_CIDRS", wide)]
    pub allowed_cidrs: Option<Value>,

    /// A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2
    ///
    /// **New in version 2.20**
    ///
    #[serde(default)]
    #[structable(optional, title = "ALPN_PROTOCOLS", wide)]
    pub alpn_protocols: Option<Value>,

    /// The TLS client authentication mode. One of the options `NONE`,
    /// `OPTIONAL` or `MANDATORY`.
    ///
    /// **New in version 2.8**
    ///
    #[serde(default)]
    #[structable(optional, title = "CLIENT_AUTHENTICATION", wide)]
    pub client_authentication: Option<String>,

    /// The ref of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format client CA certificate bundle for
    /// `TERMINATED_HTTPS` listeners.
    ///
    /// **New in version 2.8**
    ///
    #[serde(default)]
    #[structable(optional, title = "CLIENT_CA_TLS_CONTAINER_REF", wide)]
    pub client_ca_tls_container_ref: Option<String>,

    /// The URI of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA revocation list file for
    /// `TERMINATED_HTTPS` listeners.
    ///
    /// **New in version 2.8**
    ///
    #[serde(default)]
    #[structable(optional, title = "CLIENT_CRL_CONTAINER_REF", wide)]
    pub client_crl_container_ref: Option<String>,

    /// The maximum number of connections permitted for this listener. Default
    /// value is -1 which represents infinite connections or a default value
    /// defined by the provider driver.
    ///
    #[serde(default)]
    #[structable(optional, title = "CONNECTION_LIMIT", wide)]
    pub connection_limit: Option<i32>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde(default)]
    #[structable(optional, title = "CREATED_AT")]
    pub created_at: Option<String>,

    /// The ID of the pool used by the listener if no L7 policies match. The
    /// pool has some restrictions. See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    ///
    #[serde(default)]
    #[structable(optional, title = "DEFAULT_POOL_ID", wide)]
    pub default_pool_id: Option<String>,

    /// The URI of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PKCS12 format certificate/key bundle for
    /// `TERMINATED_HTTPS` listeners. DEPRECATED: A secret container of type
    /// “certificate” containing the certificate and key for `TERMINATED_HTTPS`
    /// listeners.
    ///
    #[serde(default)]
    #[structable(optional, title = "DEFAULT_TLS_CONTAINER_REF", wide)]
    pub default_tls_container_ref: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "DESCRIPTION", wide)]
    pub description: Option<String>,

    /// Defines whether the `includeSubDomains` directive should be added to
    /// the Strict-Transport-Security HTTP response header.
    ///
    /// **New in version 2.27**
    ///
    #[serde(default)]
    #[structable(optional, title = "HSTS_INCLUDE_SUBDOMAINS", wide)]
    pub hsts_include_subdomains: Option<bool>,

    /// The value of the `max_age` directive for the Strict-Transport-Security
    /// HTTP response header.
    ///
    /// **New in version 2.27**
    ///
    #[serde(default)]
    #[structable(optional, title = "HSTS_MAX_AGE", wide)]
    pub hsts_max_age: Option<i32>,

    /// Defines whether the `preload` directive should be added to the
    /// Strict-Transport-Security HTTP response header.
    ///
    /// **New in version 2.27**
    ///
    #[serde(default)]
    #[structable(optional, title = "HSTS_PRELOAD", wide)]
    pub hsts_preload: Option<bool>,

    /// The ID of the listener.
    ///
    #[serde(default)]
    #[structable(optional, title = "ID", wide)]
    pub id: Option<String>,

    /// A dictionary of optional headers to insert into the request before it
    /// is sent to the backend `member`. See
    /// [Supported HTTP Header Insertions](#header-insertions). Both keys and
    /// values are always specified as strings.
    ///
    #[serde(default)]
    #[structable(optional, title = "INSERT_HEADERS", wide)]
    pub insert_headers: Option<Value>,

    /// A list of L7 policy IDs.
    ///
    #[serde(default)]
    #[structable(optional, title = "L7POLICIES", wide)]
    pub l7policies: Option<Value>,

    /// A list of load balancer IDs.
    ///
    #[serde(default)]
    #[structable(optional, title = "LOADBALANCERS", wide)]
    pub loadbalancers: Option<Value>,

    /// Human-readable name of the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "NAME")]
    pub name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde(default)]
    #[structable(optional, status, title = "OPERATING_STATUS")]
    pub operating_status: Option<String>,

    /// The ID of the project owning this resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "PROJECT_ID", wide)]
    pub project_id: Option<String>,

    /// The protocol for the resource. One of `HTTP`, `HTTPS`, `SCTP`,
    /// `PROMETHEUS`, `TCP`, `TERMINATED_HTTPS`, or `UDP`.
    ///
    #[serde(default)]
    #[structable(optional, title = "PROTOCOL", wide)]
    pub protocol: Option<String>,

    /// The protocol port number for the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "PROTOCOL_PORT", wide)]
    pub protocol_port: Option<i32>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[serde(default)]
    #[structable(optional, title = "PROVISIONING_STATUS", wide)]
    pub provisioning_status: Option<String>,

    /// A list of URIs to the
    /// [key manager service](https://docs.openstack.org/barbican/latest/)
    /// secrets containing PKCS12 format certificate/key bundles for
    /// `TERMINATED_HTTPS` listeners. (DEPRECATED) Secret containers of type
    /// “certificate” containing the certificates and keys for
    /// `TERMINATED_HTTPS` listeners.
    ///
    #[serde(default)]
    #[structable(optional, title = "SNI_CONTAINER_REFS", wide)]
    pub sni_container_refs: Option<Value>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde(default)]
    #[structable(optional, title = "TAGS", wide)]
    pub tags: Option<Value>,

    #[serde(default)]
    #[structable(optional, title = "TENANT_ID", wide)]
    pub tenant_id: Option<String>,

    /// Frontend client inactivity timeout in milliseconds. Default: 50000.
    ///
    /// **New in version 2.1**
    ///
    #[serde(default)]
    #[structable(optional, title = "TIMEOUT_CLIENT_DATA", wide)]
    pub timeout_client_data: Option<i32>,

    /// Backend member connection timeout in milliseconds. Default: 5000.
    ///
    /// **New in version 2.1**
    ///
    #[serde(default)]
    #[structable(optional, title = "TIMEOUT_MEMBER_CONNECT", wide)]
    pub timeout_member_connect: Option<i32>,

    /// Backend member inactivity timeout in milliseconds. Default: 50000.
    ///
    /// **New in version 2.1**
    ///
    #[serde(default)]
    #[structable(optional, title = "TIMEOUT_MEMBER_DATA", wide)]
    pub timeout_member_data: Option<i32>,

    /// Time, in milliseconds, to wait for additional TCP packets for content
    /// inspection. Default: 0.
    ///
    /// **New in version 2.1**
    ///
    #[serde(default)]
    #[structable(optional, title = "TIMEOUT_TCP_INSPECT", wide)]
    pub timeout_tcp_inspect: Option<i32>,

    /// List of ciphers in OpenSSL format (colon-separated). See
    /// <https://www.openssl.org/docs/man1.1.1/man1/ciphers.html>
    ///
    /// **New in version 2.15**
    ///
    #[serde(default)]
    #[structable(optional, title = "TLS_CIPHERS", wide)]
    pub tls_ciphers: Option<String>,

    /// A list of TLS protocol versions. Available versions: SSLv3, TLSv1,
    /// TLSv1.1, TLSv1.2, TLSv1.3
    ///
    /// **New in version 2.17**
    ///
    #[serde(default)]
    #[structable(optional, title = "TLS_VERSIONS", wide)]
    pub tls_versions: Option<Value>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde(default)]
    #[structable(optional, title = "UPDATED_AT")]
    pub updated_at: Option<String>,
}
