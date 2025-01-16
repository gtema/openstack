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

use crate::cloud_worker::ConfirmableRequest;
use openstack_sdk::api::ignore;
use openstack_sdk::api::load_balancer::v2::pool::member::delete::RequestBuilder;
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct LoadBalancerPoolMemberDelete {
    pub id: String,
    #[builder(default)]
    pub name: Option<String>,
    pub pool_id: String,
    #[builder(default)]
    pub pool_name: Option<String>,
}

impl fmt::Display for LoadBalancerPoolMemberDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "name/id: {}",
            self.name.clone().unwrap_or(self.id.clone())
        ));
        parts.push(format!(
            "pool: {}",
            self.pool_name.clone().unwrap_or(self.pool_id.clone())
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&LoadBalancerPoolMemberDelete> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &LoadBalancerPoolMemberDelete) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        ep_builder.pool_id(value.pool_id.clone());
        ep_builder.id(value.id.clone());

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for LoadBalancerPoolMemberDelete {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        _request: &ApiRequest,
        _app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = TryInto::<RequestBuilder>::try_into(self)?
            .build()
            .wrap_err("Cannot prepare request")?;
        ignore(ep).query_async(session).await?;
        Ok(())
    }
}
impl ConfirmableRequest for LoadBalancerPoolMemberDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete LoadBalancer Member {} ?",
            self.name.clone().unwrap_or(self.id.clone())
        ))
    }
}
