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

//! Create Protocol command
//!
//! Wraps invoking of the `v3/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::identity::v3::os_federation::identity_provider::protocol::create;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Create protocol for an IDP.
///
/// PUT /OS-Federation/identity_providers/{idp_id}/protocols/{protocol_id}
///
#[derive(Args)]
pub struct ProtocolCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    protocol: Protocol,
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

    /// protocol_id parameter for
    /// /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Protocol Body data
#[derive(Args, Clone)]
struct Protocol {
    #[arg(help_heading = "Body parameters", long)]
    mapping_id: String,

    #[arg(help_heading = "Body parameters", long)]
    remote_id_attribute: Option<String>,
}

/// Protocol response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The federation protocol ID
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    mapping_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    remote_id_attribute: Option<String>,
}

impl ProtocolCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Protocol");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.idp_id(&self.path.idp_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.protocol data
        let args = &self.protocol;
        let mut protocol_builder = create::ProtocolBuilder::default();

        protocol_builder.mapping_id(&args.mapping_id);

        if let Some(val) = &args.remote_id_attribute {
            protocol_builder.remote_id_attribute(Some(val.into()));
        }

        ep_builder.protocol(protocol_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
