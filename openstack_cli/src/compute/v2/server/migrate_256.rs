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
use openstack_sdk::api::compute::v2::server::migrate_256;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Migrates a server to a host.
///
/// Specify the `migrate` action in the request body.
///
/// Up to microversion 2.55, the scheduler chooses the host.
/// Starting from microversion 2.56, the `host` parameter is available
/// to specify the destination host. If you specify `null` or don’t specify
/// this parameter, the scheduler chooses a host.
///
/// **Asynchronous Postconditions**
///
/// A successfully migrated server shows a `VERIFY\_RESIZE` status and
/// `finished`
/// migration status. If the cloud has configured the [resize\_confirm\_window]
/// (https://docs.openstack.org/nova/latest/configuration/config.html#DEFAULT.r
/// esize_confirm_window)
/// option of the Compute service to a positive value, the Compute service
/// automatically confirms the migrate operation after the configured interval.
///
/// There are two different policies for this action, depending on whether the
/// host
/// parameter is set. Both defaults enable only users with the administrative
/// role
/// to perform this operation. Cloud providers can change these permissions
/// through the `policy.json` file.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403)
/// itemNotFound(404), conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Migrate Server (migrate Action) (microversion = 2.56)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    migrate: Option<Migrate>,
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
/// Migrate Body data
#[derive(Args, Debug, Clone)]
struct Migrate {
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

        let mut ep_builder = migrate_256::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.56");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.migrate data
        let args = &self.args.migrate;

        if let Some(lmigrate) = &args {
            let mut migrate_builder = migrate_256::MigrateBuilder::default();
            if let Some(val) = &lmigrate.host {
                migrate_builder.host(Some(val.into()));
            }
            ep_builder.migrate(migrate_builder.build().expect("A valid object"));
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
