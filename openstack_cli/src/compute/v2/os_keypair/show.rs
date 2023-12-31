//! Shows details for a keypair that is associated with the account.
//!
//! Normal response codes: 200
//!
//! Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
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

use openstack_sdk::api::compute::v2::os_keypair::find;
use openstack_sdk::api::compute::v2::os_keypair::get;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct OsKeypairArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {
    #[arg(long)]
    user_id: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.1/os-keypairs/{id} API
    #[arg()]
    id: String,
}

/// OsKeypair show command
pub struct OsKeypairCmd {
    pub args: OsKeypairArgs,
}
/// OsKeypair response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The user\_id for a keypair.
    #[serde()]
    #[structable(optional, wide)]
    user_id: Option<String>,

    /// A boolean indicates whether this keypair is deleted or not.
    /// The value is always `false` (not deleted).
    #[serde()]
    #[structable(optional, wide)]
    deleted: Option<bool>,

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

    /// It is always `null`.
    #[serde()]
    #[structable(optional, wide)]
    deleted_at: Option<String>,

    /// It is always `null`.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The keypair ID.
    #[serde()]
    #[structable(optional)]
    id: Option<i32>,

    /// The name for the keypair.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The keypair public key.
    #[serde()]
    #[structable(optional, wide)]
    public_key: Option<String>,

    /// The fingerprint for the keypair.
    #[serde()]
    #[structable(optional, wide)]
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
impl Command for OsKeypairCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show OsKeypair with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = find::Request::builder();
        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        if let Some(val) = &self.args.query.user_id {
            ep_builder.user_id(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let data = find(ep).query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
