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

use openstack_sdk::api::compute::v2::server::volume_attachment::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use structable_derive::StructTable;

/// List volume attachments for an instance.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "List volume attachments for an instance")]
pub struct VolumeAttachmentsArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {
    #[arg(long)]
    limit: Option<i32>,

    #[arg(long)]
    offset: Option<i32>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[arg(value_name = "SERVER_ID", id = "path_param_server_id")]
    server_id: String,
}

/// VolumeAttachments list command
pub struct VolumeAttachmentsCmd {
    pub args: VolumeAttachmentsArgs,
}
/// VolumeAttachments response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// Name of the device in the attachment object, such as, `/dev/vdb`.
    #[serde()]
    #[structable(optional, wide)]
    device: Option<String>,

    /// The volume ID of the attachment.
    ///
    ///
    /// **Available until version 2.88**
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The UUID of the server.
    #[serde(rename = "serverId")]
    #[structable(optional, title = "serverId", wide)]
    server_id: Option<String>,

    /// The UUID of the attached volume.
    #[serde(rename = "volumeId")]
    #[structable(optional, title = "volumeId", wide)]
    volume_id: Option<String>,

    /// The device tag applied to the volume block device or `null`.
    ///
    ///
    /// **New in version 2.70**
    #[serde()]
    #[structable(optional, wide)]
    tag: Option<String>,

    /// A flag indicating if the attached volume will be deleted when the
    /// server is
    /// deleted.
    ///
    ///
    /// **New in version 2.79**
    #[serde()]
    #[structable(optional, wide)]
    delete_on_termination: Option<bool>,

    /// The UUID of the associated volume attachment in Cinder.
    ///
    ///
    /// **New in version 2.89**
    #[serde()]
    #[structable(optional, wide)]
    attachment_id: Option<String>,

    /// The UUID of the block device mapping record in Nova for the attachment.
    ///
    ///
    /// **New in version 2.89**
    #[serde()]
    #[structable(optional, wide)]
    bdm_uuid: Option<String>,
}

#[async_trait]
impl OSCCommand for VolumeAttachmentsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List VolumeAttachments with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        // Set query parameters
        if let Some(val) = &self.args.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.args.query.offset {
            ep_builder.offset(*val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
