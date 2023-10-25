//! Pauses a server. Changes its status to PAUSED.
//! Specify the pause action in the request body.
//! Policy defaults enable only users with the administrative role or the owner
//! of the server to perform this operation. Cloud providers can change these
//! permissions through the policy.json file.
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::compute::v2::server::action::pause;
use openstack_sdk::api::compute::v2::server::find;
use openstack_sdk::api::find;
use openstack_sdk::api::RawQueryAsync;

/// Pauses a server. Changes its status to PAUSED.
/// Specify the pause action in the request body.
/// Policy defaults enable only users with the administrative role or the owner
/// of the server to perform this operation. Cloud providers can change these
/// permissions through the policy.json file.
#[derive(Args, Clone, Debug)]
pub struct ServerArgs {
    /// Server ID
    #[arg()]
    id: String,
}

pub struct ServerCmd {
    pub args: ServerArgs,
}

/// Server
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Server {}

#[async_trait]
impl Command for ServerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Server with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = pause::Server::builder();
        // Set path parameters
        ep_builder.id(&self.args.id);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Compute)
            .await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = Server {};
        // Maybe output some headers metadata
        op.output_human::<Server>(&data)?;
        Ok(())
    }
}
