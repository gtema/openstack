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

//! Action Volume command [microversion = 3.0]
//!
//! Wraps invoking of the `v3/volumes/{id}/action` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::block_storage::v3::volume::os_migrate_volume_30;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct VolumeCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    os_migrate_volume: OsMigrateVolume,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v3/volumes/{id} API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}
/// OsMigrateVolume Body data
#[derive(Args)]
struct OsMigrateVolume {
    #[arg(long)]
    host: String,

    #[arg(action=clap::ArgAction::Set, long)]
    force_host_copy: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    lock_volume: Option<bool>,
}

/// Volume response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl VolumeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Volume");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = os_migrate_volume_30::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.0");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_migrate_volume data
        let args = &self.os_migrate_volume;
        let mut os_migrate_volume_builder = os_migrate_volume_30::OsMigrateVolumeBuilder::default();

        os_migrate_volume_builder.host(args.host.clone());

        if let Some(val) = &args.force_host_copy {
            os_migrate_volume_builder.force_host_copy(*val);
        }

        if let Some(val) = &args.lock_volume {
            os_migrate_volume_builder.lock_volume(*val);
        }

        ep_builder.os_migrate_volume(os_migrate_volume_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
