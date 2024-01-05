//! Deletes a floating IP port forwarding.
//!
//! Normal response codes: 204
//!
//! Error response codes: 404
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
use openstack_sdk::api::network::v2::floatingip::port_forwarding::delete;
use openstack_sdk::api::network::v2::floatingip::port_forwarding::find;
use openstack_sdk::api::QueryAsync;

/// Deletes a floating IP port forwarding.
///
/// Normal response codes: 204
///
/// Error response codes: 404
#[derive(Args, Clone, Debug)]
pub struct PortForwardingArgs {
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
    /// floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id}
    /// API
    #[arg()]
    floatingip_id: String,

    /// id parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API
    #[arg()]
    id: String,
}

pub struct PortForwardingCmd {
    pub args: PortForwardingArgs,
}

#[async_trait]
impl Command for PortForwardingCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete PortForwarding with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = find::Request::builder();
        // Set path parameters
        ep_builder.floatingip_id(&self.args.path.floatingip_id);
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data = find(ep).query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
