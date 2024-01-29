//! Update some external gateways of router.
//!
//! For general information on the add/update/remove external gateways
//! operations see `add\_external\_gateways` above.
//!
//! The external gateways to be updated are identified by the `network\_ids`
//! found in the PUT request. The `external\_fixed\_ips`, `enable\_snat`,
//! fields can be updated. The `network\_id` field cannot be updated - any
//! changes will cause a gateway port to be removed and recreated.
//!
//! The format of the request body is the same as the format of the read-only
//! `router.external\_gateways` parameter, but wrapped as follows:
//!
//! The `enable\_snat` field does not have any effect for extra gateways except
//! for the first external gateway in the list.
//!
//! The `network\_id` field is used to identify a particular gateway port along
//! with the `external\_fixed\_ips` field. Specifying just the `network\_id`
//! field
//! is ambiguous: Neutron will attempt to find the matching gateway port but if
//! there are multiple matches it will return an error response code.
//!
//! The `enable\_snat` field can be omitted from the request. Specifying
//! `external\_fixed\_ips` will result in matching ports based on those
//! fixed IPs. If a gateway port has a subset of the specified fixed IPs,
//! then the set of IPs will be updated to match the ones in the request.
//! Alternatively, if a gateway port has a superset of fixed IPs from the
//! request the IPs will be removed from the gateway port.
//!
//! The response codes and response body are the same as to the update of
//! the router. That is the whole router object is returned including the
//! `external\_gateway\_info` and `external\_gateways` parameters which
//! represents the result of the operation.
//!
//! Please note that updating `external\_gateway\_info` also updates
//! the first element of `external\_gateways` and it leaves the rest of
//! `external\_gateways` unchanged.
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
use crate::{error::OpenStackCliError, OSCCommand};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use crate::common::parse_key_val;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::router::find;
use openstack_sdk::api::network::v2::router::update_external_gateways;
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
impl OSCCommand for RouterCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Router with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = update_external_gateways::Request::builder();

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
