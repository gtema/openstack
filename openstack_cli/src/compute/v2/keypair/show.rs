//! Shows details for a keypair that is associated with the account.
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

use openstack_sdk::api::compute::v2::os_keypair::get;
use openstack_sdk::api::QueryAsync;

/// Shows details for a keypair that is associated with the account.
#[derive(Args, Clone, Debug)]
pub struct KeypairArgs {
    /// This allows administrative users to operate key-pairs of specified user
    /// ID.
    /// New in version 2.10
    #[arg()]
    keypair_name: String,

    /// This allows administrative users to operate key-pairs of specified user
    /// ID.
    /// New in version 2.10
    #[arg(long)]
    user_id: Option<String>,
}

pub struct KeypairCmd {
    pub args: KeypairArgs,
}

/// Keypair
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Keypair {
    /// The name for the keypair.
    #[structable(optional)]
    name: Option<String>,

    /// The keypair public key.
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
}

#[async_trait]
impl Command for KeypairCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Keypair with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Keypair::builder();
        // Set path parameters
        ep_builder.keypair_name(&self.args.keypair_name);
        // Set query parameters
        if let Some(val) = &self.args.user_id {
            ep_builder.user_id(val);
        }
        // Set body parameters
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
