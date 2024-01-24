//! Creates a flavor.
//!
//! Creating a flavor is typically only available to administrators of a
//! cloud because this has implications for scheduling efficiently in the
//! cloud.
//!
//! Normal response codes: 200
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

use crate::common::IntString;
use crate::common::NumString;
use openstack_sdk::api::compute::v2::flavor::create_20;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct FlavorArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    flavor: Flavor,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}
/// Flavor Body data
#[derive(Args, Debug, Clone)]
struct Flavor {
    /// The display name of a flavor.
    #[arg(long)]
    name: String,

    /// Only alphanumeric characters with hyphen ‘-’, underscore ‘\_’, spaces
    /// and dots ‘.’ are permitted. If an ID is not provided, then a default
    /// UUID
    /// will be assigned.
    #[arg(long)]
    id: Option<String>,

    /// The number of virtual CPUs that will be allocated to the server.
    #[arg(long)]
    ram: String,

    /// The number of virtual CPUs that will be allocated to the server.
    #[arg(long)]
    vcpus: String,

    /// The size of a dedicated swap disk that will be allocated, in
    /// MiB. If 0 (the default), no dedicated swap disk will be created.
    #[arg(long)]
    disk: String,

    /// The size of a dedicated swap disk that will be allocated, in
    /// MiB. If 0 (the default), no dedicated swap disk will be created.
    #[arg(long)]
    os_flv_ext_data_ephemeral: Option<String>,

    /// The size of a dedicated swap disk that will be allocated, in
    /// MiB. If 0 (the default), no dedicated swap disk will be created.
    #[arg(long)]
    swap: Option<String>,

    /// The receive / transmit factor (as a float) that will be set on
    /// ports if the network backend supports the QOS extension.
    /// Otherwise it will be ignored. It defaults to 1.0.
    #[arg(long)]
    rxtx_factor: Option<String>,

    /// Whether the flavor is public (available to all projects) or scoped
    /// to a set of projects. Default is True if not specified.
    #[arg(action=clap::ArgAction::Set, long)]
    os_flavor_access_is_public: Option<bool>,
}

/// Flavor create command
pub struct FlavorCmd {
    pub args: FlavorArgs,
}
/// Flavor response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The display name of a flavor.
    #[serde()]
    #[structable()]
    name: String,

    /// The ID of the flavor. While people often make this look like
    /// an int, this is really a string.
    #[serde()]
    #[structable(optional)]
    id: Option<NumString>,

    /// The number of virtual CPUs that will be allocated to the server.
    #[serde()]
    #[structable(wide)]
    ram: IntString,

    /// The number of virtual CPUs that will be allocated to the server.
    #[serde()]
    #[structable(wide)]
    vcpus: IntString,

    /// The size of a dedicated swap disk that will be allocated, in
    /// MiB. If 0 (the default), no dedicated swap disk will be created.
    /// Currently, the empty string (‘’) is used to represent 0.
    /// As of microversion 2.75 default return value of swap is 0
    /// instead of empty string.
    #[serde()]
    #[structable(wide)]
    disk: IntString,

    /// The size of a dedicated swap disk that will be allocated, in
    /// MiB. If 0 (the default), no dedicated swap disk will be created.
    /// Currently, the empty string (‘’) is used to represent 0.
    /// As of microversion 2.75 default return value of swap is 0
    /// instead of empty string.
    #[serde(rename = "OS-FLV-EXT-DATA:ephemeral")]
    #[structable(optional, title = "OS-FLV-EXT-DATA:ephemeral", wide)]
    os_flv_ext_data_ephemeral: Option<IntString>,

    /// The size of a dedicated swap disk that will be allocated, in
    /// MiB. If 0 (the default), no dedicated swap disk will be created.
    /// Currently, the empty string (‘’) is used to represent 0.
    /// As of microversion 2.75 default return value of swap is 0
    /// instead of empty string.
    #[serde()]
    #[structable(optional, wide)]
    swap: Option<IntString>,

    /// The receive / transmit factor (as a float) that will be set on
    /// ports if the network backend supports the QOS extension.
    /// Otherwise it will be ignored. It defaults to 1.0.
    #[serde()]
    #[structable(optional, wide)]
    rxtx_factor: Option<NumString>,

    /// Whether the flavor is public (available to all projects) or scoped
    /// to a set of projects. Default is True if not specified.
    #[serde(rename = "os-flavor-access:is_public")]
    #[structable(optional, title = "os-flavor-access:is_public", wide)]
    os_flavor_access_is_public: Option<bool>,

    /// A dictionary of the flavor’s extra-specs key-and-value pairs. This will
    /// only be included if the user is allowed by policy to index flavor
    /// extra\_specs.
    ///
    ///
    /// **New in version 2.61**
    #[serde()]
    #[structable(optional, wide)]
    extra_specs: Option<HashMapStringNumString>,

    /// Links to the resources in question. See [API Guide / Links and
    /// References](https://docs.openstack.org/api-
    /// guide/compute/links_and_references.html)
    /// for more info.
    #[serde()]
    #[structable(optional, wide)]
    links: Option<Value>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringNumString(HashMap<String, NumString>);
impl fmt::Display for HashMapStringNumString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseLinks {
    href: Option<String>,
    rel: Option<String>,
}

impl fmt::Display for ResponseLinks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "href={}",
                self.href
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rel={}",
                self.rel
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

#[async_trait]
impl Command for FlavorCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Flavor with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create_20::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.0");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.flavor data
        let args = &self.args.flavor;
        let mut flavor_builder = create_20::FlavorBuilder::default();

        flavor_builder.name(&args.name);

        if let Some(val) = &args.id {
            flavor_builder.id(Some(val.into()));
        }

        flavor_builder.ram(&args.ram);

        flavor_builder.vcpus(&args.vcpus);

        flavor_builder.disk(&args.disk);

        if let Some(val) = &args.os_flv_ext_data_ephemeral {
            flavor_builder.os_flv_ext_data_ephemeral(val);
        }

        if let Some(val) = &args.swap {
            flavor_builder.swap(val);
        }

        if let Some(val) = &args.rxtx_factor {
            flavor_builder.rxtx_factor(val);
        }

        if let Some(val) = &args.os_flavor_access_is_public {
            flavor_builder.os_flavor_access_is_public(*val);
        }

        ep_builder.flavor(flavor_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
