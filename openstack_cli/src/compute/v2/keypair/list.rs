//! Lists keypairs that are associated with the account.
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

use openstack_sdk::api::compute::v2::os_keypairs::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Lists keypairs that are associated with the account.
#[derive(Args, Clone, Debug)]
pub struct KeypairsArgs {
    /// This allows administrative users to operate key-pairs of specified user
    /// ID.
    /// New in version 2.10
    #[arg(long)]
    user_id: Option<String>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the last-seen item from the response as the marker parameter value
    /// in a subsequent limited request.
    /// New in version 2.35
    #[arg(long)]
    limit: Option<String>,

    /// The last-seen item. Use the limit parameter to make an initial limited
    /// request and use the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    /// New in version 2.35
    #[arg(long)]
    marker: Option<String>,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

pub struct KeypairsCmd {
    pub args: KeypairsArgs,
}

/// Keypairs
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Keypairs {
    /// The name for the keypair.
    #[structable(optional)]
    name: Option<String>,

    /// The keypair public key.
    #[structable(optional, wide)]
    public_key: Option<String>,

    /// The fingerprint for the keypair.
    #[structable(optional, wide)]
    fingerprint: Option<String>,

    /// The type of the keypair. Allowed values are ssh or x509.
    /// New in version 2.2
    #[serde(rename = "type")]
    #[structable(title = "type", optional, wide)]
    xtype: Option<String>,
}

#[async_trait]
impl Command for KeypairsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Keypairs with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Keypairs::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.user_id {
            ep_builder.user_id(val);
        }
        if let Some(val) = &self.args.limit {
            ep_builder.limit(val);
        }
        if let Some(val) = &self.args.marker {
            ep_builder.marker(val);
        }
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Compute)
            .await?;
        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<Keypairs>(data)?;
        Ok(())
    }
}
