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

//! Create NdpProxy command
//!
//! Wraps invoking of the `v2.0/ndp-proxies` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::ndp_proxy::create;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct NdpProxyCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    ndp_proxy: NdpProxy,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// NdpProxy Body data
#[derive(Args, Clone)]
struct NdpProxy {
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    ip_address: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    port_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    router_id: Option<String>,
}

/// NdpProxy response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    ip_address: Option<String>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional)]
    port_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    #[serde()]
    #[structable(optional)]
    router_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl NdpProxyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create NdpProxy");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.ndp_proxy data
        let args = &self.ndp_proxy;
        let mut ndp_proxy_builder = create::NdpProxyBuilder::default();
        if let Some(val) = &args.name {
            ndp_proxy_builder.name(val);
        }

        if let Some(val) = &args.project_id {
            ndp_proxy_builder.project_id(val);
        }

        if let Some(val) = &args.router_id {
            ndp_proxy_builder.router_id(val);
        }

        if let Some(val) = &args.port_id {
            ndp_proxy_builder.port_id(val);
        }

        if let Some(val) = &args.ip_address {
            ndp_proxy_builder.ip_address(val);
        }

        if let Some(val) = &args.description {
            ndp_proxy_builder.description(val);
        }

        ep_builder.ndp_proxy(ndp_proxy_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
