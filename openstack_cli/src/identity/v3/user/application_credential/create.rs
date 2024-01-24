//! Creates an application credential for a user on the project to which the
//! current token is scoped.
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

use crate::common::parse_json;
use crate::common::parse_key_val;
use openstack_sdk::api::identity::v3::user::application_credential::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ApplicationCredentialArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long)]
    name: String,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)]
    secret: Option<String>,
    #[arg(long)]
    expires_at: Option<String>,
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    roles: Option<Vec<Value>>,
    #[arg(action=clap::ArgAction::Set, long)]
    unrestricted: Option<bool>,
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    access_rules: Option<Vec<Value>>,
    /// Additional properties to be sent with the request
    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    properties: Option<Vec<(String, Value)>>,
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

/// ApplicationCredential create command
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

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    #[serde()]
    #[structable(optional)]
    secret: Option<String>,

    #[serde()]
    #[structable(optional)]
    expires_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    roles: Option<VecResponseRoles>,

    #[serde()]
    #[structable(optional)]
    unrestricted: Option<bool>,

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
        info!("Create ApplicationCredential with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.args.path.user_id);
        // Set query parameters
        // Set body parameters
        // Set Request.name data
        let args = &self.args.name;

        ep_builder.name(args);

        // Set Request.description data
        if let Some(args) = &self.args.description {
            ep_builder.description(Some(args.into()));
        }

        // Set Request.secret data
        if let Some(args) = &self.args.secret {
            ep_builder.secret(Some(args.into()));
        }

        // Set Request.expires_at data
        if let Some(args) = &self.args.expires_at {
            ep_builder.expires_at(Some(args.into()));
        }

        // Set Request.roles data
        if let Some(args) = &self.args.roles {
            let sub: Vec<create::Roles> = args
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Roles>(v.clone()))
                .collect::<Vec<create::Roles>>();
            ep_builder.roles(sub);
        }

        // Set Request.unrestricted data
        if let Some(args) = &self.args.unrestricted {
            ep_builder.unrestricted(*args);
        }

        // Set Request.access_rules data
        if let Some(args) = &self.args.access_rules {
            let sub: Vec<create::AccessRules> = args
                .iter()
                .flat_map(|v| serde_json::from_value::<create::AccessRules>(v.clone()))
                .collect::<Vec<create::AccessRules>>();
            ep_builder.access_rules(sub);
        }

        if let Some(properties) = &self.args.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
