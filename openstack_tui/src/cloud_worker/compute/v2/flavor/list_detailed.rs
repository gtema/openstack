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

use openstack_sdk::api::compute::v2::flavor::list_detailed::RequestBuilder;
use openstack_sdk::api::{Pagination, paged};
use openstack_sdk::{AsyncOpenStack, api::QueryAsync};

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct ComputeFlavorList {
    #[builder(default)]
    pub is_public: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub marker: Option<String>,
    #[builder(default)]
    pub min_disk: Option<String>,
    #[builder(default)]
    pub min_ram: Option<String>,
    #[builder(default)]
    pub sort_dir: Option<String>,
    #[builder(default)]
    pub sort_key: Option<String>,
}

impl fmt::Display for ComputeFlavorList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts: Vec<String> = Vec::new();
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&ComputeFlavorList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &ComputeFlavorList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &value.is_public {
            ep_builder.is_public(val.clone());
        }
        if let Some(val) = &value.min_ram {
            ep_builder.min_ram(val.clone());
        }
        if let Some(val) = &value.min_disk {
            ep_builder.min_disk(val.clone());
        }
        if let Some(val) = &value.sort_key {
            ep_builder.sort_key(val.clone());
        }
        if let Some(val) = &value.sort_dir {
            ep_builder.sort_dir(val.clone());
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for ComputeFlavorList {
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
