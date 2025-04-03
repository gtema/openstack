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
use openstack_sdk::api::identity::v3::group::user::delete::RequestBuilder;
use openstack_sdk::api::ignore;
use openstack_sdk::{AsyncOpenStack, api::QueryAsync};

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct IdentityGroupUserDelete {
    pub group_id: String,
    #[builder(default)]
    pub group_name: Option<String>,
    pub id: String,
    #[builder(default)]
    pub name: Option<String>,
}

impl fmt::Display for IdentityGroupUserDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "group: {}",
            self.group_name.clone().unwrap_or(self.group_id.clone())
        ));
        parts.push(format!(
            "name/id: {}",
            self.name.clone().unwrap_or(self.id.clone())
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&IdentityGroupUserDelete> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &IdentityGroupUserDelete) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        ep_builder.group_id(value.group_id.clone());
        ep_builder.id(value.id.clone());

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for IdentityGroupUserDelete {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = TryInto::<RequestBuilder>::try_into(self)?
            .build()
            .wrap_err("Cannot prepare request")?;
        ignore(ep).query_async(session).await?;
        // Let caller know deletion was completed
        app_tx.send(Action::ApiResponseData {
            request: request.clone(),
            data: serde_json::Value::Null,
        })?;
        Ok(())
    }
}
impl ConfirmableRequest for IdentityGroupUserDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete Identity User {} ?",
            self.name.clone().unwrap_or(self.id.clone())
        ))
    }
}
