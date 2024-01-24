//! Lists users.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/users`
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

use openstack_sdk::api::identity::v3::user::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct UsersArgs {
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
    /// Filters the response by a domain ID.
    #[arg(long)]
    domain_id: Option<String>,

    /// If set to true, then only enabled projects will be returned. Any value
    /// other than 0 (including no value) will be interpreted as true.
    #[arg(long)]
    enabled: Option<bool>,

    /// Filters the response by a domain ID.
    #[arg(long)]
    idp_id: Option<String>,

    /// Filters the response by a resource name.
    #[arg(long)]
    name: Option<String>,

    /// Filter results based on which user passwords have expired. The query
    /// should include an operator and a timestamp with a colon (:) separating
    /// the two, for example: `password_expires_at={operator}:{timestamp}`.
    /// Valid operators are: `lt`, `lte`, `gt`, `gte`, `eq`, and `neq`.
    /// Valid timestamps are of the form: YYYY-MM-DDTHH:mm:ssZ.
    #[arg(long)]
    password_expires_at: Option<String>,

    /// Filters the response by a protocol ID.
    #[arg(long)]
    protocol_id: Option<String>,

    /// Filters the response by a unique ID.
    #[arg(long)]
    unique_id: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// Users list command
pub struct UsersCmd {
    pub args: UsersArgs,
}
/// Users response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The user ID.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the default project for the user.
    #[serde()]
    #[structable(optional, wide)]
    default_project_id: Option<String>,

    /// The new description of the group.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the domain.
    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    /// List of federated objects associated with a user. Each object in the
    /// list
    /// contains the `idp\_id` and `protocols`. `protocols` is a list of
    /// objects, each of which contains `protocol\_id` and `unique\_id` of
    /// the protocol and user respectively. For example:
    ///
    ///
    ///
    /// ```text
    /// "federated": [
    ///   {
    ///     "idp\_id": "efbab5a6acad4d108fec6c63d9609d83",
    ///     "protocols": [
    ///       {"protocol\_id": "mapped", "unique\_id": "test@example.com"}
    ///     ]
    ///   }
    /// ]
    ///
    /// ```
    #[serde()]
    #[structable(optional, wide)]
    federated: Option<VecResponseFederated>,

    /// The user name. Must be unique within the owning domain.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The new password for the user.
    #[serde()]
    #[structable(optional, wide)]
    password: Option<String>,

    /// The resource options for the user. Available resource options are
    /// `ignore\_change\_password\_upon\_first\_use`,
    /// `ignore\_password\_expiry`,
    /// `ignore\_lockout\_failure\_attempts`, `lock\_password`,
    /// `multi\_factor\_auth\_enabled`, and `multi\_factor\_auth\_rules`
    /// `ignore\_user\_inactivity`.
    #[serde()]
    #[structable(optional, wide)]
    options: Option<ResponseOptions>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseProtocols {
    protocol_id: String,
    unique_id: String,
}

impl fmt::Display for ResponseProtocols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!("protocol_id={}", self.protocol_id),
            format!("unique_id={}", self.unique_id),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseProtocols(Vec<ResponseProtocols>);
impl fmt::Display for VecResponseProtocols {
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
struct ResponseFederated {
    idp_id: String,
    protocols: VecResponseProtocols,
}

impl fmt::Display for ResponseFederated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!("idp_id={}", self.idp_id),
            format!("protocols={}", self.protocols),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseFederated(Vec<ResponseFederated>);
impl fmt::Display for VecResponseFederated {
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecVecString(Vec<VecString>);
impl fmt::Display for VecVecString {
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
struct ResponseOptions {
    ignore_change_password_upon_first_use: Option<bool>,
    ignore_password_expiry: Option<bool>,
    ignore_lockout_failure_attempts: Option<bool>,
    lock_password: Option<bool>,
    ignore_user_inactivity: Option<bool>,
    multi_factor_auth_rules: Option<VecVecString>,
    multi_factor_auth_enabled: Option<bool>,
}

impl fmt::Display for ResponseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ignore_change_password_upon_first_use={}",
                self.ignore_change_password_upon_first_use
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_password_expiry={}",
                self.ignore_password_expiry
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_lockout_failure_attempts={}",
                self.ignore_lockout_failure_attempts
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "lock_password={}",
                self.lock_password
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_user_inactivity={}",
                self.ignore_user_inactivity
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "multi_factor_auth_rules={}",
                self.multi_factor_auth_rules
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "multi_factor_auth_enabled={}",
                self.multi_factor_auth_enabled
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

#[async_trait]
impl Command for UsersCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Users with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.query.domain_id {
            ep_builder.domain_id(val);
        }
        if let Some(val) = &self.args.query.enabled {
            ep_builder.enabled(*val);
        }
        if let Some(val) = &self.args.query.idp_id {
            ep_builder.idp_id(val);
        }
        if let Some(val) = &self.args.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.query.password_expires_at {
            ep_builder.password_expires_at(val);
        }
        if let Some(val) = &self.args.query.protocol_id {
            ep_builder.protocol_id(val);
        }
        if let Some(val) = &self.args.query.unique_id {
            ep_builder.unique_id(val);
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
