//! Deletes an internal interface from a logical router.
//!
//! This operation deletes an internal router interface, which detaches
//! a subnet from the router. If this subnet ID is the last subnet on
//! the port, this operation deletes the port itself. You must specify
//! either a subnet ID or port ID in the request body; the
//! operation uses this value to identify which router interface to
//! deletes.
//!
//! You can also specify both a subnet ID and port ID. If you
//! specify both IDs, the subnet ID must correspond to the subnet
//! ID of the first IP address on the port. Otherwise, this operation
//! returns the `Conflict (409)` response code with information about
//! the affected router and interface.
//!
//! If you try to delete the router interface for subnets that are used
//! by one or more `routes`, this operation returns the `Conflict (409)`
//! response. In this case, you first need to delete such routes from
//! the router.
//!
//! If the router or the subnet and port do not exist or are not
//! visible to you, this operation returns the `Not Found (404)`
//! response code. As a consequence of this operation, the operation
//! removes the port connecting the router with the subnet from the
//! subnet for the network.
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
use openstack_sdk::api::network::v2::router::find;
use openstack_sdk::api::network::v2::router::remove_router_interface;
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
        let mut ep_builder = remove_router_interface::Request::builder();
        // Set path parameters
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
