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
use openstack_sdk::api::compute::v2::aggregate::list::RequestBuilder;
use openstack_sdk::{AsyncOpenStack, api::QueryAsync};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct ComputeAggregateList {}

impl fmt::Display for ComputeAggregateList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts: Vec<String> = Vec::new();
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&ComputeAggregateList> for RequestBuilder {
    type Error = Report;
    fn try_from(value: &ComputeAggregateList) -> Result<Self, Self::Error> {
        let ep_builder = Self::default();

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for ComputeAggregateList {
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
/// ComputeAggregate response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ComputeAggregate {
    /// The availability zone of the host aggregate.
    ///
    #[serde(default)]
    #[structable(optional, title = "AVAILABILITY_ZONE", wide)]
    pub availability_zone: Option<String>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    #[serde()]
    #[structable(title = "CREATED_AT")]
    pub created_at: String,

    /// A boolean indicates whether this aggregate is deleted or not, if it has
    /// not been deleted, `false` will appear.
    ///
    #[serde()]
    #[structable(title = "DELETED", wide)]
    pub deleted: bool,

    /// The date and time when the resource was deleted. If the resource has
    /// not been deleted yet, this field will be `null`, The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    #[serde(default)]
    #[structable(optional, title = "DELETED_AT", wide)]
    pub deleted_at: Option<String>,

    /// A list of host ids in this aggregate.
    ///
    #[serde(default)]
    #[structable(optional, title = "HOSTS", wide)]
    pub hosts: Option<Value>,

    /// The ID of the host aggregate.
    ///
    #[serde()]
    #[structable(title = "ID", wide)]
    pub id: i32,

    /// Metadata key and value pairs associated with the aggregate.
    ///
    #[serde(default)]
    #[structable(optional, title = "METADATA", wide)]
    pub metadata: Option<Value>,

    /// The name of the host aggregate.
    ///
    #[serde()]
    #[structable(title = "NAME")]
    pub name: String,

    /// The date and time when the resource was updated, if the resource has
    /// not been updated, this field will show as `null`. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    #[serde(default)]
    #[structable(optional, title = "UPDATED_AT")]
    pub updated_at: Option<String>,

    /// The UUID of the host aggregate.
    ///
    /// **New in version 2.41**
    ///
    #[serde()]
    #[structable(title = "UUID", wide)]
    pub uuid: String,
}
