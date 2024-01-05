//! Lists keypairs that are associated with the account.
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

use openstack_sdk::api::compute::v2::os_keypair::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct OsKeypairsArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {
    #[arg(long)]
    user_id: Option<String>,

    #[arg(long)]
    limit: Option<i32>,

    #[arg(long)]
    marker: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// OsKeypairs list command
pub struct OsKeypairsCmd {
    pub args: OsKeypairsArgs,
}
/// OsKeypairs response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The name for the keypair.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The keypair public key.
    #[serde()]
    #[structable(optional)]
    public_key: Option<String>,

    /// The fingerprint for the keypair.
    #[serde()]
    #[structable(optional)]
    fingerprint: Option<String>,

    /// The type of the keypair. Allowed values are `ssh` or `x509`.
    ///
    ///
    /// **New in version 2.2**
    #[serde(rename = "type")]
    #[structable(optional, title = "type", wide)]
    _type: Option<String>,
}

#[async_trait]
impl Command for OsKeypairsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List OsKeypairs with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = list::Request::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.query.user_id {
            ep_builder.user_id(val);
        }
        if let Some(val) = &self.args.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.args.query.marker {
            ep_builder.marker(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
