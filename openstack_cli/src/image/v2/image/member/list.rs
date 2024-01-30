//! Lists the tenants that share this image.
//! *(Since Image API v2.1)*
//!
//! If the image owner makes this call, the complete member list is
//! returned.
//!
//! If a user who is an image member makes this call, the member list
//! contains only information for that user.
//!
//! If a user who is not an image member makes this call, the call
//! returns the HTTP `404` response code.
//!
//! Preconditions
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 403, 404
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

use openstack_sdk::api::image::v2::image::member::list;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct MembersArgs {
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
pub struct PathParameters {
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg()]
    image_id: String,
}

/// Members list command
pub struct MembersCmd {
    pub args: MembersArgs,
}
/// Members response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// An identifier for the image member (tenantId)
    #[serde()]
    #[structable(optional)]
    member_id: Option<String>,

    /// An identifier for the image
    #[serde()]
    #[structable(optional)]
    image_id: Option<String>,

    /// Date and time of image member creation
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Date and time of last modification of image member
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The status of this image member
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    #[serde()]
    #[structable(optional)]
    schema: Option<String>,
}

#[async_trait]
impl OSCCommand for MembersCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Members with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.image_id(&self.args.path.image_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
