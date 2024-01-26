//! Gets detailed availability zone information.
//! Policy defaults enable only users with the administrative role to perform
//! this operation.
//! Cloud providers can change these permissions through the `policy.json`
//! file.
//!
//! Normal response codes: 200
//!
//! Error response codes: unauthorized(401), forbidden(403)
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

use openstack_sdk::api::compute::v2::availability_zone::list_detailed;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct AvailabilityZonesArgs {
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
pub struct PathParameters {}

/// AvailabilityZones list command
pub struct AvailabilityZonesCmd {
    pub args: AvailabilityZonesArgs,
}
/// AvailabilityZones response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The availability zone name.
    #[serde(rename = "zoneName")]
    #[structable(optional, title = "zoneName", wide)]
    zone_name: Option<String>,

    /// The current state of the availability zone.
    #[serde(rename = "zoneState")]
    #[structable(optional, title = "zoneState", wide)]
    zone_state: Option<ResponseZoneState>,

    /// An object containing a list of host information. The host information
    /// is comprised
    /// of host and service objects. The service object returns three
    /// parameters representing
    /// the states of the service: `active`, `available`, and `updated\_at`.
    #[serde()]
    #[structable(optional)]
    hosts: Option<HashMapStringValue>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseZoneState {
    available: Option<bool>,
}

impl fmt::Display for ResponseZoneState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "available={}",
            self.available
                .clone()
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[async_trait]
impl Command for AvailabilityZonesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List AvailabilityZones with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list_detailed::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
