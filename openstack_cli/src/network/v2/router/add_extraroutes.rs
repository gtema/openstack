//! Atomically adds a set of extra routes to the router’s already existing
//! extra routes.
//!
//! This operation is a variation on updating the router’s `routes`
//! parameter. In all ways it works the same, except the extra routes sent
//! in the request body do not replace the existing set of extra routes.
//! Instead the extra routes sent are added to the existing set of
//! extra routes.
//!
//! The use of the add\_extraroutes/remove\_extraroutes member actions
//! is preferred to updating the `routes` attribute in all cases when
//! concurrent updates to the set of extra routes are possible.
//!
//! The addition’s corner cases behave the following way:
//!
//! The format of the request body is the same as the format of a PUT
//! request to the router changing the `routes` parameter only.
//!
//! The response codes and response body are the same as to the update of
//! the `routes` parameter. That is the whole router object is returned
//! including the `routes` parameter which represents the result of the
//! addition.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 404, 412
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
use openstack_sdk::api::network::v2::router::add_extraroutes;
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
        let mut ep_builder = add_extraroutes::Request::builder();
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
