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

use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::compute::types::ComputeApiRequest;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

/// QuotaSet
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeQuotaSetApiRequest {
    /// Details
    Details(ComputeQuotaSetDetails),
}

impl From<ComputeQuotaSetApiRequest> for ApiRequest {
    fn from(item: ComputeQuotaSetApiRequest) -> Self {
        ApiRequest::Compute(ComputeApiRequest::from(item))
    }
}

impl ExecuteApiRequest for ComputeQuotaSetApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            ComputeQuotaSetApiRequest::Details(ref filters) => {
                let ep: openstack_sdk::api::compute::v2::quota_set::details::Request<'_> =
                    filters.try_into().wrap_err("Cannot prepare request")?;
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
pub struct ComputeQuotaSetDetails {
    pub project_id: String,
}

impl fmt::Display for ComputeQuotaSetDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&ComputeQuotaSetDetails>
    for openstack_sdk::api::compute::v2::quota_set::details::Request<'_>
{
    type Error = openstack_sdk::api::compute::v2::quota_set::details::RequestBuilderError;

    fn try_from(value: &ComputeQuotaSetDetails) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();

        ep_builder.id(value.project_id.clone());
        ep_builder.build()
    }
}