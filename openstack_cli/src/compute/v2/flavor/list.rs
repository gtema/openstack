//! List Flavors
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

use crate::common::parse_key_val;
use crate::common::HashMapStringString;
use openstack_sdk::api::compute::v2::flavors::detail::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// List Flavors
#[derive(Args, Clone, Debug)]
pub struct FlavorsArgs {
    /// Filters the response by a minimum disk space, in GiB. For example, 100.
    #[arg(long)]
    min_disk: Option<u32>,

    /// Filters the response by a minimum RAM, in MiB. For example, 512.
    #[arg(long)]
    min_ram: Option<u32>,

    /// This parameter is only applicable to users with the administrative
    /// role. For all other non-admin users, the parameter is ignored and only
    /// public flavors will be returned. Filters the flavor list based on
    /// whether the flavor is public or private. If the value of this parameter
    /// is not specified, it is treated as True. If the value is specified, 1,
    /// t, true, on, y and yes are treated as True. 0, f, false, off, n and no
    /// are treated as False (they are case-insensitive). If the value is None
    /// (case-insensitive) both public and private flavors will be listed in a
    /// single request.
    #[arg(long)]
    is_public: Option<bool>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[arg(long)]
    limit: Option<u32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[arg(long)]
    marker: Option<String>,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

pub struct FlavorsCmd {
    pub args: FlavorsArgs,
}

/// Flavors
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Flavors {
    /// Whether or not the flavor has been administratively disabled. This is
    /// typically only visible to administrative users.
    #[serde(rename = "OS-FLV-DISABLED:disabled")]
    #[structable(optional, wide)]
    is_disabled: Option<bool>,

    /// The size of the ephemeral disk that will be created, in GiB. Ephemeral
    /// disks may be written over on server state changes. So should only be
    /// used as a scratch space for applications that are aware of its
    /// limitations. Defaults to 0.
    #[serde(rename = "OS-FLV-EXT-DATA:ephemeral")]
    #[structable(optional, wide)]
    ephemeral: Option<u32>,

    /// The description of the flavor.
    #[structable(optional, wide)]
    description: Option<String>,

    /// The size of the root disk that will be created in GiB. If 0 the root
    /// disk will be set to exactly the size of the image used to deploy the
    /// instance. However, in this case the scheduler cannot select the compute
    /// host based on the virtual image size. Therefore, 0 should only be used
    /// for volume booted instances or for testing purposes. Volume-backed
    /// instances can be enforced for flavors with zero root disk via the
    /// os_compute_api:servers:create:zero_disk_flavor policy rule.
    #[structable(optional, wide)]
    disk: Option<u32>,

    /// A dictionary of the flavor's extra-specs key-and-value pairs.
    #[structable(wide)]
    extra_specs: HashMapStringString,

    /// The ID of the flavor. While people often make this look like an int,
    /// this is really a string.
    #[structable(optional)]
    id: Option<String>,

    /// The name of this flavor.
    #[structable(optional)]
    name: Option<String>,

    /// The name of this flavor when returned by server list/show
    #[structable(optional, wide)]
    original_name: Option<String>,

    /// ``True`` if this is a publicly visible flavor. ``False`` if this is a
    /// private image.
    #[serde(rename = "os-flavor-access:is_public")]
    #[structable(optional, wide)]
    is_public: Option<bool>,

    /// The amount of RAM (in MB) this flavor offers.
    #[structable(optional, wide)]
    ram: Option<u32>,

    /// The receive / transmit factor (as a float) that will be set on ports if
    /// the network backend supports the QOS extension. Otherwise it will be
    /// ignored. It defaults to 1.0.
    #[structable(optional, wide)]
    rxtx_factor: Option<f32>,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created. Currently, the
    /// empty string ('') is used to represent 0. As of microversion 2.75
    /// default return value of swap is 0 instead of empty string.
    #[structable(optional, wide)]
    swap: Option<u32>,

    /// The number of virtual CPUs that will be allocated to the server.
    #[structable(optional, wide)]
    vcpus: Option<u32>,
}

#[async_trait]
impl Command for FlavorsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Flavors with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Flavors::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.min_disk {
            ep_builder.min_disk(*val);
        }
        if let Some(val) = &self.args.min_ram {
            ep_builder.min_ram(*val);
        }
        if let Some(val) = &self.args.is_public {
            ep_builder.is_public(*val);
        }
        if let Some(val) = &self.args.limit {
            ep_builder.limit(*val);
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

        op.output_list::<Flavors>(data)?;
        Ok(())
    }
}
