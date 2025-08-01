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

//! Create ConntrackHelper command
//!
//! Wraps invoking of the `v2.0/routers/{router_id}/conntrack_helpers` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::router::conntrack_helper::create;
use openstack_types::network::v2::router::conntrack_helper::response::create::ConntrackHelperResponse;

/// Creates a router conntrack helper.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 404
#[derive(Args)]
#[command(about = "Create conntrack helper")]
pub struct ConntrackHelperCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A router `conntrack helper` object.
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
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_router_id",
        value_name = "ROUTER_ID"
    )]
    router_id: String,
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
    #[arg(help_heading = "Body parameters", long)]
    helper: Option<String>,

    /// The network port for the netfilter conntrack target rule.
    #[arg(help_heading = "Body parameters", long)]
    port: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    /// The network protocol for the netfilter conntrack target rule.
    #[arg(help_heading = "Body parameters", long)]
    protocol: Option<Protocol>,
}

impl ConntrackHelperCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ConntrackHelper");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("network.router/conntrack_helper"),
            Some("create"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        ep_builder.router_id(&self.path.router_id);

        // Set body parameters
        // Set Request.conntrack_helper data
        let args = &self.conntrack_helper;
        let mut conntrack_helper_builder = create::ConntrackHelperBuilder::default();
        if let Some(val) = &args.helper {
            conntrack_helper_builder.helper(val);
        }

        if let Some(val) = &args.port {
            conntrack_helper_builder.port(*val);
        }

        if let Some(val) = &args.project_id {
            conntrack_helper_builder.project_id(val);
        }

        if let Some(val) = &args.protocol {
            let tmp = match val {
                Protocol::Dccp => create::Protocol::Dccp,
                Protocol::Icmp => create::Protocol::Icmp,
                Protocol::Ipv6Icmp => create::Protocol::Ipv6Icmp,
                Protocol::Sctp => create::Protocol::Sctp,
                Protocol::Tcp => create::Protocol::Tcp,
                Protocol::Udp => create::Protocol::Udp,
            };
            conntrack_helper_builder.protocol(tmp);
        }

        ep_builder.conntrack_helper(conntrack_helper_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ConntrackHelperResponse>(data)?;
        Ok(())
    }
}
