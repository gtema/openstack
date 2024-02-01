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

use openstack_sdk::api::compute::v2::server::volume_attachment::find;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Show a detail of a volume attachment.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "Show a detail of a volume attachment")]
pub struct VolumeAttachmentArgs {
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
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[arg(value_name = "SERVER_ID", id = "path_param_server_id")]
    server_id: String,

    /// id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id}
    /// API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}

/// VolumeAttachment show command
pub struct VolumeAttachmentCmd {
    pub args: VolumeAttachmentArgs,
}
/// VolumeAttachment response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// Name of the device in the attachment object, such as, `/dev/vdb`.
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional, title = "serverId")]
    server_id: Option<String>,

    /// The UUID of the attached volume.
    #[serde(rename = "volumeId")]
    #[structable(optional, title = "volumeId")]
    volume_id: Option<String>,

    /// The device tag applied to the volume block device or `null`.
    ///
    ///
    /// **New in version 2.70**
    #[serde()]
    #[structable(optional)]
    tag: Option<String>,

    /// A flag indicating if the attached volume will be deleted when the
    /// server is
    /// deleted.
    ///
    ///
    /// **New in version 2.79**
    #[serde()]
    #[structable(optional)]
    delete_on_termination: Option<bool>,

    /// The UUID of the associated volume attachment in Cinder.
    ///
    ///
    /// **New in version 2.89**
    #[serde()]
    #[structable(optional)]
    attachment_id: Option<String>,

    /// The UUID of the block device mapping record in Nova for the attachment.
    ///
    ///
    /// **New in version 2.89**
    #[serde()]
    #[structable(optional)]
    bdm_uuid: Option<String>,
}

#[async_trait]
impl OSCCommand for VolumeAttachmentCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show VolumeAttachment with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.server_id(&self.args.path.server_id);
        find_builder.id(&self.args.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
