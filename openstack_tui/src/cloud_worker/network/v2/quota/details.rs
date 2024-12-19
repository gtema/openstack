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
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

use openstack_sdk::api::network::v2::quota::details::RequestBuilder;
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct NetworkQuotaShow {
    pub id: String,
    #[builder(default)]
    pub name: Option<String>,
}

impl fmt::Display for NetworkQuotaShow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "name/id: {}",
            self.name.clone().unwrap_or(self.id.clone())
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl From<&NetworkQuotaShow> for RequestBuilder<'_> {
    fn from(value: &NetworkQuotaShow) -> Self {
        let mut ep_builder = Self::default();
        ep_builder.id(value.id.clone());
        ep_builder
    }
}

impl ExecuteApiRequest for NetworkQuotaShow {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = Into::<RequestBuilder>::into(self)
            .build()
            .wrap_err("Cannot prepare request")?;
        app_tx.send(Action::ApiResponseData {
            request: request.clone(),
            data: ep.query_async(session).await?,
        })?;
        Ok(())
    }
}
