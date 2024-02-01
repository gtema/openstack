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
use openstack_sdk::api::compute::v2::server::delete;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Deletes a server.
///
/// By default, the instance is going to be (hard) deleted immediately from
/// the system, but you can set `reclaim\_instance\_interval` > 0 to make
/// the API soft delete the instance, so that the instance won’t be
/// deleted until the `reclaim\_instance\_interval` has expired since the
/// instance was soft deleted. The instance marked as `SOFT\_DELETED` can
/// be recovered via `restore` action before it’s really deleted from the
/// system.
///
/// **Preconditions**
///
/// **Asynchronous postconditions**
///
/// **Troubleshooting**
///
/// Normal response codes: 204
///
/// Error response codes: unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Delete Server")]
pub struct ServerArgs {
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
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}

/// Server delete command
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
        info!("Delete Server with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = delete::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
