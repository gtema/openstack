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

pub trait ImageExt {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()>;

    async fn get_images(&mut self, filters: &ImageFilters) -> Result<Vec<Value>>;
}

impl ImageExt for Cloud {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()> {
        match request {
            ApiRequest::ImageImages(ref filters) => match self.get_images(filters).await {
                Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                Err(err) => {
                    app_tx.send(Action::Error(format!("Failed to fetch images: {:?}", err)))?
                }
            },
            _ => {
                todo!()
            }
        }
        Ok(())
    }

    async fn get_images(&mut self, filters: &ImageFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let mut ep_builder = openstack_sdk::api::image::v2::image::list::Request::builder();
            ep_builder.sort_key("name");
            ep_builder.sort_dir("asc");

            if let Some(vis) = &filters.visibility {
                ep_builder.visibility(vis);
            }
            let ep = ep_builder.build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }
}
