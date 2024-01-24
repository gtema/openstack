//! Get specific access token.
//!
//! GET/HEAD /v3/users/{user_id}/OS-OAUTH1/access_tokens/{access_token_id}
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

use openstack_sdk::api::identity::v3::user::os_oauth1::access_token::get;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct AccessTokenArgs {
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
    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[arg()]
    user_id: String,

    /// access_token_id parameter for /v3/users/{user_id}/OS-
    /// OAUTH1/access_tokens/{access_token_id}/roles/{role_id} API
    #[arg()]
    id: String,
}

/// AccessToken show command
pub struct AccessTokenCmd {
    pub args: AccessTokenArgs,
}
/// AccessToken response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for AccessTokenCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show AccessToken with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.args.path.user_id);
        ep_builder.id(&self.args.path.id);
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
