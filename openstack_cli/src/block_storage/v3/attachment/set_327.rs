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

//! Set Attachment command [microversion = 3.27]
//!
//! Wraps invoking of the `v3/attachments/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_key_val;
use openstack_sdk::api::block_storage::v3::attachment::set_327;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Update an attachment record.
///
/// Update a reserved attachment record with connector information and set up
/// the appropriate connection_info from the driver.
///
/// Expected format of the input parameter 'body':
///
/// ```text
///
/// {
///     "attachment":
///     {
///         "connector":
///         {
///             "initiator": "iqn.1993-08.org.debian:01:cad181614cec",
///             "ip": "192.168.1.20",
///             "platform": "x86_64",
///             "host": "tempest-1",
///             "os_type": "linux2",
///             "multipath": false,
///             "mountpoint": "/dev/vdb",
///             "mode": "None|rw|ro"
///         }
///     }
/// }
///
/// ```
///
#[derive(Args)]
pub struct AttachmentCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    attachment: Attachment,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v3/attachments/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Attachment Body data
#[derive(Args, Clone)]
struct Attachment {
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    connector: Vec<(String, Value)>,
}

/// Attachment response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The attach mode of attachment, read-only (‘ro’) or read-and-write
    /// (‘rw’), default is ‘rw’.
    ///
    #[serde()]
    #[structable(optional)]
    attach_mode: Option<String>,

    /// The time when attachment is attached.
    ///
    #[serde()]
    #[structable(optional)]
    attached_at: Option<String>,

    /// The connection info used for server to connect the volume.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    connecttion_info: Option<Value>,

    /// The time when attachment is detached.
    ///
    #[serde()]
    #[structable(optional)]
    detached_at: Option<String>,

    /// The ID of attachment.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The UUID of the attaching instance.
    ///
    #[serde()]
    #[structable(optional)]
    instance: Option<String>,

    /// The status of the attachment.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The UUID of the volume which the attachment belongs to.
    ///
    #[serde()]
    #[structable(optional)]
    volume_id: Option<String>,
}

impl AttachmentCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Attachment");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_327::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.27");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.attachment data
        let args = &self.attachment;
        let mut attachment_builder = set_327::AttachmentBuilder::default();

        attachment_builder.connector(args.connector.iter().cloned());

        ep_builder.attachment(attachment_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
