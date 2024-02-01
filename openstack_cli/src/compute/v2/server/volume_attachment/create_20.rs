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

use openstack_sdk::api::compute::v2::server::volume_attachment::create_20;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Attach a volume to an instance.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Attach a volume to an instance (microversion = 2.0)")]
pub struct VolumeAttachmentArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    volume_attachment: VolumeAttachment,
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
}
/// VolumeAttachment Body data
#[derive(Args, Debug, Clone)]
struct VolumeAttachment {
    /// The UUID of the volume to attach.
    #[arg(long)]
    volume_id: String,

    /// Name of the device such as, `/dev/vdb`. Omit or set this parameter to
    /// null for
    /// auto-assignment, if supported. If you specify this parameter, the
    /// device must
    /// not exist in the guest operating system. Note that as of the 12.0.0
    /// Liberty release,
    /// the Nova libvirt driver no longer honors a user-supplied device name.
    /// This is
    /// the same behavior as if the device name parameter is not supplied on
    /// the request.
    #[arg(long)]
    device: Option<String>,
}

/// VolumeAttachment create command
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
        info!("Create VolumeAttachment with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create_20::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.0");

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        // Set query parameters
        // Set body parameters
        // Set Request.volume_attachment data
        let args = &self.args.volume_attachment;
        let mut volume_attachment_builder = create_20::VolumeAttachmentBuilder::default();

        volume_attachment_builder.volume_id(args.volume_id.clone());

        if let Some(val) = &args.device {
            volume_attachment_builder.device(Some(val.into()));
        }

        ep_builder.volume_attachment(volume_attachment_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
