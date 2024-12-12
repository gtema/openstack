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

use eyre::{OptionExt, Result, WrapErr};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::{
    api::{paged, Pagination, QueryAsync},
    AsyncOpenStack,
};

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::common::ConfirmableRequest;
use crate::cloud_worker::compute::types::ComputeApiRequest;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

/// Server API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeServerApiRequest {
    /// List servers
    List(ComputeServerList),
    /// Delete server
    Delete(ComputeServerDelete),
    /// Get server console output
    GetConsoleOutput(ComputeServerGetConsoleOutput),
    /// Get instance actions
    InstanceActionList(ComputeServerInstanceActionList),
    /// Get single instance action data
    InstanceActionShow(ComputeServerInstanceActionList),
}

impl From<ComputeServerApiRequest> for ApiRequest {
    fn from(item: ComputeServerApiRequest) -> Self {
        ApiRequest::Compute(ComputeApiRequest::from(item))
    }
}

impl ConfirmableRequest for ComputeServerApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            ComputeServerApiRequest::Delete(x) => x.get_confirm_message(),
            _ => None,
        }
    }
}

impl ExecuteApiRequest for ComputeServerApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            ComputeServerApiRequest::List(ref req) => {
                let ep: openstack_sdk::api::compute::v2::server::list_detailed::Request<'_> =
                    req.try_into().wrap_err("cannot prepare request")?;

                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: paged(ep, Pagination::All).query_async(session).await?,
                })?;
            }
            ComputeServerApiRequest::Delete(ref req) => {
                let ep: openstack_sdk::api::compute::v2::server::delete::Request<'_> =
                    req.try_into().wrap_err("cannot prepare request")?;

                openstack_sdk::api::ignore(ep).query_async(session).await?;

                app_tx.send(Action::Refresh)?;
            }
            ComputeServerApiRequest::GetConsoleOutput(ref req) => {
                let ep = openstack_sdk::api::compute::v2::server::os_get_console_output::Request::
                    try_from(req).wrap_err("cannot prepare request")?;

                let res: Value = ep.query_async(session).await?;
                app_tx.send(Action::ApiResponseData {
                    request: request.clone(),
                    data: res.get("output").unwrap_or(&Value::Null).to_owned(),
                })?;
            }
            ComputeServerApiRequest::InstanceActionList(ref req) => {
                let ep =
                openstack_sdk::api::compute::v2::server::instance_action::list::Request::try_from(req).wrap_err("cannot prepare request")?;

                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: paged(ep, Pagination::All).query_async(session).await?,
                })?;
            }
            ComputeServerApiRequest::InstanceActionShow(ref filters) => {
                let ep =
                openstack_sdk::api::compute::v2::server::instance_action::get::Request::try_from(filters).wrap_err("cannot prepare request")?;

                app_tx.send(Action::ApiResponseData {
                    request: request.clone(),
                    data: ep.query_async(session).await?,
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeServerList {
    /// All tenants (admin only)
    pub all_tenants: Option<bool>,
    /// List servers with specific flavor
    pub flavor_id: Option<String>,
    /// Flavor name (used only for display)
    pub flavor_name: Option<String>,
}

impl fmt::Display for ComputeServerList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.all_tenants.is_some() {
            parts.push(String::from("all"));
        }
        if self.flavor_id.is_some() || self.flavor_name.is_some() {
            parts.push(format!(
                "flavor: {}",
                self.flavor_name
                    .as_ref()
                    .or(self.flavor_name.as_ref())
                    .unwrap_or(&String::new())
            ));
        }
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&ComputeServerList>
    for openstack_sdk::api::compute::v2::server::list_detailed::Request<'_>
{
    type Error = openstack_sdk::api::compute::v2::server::list_detailed::RequestBuilderError;

    fn try_from(value: &ComputeServerList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();

        ep_builder.sort_key("display_name");
        ep_builder.sort_dir("asc");

        if let Some(true) = &value.all_tenants {
            ep_builder.all_tenants("true");
        }

        if let Some(flavor_id) = &value.flavor_id {
            ep_builder.flavor(flavor_id.clone());
        }

        ep_builder.build()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeServerDelete {
    pub server_id: String,
    pub server_name: Option<String>,
}

impl fmt::Display for ComputeServerDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&ComputeServerDelete>
    for openstack_sdk::api::compute::v2::server::delete::Request<'_>
{
    type Error = openstack_sdk::api::compute::v2::server::delete::RequestBuilderError;

    fn try_from(value: &ComputeServerDelete) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();
        ep_builder.id(value.server_id.clone());
        ep_builder.build()
    }
}

impl ConfirmableRequest for ComputeServerDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete server {} ?",
            self.server_name.clone().unwrap_or(self.server_id.clone())
        ))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeServerGetConsoleOutput {
    pub server_id: String,
    pub length: Option<i32>,
}

impl TryFrom<&ComputeServerGetConsoleOutput>
    for openstack_sdk::api::compute::v2::server::os_get_console_output::Request<'_>
{
    type Error = eyre::Report;

    fn try_from(value: &ComputeServerGetConsoleOutput) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();
        ep_builder.id(value.server_id.clone());
        let mut len = openstack_sdk::api::compute::v2::server::os_get_console_output::OsGetConsoleOutputBuilder::default();
        if let Some(length) = value.length {
            len.length(length);
        }
        ep_builder.os_get_console_output(len.build()?);
        Ok(ep_builder.build()?)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeServerInstanceActionList {
    /// Server ID
    pub server_id: Option<String>,
    /// Server name
    pub server_name: Option<String>,
    /// Request id
    pub request_id: Option<String>,
}

impl fmt::Display for ComputeServerInstanceActionList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.server_id.is_some() || self.server_name.is_some() {
            write!(
                f,
                "server: {}",
                self.server_name
                    .as_ref()
                    .or(self.server_name.as_ref())
                    .unwrap_or(&String::new())
            )?;
        }
        write!(f, "")
    }
}

impl TryFrom<&ComputeServerInstanceActionList>
    for openstack_sdk::api::compute::v2::server::instance_action::list::Request<'_>
{
    type Error = eyre::Report;

    fn try_from(filters: &ComputeServerInstanceActionList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();

        ep_builder.server_id(
            filters
                .server_id
                .clone()
                .ok_or_eyre("Server ID must be set")?,
        );

        Ok(ep_builder.build()?)
    }
}

impl TryFrom<&ComputeServerInstanceActionList>
    for openstack_sdk::api::compute::v2::server::instance_action::get::Request<'_>
{
    type Error = eyre::Report;

    fn try_from(filters: &ComputeServerInstanceActionList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();

        ep_builder.server_id(
            filters
                .server_id
                .clone()
                .ok_or_eyre("Server ID must be set")?,
        );
        ep_builder.id(filters
            .request_id
            .clone()
            .ok_or_eyre("InstanceAction ID must be set")?);

        Ok(ep_builder.build()?)
    }
}
