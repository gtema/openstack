//! Reactivates an image. (Since Image API v2.3)
//! By default, this operation is restricted to administrators only.
//! The reactivate operation returns an error if the image status is not active
//! or deactivated.
//! Preconditions:
//!   - The image must exist.
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::find;
use openstack_sdk::api::image::v2::image::actions::reactivate::post;
use openstack_sdk::api::image::v2::image::find;
use openstack_sdk::api::RawQueryAsync;

/// Reactivates an image. (Since Image API v2.3)
/// By default, this operation is restricted to administrators only.
/// The reactivate operation returns an error if the image status is not active
/// or deactivated.
/// Preconditions:
///   - The image must exist.
#[derive(Args, Clone, Debug)]
pub struct ImageArgs {
    /// Image ID
    #[arg()]
    id: String,
}

pub struct ImageCmd {
    pub args: ImageArgs,
}

/// Image
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Image {}

#[async_trait]
impl Command for ImageCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Image with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = post::Image::builder();
        // Set path parameters
        ep_builder.id(&self.args.id);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Image)
            .await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = Image {};
        // Maybe output some headers metadata
        op.output_human::<Image>(&data)?;
        Ok(())
    }
}
