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

//! Create ResourceProvider command [microversion = 1.14]
//!
//! Wraps invoking of the `resource_providers` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::placement::v1::resource_provider::create_114;
use openstack_types::placement::v1::resource_provider::response::create::ResourceProviderResponse;

/// Create a new resource provider.
///
/// Normal Response Codes: 201 (microversions 1.0 - 1.19), 200 (microversions
/// 1.20 - )
///
/// Error response codes: conflict(409)
///
/// A 409 Conflict response code will be returned if another resource provider
/// exists with the provided name or uuid.
#[derive(Args)]
#[command(about = "Create resource provider (microversion = 1.14)")]
pub struct ResourceProviderCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The name of one resource provider.
    #[arg(help_heading = "Body parameters", long)]
    name: String,

    /// The UUID of the immediate parent of the resource provider.
    ///
    /// - Before version `1.37`, once set, the parent of a resource provider
    ///   cannot be changed.
    /// - Since version `1.37`, it can be set to any existing provider UUID
    ///   excepts to providers that would cause a loop. Also it can be set to
    ///   null to transform the provider to a new root provider. This operation
    ///   needs to be used carefully. Moving providers can mean that the
    ///   original rules used to create the existing resource allocations may
    ///   be invalidated by that move.
    ///
    /// **New in version 1.14**
    #[arg(help_heading = "Body parameters", long)]
    parent_provider_uuid: Option<String>,

    /// The uuid of a resource provider.
    #[arg(help_heading = "Body parameters", long)]
    uuid: Option<String>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

impl ResourceProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ResourceProvider");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_114::Request::builder();
        ep_builder.header("OpenStack-API-Version", "placement 1.14");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.name data
        ep_builder.name(&self.name);

        // Set Request.parent_provider_uuid data
        if let Some(arg) = &self.parent_provider_uuid {
            ep_builder.parent_provider_uuid(Some(arg.into()));
        }

        // Set Request.uuid data
        if let Some(arg) = &self.uuid {
            ep_builder.uuid(arg);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResourceProviderResponse>(data)?;
        Ok(())
    }
}
