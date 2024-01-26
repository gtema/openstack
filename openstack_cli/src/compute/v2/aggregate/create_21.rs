//! Creates an aggregate. If specifying an option availability\_zone, the
//! aggregate is
//! created as an availability zone and the availability zone is visible to
//! normal users.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! conflict(409)
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

use openstack_sdk::api::compute::v2::aggregate::create_21;
use openstack_sdk::api::QueryAsync;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct AggregateArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    aggregate: Aggregate,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}
/// Aggregate Body data
#[derive(Args, Debug, Clone)]
struct Aggregate {
    /// The name of the host aggregate.
    #[arg(long)]
    name: String,

    /// The availability zone of the host aggregate. You should use a custom
    /// availability zone rather than the default returned by the
    /// os-availability-zone API. The availability zone must not include ‘:’
    /// in its name.
    #[arg(long)]
    availability_zone: Option<String>,
}

/// Aggregate create command
pub struct AggregateCmd {
    pub args: AggregateArgs,
}
/// Aggregate response representation
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

    /// An array of host information.
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
impl Command for AggregateCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Aggregate with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create_21::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.1");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.aggregate data
        let args = &self.args.aggregate;
        let mut aggregate_builder = create_21::AggregateBuilder::default();

        aggregate_builder.name(args.name.clone());

        if let Some(val) = &args.availability_zone {
            aggregate_builder.availability_zone(Some(val.into()));
        }

        ep_builder.aggregate(aggregate_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
