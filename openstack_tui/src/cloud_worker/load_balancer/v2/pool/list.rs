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
use openstack_sdk::api::load_balancer::v2::pool::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct LoadBalancerPoolList {
    #[builder(default)]
    pub admin_state_up: Option<bool>,
    #[builder(default)]
    pub alpn_protocols: Option<String>,
    #[builder(default)]
    pub created_at: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub loadbalancer_id: Option<String>,
    #[builder(default)]
    pub loadbalancer_name: Option<String>,
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
    pub provisioning_status: Option<String>,
    #[builder(default)]
    pub tags: Option<String>,
    #[builder(default)]
    pub tags_any: Option<String>,
    #[builder(default)]
    pub tls_ciphers: Option<String>,
    #[builder(default)]
    pub tls_enabled: Option<bool>,
    #[builder(default)]
    pub tls_versions: Option<String>,
    #[builder(default)]
    pub updated_at: Option<String>,
}

impl fmt::Display for LoadBalancerPoolList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.id.is_some() || self.name.is_some() {
            parts.push(format!(
                "name/id: {}",
                self.name
                    .as_ref()
                    .or(self.id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.loadbalancer_id.is_some() || self.loadbalancer_name.is_some() {
            parts.push(format!(
                "loadbalancer: {}",
                self.loadbalancer_name
                    .as_ref()
                    .or(self.loadbalancer_id.as_ref())
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

impl TryFrom<&LoadBalancerPoolList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &LoadBalancerPoolList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &value.alpn_protocols {
            ep_builder.alpn_protocols(val.clone());
        }
        if let Some(val) = &value.created_at {
            ep_builder.created_at(val.clone());
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &value.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.loadbalancer_id {
            ep_builder.loadbalancer_id(val.clone());
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
        if let Some(val) = &value.tls_enabled {
            ep_builder.tls_enabled(*val);
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

impl ExecuteApiRequest for LoadBalancerPoolList {
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
/// LoadBalancerPool response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct LoadBalancerPool {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde(default)]
    #[structable(optional, title = "ADMIN_STATE_UP", wide)]
    pub admin_state_up: Option<bool>,

    /// A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2
    ///
    /// **New in version 2.24**
    ///
    #[serde(default)]
    #[structable(optional, title = "ALPN_PROTOCOLS", wide)]
    pub alpn_protocols: Option<Value>,

    /// The reference of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA certificate bundle for `tls_enabled`
    /// pools.
    ///
    /// **New in version 2.8**
    ///
    #[serde(default)]
    #[structable(optional, title = "CA_TLS_CONTAINER_REF", wide)]
    pub ca_tls_container_ref: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde(default)]
    #[structable(optional, title = "CREATED_AT")]
    pub created_at: Option<String>,

    /// The reference of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA revocation list file for
    /// `tls_enabled` pools.
    ///
    #[serde(default)]
    #[structable(optional, title = "CRL_CONTAINER_REF", wide)]
    pub crl_container_ref: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "DESCRIPTION", wide)]
    pub description: Option<String>,

    /// The associated health monitor ID.
    ///
    #[serde(default)]
    #[structable(optional, title = "HEALTHMONITOR_ID", wide)]
    pub healthmonitor_id: Option<String>,

    /// The ID of the pool.
    ///
    #[serde(default)]
    #[structable(optional, title = "ID", wide)]
    pub id: Option<String>,

    /// The load balancing algorithm for the pool. One of `LEAST_CONNECTIONS`,
    /// `ROUND_ROBIN`, `SOURCE_IP`, or `SOURCE_IP_PORT`.
    ///
    #[serde(default)]
    #[structable(optional, title = "LB_ALGORITHM", wide)]
    pub lb_algorithm: Option<String>,

    /// A list of listener IDs.
    ///
    #[serde(default)]
    #[structable(optional, title = "LISTENERS", wide)]
    pub listeners: Option<Value>,

    /// A list of load balancer IDs.
    ///
    #[serde(default)]
    #[structable(optional, title = "LOADBALANCERS", wide)]
    pub loadbalancers: Option<Value>,

    /// A list of member IDs.
    ///
    #[serde(default)]
    #[structable(optional, title = "MEMBERS", wide)]
    pub members: Option<Value>,

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

    /// The protocol for the resource. One of `HTTP`, `HTTPS`, `PROXY`,
    /// `PROXYV2`, `SCTP`, `TCP`, or `UDP`.
    ///
    #[serde(default)]
    #[structable(optional, title = "PROTOCOL", wide)]
    pub protocol: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[serde(default)]
    #[structable(optional, title = "PROVISIONING_STATUS", wide)]
    pub provisioning_status: Option<String>,

    /// A JSON object specifying the session persistence for the pool or `null`
    /// for no session persistence. See
    /// [Pool Session Persistence](#session-persistence). Default is `null`.
    ///
    #[serde(default)]
    #[structable(optional, title = "SESSION_PERSISTENCE", wide)]
    pub session_persistence: Option<Value>,

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

    /// List of ciphers in OpenSSL format (colon-separated). See
    /// <https://www.openssl.org/docs/man1.1.1/man1/ciphers.html>
    ///
    /// **New in version 2.15**
    ///
    #[serde(default)]
    #[structable(optional, title = "TLS_CIPHERS", wide)]
    pub tls_ciphers: Option<String>,

    /// The reference to the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PKCS12 format certificate/key bundle for
    /// `tls_enabled` pools for TLS client authentication to the member
    /// servers.
    ///
    /// **New in version 2.8**
    ///
    #[serde(default)]
    #[structable(optional, title = "TLS_CONTAINER_REF", wide)]
    pub tls_container_ref: Option<String>,

    /// When `true` connections to backend member servers will use TLS
    /// encryption. Default is `false`.
    ///
    /// **New in version 2.8**
    ///
    #[serde(default)]
    #[structable(optional, title = "TLS_ENABLED", wide)]
    pub tls_enabled: Option<bool>,

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
