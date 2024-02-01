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
use clap::ValueEnum;
use http::Response;
use openstack_sdk::api::compute::v2::server::os_reset_state;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Resets the state of a server.
///
/// Specify the `os-resetState` action and the `state` in the request body.
///
/// Policy defaults enable only users with the administrative role to
/// perform this operation. Cloud providers can change these permissions
/// through the `policy.json` file.
///
/// Normal response codes: 202
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "Reset Server State (os-resetState Action)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    os_reset_state: OsResetState,
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

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum State {
    Active,
    Error,
}

/// OsResetState Body data
#[derive(Args, Debug, Clone)]
struct OsResetState {
    /// The state of the server to be set, `active` or `error` are valid.
    #[arg(long)]
    state: State,
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

        let mut ep_builder = os_reset_state::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_reset_state data
        let args = &self.args.os_reset_state;
        let mut os_reset_state_builder = os_reset_state::OsResetStateBuilder::default();

        let tmp = match &args.state {
            State::Active => os_reset_state::State::Active,
            State::Error => os_reset_state::State::Error,
        };
        os_reset_state_builder.state(tmp);

        ep_builder.os_reset_state(os_reset_state_builder.build().unwrap());

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
