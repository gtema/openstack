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
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

use crate::cloud_worker::ConfirmableRequest;
use openstack_sdk::api::ignore;
use openstack_sdk::api::load_balancer::v2::healthmonitor::delete::RequestBuilder;
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadBalancerHealthmonitorDelete {
    id: String,
    name: Option<String>,
}

impl fmt::Display for LoadBalancerHealthmonitorDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            ": {}",
            self.name.clone().unwrap_or(self.id.clone())
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl From<&LoadBalancerHealthmonitorDelete> for RequestBuilder<'_> {
    fn from(value: &LoadBalancerHealthmonitorDelete) -> Self {
        let mut ep_builder = Self::default();
        ep_builder.id(value.id.clone());
        ep_builder
    }
}

impl ExecuteApiRequest for LoadBalancerHealthmonitorDelete {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = Into::<RequestBuilder>::into(self)
            .build()
            .wrap_err("Cannot prepare request")?;
        ignore(ep).query_async(session).await?;
        Ok(())
    }
}
impl ConfirmableRequest for LoadBalancerHealthmonitorDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete LoadBalancer Healthmonitor {} ?",
            self.name.clone().unwrap_or(self.id.clone())
        ))
    }
}
