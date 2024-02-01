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

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::server::unshelve_291;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

#[derive(Args, Clone, Debug)]
#[command(about = "Unshelve (Restore) Shelved Server (unshelve Action) (microversion = 2.91)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    unshelve: Option<Unshelve>,
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
/// Unshelve Body data
#[derive(Args, Debug, Clone)]
struct Unshelve {
    #[arg(long)]
    availability_zone: Option<String>,

    #[arg(long)]
    host: Option<String>,
}

/// Server action command
pub struct ServerCmd {
    pub args: ServerArgs,
}
/// Server response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

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

        let mut ep_builder = unshelve_291::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.91");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.unshelve data
        let args = &self.args.unshelve;

        if let Some(lunshelve) = &args {
            let mut unshelve_builder = unshelve_291::UnshelveBuilder::default();
            if let Some(val) = &lunshelve.availability_zone {
                unshelve_builder.availability_zone(Some(val.into()));
            }
            if let Some(val) = &lunshelve.host {
                unshelve_builder.host(val.clone());
            }
            ep_builder.unshelve(unshelve_builder.build().expect("A valid object"));
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
