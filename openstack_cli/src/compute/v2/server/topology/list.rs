use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::compute::v2::server::topology::list;
use openstack_sdk::api::QueryAsync;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Shows NUMA topology information for a server.
///
/// Policy defaults enable only users with the administrative role or the
/// owners
/// of the server to perform this operation. Cloud providers can change these
/// permissions through the `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), notfound(404), forbidden(403)
#[derive(Args, Clone, Debug)]
#[command(about = "Show Server Topology")]
pub struct TopologiesArgs {
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
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[arg(value_name = "SERVER_ID", id = "path_param_server_id")]
    server_id: String,
}

/// Topologies list command
pub struct TopologiesCmd {
    pub args: TopologiesArgs,
}
/// Topologies response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The mapping of server cores to host physical CPU
    #[serde()]
    #[structable(optional)]
    cpu_pinning: Option<HashMapStringi32>,

    /// A list of IDs of the virtual CPU assigned to this NUMA node.
    #[serde()]
    #[structable(optional)]
    vcpu_set: Option<Veci32>,

    /// A mapping of host cpus thread sibling.
    #[serde()]
    #[structable(optional)]
    siblings: Option<Veci32>,

    /// The amount of memory assigned to this NUMA node in MB.
    #[serde()]
    #[structable(optional)]
    memory_mb: Option<i32>,

    /// The host NUMA node the virtual NUMA node is map to.
    #[serde()]
    #[structable(optional)]
    host_node: Option<i32>,

    /// The page size in KB of a server. This field is `null` if the
    /// page size information is not available.
    #[serde()]
    #[structable(optional)]
    pagesize_kb: Option<i32>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringi32(HashMap<String, i32>);
impl fmt::Display for HashMapStringi32 {
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct Veci32(Vec<i32>);
impl fmt::Display for Veci32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[async_trait]
impl OSCCommand for TopologiesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Topologies with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
