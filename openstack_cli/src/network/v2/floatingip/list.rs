//! Lists floating IPs visible to the user.
//!
//! Default policy settings return only the floating IPs owned by the user’s
//! project, unless the user has admin role.
//!
//! This example request lists floating IPs in JSON format:
//!
//! Use the `fields` query parameter to control which fields are
//! returned in the response body. Additionally, you can filter results
//! by using query string parameters. For information, see [Filtering
//! and Column Selection](https://wiki.openstack.org/wiki/Neutron/APIv2-
//! specification#Filtering_and_Column_Selection).
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
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

use openstack_sdk::api::network::v2::floatingip::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct FloatingipsArgs {
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
    /// floating_ip_address query parameter for /v2.0/floatingips API
    #[arg(long)]
    floating_ip_address: Option<String>,

    /// floating_network_id query parameter for /v2.0/floatingips API
    #[arg(long)]
    floating_network_id: Option<String>,

    /// router_id query parameter for /v2.0/floatingips API
    #[arg(long)]
    router_id: Option<String>,

    /// port_id query parameter for /v2.0/floatingips API
    #[arg(long)]
    port_id: Option<String>,

    /// fixed_ip_address query parameter for /v2.0/floatingips API
    #[arg(long)]
    fixed_ip_address: Option<String>,

    /// tenant_id query parameter for /v2.0/floatingips API
    #[arg(long)]
    tenant_id: Option<String>,

    /// status query parameter for /v2.0/floatingips API
    #[arg(long)]
    status: Option<String>,

    /// revision_number query parameter for /v2.0/floatingips API
    #[arg(long)]
    revision_number: Option<String>,

    /// tags query parameter for /v2.0/floatingips API
    #[arg(long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/floatingips API
    #[arg(long)]
    tags_any: Option<Vec<String>>,

    /// not-tags query parameter for /v2.0/floatingips API
    #[arg(long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/floatingips API
    #[arg(long)]
    not_tags_any: Option<Vec<String>>,

    /// description query parameter for /v2.0/floatingips API
    #[arg(long)]
    description: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// Floatingips list command
pub struct FloatingipsCmd {
    pub args: FloatingipsArgs,
}
/// Floatingips response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the floating IP address.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The floating IP address.
    #[serde()]
    #[structable(optional)]
    floating_ip_address: Option<String>,

    /// The ID of the network associated with the
    /// floating IP.
    #[serde()]
    #[structable(optional, wide)]
    floating_network_id: Option<String>,

    /// The ID of the router for the floating IP.
    #[serde()]
    #[structable(optional, wide)]
    router_id: Option<String>,

    /// The ID of a port associated with the floating IP.
    #[serde()]
    #[structable(optional, wide)]
    port_id: Option<String>,

    /// The fixed IP address that is associated with the
    /// floating IP address.
    #[serde()]
    #[structable(optional, wide)]
    fixed_ip_address: Option<String>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The status of the floating IP. Values are
    /// `ACTIVE`, `DOWN` and `ERROR`.
    #[serde()]
    #[structable(optional, wide)]
    status: Option<String>,

    /// The ID of the QoS policy associated with the floating IP.
    #[serde()]
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

    /// The revision number of the resource.
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    /// The list of tags on the resource.
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// A valid DNS name.
    #[serde()]
    #[structable(optional, wide)]
    dns_name: Option<String>,

    /// A valid DNS domain.
    #[serde()]
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// The information of the port that this floating IP associates with.
    /// In particular, if the floating IP is associated with a port, this field
    /// contains some attributes of the associated port, including `name`,
    /// `network\_id`, `mac\_address`, `admin\_state\_up`, `status`,
    /// `device\_id` and `device\_owner`. If the floating IP is not associated
    /// with a port, this field is `null`.
    #[serde()]
    #[structable(optional, wide)]
    port_details: Option<String>,

    /// The associated port forwarding resources for the floating IP. If the
    /// floating IP has multiple port forwarding resources, this field has
    /// multiple entries. Each entry consists of network IP protocol
    /// (`protocol`), the fixed IP address of internal neutron port
    /// (`internal\_ip\_address`), the TCP or UDP port or port range used by
    /// internal neutron port (`internal\_port`) or (`internal\_port\_range`)
    /// and the TCP or UDP port or port range used by floating IP
    /// (`external\_port`) or (`external\_port\_range`).
    #[serde()]
    #[structable(optional, wide)]
    port_forwardings: Option<String>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,
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

#[async_trait]
impl Command for FloatingipsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Floatingips with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set query parameters
        if let Some(val) = &self.args.query.floating_ip_address {
            ep_builder.floating_ip_address(val);
        }
        if let Some(val) = &self.args.query.floating_network_id {
            ep_builder.floating_network_id(val);
        }
        if let Some(val) = &self.args.query.router_id {
            ep_builder.router_id(val);
        }
        if let Some(val) = &self.args.query.port_id {
            ep_builder.port_id(val);
        }
        if let Some(val) = &self.args.query.fixed_ip_address {
            ep_builder.fixed_ip_address(val);
        }
        if let Some(val) = &self.args.query.tenant_id {
            ep_builder.tenant_id(val);
        }
        if let Some(val) = &self.args.query.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.args.query.revision_number {
            ep_builder.revision_number(val);
        }
        if let Some(val) = &self.args.query.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.args.query.tags_any {
            ep_builder.tags_any(val.iter());
        }
        if let Some(val) = &self.args.query.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.args.query.not_tags_any {
            ep_builder.not_tags_any(val.iter());
        }
        if let Some(val) = &self.args.query.description {
            ep_builder.description(val);
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
