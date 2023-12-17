//! Lists available extensions.
//!
//! Lists available Networking API v2.0 extensions and shows details
//! for an extension.
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
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

use openstack_sdk::api::network::v2::extension::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Lists available extensions.
///
/// Lists available Networking API v2.0 extensions and shows details
/// for an extension.
///
/// Normal response codes: 200
///
/// Error response codes: 401
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
    /// The alias for the extension. For example “quotas” or
    /// “security-group”.
    #[serde()]
    #[structable(optional, wide)]
    alias: Option<String>,

    /// The human-readable description for the resource.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// Human-readable name of the resource.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A URL pointing to the namespace for this extension.
    #[serde()]
    #[structable(optional, wide)]
    namespace: Option<String>,

    /// The date and timestamp when the extension was
    /// last updated.
    #[serde()]
    #[structable(optional, wide)]
    updated: Option<String>,
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
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
