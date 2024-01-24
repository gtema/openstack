//! Show details of an application credential.
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

use openstack_sdk::api::identity::v3::user::application_credential::get;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ApplicationCredentialArgs {
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

    /// application_credential_id parameter for
    /// /v3/users/{user_id}/application_credentials/{application_credential_id}
    /// API
    #[arg()]
    application_credential_id: String,
}

/// ApplicationCredential show command
pub struct ApplicationCredentialCmd {
    pub args: ApplicationCredentialArgs,
}
/// ApplicationCredential response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the project the application credential was created for and
    /// that authentication requests using this application credential will be
    /// scoped to.
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The role name.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A description of the application credential’s purpose.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The secret that the application credential will be created with. If not
    /// provided, one will be generated.
    #[serde()]
    #[structable(optional)]
    secret: Option<String>,

    /// An optional expiry time for the application credential. If unset, the
    /// application credential does not expire.
    #[serde()]
    #[structable(optional)]
    expires_at: Option<String>,

    /// An optional list of role objects, identified by ID or name. The list
    /// may only contain roles that the user has assigned on the project.
    /// If not provided, the roles assigned to the application credential will
    /// be the same as the roles in the current token.
    #[serde()]
    #[structable(optional)]
    roles: Option<VecResponseRoles>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional)]
    unrestricted: Option<bool>,

    /// A list of `access\_rules` objects
    #[serde()]
    #[structable(optional)]
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
impl Command for ApplicationCredentialCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show ApplicationCredential with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.args.path.user_id);
        ep_builder.application_credential_id(&self.args.path.application_credential_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
