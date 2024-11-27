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

use openstack_sdk::api::QueryAsync;

use crate::action::Action;
use crate::cloud_worker::{ApiRequest, Cloud};

pub mod types;
use types::*;

pub trait LoadBalancerExt {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()>;

    /// List Loadbalancers
    async fn get_load_balancers(&mut self, filters: &LoadBalancerFilters) -> Result<Vec<Value>>;
    /// List Listeners
    async fn get_listeners(&mut self, filters: &LoadBalancerListenerFilters) -> Result<Vec<Value>>;
    /// List Pools
    async fn get_pools(&mut self, filters: &LoadBalancerPoolFilters) -> Result<Vec<Value>>;
    /// List Listeners
    async fn get_pool_members(
        &mut self,
        filters: &LoadBalancerPoolMemberFilters,
    ) -> Result<Vec<Value>>;
    /// List Listeners
    async fn get_health_monitors(
        &mut self,
        filters: &LoadBalancerHealthMonitorFilters,
    ) -> Result<Vec<Value>>;
}

impl LoadBalancerExt for Cloud {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()> {
        match request {
            ApiRequest::LoadBalancers(ref filters) => {
                match self.get_load_balancers(filters).await {
                    Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch load balancers: {:?}",
                        err
                    )))?,
                }
            }
            ApiRequest::LoadBalancerListeners(ref filters) => {
                match self.get_listeners(filters).await {
                    Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch lb listeners: {:?}",
                        err
                    )))?,
                }
            }
            ApiRequest::LoadBalancerPools(ref filters) => match self.get_pools(filters).await {
                Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch lb pools: {:?}",
                    err
                )))?,
            },
            ApiRequest::LoadBalancerPoolMembers(ref filters) => {
                match self.get_pool_members(filters).await {
                    Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch lb members: {:?}",
                        err
                    )))?,
                }
            }
            ApiRequest::LoadBalancerHealthMonitors(ref filters) => {
                match self.get_health_monitors(filters).await {
                    Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch lb healthmonitors: {:?}",
                        err
                    )))?,
                }
            }
            _ => {
                todo!()
            }
        }
        Ok(())
    }

    async fn get_listeners(&mut self, filters: &LoadBalancerListenerFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::load_balancer::v2::listener::list::RequestBuilder::try_from(
                    filters,
                )?
                .build()?;

            let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_load_balancers(&mut self, filters: &LoadBalancerFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep = openstack_sdk::api::load_balancer::v2::loadbalancer::list::RequestBuilder::try_from(
                    filters,
                )?
                .build()?;

            let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_pools(&mut self, filters: &LoadBalancerPoolFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep = openstack_sdk::api::load_balancer::v2::pool::list::RequestBuilder::try_from(
                filters,
            )?
            .build()?;

            let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_pool_members(
        &mut self,
        filters: &LoadBalancerPoolMemberFilters,
    ) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::load_balancer::v2::pool::member::list::RequestBuilder::try_from(
                    filters,
                )?
                .build()?;

            let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_health_monitors(
        &mut self,
        filters: &LoadBalancerHealthMonitorFilters,
    ) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::load_balancer::v2::healthmonitor::list::RequestBuilder::try_from(
                    filters,
                )?
                .build()?;

            let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }
}
