//! Lists all extensions to the API.
//!
//! Normal response codes: 200
//!
//! Error response codes: unauthorized(401)
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

use openstack_sdk::api::compute::v2::extension::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;

/// Lists all extensions to the API.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401)
#[derive(Args, Clone, Debug)]
pub struct ExtensionsArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

pub struct ExtensionsCmd {
    pub args: ExtensionsArgs,
}
/// Extensions
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// A short name by which this extension is also known.
    #[serde()]
    #[structable(optional, wide)]
    alias: Option<String>,

    /// Text describing this extension’s purpose.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// Links pertaining to this extension. This is a list of dictionaries,
    /// each including
    /// keys `href` and `rel`.
    #[serde()]
    #[structable(optional, wide)]
    links: Option<Value>,

    /// Name of the extension.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A URL pointing to the namespace for this extension.
    #[serde()]
    #[structable(optional, wide)]
    namespace: Option<String>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```
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
    updated: Option<String>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct Links {
    href: Option<String>,
    rel: Option<String>,
}

impl fmt::Display for Links {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "href={}",
                self.href
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rel={}",
                self.rel
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        return write!(f, "{}", data.join(";"));
    }
}

#[async_trait]
impl Command for ExtensionsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Extensions with {:?}", self.args);

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
        client
            .discover_service_endpoint(&ServiceType::Compute)
            .await?;
        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
