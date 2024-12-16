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
use openstack_sdk::api::dns::v2::zone::recordset::delete::RequestBuilder;
use openstack_sdk::api::ignore;
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsRecordsetDelete {
    id: String,
    name: Option<String>,
    zone_id: String,
    zone_name: Option<String>,
}

impl fmt::Display for DnsRecordsetDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "zone: {}",
            self.zone_name.clone().unwrap_or(self.zone_id.clone())
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl From<&DnsRecordsetDelete> for RequestBuilder<'_> {
    fn from(value: &DnsRecordsetDelete) -> Self {
        let mut ep_builder = Self::default();
        ep_builder.zone_id(value.zone_id.clone());
        ep_builder.id(value.id.clone());
        ep_builder
    }
}

impl ExecuteApiRequest for DnsRecordsetDelete {
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
impl ConfirmableRequest for DnsRecordsetDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete Dns Recordset {} ?",
            self.name.clone().unwrap_or(self.id.clone())
        ))
    }
}
