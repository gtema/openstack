//! List all application credentials for a user.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/application\_credentials`
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

use openstack_sdk::api::identity::v3::user::application_credential::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ApplicationCredentialsArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {
    /// The name of the application credential. Must be unique to a user.
    #[arg(long)]
    name: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[arg()]
    user_id: String,
}

/// ApplicationCredentials list command
pub struct ApplicationCredentialsCmd {
    pub args: ApplicationCredentialsArgs,
}
/// ApplicationCredentials response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the application credential.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the project the application credential was created for and
    /// that authentication requests using this application credential will be
    /// scoped to.
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    expires_at: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    roles: Option<VecResponseRoles>,

    #[serde()]
    #[structable(optional, wide)]
    unrestricted: Option<bool>,

    #[serde()]
    #[structable(optional, wide)]
    access_rules: Option<VecResponseAccessRules>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseRoles {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseRoles(Vec<ResponseRoles>);
impl fmt::Display for VecResponseRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseAccessRules {
    path: Option<String>,
    method: Option<String>,
    service: Option<String>,
    id: Option<String>,
}

impl fmt::Display for ResponseAccessRules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "path={}",
                self.path
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "method={}",
                self.method
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "service={}",
                self.service
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseAccessRules(Vec<ResponseAccessRules>);
impl fmt::Display for VecResponseAccessRules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[async_trait]
impl Command for ApplicationCredentialsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List ApplicationCredentials with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.args.path.user_id);
        // Set query parameters
        if let Some(val) = &self.args.query.name {
            ep_builder.name(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
