//! Imports (or generates) a keypair.
//!
//! Normal response codes: 200, 201
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

use clap::ValueEnum;
use openstack_sdk::api::compute::v2::os_keypair::create;
use openstack_sdk::api::RawQueryAsync;

/// Imports (or generates) a keypair.
///
/// Normal response codes: 200, 201
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// conflict(409)
#[derive(Args, Clone, Debug)]
pub struct OsKeypairArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    keypair: Keypair,
}

#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Type {
    X509,
    Ssh,
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Args, Debug, Clone)]

struct Keypair {
    /// A name for the keypair which will be used to reference it later.
    ///
    ///
    ///
    /// Note
    ///
    ///
    /// Since microversion 2.92, allowed characters are ASCII letters
    /// `[a-zA-Z]`, digits `[0-9]` and the following special
    /// characters: `[@.\_- ]`.
    #[arg(long)]
    name: String,

    /// The type of the keypair. Allowed values are `ssh` or `x509`.
    ///
    ///
    /// **New in version 2.2**
    #[arg(long)]
    _type: Option<Type>,

    /// The public ssh key to import.
    /// Was optional before microversion 2.92 : if you were omitting this
    /// value, a
    /// keypair was generated for you.
    #[arg(long)]
    public_key: String,

    /// The user\_id for a keypair. This allows administrative users to
    /// upload keys for other users than themselves.
    ///
    ///
    /// **New in version 2.10**
    #[arg(long)]
    user_id: Option<String>,
}

pub struct OsKeypairCmd {
    pub args: OsKeypairArgs,
}

#[async_trait]
impl Command for OsKeypairCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post OsKeypair with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = create::Request::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters

        // Set Request.keypair data
        let args = &self.args.keypair;
        let mut keypair_builder = create::KeypairBuilder::default();

        keypair_builder.name(&args.name);

        if let Some(val) = &args._type {
            let tmp = match val {
                Type::X509 => create::Type::X509,
                Type::Ssh => create::Type::Ssh,
            };
            keypair_builder._type(tmp);
        }

        keypair_builder.public_key(&args.public_key);

        if let Some(val) = &args.user_id {
            keypair_builder.user_id(val);
        }

        ep_builder.keypair(keypair_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Compute)
            .await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
