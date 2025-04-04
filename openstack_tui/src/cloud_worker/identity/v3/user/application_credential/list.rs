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
use openstack_sdk::api::identity::v3::user::application_credential::list::RequestBuilder;
use openstack_sdk::{AsyncOpenStack, api::QueryAsync};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct IdentityUserApplicationCredentialList {
    #[builder(default)]
    pub name: Option<String>,
    pub user_id: String,
    #[builder(default)]
    pub user_name: Option<String>,
}

impl fmt::Display for IdentityUserApplicationCredentialList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "user: {}",
            self.user_name.clone().unwrap_or(self.user_id.clone())
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&IdentityUserApplicationCredentialList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &IdentityUserApplicationCredentialList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        ep_builder.user_id(value.user_id.clone());
        if let Some(val) = &value.name {
            ep_builder.name(val.clone());
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for IdentityUserApplicationCredentialList {
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
/// IdentityUserApplicationCredential response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct IdentityUserApplicationCredential {
    /// A list of access_rules objects
    ///
    #[serde(default)]
    #[structable(optional, title = "ACCESS_RULES", wide)]
    pub access_rules: Option<Value>,

    /// A description of the application credential's purpose.
    ///
    #[serde(default)]
    #[structable(optional, title = "DESCRIPTION", wide)]
    pub description: Option<String>,

    #[serde(default)]
    #[structable(optional, title = "EXPIRES_AT", wide)]
    pub expires_at: Option<String>,

    /// The ID of the application credential.
    ///
    #[serde(default)]
    #[structable(optional, title = "ID", wide)]
    pub id: Option<String>,

    /// The name of the application credential. Must be unique to a user.
    ///
    #[serde(default)]
    #[structable(optional, title = "NAME")]
    pub name: Option<String>,

    /// The ID of the project the application credential was created for and
    /// that authentication requests using this application credential will be
    /// scoped to.
    ///
    #[serde(default)]
    #[structable(optional, title = "PROJECT_ID", wide)]
    pub project_id: Option<String>,

    /// An optional list of role objects, identified by ID or name. The list
    /// may only contain roles that the user has assigned on the project. If
    /// not provided, the roles assigned to the application credential will be
    /// the same as the roles in the current token.
    ///
    #[serde(default)]
    #[structable(optional, title = "ROLES", wide)]
    pub roles: Option<Value>,

    /// An optional flag to restrict whether the application credential may be
    /// used for the creation or destruction of other application credentials
    /// or trusts. Defaults to false.
    ///
    #[serde(default)]
    #[structable(optional, title = "UNRESTRICTED", wide)]
    pub unrestricted: Option<bool>,
}
