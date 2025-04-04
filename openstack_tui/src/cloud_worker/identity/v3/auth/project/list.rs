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
use openstack_sdk::api::identity::v3::auth::project::list::RequestBuilder;
use openstack_sdk::{AsyncOpenStack, api::QueryAsync};
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct IdentityAuthProjectList {}

impl fmt::Display for IdentityAuthProjectList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts: Vec<String> = Vec::new();
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&IdentityAuthProjectList> for RequestBuilder {
    type Error = Report;
    fn try_from(value: &IdentityAuthProjectList) -> Result<Self, Self::Error> {
        let ep_builder = Self::default();

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for IdentityAuthProjectList {
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
            data: ep.query_async(session).await?,
        })?;
        Ok(())
    }
}
/// IdentityAuthProject response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct IdentityAuthProject {
    /// The ID of the domain for the project.
    ///
    #[serde(default)]
    #[structable(optional, title = "DOMAIN_ID", wide)]
    pub domain_id: Option<String>,

    /// If set to `true`, project is enabled. If set to `false`, project is
    /// disabled.
    ///
    #[serde(default)]
    #[structable(optional, title = "ENABLED", wide)]
    pub enabled: Option<bool>,

    /// The ID for the project.
    ///
    #[serde(default)]
    #[structable(optional, title = "ID", wide)]
    pub id: Option<String>,

    /// The name of the project.
    ///
    #[serde(default)]
    #[structable(optional, title = "NAME")]
    pub name: Option<String>,
}
