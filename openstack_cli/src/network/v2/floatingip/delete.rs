//! Deletes a floating IP and, if present, its associated port.
//!
//! This example deletes a floating IP:
//!
//! Normal response codes: 204
//!
//! Error response codes: 401, 404, 412
//!
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
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::floatingip::delete;
use openstack_sdk::api::network::v2::floatingip::find;
use openstack_sdk::api::RawQueryAsync;

/// Deletes a floating IP and, if present, its associated port.
///
/// This example deletes a floating IP:
///
/// Normal response codes: 204
///
/// Error response codes: 401, 404, 412
#[derive(Args, Clone, Debug)]
pub struct FloatingipArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.0/floatingips/{id} API
    #[arg()]
    id: String,
}

pub struct FloatingipCmd {
    pub args: FloatingipArgs,
}

#[async_trait]
impl Command for FloatingipCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Floatingip with {:?}", self.args);

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
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
