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

//! Create Auth command
//!
//! Wraps invoking of the `v3/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/auth` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::identity::v3::os_federation::identity_provider::protocol::auth::create;
use openstack_sdk::api::QueryAsync;
use openstack_types::identity::v3::os_federation::identity_provider::protocol::auth::response::create::AuthResponse;
use serde_json::Value;

/// Authenticate from dedicated uri endpoint.
///
/// POST /OS-FEDERATION/identity_providers/
/// {idp_id}/protocols/{protocol_id}/auth
#[derive(Args)]
pub struct AuthCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    #[arg(help_heading = "Body parameters")]
    properties: Option<Vec<(String, Value)>>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// idp_id parameter for
    /// /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/auth
    /// API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_idp_id",
        value_name = "IDP_ID"
    )]
    idp_id: String,

    /// protocol_id parameter for
    /// /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/auth
    /// API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_protocol_id",
        value_name = "PROTOCOL_ID"
    )]
    protocol_id: String,
}

impl AuthCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Auth");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.idp_id(&self.path.idp_id);
        ep_builder.protocol_id(&self.path.protocol_id);
        // Set query parameters
        // Set body parameters
        if let Some(properties) = &self.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<AuthResponse>(data)?;
        Ok(())
    }
}
