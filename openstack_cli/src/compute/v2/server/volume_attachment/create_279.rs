// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Create VolumeAttachment command [microversion = 2.79]
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/os-volume_attachments` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::server::volume_attachment::create_279;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Attach a volume to an instance.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
///
#[derive(Args)]
#[command(about = "Attach a volume to an instance (microversion = 2.79)")]
pub struct VolumeAttachmentCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A dictionary representation of a volume attachment containing the
    /// fields `device` and `volumeId`.
    ///
    #[command(flatten)]
    volume_attachment: VolumeAttachment,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// server_id parameter for
    /// /v2.1/servers/{server_id}/os-volume_attachments/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_server_id",
        value_name = "SERVER_ID"
    )]
    server_id: String,
}
/// VolumeAttachment Body data
#[derive(Args, Clone)]
struct VolumeAttachment {
    /// To delete the attached volume when the server is destroyed, specify
    /// `true`. Otherwise, specify `false`. Default: `false`
    ///
    /// **New in version 2.79**
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    delete_on_termination: Option<bool>,

    /// Name of the device such as, `/dev/vdb`. Omit or set this parameter to
    /// null for auto-assignment, if supported. If you specify this parameter,
    /// the device must not exist in the guest operating system. Note that as
    /// of the 12.0.0 Liberty release, the Nova libvirt driver no longer honors
    /// a user-supplied device name. This is the same behavior as if the device
    /// name parameter is not supplied on the request.
    ///
    #[arg(help_heading = "Body parameters", long)]
    device: Option<String>,

    /// A device role tag that can be applied to a volume when attaching it to
    /// the VM. The guest OS of a server that has devices tagged in this manner
    /// can access hardware metadata about the tagged devices from the metadata
    /// API and on the config drive, if enabled.
    ///
    /// Note
    ///
    /// Tagged volume attachment is not supported for shelved-offloaded
    /// instances.
    ///
    /// **New in version 2.49**
    ///
    #[arg(help_heading = "Body parameters", long)]
    tag: Option<String>,

    /// The UUID of the volume to attach.
    ///
    #[arg(help_heading = "Body parameters", long)]
    volume_id: String,
}

/// VolumeAttachment response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The UUID of the associated volume attachment in Cinder.
    ///
    /// **New in version 2.89**
    ///
    #[serde()]
    #[structable(optional)]
    attachment_id: Option<String>,

    /// The UUID of the block device mapping record in Nova for the attachment.
    ///
    /// **New in version 2.89**
    ///
    #[serde()]
    #[structable(optional)]
    bdm_uuid: Option<String>,

    /// A flag indicating if the attached volume will be deleted when the
    /// server is deleted.
    ///
    /// **New in version 2.79**
    ///
    #[serde()]
    #[structable(optional)]
    delete_on_termination: Option<bool>,

    /// Name of the device in the attachment object, such as, `/dev/vdb`.
    ///
    #[serde()]
    #[structable(optional)]
    device: Option<String>,

    /// The volume ID of the attachment.
    ///
    /// **Available until version 2.88**
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The UUID of the server.
    ///
    #[serde(rename = "serverId")]
    #[structable(optional, title = "serverId")]
    server_id: Option<String>,

    /// The device tag applied to the volume block device or `null`.
    ///
    /// **New in version 2.70**
    ///
    #[serde()]
    #[structable(optional)]
    tag: Option<String>,

    /// The UUID of the attached volume.
    ///
    #[serde(rename = "volumeId")]
    #[structable(optional, title = "volumeId")]
    volume_id: Option<String>,
}

impl VolumeAttachmentCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create VolumeAttachment");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_279::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.79");

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        // Set body parameters
        // Set Request.volume_attachment data
        let args = &self.volume_attachment;
        let mut volume_attachment_builder = create_279::VolumeAttachmentBuilder::default();

        volume_attachment_builder.volume_id(&args.volume_id);

        if let Some(val) = &args.device {
            volume_attachment_builder.device(Some(val.into()));
        }

        if let Some(val) = &args.tag {
            volume_attachment_builder.tag(val);
        }

        if let Some(val) = &args.delete_on_termination {
            volume_attachment_builder.delete_on_termination(*val);
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
