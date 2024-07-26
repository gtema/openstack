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

//! Set ConntrackHelper command
//!
//! Wraps invoking of the `v2.0/routers/{router_id}/conntrack_helpers/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use clap::ValueEnum;
use openstack_sdk::api::network::v2::router::conntrack_helper::set;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Updates a router conntrack helper.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 404
///
#[derive(Args)]
#[command(about = "Update a conntrack helper")]
pub struct ConntrackHelperCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    conntrack_helper: ConntrackHelper,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// router_id parameter for
    /// /v2.0/routers/{router_id}/conntrack_helpers/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_router_id",
        value_name = "ROUTER_ID"
    )]
    router_id: String,

    /// id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Protocol {
    Dccp,
    Icmp,
    Ipv6Icmp,
    Sctp,
    Tcp,
    Udp,
}

/// ConntrackHelper Body data
#[derive(Args, Clone)]
struct ConntrackHelper {
    /// The netfilter conntrack helper module.
    ///
    #[arg(help_heading = "Body parameters", long)]
    helper: Option<String>,

    /// The network port for the netfilter conntrack target rule.
    ///
    #[arg(help_heading = "Body parameters", long)]
    port: Option<f32>,

    /// The network protocol for the netfilter conntrack target rule.
    ///
    #[arg(help_heading = "Body parameters", long)]
    protocol: Option<Protocol>,
}

/// ConntrackHelper response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The netfilter conntrack helper module.
    ///
    #[serde()]
    #[structable(optional)]
    helper: Option<String>,

    /// The ID of the conntrack helper.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The network port for the netfilter conntrack target rule.
    ///
    #[serde()]
    #[structable(optional)]
    port: Option<f32>,

    /// The network protocol for the netfilter conntrack target rule.
    ///
    #[serde()]
    #[structable(optional)]
    protocol: Option<String>,
}

impl ConntrackHelperCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set ConntrackHelper");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.router_id(&self.path.router_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.conntrack_helper data
        let args = &self.conntrack_helper;
        let mut conntrack_helper_builder = set::ConntrackHelperBuilder::default();
        if let Some(val) = &args.protocol {
            let tmp = match val {
                Protocol::Dccp => set::Protocol::Dccp,
                Protocol::Icmp => set::Protocol::Icmp,
                Protocol::Ipv6Icmp => set::Protocol::Ipv6Icmp,
                Protocol::Sctp => set::Protocol::Sctp,
                Protocol::Tcp => set::Protocol::Tcp,
                Protocol::Udp => set::Protocol::Udp,
            };
            conntrack_helper_builder.protocol(tmp);
        }

        if let Some(val) = &args.port {
            conntrack_helper_builder.port(*val);
        }

        if let Some(val) = &args.helper {
            conntrack_helper_builder.helper(val);
        }

        ep_builder.conntrack_helper(conntrack_helper_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
