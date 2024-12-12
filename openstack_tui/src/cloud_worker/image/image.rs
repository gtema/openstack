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

use openstack_sdk::{
    api::{paged, Pagination, QueryAsync},
    AsyncOpenStack,
};

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::image::types::ImageApiRequest;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};
use crate::cloud_worker::ConfirmableRequest;

/// Image API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageImageApiRequest {
    /// Delete
    Delete(ImageImageDelete),
    /// List Images
    List(ImageImageList),
}

impl From<ImageImageApiRequest> for ApiRequest {
    fn from(item: ImageImageApiRequest) -> Self {
        ApiRequest::Image(ImageApiRequest::from(item))
    }
}

impl ConfirmableRequest for ImageImageApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            ImageImageApiRequest::Delete(req) => req.get_confirm_message(),
            _ => None,
        }
    }
}

impl ExecuteApiRequest for ImageImageApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            ImageImageApiRequest::Delete(ref filters) => {
                let ep =
                    TryInto::<openstack_sdk::api::image::v2::image::delete::Request<'_>>::try_into(
                        filters,
                    )
                    .wrap_err("Cannot prepare request")?;
                openstack_sdk::api::ignore(ep).query_async(session).await?;
                app_tx.send(Action::Refresh)?;
            }
            ImageImageApiRequest::List(ref filters) => {
                let ep: openstack_sdk::api::image::v2::image::list::Request<'_> =
                    filters.try_into().wrap_err("Cannot prepare request")?;
                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: paged(ep, Pagination::All).query_async(session).await?,
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageImageList {
    pub visibility: Option<String>,
}

impl fmt::Display for ImageImageList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(val) = &self.visibility {
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

impl TryFrom<&ImageImageList> for openstack_sdk::api::image::v2::image::list::Request<'_> {
    type Error = openstack_sdk::api::image::v2::image::list::RequestBuilderError;

    fn try_from(value: &ImageImageList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();
        ep_builder.sort_key("name");
        ep_builder.sort_dir("asc");

        if let Some(vis) = &value.visibility {
            ep_builder.visibility(vis.clone());
        }

        ep_builder.build()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageImageDelete {
    pub image_id: String,
    pub image_name: Option<String>,
}

impl fmt::Display for ImageImageDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl ConfirmableRequest for ImageImageDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete image {} ?",
            self.image_name.clone().unwrap_or(self.image_id.clone())
        ))
    }
}

impl TryFrom<&ImageImageDelete> for openstack_sdk::api::image::v2::image::delete::Request<'_> {
    type Error = openstack_sdk::api::image::v2::image::delete::RequestBuilderError;

    fn try_from(value: &ImageImageDelete) -> Result<Self, Self::Error> {
        Self::builder().id(value.image_id.clone()).build()
    }
}
