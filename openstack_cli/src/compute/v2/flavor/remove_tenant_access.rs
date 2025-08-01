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

//! Action Flavor command
//!
//! Wraps invoking of the `v2.1/flavors/{id}/action` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::flavor::remove_tenant_access;
use openstack_types::compute::v2::flavor::response::remove_tenant_access::FlavorResponse;

/// Removes flavor access from a tenant and flavor.
///
/// Specify the `removeTenantAccess` action and the `tenant` in the request
/// body.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args)]
#[command(about = "Remove Flavor Access From Tenant (removeTenantAccess Action)")]
pub struct FlavorCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action.
    #[command(flatten)]
    remove_tenant_access: RemoveTenantAccess,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/flavors/{id}/action API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// RemoveTenantAccess Body data
#[derive(Args, Clone)]
struct RemoveTenantAccess {
    /// The UUID of the tenant in a multi-tenancy cloud.
    #[arg(help_heading = "Body parameters", long)]
    tenant: String,
}

impl FlavorCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Flavor");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("compute.flavor"),
            Some("remove_tenant_access"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = remove_tenant_access::Request::builder();

        ep_builder.id(&self.path.id);

        // Set body parameters
        // Set Request.remove_tenant_access data
        let args = &self.remove_tenant_access;
        let mut remove_tenant_access_builder =
            remove_tenant_access::RemoveTenantAccessBuilder::default();

        remove_tenant_access_builder.tenant(&args.tenant);

        ep_builder.remove_tenant_access(remove_tenant_access_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<FlavorResponse>(data)?;
        Ok(())
    }
}
