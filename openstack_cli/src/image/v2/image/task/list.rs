//! Shows tasks associated with an image.
//! *(Since Image API v2.12)*
//!
//! The response body contains list of tasks, possibly empty, associated
//! with the specified image.
//!
//! Preconditions
//!
//! Normal response codes: 200
//!
//! Error response codes: 404
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
use crate::{error::OpenStackCliError, OSCCommand};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::image::v2::image::task::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct TasksArgs {
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
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg()]
    image_id: String,
}

/// Tasks list command
pub struct TasksCmd {
    pub args: TasksArgs,
}
/// Tasks response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// An identifier for the task
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The type of task represented by this content
    #[serde(rename = "type")]
    #[structable(optional, title = "type", wide)]
    _type: Option<String>,

    /// The current status of this task
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The parameters required by task, JSON blob
    #[serde()]
    #[structable(optional, wide)]
    input: Option<HashMapStringValue>,

    /// The result of current task, JSON blob
    #[serde()]
    #[structable(optional, wide)]
    result: Option<HashMapStringValue>,

    /// An identifier for the owner of this task
    #[serde()]
    #[structable(optional, wide)]
    owner: Option<String>,

    /// Human-readable informative message only included when appropriate
    /// (usually on failure)
    #[serde()]
    #[structable(optional, wide)]
    message: Option<String>,

    /// Image associated with the task
    #[serde()]
    #[structable(optional, wide)]
    image_id: Option<String>,

    /// Human-readable informative request-id
    #[serde()]
    #[structable(optional, wide)]
    request_id: Option<String>,

    /// User associated with the task
    #[serde()]
    #[structable(optional, wide)]
    user_id: Option<String>,

    /// Datetime when this resource would be subject to removal
    #[serde()]
    #[structable(optional, wide)]
    expires_at: Option<String>,

    /// Datetime when this resource was created
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Datetime when this resource was updated
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    #[serde(rename = "self")]
    #[structable(optional, title = "self", wide)]
    _self: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    schema: Option<String>,
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

#[async_trait]
impl OSCCommand for TasksCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Tasks with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.image_id(&self.args.path.image_id);
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
