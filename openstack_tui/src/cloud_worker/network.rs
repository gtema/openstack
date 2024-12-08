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

use eyre::Result;
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::{api::Pagination, api::QueryAsync};

use crate::action::Action;
use crate::cloud_worker::{ApiRequest, Cloud};

pub mod types;
use types::*;

pub trait NetworkExt {
    /// Perform network request
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()>;

    /// List networks
    async fn get_networks(&mut self, _filters: &NetworkNetworkFilters) -> Result<Vec<Value>>;

    /// List routers
    async fn get_routers(&mut self, _filters: &NetworkRouterFilters) -> Result<Vec<Value>>;

    /// List Security Groups
    async fn get_security_groups(
        &mut self,
        filters: &NetworkSecurityGroupFilters,
    ) -> Result<Vec<Value>>;

    /// List Security Group Rules
    async fn get_security_group_rules(
        &mut self,
        filters: &NetworkSecurityGroupRuleFilters,
    ) -> Result<Vec<Value>>;

    /// List subnets
    async fn get_subnets(&mut self, filters: &NetworkSubnetFilters) -> Result<Vec<Value>>;

    /// Get network quota
    async fn get_quota(&mut self) -> Result<Value>;
}

impl NetworkExt for Cloud {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()> {
        match request {
            ApiRequest::NetworkNetworks(ref filters) => match self.get_networks(filters).await {
                Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch networks: {:?}",
                    err
                )))?,
            },
            ApiRequest::NetworkRouters(ref filters) => match self.get_routers(filters).await {
                Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                Err(err) => {
                    app_tx.send(Action::Error(format!("Failed to fetch routers: {:?}", err)))?
                }
            },
            ApiRequest::NetworkQuota => match self.get_quota().await {
                Ok(data) => app_tx.send(Action::ApiResponseData { request, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch network quota: {:?}",
                    err
                )))?,
            },
            ApiRequest::NetworkSecurityGroups(ref filters) => {
                match self.get_security_groups(filters).await {
                    Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch security groups: {:?}",
                        err
                    )))?,
                }
            }
            ApiRequest::NetworkSecurityGroupRules(ref filters) => {
                match self.get_security_group_rules(filters).await {
                    Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch security group rules: {:?}",
                        err
                    )))?,
                }
            }
            ApiRequest::NetworkSubnets(ref filters) => match self.get_subnets(filters).await {
                Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                Err(err) => {
                    app_tx.send(Action::Error(format!("Failed to fetch subnets: {:?}", err)))?
                }
            },

            _ => {
                todo!()
            }
        }
        Ok(())
    }

    async fn get_networks(&mut self, filters: &NetworkNetworkFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::network::v2::network::list::RequestBuilder::try_from(filters)?
                    .build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_routers(&mut self, filters: &NetworkRouterFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::network::v2::router::list::RequestBuilder::try_from(filters)?
                    .build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_security_groups(
        &mut self,
        _filters: &NetworkSecurityGroupFilters,
    ) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let mut ep_builder =
                openstack_sdk::api::network::v2::security_group::list::Request::builder();
            ep_builder.sort_key(["name"].into_iter());
            ep_builder.sort_dir(["asc"].into_iter());

            let ep = ep_builder.build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_security_group_rules(
        &mut self,
        filters: &NetworkSecurityGroupRuleFilters,
    ) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::network::v2::security_group_rule::list::RequestBuilder::try_from(filters)?.build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_subnets(&mut self, filters: &NetworkSubnetFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::network::v2::subnet::list::RequestBuilder::try_from(filters)?
                    .build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_quota(&mut self) -> Result<Value> {
        if let Some(session) = &self.cloud {
            let mut ep_builder =
                openstack_sdk::api::network::v2::quota::details::Request::builder();

            if let Some(auth) = session.get_auth_info() {
                if let Some(project) = auth.token.project {
                    if let Some(pid) = project.id {
                        ep_builder.id(pid);
                        let ep = ep_builder.build()?;
                        let res: Value = ep.query_async(session).await?;
                        return Ok(res);
                    }
                }
            }
        }
        Ok(Value::Null)
    }
}
