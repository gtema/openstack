//! Delete Subnet
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::subnet::delete;
use openstack_sdk::api::network::v2::subnet::find;
use openstack_sdk::api::RawQueryAsync;

/// Delete Subnet
#[derive(Args, Clone, Debug)]
pub struct SubnetArgs {
    /// Subnet ID
    #[arg()]
    id: String,
}

pub struct SubnetCmd {
    pub args: SubnetArgs,
}

/// Subnet
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Subnet {}

#[async_trait]
impl Command for SubnetCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Subnet with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = delete::Subnet::builder();
        // Set path parameters
        ep_builder.id(&self.args.id);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
