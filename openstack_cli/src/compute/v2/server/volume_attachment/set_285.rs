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

//! Set VolumeAttachment command [microversion = 2.85]
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/os-volume_attachments/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::server::volume_attachment::set_285;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Update a volume attachment.
///
/// Policy default role is ‘rule:system_admin_or_owner’, its scope is \[system,
/// project\], which allow project members or system admins to change the
/// fields of an attached volume of a server. Policy defaults enable only users
/// with the administrative role to change `volumeId` via this operation. Cloud
/// providers can change these permissions through the `policy.json` file.
///
/// Updating, or what is commonly referred to as “swapping”, volume attachments
/// with volumes that have more than one read/write attachment, is not
/// supported.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
///
#[derive(Args)]
#[command(about = "Update a volume attachment (microversion = 2.85)")]
pub struct VolumeAttachmentCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A dictionary representation of a volume attachment containing the field
    /// `volumeId` which is the UUID of the replacement volume, and other
    /// fields to update in the attachment.
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

    /// id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// VolumeAttachment Body data
#[derive(Args, Clone)]
struct VolumeAttachment {
    /// A flag indicating if the attached volume will be deleted when the
    /// server is deleted.
    ///
    /// **New in version 2.85**
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    delete_on_termination: Option<bool>,

    /// Name of the device in the attachment object, such as, `/dev/vdb`.
    ///
    /// **New in version 2.85**
    ///
    #[arg(help_heading = "Body parameters", long)]
    device: Option<String>,

    /// The UUID of the attachment.
    ///
    /// **New in version 2.85**
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// The UUID of the server.
    ///
    /// **New in version 2.85**
    ///
    #[arg(help_heading = "Body parameters", long)]
    server_id: Option<String>,

    /// The device tag applied to the volume block device or `null`.
    ///
    /// **New in version 2.85**
    ///
    #[arg(help_heading = "Body parameters", long)]
    tag: Option<String>,

    /// The UUID of the volume to attach instead of the attached volume.
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
    #[structable()]
    device: String,

    /// The volume ID of the attachment.
    ///
    /// **Available until version 2.88**
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// The UUID of the server.
    ///
    #[serde(rename = "serverId")]
    #[structable(title = "serverId")]
    server_id: String,

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
    #[structable(title = "volumeId")]
    volume_id: String,
}

impl VolumeAttachmentCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set VolumeAttachment");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_285::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.85");

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.volume_attachment data
        let args = &self.volume_attachment;
        let mut volume_attachment_builder = set_285::VolumeAttachmentBuilder::default();

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

        if let Some(val) = &args.server_id {
            volume_attachment_builder.server_id(val);
        }

        if let Some(val) = &args.id {
            volume_attachment_builder.id(val);
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
