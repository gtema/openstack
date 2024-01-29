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
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use crate::common::parse_json;
use crate::common::parse_key_val;
use openstack_sdk::api::block_storage::v3::volume::os_force_detach;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct VolumeArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    os_force_detach: OsForceDetach,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v3/volumes/{id} API
    #[arg()]
    id: String,
}
/// OsForceDetach Body data
#[derive(Args, Debug, Clone)]
struct OsForceDetach {
    #[arg(long, value_name="JSON", value_parser=parse_json)]
    connector: Option<Option<Value>>,

    #[arg(long)]
    attachment_id: Option<String>,
}

/// Volume action command
pub struct VolumeCmd {
    pub args: VolumeArgs,
}
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ResponseData(HashMap<String, serde_json::Value>);

impl StructTable for ResponseData {
    fn build(&self, options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Name".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.extend(self.0.iter().map(|(k, v)| {
            Vec::from([
                k.clone(),
                serde_json::to_string(&v).expect("Is a valid data"),
            ])
        }));
        (headers, rows)
    }
}

#[async_trait]
impl OSCCommand for VolumeCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Volume with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = os_force_detach::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_force_detach data
        let args = &self.args.os_force_detach;
        let mut os_force_detach_builder = os_force_detach::OsForceDetachBuilder::default();
        if let Some(val) = &args.connector {
            os_force_detach_builder.connector(val.clone().map(|v| v.into()));
        }

        if let Some(val) = &args.attachment_id {
            os_force_detach_builder.attachment_id(Some(val.into()));
        }

        ep_builder.os_force_detach(os_force_detach_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
