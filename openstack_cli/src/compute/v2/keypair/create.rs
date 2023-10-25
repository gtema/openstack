//! Imports (or generates) a keypair.
//!
//! Warning: Generating a keypair is no longer possible starting from
//!   version 2.92.
//!
//! Normal response codes: 200, 201
//! Note: The success status code was changed from 200 to 201 in version
//!   2.2
use async_trait::async_trait;
use clap::Args;
use http::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::compute::v2::os_keypairs::post;
use openstack_sdk::api::QueryAsync;

/// Imports (or generates) a keypair.
///
/// Warning: Generating a keypair is no longer possible starting from
///   version 2.92.
///
/// Normal response codes: 200, 201
/// Note: The success status code was changed from 200 to 201 in version
///   2.2
#[derive(Args, Clone, Debug)]
pub struct KeypairArgs {
    /// A name for the keypair which will be used to reference it later.
    /// Note: Since microversion 2.92, allowed characters are ASCII letters
    /// [a-zA-Z], digits [0-9] and the following special characters: [@._- ].
    #[arg(long)]
    name: String,

    /// The public ssh key to import. Was optional before microversion 2.92 :
    /// if you were omitting this value, a keypair was generated for you.
    #[arg(long)]
    public_key: String,

    /// The type of the keypair. Allowed values are ssh or x509.
    /// New in version 2.2
    #[arg(long)]
    xtype: Option<String>,

    /// The user_id for a keypair.
    #[arg(long)]
    user_id: Option<String>,
}

pub struct KeypairCmd {
    pub args: KeypairArgs,
}

/// Keypair
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Keypair {
    /// A name for the keypair which will be used to reference it later.
    /// Note: Since microversion 2.92, allowed characters are ASCII letters
    /// [a-zA-Z], digits [0-9] and the following special characters: [@._- ].
    #[structable(optional)]
    name: Option<String>,

    /// The public ssh key to import. Was optional before microversion 2.92 :
    /// if you were omitting this value, a keypair was generated for you.
    #[structable(optional)]
    public_key: Option<String>,

    /// The fingerprint for the keypair.
    #[structable(optional)]
    fingerprint: Option<String>,

    /// The type of the keypair. Allowed values are ssh or x509.
    /// New in version 2.2
    #[serde(rename = "type")]
    #[structable(title = "type", optional)]
    xtype: Option<String>,

    /// The user_id for a keypair.
    #[structable(optional)]
    user_id: Option<String>,

    /// If you do not provide a public key on create, a new keypair will be
    /// built for you, and the private key will be returned during the initial
    /// create call. Make sure to save this, as there is no way to get this
    /// private key again in the future.
    /// Available until version 2.91
    #[structable(optional)]
    private_key: Option<String>,
}

#[async_trait]
impl Command for KeypairCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Keypair with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = post::Keypairs::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        ep_builder.name(&self.args.name);
        ep_builder.public_key(&self.args.public_key);
        if let Some(val) = &self.args.xtype {
            ep_builder.xtype(val);
        }
        if let Some(val) = &self.args.user_id {
            ep_builder.user_id(val);
        }
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Compute)
            .await?;
        let data = ep.query_async(client).await?;
        op.output_single::<Keypair>(data)?;
        Ok(())
    }
}
