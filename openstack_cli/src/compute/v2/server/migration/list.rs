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

use openstack_sdk::api::compute::v2::server::migration::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists in-progress live migrations for a given server.
///
/// Policy defaults enable only users with the administrative role to perform
/// this operation. Cloud providers can change these permissions through the
/// `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "List Migrations")]
pub struct MigrationsArgs {
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

/// Migrations list command
pub struct MigrationsCmd {
    pub args: MigrationsArgs,
}
/// Migrations response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The target compute for a migration.
    #[serde()]
    #[structable(optional, wide)]
    dest_compute: Option<String>,

    /// The target host for a migration.
    #[serde()]
    #[structable(optional, wide)]
    dest_host: Option<String>,

    /// The target node for a migration.
    #[serde()]
    #[structable(optional, wide)]
    dest_node: Option<String>,

    /// The ID of the server migration.
    #[serde()]
    #[structable(optional)]
    id: Option<i32>,

    /// The source compute for a migration.
    #[serde()]
    #[structable(optional, wide)]
    source_compute: Option<String>,

    /// The source node for a migration.
    #[serde()]
    #[structable(optional, wide)]
    source_node: Option<String>,

    /// The current status of the migration.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The ID of the project which initiated the server migration. The value
    /// may be `null` for older migration records.
    ///
    ///
    /// **New in version 2.80**
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// The ID of the user which initiated the server migration. The value
    /// may be `null` for older migration records.
    ///
    ///
    /// **New in version 2.80**
    #[serde()]
    #[structable(optional, wide)]
    user_id: Option<String>,

    /// The UUID of the migration.
    ///
    ///
    /// **New in version 2.59**
    #[serde()]
    #[structable(optional)]
    uuid: Option<String>,

    /// The amount of disk, in bytes, that has been processed during the
    /// migration.
    #[serde()]
    #[structable(optional, wide)]
    disk_processed_bytes: Option<i32>,

    /// The amount of disk, in bytes, that still needs to be migrated.
    #[serde()]
    #[structable(optional, wide)]
    disk_remaining_bytes: Option<i32>,

    /// The total amount of disk, in bytes, that needs to be migrated.
    #[serde()]
    #[structable(optional, wide)]
    disk_total_bytes: Option<i32>,

    /// The amount of memory, in bytes, that has been processed during the
    /// migration.
    #[serde()]
    #[structable(optional, wide)]
    memory_processed_bytes: Option<i32>,

    /// The amount of memory, in bytes, that still needs to be migrated.
    #[serde()]
    #[structable(optional, wide)]
    memory_remaining_bytes: Option<i32>,

    /// The total amount of memory, in bytes, that needs to be migrated.
    #[serde()]
    #[structable(optional, wide)]
    memory_total_bytes: Option<i32>,

    /// The UUID of the server.
    #[serde()]
    #[structable(optional, wide)]
    server_uuid: Option<String>,
}

#[async_trait]
impl OSCCommand for MigrationsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Migrations with {:?}", self.args);

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
