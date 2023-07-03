//! Shows a JSON schema document that represents an images entity.
//! An images entity is a container of image entities.
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

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::image::v2::schemas::images::get;
use openstack_sdk::api::RawQueryAsync;

/// Shows a JSON schema document that represents an images entity.
/// An images entity is a container of image entities.
#[derive(Args, Clone, Debug)]
pub struct ImagesArgs {}

pub struct ImagesCmd {
    pub args: ImagesArgs,
}

/// Images
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Images {}

#[async_trait]
impl Command for ImagesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Images with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Schema::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client.discover_service_endpoint("image").await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data: serde_json::Value = serde_json::from_slice(rsp.body())?;
        op.output_machine(data)?;
        Ok(())
    }
}
