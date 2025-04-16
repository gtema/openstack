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

//! Set Service command [microversion = 2.0]
//!
//! Wraps invoking of the `v2.1/os-services/{id}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::service::set_20;
use openstack_types::compute::v2::service::response::set::ServiceResponse;

/// Update a compute service to enable or disable scheduling, including
/// recording a reason why a compute service was disabled from scheduling. Set
/// or unset the `forced_down` flag for the service. This operation is only
/// allowed on services whose `binary` is `nova-compute`.
///
/// This API is available starting with microversion 2.53.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404)
#[derive(Args)]
#[command(about = "Update Compute Service (microversion = 2.0)")]
pub struct ServiceCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long)]
    binary: String,

    /// The reason for disabling a service. The minimum length is 1 and the
    /// maximum length is 255. This may only be requested with
    /// `status=disabled`.
    #[arg(help_heading = "Body parameters", long)]
    disabled_reason: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    host: String,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/os-services/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

impl ServiceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Service");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_20::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.0");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.binary data
        ep_builder.binary(&self.binary);

        // Set Request.disabled_reason data
        if let Some(arg) = &self.disabled_reason {
            ep_builder.disabled_reason(arg);
        }

        // Set Request.host data
        ep_builder.host(&self.host);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ServiceResponse>(data)?;
        Ok(())
    }
}
