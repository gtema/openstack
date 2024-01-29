//! Deletes an extra spec, by key, for a flavor, by ID.
//!
//! Normal response codes: 200
//!
//! Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
//!
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;

use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, OSCCommand};

use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::compute::v2::flavor::extra_spec::delete;
use openstack_sdk::api::RawQueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ExtraSpecArgs {
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
    /// flavor_id parameter for /v2.1/flavors/{flavor_id}/os-flavor-access API
    #[arg()]
    flavor_id: String,

    /// id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API
    #[arg()]
    id: String,
}

/// ExtraSpec delete command
pub struct ExtraSpecCmd {
    pub args: ExtraSpecArgs,
}
/// ExtraSpec response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl OSCCommand for ExtraSpecCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete ExtraSpec with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = delete::Request::builder();

        // Set path parameters
        ep_builder.flavor_id(&self.args.path.flavor_id);
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
