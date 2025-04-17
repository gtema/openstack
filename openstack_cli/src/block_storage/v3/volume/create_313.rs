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

//! Create Volume command [microversion = 3.13]
//!
//! Wraps invoking of the `v3/volumes` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::block_storage::v3::volume::create_313;
use openstack_types::block_storage::v3::volume::response::create::VolumeResponse;
use serde_json::Value;

/// Creates a new volume.
///
/// | | | | --- | --- | | param req: | the request | | param body: | the
/// request body | | returns: | dict -- the new volume dictionary | | raises
/// HTTPNotFound, HTTPBadRequest: | | | | |
#[derive(Args)]
pub struct VolumeCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The dictionary of data to send to the scheduler.
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    os_sch_hnt_scheduler_hints: Option<Vec<(String, Value)>>,

    /// A `volume` object.
    #[command(flatten)]
    volume: Volume,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Volume Body data
#[derive(Args, Clone)]
struct Volume {
    /// The name of the availability zone.
    #[arg(help_heading = "Body parameters", long)]
    availability_zone: Option<String>,

    /// The UUID of the consistency group.
    #[arg(help_heading = "Body parameters", long)]
    consistencygroup_id: Option<String>,

    /// The volume description.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    display_description: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    display_name: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    group_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    image_id: Option<String>,

    /// The UUID of the image from which you want to create the volume.
    /// Required to create a bootable volume.
    ///
    /// **New in version 3.46**: Instead of directly consuming a zero-byte
    /// image that has been created by the Compute service when an instance
    /// snapshot was requested, the Block Storage service will use the
    /// `snapshot_id` contained in the `block_device_mapping` image property to
    /// locate the volume snapshot, and will use that to create the volume
    /// instead.
    #[arg(help_heading = "Body parameters", long)]
    image_ref: Option<String>,

    /// One or more metadata key and value pairs to be associated with the new
    /// volume.
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    multiattach: Option<Option<bool>>,

    /// The volume name.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The size of the volume, in gibibytes (GiB).
    #[arg(help_heading = "Body parameters", long)]
    size: Option<Option<i32>>,

    /// The UUID of the consistency group.
    #[arg(help_heading = "Body parameters", long)]
    snapshot_id: Option<String>,

    /// The UUID of the consistency group.
    #[arg(help_heading = "Body parameters", long)]
    source_volid: Option<String>,

    /// The volume type (either name or ID). To create an environment with
    /// multiple-storage back ends, you must specify a volume type. Block
    /// Storage volume back ends are spawned as children to `cinder- volume`,
    /// and they are keyed from a unique queue. They are named
    /// `cinder- volume.HOST.BACKEND`. For example,
    /// `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the
    /// scheduler chooses an appropriate back end to handle the request based
    /// on the volume type. Default is `None`. For information about how to use
    /// volume types to create multiple- storage back ends, see
    /// [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html).
    #[arg(help_heading = "Body parameters", long)]
    volume_type: Option<String>,
}

impl VolumeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Volume");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_313::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.13");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.os_sch_hnt_scheduler_hints data
        if let Some(arg) = &self.os_sch_hnt_scheduler_hints {
            ep_builder.os_sch_hnt_scheduler_hints(arg.iter().cloned());
        }

        // Set Request.volume data
        let args = &self.volume;
        let mut volume_builder = create_313::VolumeBuilder::default();
        if let Some(val) = &args.name {
            volume_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.description {
            volume_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.display_name {
            volume_builder.display_name(Some(val.into()));
        }

        if let Some(val) = &args.display_description {
            volume_builder.display_description(Some(val.into()));
        }

        if let Some(val) = &args.volume_type {
            volume_builder.volume_type(Some(val.into()));
        }

        if let Some(val) = &args.metadata {
            volume_builder.metadata(val.iter().cloned());
        }

        if let Some(val) = &args.snapshot_id {
            volume_builder.snapshot_id(Some(val.into()));
        }

        if let Some(val) = &args.source_volid {
            volume_builder.source_volid(Some(val.into()));
        }

        if let Some(val) = &args.consistencygroup_id {
            volume_builder.consistencygroup_id(Some(val.into()));
        }

        if let Some(val) = &args.size {
            volume_builder.size(*val);
        }

        if let Some(val) = &args.availability_zone {
            volume_builder.availability_zone(Some(val.into()));
        }

        if let Some(val) = &args.multiattach {
            volume_builder.multiattach(*val);
        }

        if let Some(val) = &args.image_id {
            volume_builder.image_id(Some(val.into()));
        }

        if let Some(val) = &args.image_ref {
            volume_builder.image_ref(Some(val.into()));
        }

        if let Some(val) = &args.group_id {
            volume_builder.group_id(Some(val.into()));
        }

        ep_builder.volume(volume_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<VolumeResponse>(data)?;
        Ok(())
    }
}
