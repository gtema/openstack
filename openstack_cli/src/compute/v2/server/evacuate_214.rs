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

use openstack_sdk::api::compute::v2::server::evacuate_214;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

#[derive(Args, Clone, Debug)]
#[command(about = "Evacuate Server (evacuate Action) (microversion = 2.14)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    evacuate: Evacuate,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}
/// Evacuate Body data
#[derive(Args, Debug, Clone)]
struct Evacuate {
    /// The name or ID of the host to which the server is evacuated.
    /// If you omit this parameter, the scheduler chooses a host.
    ///
    ///
    ///
    /// Warning
    ///
    ///
    /// Prior to microversion 2.29, specifying a host will bypass
    /// validation by the scheduler, which could result in failures to actually
    /// evacuate the instance to the specified host, or over-subscription of
    /// the
    /// host. It is recommended to either not specify a host so that the
    /// scheduler will pick one, or specify a host with microversion >= 2.29
    /// and
    /// without `force=True` set.
    #[arg(long)]
    host: Option<String>,

    /// An administrative password to access the evacuated server.
    /// If you omit this parameter, the operation generates a new password.
    /// Up to API version 2.13, if `onSharedStorage` is set to `True` and
    /// this parameter is specified, an error is raised.
    #[arg(long)]
    admin_pass: Option<String>,
}

/// Server action command
pub struct ServerCmd {
    pub args: ServerArgs,
}
/// Server response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// An administrative password to access the evacuated instance.
    /// If you set `enable\_instance\_password` configuration option to
    /// `False`,
    /// the API wouldn’t return the `adminPass` field in response.
    ///
    ///
    /// **Available until version 2.13**
    #[serde(rename = "adminPass")]
    #[structable(title = "adminPass")]
    admin_pass: String,
}

#[async_trait]
impl OSCCommand for ServerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = evacuate_214::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.14");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.evacuate data
        let args = &self.args.evacuate;
        let mut evacuate_builder = evacuate_214::EvacuateBuilder::default();
        if let Some(val) = &args.host {
            evacuate_builder.host(val.clone());
        }

        if let Some(val) = &args.admin_pass {
            evacuate_builder.admin_pass(val.clone());
        }

        ep_builder.evacuate(evacuate_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
