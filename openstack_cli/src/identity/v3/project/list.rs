//! Lists projects.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/projects`
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

use openstack_sdk::api::identity::v3::project::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use std::collections::BTreeMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ProjectsArgs {
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

    /// If this is specified as true, then only projects acting as a domain are
    /// included. Otherwise, only projects that are not acting as a domain are
    /// included.
    #[arg(long)]
    is_domain: Option<bool>,

    /// Filters the response by a resource name.
    #[arg(long)]
    name: Option<String>,

    /// Filters the response by a parent ID.
    #[arg(long)]
    parent_id: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// Projects list command
pub struct ProjectsCmd {
    pub args: ProjectsArgs,
}
/// Projects response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID for the project.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The description of the project.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the domain for the project.
    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional, wide)]
    is_domain: Option<bool>,

    /// The ID of the parent for the project.
    ///
    ///
    /// **New in version 3.4**
    #[serde()]
    #[structable(optional, wide)]
    parent_id: Option<String>,

    /// The name of the project.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A list of simple strings assigned to a project.
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    #[serde()]
    #[structable(optional, wide)]
    options: Option<ResponseOptions>,
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
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseOptions {
    immutable: Option<bool>,
}

impl fmt::Display for ResponseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "immutable={}",
            self.immutable
                .clone()
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}

#[async_trait]
impl Command for ProjectsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Projects with {:?}", self.args);

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
        if let Some(val) = &self.args.query.is_domain {
            ep_builder.is_domain(*val);
        }
        if let Some(val) = &self.args.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.query.parent_id {
            ep_builder.parent_id(val);
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
