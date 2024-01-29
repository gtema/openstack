//! Lists groups to which a user belongs.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/user\_groups`
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
use crate::{error::OpenStackCliError, OSCCommand};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::identity::v3::user::group::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct GroupsArgs {
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
}

/// Groups list command
pub struct GroupsCmd {
    pub args: GroupsArgs,
}
/// Groups response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The description of the group.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the domain of the group.
    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    /// The ID of the group.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The name of the group.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The date and time when the group membership expires.
    /// A `null` value indicates that the membership never expires.
    ///
    ///
    /// **New in version 3.14**
    #[serde()]
    #[structable(optional, wide)]
    membership_expires_at: Option<String>,
}

#[async_trait]
impl OSCCommand for GroupsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Groups with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.args.path.user_id);
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
