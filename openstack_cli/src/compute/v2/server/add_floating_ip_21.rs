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

//! Action Server command [microversion = 2.1]
//!
//! Wraps invoking of the `v2.1/servers/{id}/action` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::server::add_floating_ip_21;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Adds a floating IP address to a server, which associates that address with
/// the server.
///
/// A pool of floating IP addresses, configured by the cloud administrator, is
/// available in OpenStack Compute. The project quota defines the maximum
/// number of floating IP addresses that you can allocate to the project. After
/// you
/// [create (allocate) a floating IPaddress](https://docs.openstack.org/api-ref/compute/#create-allocate-floating-ip-address)
/// for a project, you can associate that address with the server. Specify the
/// `addFloatingIp` action in the request body.
///
/// If an instance is connected to multiple networks, you can associate a
/// floating IP address with a specific fixed IP address by using the optional
/// `fixed_address` parameter.
///
/// **Preconditions**
///
/// The server must exist.
///
/// You can only add a floating IP address to the server when its status is
/// `ACTIVE` or `STOPPED`
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404)
///
#[derive(Args)]
#[command(
    about = "Add (Associate) Floating Ip (addFloatingIp Action) (DEPRECATED) (microversion = 2.1)"
)]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action. Contains required floating IP `address` and optional
    /// `fixed_address`.
    ///
    #[command(flatten)]
    add_floating_ip: AddFloatingIp,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// AddFloatingIp Body data
#[derive(Args, Clone)]
struct AddFloatingIp {
    /// The fixed IP address with which you want to associate the floating IP
    /// address.
    ///
    #[arg(help_heading = "Body parameters", long)]
    address: String,

    /// The fixed IP address with which you want to associate the floating IP
    /// address.
    ///
    #[arg(help_heading = "Body parameters", long)]
    fixed_address: Option<String>,
}

/// Server response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = add_floating_ip_21::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.1");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.add_floating_ip data
        let args = &self.add_floating_ip;
        let mut add_floating_ip_builder = add_floating_ip_21::AddFloatingIpBuilder::default();

        add_floating_ip_builder.address(&args.address);

        if let Some(val) = &args.fixed_address {
            add_floating_ip_builder.fixed_address(val);
        }

        ep_builder.add_floating_ip(add_floating_ip_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
