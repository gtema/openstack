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

//! `/v2.1/os-aggregates` REST operations bindings of Compute

use eyre::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::AsyncOpenStack;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};
use crate::cloud_worker::ComputeApiRequest;

pub mod delete;
pub mod get;
pub mod list;

pub use delete::*;
pub use get::*;
pub use list::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeAggregateApiRequest {
    /// Delete
    Delete(Box<ComputeAggregateDelete>),
    /// Get
    Get(Box<ComputeAggregateShow>),
    /// List
    List(Box<ComputeAggregateList>),
}

impl From<ComputeAggregateApiRequest> for ApiRequest {
    fn from(item: ComputeAggregateApiRequest) -> Self {
        ApiRequest::Compute(ComputeApiRequest::from(item))
    }
}

impl ExecuteApiRequest for ComputeAggregateApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            ComputeAggregateApiRequest::Delete(ref req) => {
                req.execute_request(session, request, app_tx).await?;
            }
            ComputeAggregateApiRequest::Get(ref req) => {
                req.execute_request(session, request, app_tx).await?;
            }
            ComputeAggregateApiRequest::List(ref req) => {
                req.execute_request(session, request, app_tx).await?;
            }
        }
        Ok(())
    }
}
