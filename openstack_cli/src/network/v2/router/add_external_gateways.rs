//! Add external gateways to a router in addition to the ones it already
//! has.
//!
//! Multiple gateways attached to the same network can be added to the
//! same router.
//!
//! The add/update/remove external gateways operations extend the use of
//! `router.external\_gateway\_info` to manage multiple external gateways.
//! The full set of external gateways is exposed in the read-only
//! `router.external\_gateways` parameter. `router.external\_gateways`
//! contains a list of `external\_gateway\_info` structures like:
//!
//! The first item (index 0) of the `external\_gateways` list is special if a
//! router does not have any gateway ports yet:
//!
//! The order of the the rest of the list (indexes 1, 2, …) is irrelevant
//! and ignored.
//!
//! The first external gateway can be managed in two
//! ways: via `router.external\_gateway\_info` or via
//! `add/update/remove\_external\_gateways`. The other external gateways
//! can only be managed via `add/update/remove\_external\_gateways`.
//!
//! The format of the request body is the same as the format of the read-only
//! `router.external\_gateways` parameter, but wrapped as follows:
//!
//! The response codes and response body are the same as to the update of
//! the router. That is the whole router object is returned including the
//! `external\_gateway\_info` and `external\_gateways` parameters which
//! represents the result of the operation.
//!
//! Changes in `router.external\_gateway\_info` are reflected
//! in `router.external\_gateways` and vice versa. Updating
//! `external\_gateway\_info` also updates the first element of
//! `external\_gateways` and it leaves the rest of `external\_gateways`
//! unchanged. Setting `external\_gateway\_info` to an empty value removes
//! a single gateway and one of the extra gateways takes its place instead.
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
use openstack_sdk::api::network::v2::router::add_external_gateways;
use openstack_sdk::api::network::v2::router::find;
use openstack_sdk::api::QueryAsync;
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

    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    properties: Option<Vec<(String, Value)>>,
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
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ResponseData(HashMap<String, serde_json::Value>);

impl StructTable for ResponseData {
    fn build(&self, options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Name".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.extend(self.0.iter().map(|(k, v)| {
            Vec::from([
                k.clone(),
                serde_json::to_string(&v).expect("Is a valid data"),
            ])
        }));
        (headers, rows)
    }
}

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

        let mut ep_builder = add_external_gateways::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        if let Some(properties) = &self.args.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
