//! Changes the password for a user.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/user\_change\_password`
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

use dialoguer::Password;
use openstack_sdk::api::identity::v3::user::password::set;
use openstack_sdk::api::RawQueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct PasswordArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    user: User,
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
/// User Body data
#[derive(Args, Debug, Clone)]
struct User {
    /// The original password for the user.
    #[arg(long)]
    original_password: Option<String>,

    /// The new password for the user.
    #[arg(long)]
    password: Option<String>,
}

/// Password set command
pub struct PasswordCmd {
    pub args: PasswordArgs,
}
/// Password response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for PasswordCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Password with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.args.path.user_id);
        // Set query parameters
        // Set body parameters
        // Set Request.user data
        let args = &self.args.user;
        let mut user_builder = set::UserBuilder::default();
        if let Some(val) = &args.original_password {
            user_builder.original_password(val.clone());
        } else {
            let secret = Password::new()
                .with_prompt("The original password for the user")
                .interact()
                .unwrap();
            user_builder.original_password(secret.to_string());
        }

        if let Some(val) = &args.password {
            user_builder.password(val.clone());
        } else {
            let secret = Password::new()
                .with_prompt("The new password for the user")
                .interact()
                .unwrap();
            user_builder.password(secret.to_string());
        }

        ep_builder.user(user_builder.build().unwrap());

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
