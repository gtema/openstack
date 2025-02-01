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
use openstack_sdk::api::network::v2::security_group_rule::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct NetworkSecurityGroupRuleList {
    #[builder(default)]
    pub belongs_to_default_sg: Option<bool>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub direction: Option<String>,
    #[builder(default)]
    pub ethertype: Option<String>,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub marker: Option<String>,
    #[builder(default)]
    pub normalized_cidr: Option<String>,
    #[builder(default)]
    pub page_reverse: Option<bool>,
    #[builder(default)]
    pub port_range_max: Option<i32>,
    #[builder(default)]
    pub port_range_min: Option<i32>,
    #[builder(default)]
    pub protocol: Option<String>,
    #[builder(default)]
    pub remote_address_group_id: Option<String>,
    #[builder(default)]
    pub remote_address_group_name: Option<String>,
    #[builder(default)]
    pub remote_group_id: Option<String>,
    #[builder(default)]
    pub remote_group_name: Option<String>,
    #[builder(default)]
    pub remote_ip_prefix: Option<String>,
    #[builder(default)]
    pub revision_number: Option<String>,
    #[builder(default)]
    pub security_group_id: Option<String>,
    #[builder(default)]
    pub security_group_name: Option<String>,
    #[builder(default)]
    pub sort_dir: Option<Vec<String>>,
    #[builder(default)]
    pub sort_key: Option<Vec<String>>,
    #[builder(default)]
    pub tenant_id: Option<String>,
    #[builder(default)]
    pub tenant_name: Option<String>,
}

impl fmt::Display for NetworkSecurityGroupRuleList {
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

