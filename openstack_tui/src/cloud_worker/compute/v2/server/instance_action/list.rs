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
use openstack_sdk::api::compute::v2::server::instance_action::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct ComputeServerInstanceActionList {
    #[builder(default)]
    pub changes_before: Option<String>,
    #[builder(default)]
    pub changes_since: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub marker: Option<String>,
    pub server_id: String,
    #[builder(default)]
    pub server_name: Option<String>,
}

impl fmt::Display for ComputeServerInstanceActionList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "server: {}",
            self.server_name.clone().unwrap_or(self.server_id.clone())
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&ComputeServerInstanceActionList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &ComputeServerInstanceActionList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        ep_builder.server_id(value.server_id.clone());
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &value.changes_since {
            ep_builder.changes_since(val.clone());
        }
        if let Some(val) = &value.changes_before {
            ep_builder.changes_before(val.clone());
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for ComputeServerInstanceActionList {
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
/// ComputeServerInstanceAction response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ComputeServerInstanceAction {
    /// List of the actions for the given instance in descending order of
    /// creation.
    ///
    #[serde(default, rename = "instanceActions")]
    #[structable(optional, title = "INSTANCEACTIONS", wide)]
    pub instance_actions: Option<Value>,
}
