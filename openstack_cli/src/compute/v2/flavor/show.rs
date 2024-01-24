//! Shows details for a flavor.
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

use crate::common::IntString;
use crate::common::NumString;
use openstack_sdk::api::compute::v2::flavor::find;
use openstack_sdk::api::compute::v2::flavor::get;
use openstack_sdk::api::find;
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
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.1/flavors/{id}/action API
    #[arg()]
    id: String,
}

/// Flavor show command
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
        info!("Show Flavor with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.args.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
