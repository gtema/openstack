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

use openstack_sdk::api::compute::v2::flavor::find;
use openstack_sdk::api::compute::v2::flavor::remove_tenant_access;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct FlavorArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    remove_tenant_access: RemoveTenantAccess,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.1/flavors/{id}/action API
    #[arg()]
    id: String,
}
/// RemoveTenantAccess Body data
#[derive(Args, Debug, Clone)]
struct RemoveTenantAccess {
    /// The UUID of the tenant in a multi-tenancy cloud.
    #[arg(long)]
    tenant: String,
}

/// Flavor action command
pub struct FlavorCmd {
    pub args: FlavorArgs,
}
/// Flavor response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// A list of objects, each with the keys `flavor\_id` and `tenant\_id`.
    #[serde()]
    #[structable()]
    flavor_access: VecResponseFlavorAccess,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseFlavorAccess {
    flavor_id: Option<String>,
    tenant_id: Option<String>,
}

impl fmt::Display for ResponseFlavorAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "flavor_id={}",
                self.flavor_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "tenant_id={}",
                self.tenant_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseFlavorAccess(Vec<ResponseFlavorAccess>);
impl fmt::Display for VecResponseFlavorAccess {
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
impl OSCCommand for FlavorCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Flavor with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = remove_tenant_access::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.remove_tenant_access data
        let args = &self.args.remove_tenant_access;
        let mut remove_tenant_access_builder =
            remove_tenant_access::RemoveTenantAccessBuilder::default();

        remove_tenant_access_builder.tenant(args.tenant.clone());

        ep_builder.remove_tenant_access(remove_tenant_access_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
