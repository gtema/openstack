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

use crate::utils::StructTable;
use openstack_sdk::api::compute::v2::quota_set::details::RequestBuilder;
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct ComputeQuotaSetShow {
    pub id: String,
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub user_id: Option<String>,
    #[builder(default)]
    pub user_name: Option<String>,
}

impl fmt::Display for ComputeQuotaSetShow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "name/id: {}",
            self.name.clone().unwrap_or(self.id.clone())
        ));
        if self.user_id.is_some() || self.user_name.is_some() {
            parts.push(format!(
                "user: {}",
                self.user_name
                    .as_ref()
                    .or(self.user_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&ComputeQuotaSetShow> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &ComputeQuotaSetShow) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        ep_builder.id(value.id.clone());
        if let Some(val) = &value.user_id {
            ep_builder.user_id(val.clone());
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for ComputeQuotaSetShow {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = TryInto::<RequestBuilder>::try_into(self)?
            .build()
            .wrap_err("Cannot prepare request")?;
        app_tx.send(Action::ApiResponseData {
            request: request.clone(),
            data: ep.query_async(session).await?,
        })?;
        Ok(())
    }
}
