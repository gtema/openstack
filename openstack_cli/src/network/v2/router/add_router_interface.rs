//! Adds an internal interface to a logical router.
//! This means a specified subnet is attached to a router
//! as an internal router interface.
//!
//! Specify the ID of a subnet or port in the request body:
//!
//! When you specify an IPv6 subnet, this operation adds the subnet to
//! an existing internal port with same network ID, on the router. If
//! a port with the same network ID does not exist, this operation
//! creates a port on the router for that subnet.
//!
//! The limitation of one IPv4 subnet per router port remains, though a
//! port can contain any number of IPv6 subnets that belong to the same
//! network ID.
//!
//! When you use the `port-create` command to add a port and then
//! call `router-interface-add` with this port ID, this operation
//! adds the port to the router if the following conditions are met:
//!
//! If you specify both subnet ID and port ID,
//! this operation returns the `Bad Request (400)` response code.
//!
//! If the port is already in use, this operation returns the
//! `Conflict (409)` response code.
//!
//! This operation returns a port ID that is either:
//!
//! After you run this operation, the operation sets:
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 404, 409
//!
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use crate::common::parse_key_val;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::router::add_router_interface;
use openstack_sdk::api::network::v2::router::find;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct RouterArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.0/routers/{id} API
    #[arg()]
    id: String,
}

/// Router action command
pub struct RouterCmd {
    pub args: RouterArgs,
}
/// Router response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for RouterCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Router with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = add_router_interface::Request::builder();

        ep_builder.id(&self.args.path.id);
        // Set query parameters

        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;

        Ok(())
    }
}
