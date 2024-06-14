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

//! Create Import command
//!
//! Wraps invoking of the `v2/images/{image_id}/import` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::image::v2::image::import::create;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Signals the Image Service to complete the image import workflow by
/// processing data that has been made available to the OpenStack image
/// service. *(Since Image API v2.6)*
///
/// In the `glance-direct` workflow, the data has been made available to the
/// Image service via the [Stage binary image data](#image-stage-call) API
/// call.
///
/// In the `web-download` workflow, the data is made available to the Image
/// service by being posted to an accessible location with a URL that you know.
///
/// In the `copy-image` workflow, the data is made available to the Image
/// service by copying existing image data to the staging area.
///
/// In the `glance-download` workflow, the data is made available to the Image
/// service by fetching an image accessible from another glance service
/// specified by a region name and an image id that you know.
///
/// Beginning with API version 2.8, an optional `stores` parameter may be added
/// to the body request. When present, it contains the list of backing store
/// identifiers to import the image binary data to. If at least one store
/// identifier specified is not recognized, a 409 (Conflict) response is
/// returned. When the parameter is not present, the image data is placed into
/// the default backing store.
///
/// For backwards compatibility, if the `stores` parameter is not specified,
/// the header ‘X-Image-Meta-Store’ is evaluated.
///
/// To import the data into the entire set of stores you may consume from this
/// particular deployment of Glance without specifying each one of them, you
/// can use the optional boolean body parameter `all_stores`. Note that this
/// can’t be used simultaneously with the `stores` parameter.
///
/// To set the behavior of the import workflow in case of error, you can use
/// the optional boolean body parameter `all_stores_must_succeed`. When set to
/// True (default), if an error occurs during the upload in at least one store,
/// the workflow fails, the data is deleted from stores where copying is done
/// and the state of the image remains unchanged. When set to False, the
/// workflow will fail only if the upload fails on all stores specified. In
/// case of a partial success, the locations added to the image will be the
/// stores where the data has been correctly uploaded.
///
/// The JSON request body specifies what import method you wish to use for this
/// image request.
///
/// **Preconditions**
///
/// Before you can complete the interoperable image import workflow, you must
/// meet the following preconditions:
///
/// **Additional Preconditions**
///
/// If you are using the `glance-direct` import method:
///
/// If you are using the `web-download` import method:
///
/// If you are using the `copy-image` import method:
///
/// If you are using the `glance-download` import method:
///
/// **Synchronous Postconditions**
///
/// Normal response codes: 202
///
/// Error response codes: 400, 401, 403, 404, 405, 409, 410, 413, 415, 503
///
/// If the image import process is not enabled in your cloud, this request will
/// result in a 404 response code with an appropriate message.
///
#[derive(Args)]
#[command(about = "Import an image")]
pub struct ImportCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// When set to True the data will be imported to the set of stores you may
    /// consume from this particular deployment of Glance (ie: the same set of
    /// stores returned to a call to /v2/info/stores on the glance-api the
    /// request hits). This can’t be used simultaneously with the `stores`
    /// parameter.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    all_stores: Option<bool>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    all_stores_must_success: Option<bool>,

    /// A JSON object indicating what import method you wish to use to import
    /// your image. The content of this JSON object is another JSON object with
    /// a `name` field whose value is the identifier for the import method.
    ///
    #[command(flatten)]
    method: Option<Method>,

    /// If present contains the list of store id to import the image binary
    /// data to.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    stores: Option<Vec<String>>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// image_id parameter for /v2/images/{image_id}/import API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_image_id",
        value_name = "IMAGE_ID"
    )]
    image_id: String,
}
/// Method Body data
#[derive(Args, Clone)]
struct Method {
    #[arg(help_heading = "Body parameters", long)]
    glance_image_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    glance_region: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    glance_service_interface: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    uri: Option<String>,
}

/// Import response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ImportCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Import");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.image_id(&self.path.image_id);
        // Set query parameters
        // Set body parameters
        // Set Request.all_stores data
        if let Some(arg) = &self.all_stores {
            ep_builder.all_stores(*arg);
        }

        // Set Request.all_stores_must_success data
        if let Some(arg) = &self.all_stores_must_success {
            ep_builder.all_stores_must_success(*arg);
        }

        // Set Request.method data
        if let Some(args) = &self.method {
            let mut method_builder = create::MethodBuilder::default();
            if let Some(val) = &args.name {
                method_builder.name(val);
            }

            if let Some(val) = &args.uri {
                method_builder.uri(val);
            }

            if let Some(val) = &args.glance_image_id {
                method_builder.glance_image_id(val);
            }

            if let Some(val) = &args.glance_region {
                method_builder.glance_region(val);
            }

            if let Some(val) = &args.glance_service_interface {
                method_builder.glance_service_interface(val);
            }

            ep_builder.method(method_builder.build().unwrap());
        }

        // Set Request.stores data
        if let Some(arg) = &self.stores {
            ep_builder.stores(arg.iter().map(Into::into).collect::<Vec<_>>());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
