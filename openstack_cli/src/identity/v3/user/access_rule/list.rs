//! List all access rules for a user.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/access\_rules`
//!
use async_trait::async_trait;

use clap::Args;

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

use openstack_sdk::api::identity::v3::user::access_rule::list;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct AccessRulesArgs {
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

/// AccessRules list command
pub struct AccessRulesCmd {
    pub args: AccessRulesArgs,
}
/// AccessRules response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    #[serde()]
    #[structable(optional, wide)]
    path: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    method: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    service: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,
}

#[async_trait]
impl OSCCommand for AccessRulesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List AccessRules with {:?}", self.args);

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
