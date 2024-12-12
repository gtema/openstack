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

/// Hypervisor API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeHypervisorApiRequest {
    /// List
    List(ComputeHypervisorList),
}

impl From<ComputeHypervisorApiRequest> for ApiRequest {
    fn from(item: ComputeHypervisorApiRequest) -> Self {
        ApiRequest::Compute(ComputeApiRequest::from(item))
    }
}

impl ExecuteApiRequest for ComputeHypervisorApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            ComputeHypervisorApiRequest::List(ref filters) => {
                let ep: openstack_sdk::api::compute::v2::hypervisor::list_detailed::Request<'_> =
                    filters.try_into().wrap_err("Cannot prepare request")?;

                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: ep
                        .query_async(session)
                        .await
                        .wrap_err("fetching hypervisors failed")?,
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeHypervisorList {}

impl fmt::Display for ComputeHypervisorList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&ComputeHypervisorList>
    for openstack_sdk::api::compute::v2::hypervisor::list_detailed::Request<'_>
{
    type Error = openstack_sdk::api::compute::v2::hypervisor::list_detailed::RequestBuilderError;

    fn try_from(_value: &ComputeHypervisorList) -> Result<Self, Self::Error> {
        openstack_sdk::api::compute::v2::hypervisor::list_detailed::Request::builder().build()
    }
}
