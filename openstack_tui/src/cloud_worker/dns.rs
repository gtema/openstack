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

pub trait DnsExt {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()>;

    async fn get_recordsets(&mut self, filters: &DnsRecordsetFilters) -> Result<Vec<Value>>;
    async fn get_zones(&mut self, filters: &DnsZoneFilters) -> Result<Vec<Value>>;
    async fn delete_zone(&mut self, request: &DnsZoneDelete) -> Result<()>;
}

impl DnsExt for Cloud {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()> {
        match request {
            ApiRequest::DnsRecordsets(ref filters) => match self.get_recordsets(filters).await {
                Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch dns recordsets: {:?}",
                    err
                )))?,
            },
            ApiRequest::DnsZones(ref filters) => match self.get_zones(filters).await {
                Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch dns zones: {:?}",
                    err
                )))?,
            },
            ApiRequest::DnsZoneDelete(ref request) => match self.delete_zone(request).await {
                Ok(_data) => app_tx.send(Action::Refresh)?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to delete dns zone: {:?}",
                    err
                )))?,
            },
            _ => {
                todo!()
            }
        }
        Ok(())
    }

    async fn get_recordsets(&mut self, request: &DnsRecordsetFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let res: Vec<Value> = if request.zone_id.is_some() {
                let ep =
                    openstack_sdk::api::dns::v2::zone::recordset::list::RequestBuilder::try_from(
                        request,
                    )?
                    .build()?;
                openstack_sdk::api::paged(ep, Pagination::All)
                    .query_async(session)
                    .await?
            } else {
                let ep = openstack_sdk::api::dns::v2::recordset::list::RequestBuilder::try_from(
                    request,
                )?
                .build()?;
                openstack_sdk::api::paged(ep, Pagination::All)
                    .query_async(session)
                    .await?
            };

            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_zones(&mut self, _request: &DnsZoneFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let mut ep_builder = openstack_sdk::api::dns::v2::zone::list::Request::builder();
            ep_builder.sort_key("name");
            ep_builder.sort_dir("asc");

            let ep = ep_builder.build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn delete_zone(&mut self, request: &DnsZoneDelete) -> Result<()> {
        if let Some(session) = &self.cloud {
            let ep = openstack_sdk::api::dns::v2::zone::delete::Request::builder()
                .id(request.zone_id.clone())
                .build()?;

            openstack_sdk::api::ignore(ep).query_async(session).await?;
            return Ok(());
        }
        Ok(())
    }
}
