//! Signals the Image Service to complete the image import workflow
//! by processing data that has been made available to the OpenStack
//! image service.
//! *(Since Image API v2.6)*
//!
//! In the `glance-direct` workflow, the data has been made available to the
//! Image service via the [Stage binary image data](#image-stage-call) API
//! call.
//!
//! In the `web-download` workflow, the data is made available to the Image
//! service by being posted to an accessible location with a URL that you know.
//!
//! In the `copy-image` workflow, the data is made available to the Image
//! service by copying existing image data to the staging area.
//!
//! In the `glance-download` workflow, the data is made available to the Image
//! service by fetching an image accessible from another glance service
//! specified
//! by a region name and an image id that you know.
//!
//! Beginning with API version 2.8, an optional `stores` parameter may be added
//! to the body request. When present, it contains the list of backing store
//! identifiers to import the image binary data to. If at least one store
//! identifier specified is not recognized, a 409 (Conflict) response is
//! returned.
//! When the parameter is not present, the image data is placed into the
//! default
//! backing store.
//!
//! For backwards compatibility, if the `stores` parameter is not specified,
//! the
//! header ‘X-Image-Meta-Store’ is evaluated.
//!
//! To import the data into the entire set of stores you may consume from this
//! particular deployment of Glance without specifying each one of them, you
//! can
//! use the optional boolean body parameter `all\_stores`.
//! Note that this can’t be used simultaneously with the `stores` parameter.
//!
//! To set the behavior of the import workflow in case of error, you can use
//! the
//! optional boolean body parameter `all\_stores\_must\_succeed`.
//! When set to True (default), if an error occurs during the upload in at
//! least one store,
//! the worfklow fails, the data is deleted from stores where copying is done
//! and
//! the state of the image remains unchanged.
//! When set to False, the workflow will fail only if the upload fails
//! on all stores specified. In case of a partial success, the locations added
//! to
//! the image will be the stores where the data has been correctly uploaded.
//!
//! The JSON request body specifies what import method you wish to use
//! for this image request.
//!
//! **Preconditions**
//!
//! Before you can complete the interoperable image import workflow, you must
//! meet
//! the following preconditions:
//!
//! **Additional Preconditions**
//!
//! If you are using the `glance-direct` import method:
//!
//! If you are using the `web-download` import method:
//!
//! If you are using the `copy-image` import method:
//!
//! If you are using the `glance-download` import method:
//!
//! **Synchronous Postconditions**
//!
//! Normal response codes: 202
//!
//! Error response codes: 400, 401, 403, 404, 405, 409, 410, 413, 415, 503
//!
//! If the image import process is not enabled in your cloud, this request
//! will result in a 404 response code with an appropriate message.
//!
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::image::v2::image::import::create;
use openstack_sdk::api::RawQueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ImportArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    method: Option<Method>,
    #[arg(action=clap::ArgAction::Append, long)]
    stores: Option<Vec<String>>,
    #[arg(action=clap::ArgAction::Set, long)]
    all_stores: Option<bool>,
    #[arg(action=clap::ArgAction::Set, long)]
    all_stores_must_success: Option<bool>,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg()]
    image_id: String,
}
/// Method Body data
#[derive(Args, Debug, Clone)]
struct Method {
    #[arg(long)]
    name: Option<String>,

    #[arg(long)]
    uri: Option<String>,

    #[arg(long)]
    glance_image_id: Option<String>,

    #[arg(long)]
    glance_region: Option<String>,

    #[arg(long)]
    glance_service_interface: Option<String>,
}

/// Import create command
pub struct ImportCmd {
    pub args: ImportArgs,
}
/// Import response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for ImportCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Import with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.image_id(&self.args.path.image_id);
        // Set query parameters
        // Set body parameters
        // Set Request.method data
        if let Some(args) = &self.args.method {
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
        if let Some(args) = &self.args.stores {
            ep_builder.stores(args.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        // Set Request.all_stores data
        if let Some(args) = &self.args.all_stores {
            ep_builder.all_stores(*args);
        }

        // Set Request.all_stores_must_success data
        if let Some(args) = &self.args.all_stores_must_success {
            ep_builder.all_stores_must_success(*args);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}