        if self.remote_address_group_id.is_some() || self.remote_address_group_name.is_some() {
            parts.push(format!(
                "remote_address_group: {}",
                self.remote_address_group_name
                    .as_ref()
                    .or(self.remote_address_group_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.remote_group_id.is_some() || self.remote_group_name.is_some() {
            parts.push(format!(
                "remote_group: {}",
                self.remote_group_name
                    .as_ref()
                    .or(self.remote_group_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.security_group_id.is_some() || self.security_group_name.is_some() {
            parts.push(format!(
                "security_group: {}",
                self.security_group_name
                    .as_ref()
                    .or(self.security_group_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.tenant_id.is_some() || self.tenant_name.is_some() {
            parts.push(format!(
                "tenant: {}",
                self.tenant_name
                    .as_ref()
                    .or(self.tenant_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&NetworkSecurityGroupRuleList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &NetworkSecurityGroupRuleList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &value.security_group_id {
            ep_builder.security_group_id(val.clone());
        }
        if let Some(val) = &value.remote_group_id {
            ep_builder.remote_group_id(val.clone());
        }
        if let Some(val) = &value.direction {
            ep_builder.direction(val.clone());
        }
        if let Some(val) = &value.protocol {
            ep_builder.protocol(val.clone());
        }
        if let Some(val) = &value.port_range_min {
            ep_builder.port_range_min(*val);
        }
        if let Some(val) = &value.port_range_max {
            ep_builder.port_range_max(*val);
        }
        if let Some(val) = &value.ethertype {
            ep_builder.ethertype(val.clone());
        }
        if let Some(val) = &value.remote_ip_prefix {
            ep_builder.remote_ip_prefix(val.clone());
        }
        if let Some(val) = &value.tenant_id {
            ep_builder.tenant_id(val.clone());
        }
        if let Some(val) = &value.revision_number {
            ep_builder.revision_number(val.clone());
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &value.normalized_cidr {
            ep_builder.normalized_cidr(val.clone());
        }
        if let Some(val) = &value.remote_address_group_id {
            ep_builder.remote_address_group_id(val.clone());
        }
        if let Some(val) = &value.belongs_to_default_sg {
            ep_builder.belongs_to_default_sg(*val);
        }
        if let Some(val) = &value.sort_key {
            ep_builder.sort_key(val.iter().cloned());
        }
        if let Some(val) = &value.sort_dir {
            ep_builder.sort_dir(val.iter().cloned());
        }
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &value.page_reverse {
            ep_builder.page_reverse(*val);
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for NetworkSecurityGroupRuleList {
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
/// NetworkSecurityGroupRule response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct NetworkSecurityGroupRule {
    /// Indicates if the security group rule belongs to the default security
    /// group of the project or not.
    ///
    #[serde(default)]
    #[structable(optional, title = "BELONGS_TO_DEFAULT_SG", wide)]
    belongs_to_default_sg: Option<bool>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde(default)]
    #[structable(optional, title = "CREATED_AT")]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "DESCRIPTION", wide)]
    description: Option<String>,

    /// Ingress or egress, which is the direction in which the security group
    /// rule is applied.
    ///
    #[serde(default)]
    #[structable(optional, title = "DIRECTION", wide)]
    direction: Option<Value>,

    /// Must be IPv4 or IPv6, and addresses represented in CIDR must match the
    /// ingress or egress rules.
    ///
    #[serde(default)]
    #[structable(optional, title = "ETHERTYPE", wide)]
    ethertype: Option<Value>,

    /// The ID of the security group rule.
    ///
    #[serde(default)]
    #[structable(optional, title = "ID", wide)]
    id: Option<String>,

    #[serde(default)]
    #[structable(optional, title = "NORMALIZED_CIDR", wide)]
    normalized_cidr: Option<String>,

    /// The maximum port number in the range that is matched by the security
    /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
    /// value must be greater than or equal to the `port_range_min` attribute
    /// value. If the protocol is ICMP, this value must be an ICMP code.
    ///
    #[serde(default)]
    #[structable(optional, title = "PORT_RANGE_MAX", wide)]
    port_range_max: Option<i32>,

    /// The minimum port number in the range that is matched by the security
    /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
    /// value must be less than or equal to the `port_range_max` attribute
    /// value. If the protocol is ICMP, this value must be an ICMP type.
    ///
    #[serde(default)]
    #[structable(optional, title = "PORT_RANGE_MIN", wide)]
    port_range_min: Option<i32>,

    /// The IP protocol can be represented by a string, an integer, or `null`.
    /// Valid string or integer values are `any` or `0`, `ah` or `51`, `dccp`
    /// or `33`, `egp` or `8`, `esp` or `50`, `gre` or `47`, `icmp` or `1`,
    /// `icmpv6` or `58`, `igmp` or `2`, `ipip` or `4`, `ipv6-encap` or `41`,
    /// `ipv6-frag` or `44`, `ipv6-icmp` or `58`, `ipv6-nonxt` or `59`,
    /// `ipv6-opts` or `60`, `ipv6-route` or `43`, `ospf` or `89`, `pgm` or
    /// `113`, `rsvp` or `46`, `sctp` or `132`, `tcp` or `6`, `udp` or `17`,
    /// `udplite` or `136`, `vrrp` or `112`. Additionally, any integer value
    /// between [0-255] is also valid. The string `any` (or integer `0`) means
    /// `all` IP protocols. See the constants in `neutron_lib.constants` for
    /// the most up-to-date list of supported strings.
    ///
    #[serde(default)]
    #[structable(optional, title = "PROTOCOL", wide)]
    protocol: Option<String>,

    /// The remote address group UUID that is associated with this security
    /// group rule.
    ///
    #[serde(default)]
    #[structable(optional, title = "REMOTE_ADDRESS_GROUP_ID", wide)]
    remote_address_group_id: Option<String>,

    /// The remote group UUID to associate with this security group rule. You
    /// can specify either the `remote_group_id` or `remote_ip_prefix`
    /// attribute in the request body.
    ///
    #[serde(default)]
    #[structable(optional, title = "REMOTE_GROUP_ID", wide)]
    remote_group_id: Option<String>,

    /// The remote IP prefix that is matched by this security group rule.
    ///
    #[serde(default)]
    #[structable(optional, title = "REMOTE_IP_PREFIX", wide)]
    remote_ip_prefix: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "REVISION_NUMBER", wide)]
    revision_number: Option<i32>,

    /// The security group ID to associate with this security group rule.
    ///
    #[serde(default)]
    #[structable(optional, title = "SECURITY_GROUP_ID", wide)]
    security_group_id: Option<String>,

    /// The ID of the project.
    ///
    #[serde(default)]
    #[structable(optional, title = "TENANT_ID", wide)]
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde(default)]
    #[structable(optional, title = "UPDATED_AT")]
    updated_at: Option<String>,
}
