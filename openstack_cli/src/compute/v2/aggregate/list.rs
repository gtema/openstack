//! Lists all aggregates. Includes the ID, name, and availability zone for each
//! aggregate.
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

use openstack_sdk::api::compute::v2::aggregate::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct AggregatesArgs {
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

/// Aggregates list command
pub struct AggregatesCmd {
    pub args: AggregatesArgs,
}
/// Aggregates response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The availability zone of the host aggregate.
    #[serde()]
    #[structable(optional, wide)]
    availability_zone: Option<String>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A boolean indicates whether this aggregate is deleted or not, if it has
    /// not been deleted, `false` will appear.
    #[serde()]
    #[structable(optional, wide)]
    deleted: Option<bool>,

    /// The date and time when the resource was deleted. If the resource has
    /// not been deleted yet, this field will be `null`, The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional, wide)]
    deleted_at: Option<String>,

    /// The ID of the host aggregate.
    #[serde()]
    #[structable(optional)]
    id: Option<i32>,

    /// Metadata key and value pairs associated with the aggregate.
    #[serde()]
    #[structable(optional, wide)]
    metadata: Option<HashMapStringString>,

    /// A list of host ids in this aggregate.
    #[serde()]
    #[structable(optional, wide)]
    hosts: Option<VecString>,

    /// The date and time when the resource was updated, if the resource has
    /// not been updated, this field will show as `null`. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The UUID of the host aggregate.
    ///
    ///
    /// **New in version 2.41**
    #[serde()]
    #[structable(optional, wide)]
    uuid: Option<String>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringString(HashMap<String, String>);
impl fmt::Display for HashMapStringString {
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[async_trait]
impl Command for AggregatesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Aggregates with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

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
