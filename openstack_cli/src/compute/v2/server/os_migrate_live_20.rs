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
use openstack_sdk::api::compute::v2::server::os_migrate_live_20;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

#[derive(Args, Clone, Debug)]
#[command(about = "Live-Migrate Server (os-migrateLive Action) (microversion = 2.0)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    os_migrate_live: OsMigrateLive,
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
/// OsMigrateLive Body data
#[derive(Args, Debug, Clone)]
struct OsMigrateLive {
    /// Set to `True` to enable over commit when the destination host is
    /// checked for
    /// available disk space. Set to `False` to disable over commit. This
    /// setting affects
    /// only the libvirt virt driver.
    ///
    ///
    /// **Available until version 2.25**
    #[arg(action=clap::ArgAction::Set, long)]
    block_migration: bool,

    /// Set to `True` to enable over commit when the destination host is
    /// checked for
    /// available disk space. Set to `False` to disable over commit. This
    /// setting affects
    /// only the libvirt virt driver.
    ///
    ///
    /// **Available until version 2.25**
    #[arg(action=clap::ArgAction::Set, long)]
    disk_over_commit: bool,

    /// The host to which to migrate the server. If this parameter is `None`,
    /// the scheduler chooses a host.
    ///
    ///
    ///
    /// Warning
    ///
    ///
    /// Prior to microversion 2.30, specifying a host will bypass
    /// validation by the scheduler, which could result in failures to actually
    /// migrate the instance to the specified host, or over-subscription of the
    /// host. It is recommended to either not specify a host so that the
    /// scheduler will pick one, or specify a host with microversion >= 2.30
    /// and
    /// without `force=True` set.
    #[arg(long)]
    host: String,
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

        let mut ep_builder = os_migrate_live_20::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.0");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_migrate_live data
        let args = &self.args.os_migrate_live;
        let mut os_migrate_live_builder = os_migrate_live_20::OsMigrateLiveBuilder::default();

        os_migrate_live_builder.block_migration(args.block_migration);

        os_migrate_live_builder.disk_over_commit(args.disk_over_commit);

        os_migrate_live_builder.host(args.host.clone());

        ep_builder.os_migrate_live(os_migrate_live_builder.build().unwrap());

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
