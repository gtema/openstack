//! Get single Flavor
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
use openstack_sdk::api::compute::v2::flavor::find;
use openstack_sdk::api::compute::v2::flavor::get;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;

/// Get single Flavor
#[derive(Args, Clone, Debug)]
pub struct FlavorArgs {
    /// Flavor ID
    #[arg()]
    id: String,
}

pub struct FlavorCmd {
    pub args: FlavorArgs,
}

/// Flavor
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Flavor {
    /// Whether or not the flavor has been administratively disabled. This is
    /// typically only visible to administrative users.
    #[serde(rename = "OS-FLV-DISABLED:disabled")]
    #[structable(optional)]
    is_disabled: Option<bool>,

    /// The size of the ephemeral disk that will be created, in GiB. Ephemeral
    /// disks may be written over on server state changes. So should only be
    /// used as a scratch space for applications that are aware of its
    /// limitations. Defaults to 0.
    #[serde(rename = "OS-FLV-EXT-DATA:ephemeral")]
    #[structable(optional)]
    ephemeral: Option<u32>,

    /// The description of the flavor.
    #[structable(optional)]
    description: Option<String>,

    /// The size of the root disk that will be created in GiB. If 0 the root
    /// disk will be set to exactly the size of the image used to deploy the
    /// instance. However, in this case the scheduler cannot select the compute
    /// host based on the virtual image size. Therefore, 0 should only be used
    /// for volume booted instances or for testing purposes. Volume-backed
    /// instances can be enforced for flavors with zero root disk via the
    /// os_compute_api:servers:create:zero_disk_flavor policy rule.
    #[structable(optional)]
    disk: Option<u32>,

    /// A dictionary of the flavor's extra-specs key-and-value pairs.
    extra_specs: HashMapStringString,

    /// The ID of the flavor. While people often make this look like an int,
    /// this is really a string.
    #[structable(optional)]
    id: Option<String>,

    /// The name of this flavor.
    #[structable(optional)]
    name: Option<String>,

    /// The name of this flavor when returned by server list/show
    #[structable(optional)]
    original_name: Option<String>,

    /// ``True`` if this is a publicly visible flavor. ``False`` if this is a
    /// private image.
    #[serde(rename = "os-flavor-access:is_public")]
    #[structable(optional)]
    is_public: Option<bool>,

    /// The amount of RAM (in MB) this flavor offers.
    #[structable(optional)]
    ram: Option<u32>,

    /// The receive / transmit factor (as a float) that will be set on ports if
    /// the network backend supports the QOS extension. Otherwise it will be
    /// ignored. It defaults to 1.0.
    #[structable(optional)]
    rxtx_factor: Option<f32>,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created. Currently, the
    /// empty string ('') is used to represent 0. As of microversion 2.75
    /// default return value of swap is 0 instead of empty string.
    #[structable(optional)]
    swap: Option<u32>,

    /// The number of virtual CPUs that will be allocated to the server.
    #[structable(optional)]
    vcpus: Option<u32>,
}

#[async_trait]
impl Command for FlavorCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Flavor with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = find::Flavor::builder();
        // Set path parameters
        ep_builder.id(&self.args.id);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Compute)
            .await?;
        let data = find(ep).query_async(client).await?;
        op.output_single::<Flavor>(data)?;
        Ok(())
    }
}
