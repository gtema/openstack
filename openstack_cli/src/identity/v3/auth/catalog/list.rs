//! New in version 3.3
//!
//! This call returns a service catalog for the X-Auth-Token provided in the
//! request, even if the token does not contain a catalog itself (for example,
//! if it was generated using ?nocatalog).
//!
//! The structure of the catalog object is identical to that contained in a
//! token.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/auth\_catalog`
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

use openstack_sdk::api::identity::v3::auth::catalog::list;
use openstack_sdk::api::RawQueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct CatalogsArgs {
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
pub struct PathParameters {}

/// Catalogs list command
pub struct CatalogsCmd {
    pub args: CatalogsArgs,
}
/// Catalogs response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for CatalogsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Catalogs with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
