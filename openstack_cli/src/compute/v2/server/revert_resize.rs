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
use openstack_sdk::api::compute::v2::server::revert_resize;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Cancels and reverts a pending resize action for a server.
///
/// Specify the `revertResize` action in the request body.
///
/// **Preconditions**
///
/// You can only revert the resized server where the status is
/// `VERIFY\_RESIZE` and the OS-EXT-STS:vm\_state is `resized`.
///
/// If the server is locked, you must have administrator privileges to revert
/// the resizing.
///
/// **Asynchronous Postconditions**
///
/// After you make this request, you typically must keep polling the server
/// status
/// to determine whether the request succeeded. A reverting resize operation
/// shows
/// a status of `REVERT\_RESIZE` and a task\_state of `resize\_reverting`. If
/// successful, the status will return to `ACTIVE` or `SHUTOFF`. You can also
/// see the reverted server in the compute node that OpenStack Compute manages.
///
/// **Troubleshooting**
///
/// If the server status remains `VERIFY\_RESIZE`, the request failed. Ensure
/// you
/// meet the preconditions and run the request again. If the request fails
/// again,
/// investigate the compute back end.
///
/// The server is not reverted in the compute node that OpenStack Compute
/// manages.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Revert Resized Server (revertResize Action)")]
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

        let mut ep_builder = revert_resize::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

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
