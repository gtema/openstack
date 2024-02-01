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

use openstack_sdk::AsyncOpenStack;

use crate::common::parse_key_val;
use openstack_sdk::api::compute::v2::server::create_image_20;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

#[derive(Args, Clone, Debug)]
#[command(about = "Create Image (createImage Action) (microversion = 2.0)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    create_image: CreateImage,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}
/// CreateImage Body data
#[derive(Args, Debug, Clone)]
struct CreateImage {
    /// The display name of an Image.
    #[arg(long)]
    name: String,

    /// Metadata key and value pairs for the image.
    /// The maximum size for each metadata key and value pair is 255 bytes.
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,
}

/// Server action command
pub struct ServerCmd {
    pub args: ServerArgs,
}
/// Server response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The UUID for the resulting image snapshot.
    #[serde()]
    #[structable()]
    image_id: String,
}

#[async_trait]
impl OSCCommand for ServerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create_image_20::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.0");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.create_image data
        let args = &self.args.create_image;
        let mut create_image_builder = create_image_20::CreateImageBuilder::default();

        create_image_builder.name(args.name.clone());

        if let Some(val) = &args.metadata {
            create_image_builder.metadata(val.iter().cloned());
        }

        ep_builder.create_image(create_image_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
