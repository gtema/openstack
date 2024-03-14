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

//! Delete IdentityProvider command
//!
//! Wraps invoking of the `v3/OS-FEDERATION/identity_providers/{idp_id}` with `DELETE` method

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
use openstack_sdk::api::identity::v3::os_federation::identity_provider::delete;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// When an identity provider is deleted, any tokens generated by that identity
/// provider will be revoked.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/ext/OS-FEDERATION/1.0/rel/identity_provider`
///
#[derive(Args)]
#[command(about = "Delete identity provider")]
pub struct IdentityProviderCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// idp_id parameter for
    /// /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_idp_id",
        value_name = "IDP_ID"
    )]
    idp_id: String,
}
/// IdentityProvider response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl IdentityProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete IdentityProvider");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = delete::Request::builder();

        // Set path parameters
        ep_builder.idp_id(&self.path.idp_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
