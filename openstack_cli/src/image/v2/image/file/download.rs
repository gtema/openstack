//! Downloads binary image data.
//! *(Since Image API v2.0)*
//!
//! Example call: `curl -i -X GET -H "X-Auth-Token: $token"
//! $image\_url/v2/images/{image\_id}/file`
//!
//! The response body contains the raw binary data that represents the
//! actual virtual disk. The `Content-Type` header contains the
//! `application/octet-stream` value. The `Content-MD5` header
//! contains an MD5 checksum of the image data. Use this checksum to
//! verify the integrity of the image data.
//!
//! **Preconditions**
//!
//! **Synchronous Postconditions**
//!
//! Normal response codes: 200, 204, 206
//!
//! Error response codes: 400, 403, 404, 416
//!
use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use crate::common::download_file;

use openstack_sdk::api::find;
use openstack_sdk::api::image::v2::image::file::download;
use openstack_sdk::api::image::v2::image::find;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::RawQueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct FileArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Destination filename (using "-" will print object to stdout)
    #[arg(long)]
    file: Option<String>,
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

/// File download command
pub struct FileCmd {
    pub args: FileArgs,
}
/// File response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl OSCCommand for FileCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Download File with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let find_ep = find::Request::builder()
            .id(&self.args.path.image_id)
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let image_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let image_id = image_data["id"]
            .as_str()
            .expect("Image ID is a string")
            .to_string();
        let image_name = image_data["name"]
            .as_str()
            .expect("Image name is a string")
            .to_string();

        let ep = download::Request::builder()
            .image_id(image_id)
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let (headers, data) = ep.download_async(client).await?;

        let size: u64 = headers
            .get("content-length")
            .map(|x| x.to_str().expect("Header is a string"))
            .unwrap_or("0")
            .parse()
            .unwrap();
        download_file(self.args.file.clone().unwrap_or(image_name), size, data).await?;
        Ok(())
    }
}
