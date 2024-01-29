//! *(Since Images v2.0)*
//!
//! Shows a JSON schema document that represents an *images* entity.
//!
//! An images entity is a container of image entities.
//!
//! The following schema is solely an example. Consider only the
//! response to the API call as authoritative.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401
//!
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;

use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, OSCCommand};

use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::image::v2::schema::images::get;
use openstack_sdk::api::RawQueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ImagesArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// Images json command
pub struct ImagesCmd {
    pub args: ImagesArgs,
}
/// Images response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl OSCCommand for ImagesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Json Images with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let ep_builder = get::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data: serde_json::Value = serde_json::from_slice(rsp.body())?;
        op.output_machine(data)?;
        Ok(())
    }
}
