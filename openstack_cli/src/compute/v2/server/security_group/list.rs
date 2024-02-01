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

use openstack_sdk::api::compute::v2::server::security_group::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Lists security groups for a server.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "List Security Groups By Server")]
pub struct SecurityGroupsArgs {
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

/// SecurityGroups list command
pub struct SecurityGroupsCmd {
    pub args: SecurityGroupsArgs,
}
/// SecurityGroups response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the security group.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The security group name.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// Security group description.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The UUID of the tenant in a multi-tenancy cloud.
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The list of security group rules.
    #[serde()]
    #[structable(optional, wide)]
    rules: Option<VecResponseRules>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
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
struct ResponseGroup {
    name: Option<String>,
}

impl fmt::Display for ResponseGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "name={}",
            self.name
                .clone()
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseRules {
    id: Option<String>,
    from_port: Option<i32>,
    to_port: Option<i32>,
    ip_protocol: Option<String>,
    ip_range: Option<HashMapStringValue>,
    group: Option<ResponseGroup>,
    parent_group_id: Option<String>,
}

impl fmt::Display for ResponseRules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "from_port={}",
                self.from_port
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "to_port={}",
                self.to_port
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ip_protocol={}",
                self.ip_protocol
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ip_range={}",
                self.ip_range
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "group={}",
                self.group
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "parent_group_id={}",
                self.parent_group_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseRules(Vec<ResponseRules>);
impl fmt::Display for VecResponseRules {
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
impl OSCCommand for SecurityGroupsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List SecurityGroups with {:?}", self.args);

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
