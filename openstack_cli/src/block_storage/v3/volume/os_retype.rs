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

//! Action Volume command
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
use clap::ValueEnum;
use http::Response;
use openstack_sdk::api::block_storage::v3::volume::os_retype;
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
    os_retype: OsRetype,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v3/volumes/{id} API
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum MigrationPolicy {
    Never,
    OnDemand,
}

/// OsRetype Body data
#[derive(Args)]
struct OsRetype {
    #[arg(long)]
    new_type: String,

    #[arg(long)]
    migration_policy: Option<MigrationPolicy>,
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

        let mut ep_builder = os_retype::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_retype data
        let args = &self.os_retype;
        let mut os_retype_builder = os_retype::OsRetypeBuilder::default();

        os_retype_builder.new_type(args.new_type.clone());

        if let Some(val) = &args.migration_policy {
            let tmp = match val {
                MigrationPolicy::Never => os_retype::MigrationPolicy::Never,
                MigrationPolicy::OnDemand => os_retype::MigrationPolicy::OnDemand,
            };
            os_retype_builder.migration_policy(tmp);
        }

        ep_builder.os_retype(os_retype_builder.build().unwrap());

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
