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

//! Action GroupSnapshot command [microversion = 3.19]
//!
//! Wraps invoking of the `v3/group_snapshots/{id}/action` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::block_storage::v3::group_snapshot::reset_status_319;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct GroupSnapshotCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    reset_status: ResetStatus,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v3/group_snapshots/{id}/action API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// ResetStatus Body data
#[derive(Args, Clone)]
struct ResetStatus {
    #[arg(help_heading = "Body parameters", long)]
    status: String,
}

impl GroupSnapshotCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action GroupSnapshot");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("block-storage.group_snapshot"),
            Some("reset_status"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = reset_status_319::Request::builder();
        ep_builder.header(
            http::header::HeaderName::from_static("openstack-api-version"),
            http::header::HeaderValue::from_static("volume 3.19"),
        );

        ep_builder.id(&self.path.id);

        // Set body parameters
        // Set Request.reset_status data
        let args = &self.reset_status;
        let mut reset_status_builder = reset_status_319::ResetStatusBuilder::default();

        reset_status_builder.status(&args.status);

        ep_builder.reset_status(reset_status_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
