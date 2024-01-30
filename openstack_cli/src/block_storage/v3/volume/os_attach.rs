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

use clap::ValueEnum;
use openstack_sdk::api::block_storage::v3::volume::os_attach;
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
    os_attach: OsAttach,
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

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Mode {
    Ro,
    Rw,
}

/// OsAttach Body data
#[derive(Args, Debug, Clone)]
struct OsAttach {
    #[arg(long)]
    instance_uuid: Option<String>,

    #[arg(long)]
    mountpoint: String,

    #[arg(long)]
    host_name: Option<String>,

    #[arg(long)]
    mode: Option<Mode>,
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

        let mut ep_builder = os_attach::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_attach data
        let args = &self.args.os_attach;
        let mut os_attach_builder = os_attach::OsAttachBuilder::default();
        if let Some(val) = &args.instance_uuid {
            os_attach_builder.instance_uuid(val.clone());
        }

        os_attach_builder.mountpoint(args.mountpoint.clone());

        if let Some(val) = &args.host_name {
            os_attach_builder.host_name(val.clone());
        }

        if let Some(val) = &args.mode {
            let tmp = match val {
                Mode::Ro => os_attach::Mode::Ro,
                Mode::Rw => os_attach::Mode::Rw,
            };
            os_attach_builder.mode(tmp);
        }

        ep_builder.os_attach(os_attach_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
