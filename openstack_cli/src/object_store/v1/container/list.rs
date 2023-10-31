//! Shows details for an account and lists containers, sorted by name, in the
//! account.
use async_trait::async_trait;
use clap::Args;
use http::Response;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::object_store::v1::account::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Shows details for an account and lists containers, sorted by name, in the
/// account.
#[derive(Args, Clone, Debug)]
pub struct ContainersArgs {
    /// For an integer value n, limits the number of results to n.
    #[arg(long)]
    limit: Option<u32>,

    /// For a string value, x, constrains the list to items whose names are
    /// greater than x.
    #[arg(long)]
    marker: Option<String>,

    /// For a string value, x, constrains the list to items whose names are
    /// less than x.
    #[arg(long)]
    end_marker: Option<String>,

    /// The response format. Valid values are json, xml, or plain. The default
    /// is plain. If you append the format=xml or format=json query parameter
    /// to the storage account URL, the response shows extended container
    /// information serialized in that format. If you append the format=plain
    /// query parameter, the response lists the container names separated by
    /// newlines.
    #[arg(long)]
    format: Option<String>,

    /// Only objects with this prefix will be returned. When combined with a
    /// delimiter query, this enables API users to simulate and traverse the
    /// objects in a container as if they were in a directory tree.
    #[arg(long)]
    prefix: Option<String>,

    /// The delimiter is a single character used to split object names to
    /// present a pseudo-directory hierarchy of objects. When combined with a
    /// prefix query, this enables API users to simulate and traverse the
    /// objects in a container as if they were in a directory tree.
    #[arg(long)]
    delimiter: Option<String>,

    /// By default, listings are returned sorted by name, ascending. If you
    /// include the reverse=true query parameter, the listing will be returned
    /// sorted by name, descending.
    #[arg(long, action=clap::ArgAction::Set)]
    reverse: Option<bool>,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

pub struct ContainersCmd {
    pub args: ContainersArgs,
}

/// Containers
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Containers {
    /// The number of objects in the container.
    #[structable(optional, wide)]
    count: Option<u64>,

    /// The total number of bytes that are stored in Object Storage for the
    /// account.
    #[structable(optional, wide)]
    bytes: Option<u64>,

    /// The name of the container.
    #[structable(optional)]
    name: Option<String>,

    /// Last modification date of the container
    #[structable(optional, wide)]
    last_modified: Option<String>,
}

#[async_trait]
impl Command for ContainersCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Containers with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Account::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.args.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.args.end_marker {
            ep_builder.end_marker(val);
        }
        if let Some(val) = &self.args.format {
            ep_builder.format(val);
        }
        if let Some(val) = &self.args.prefix {
            ep_builder.prefix(val);
        }
        if let Some(val) = &self.args.delimiter {
            ep_builder.delimiter(val);
        }
        if let Some(val) = &self.args.reverse {
            ep_builder.reverse(*val);
        }
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::ObjectStore)
            .await?;
        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<Containers>(data)?;
        Ok(())
    }
}
