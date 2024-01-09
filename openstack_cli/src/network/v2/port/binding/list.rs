//! Normal response codes: 200
//!
//! Error response codes: 401, 404
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

use openstack_sdk::api::network::v2::port::binding::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct BindingsArgs {
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
    /// host query parameter for /v2.0/ports/{port_id}/bindings API
    #[arg(long)]
    host: Option<String>,

    /// vif_type query parameter for /v2.0/ports/{port_id}/bindings API
    #[arg(long)]
    vif_type: Option<String>,

    /// vnic_type query parameter for /v2.0/ports/{port_id}/bindings API
    #[arg(long)]
    vnic_type: Option<String>,

    /// status query parameter for /v2.0/ports/{port_id}/bindings API
    #[arg(long)]
    status: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs
    /// API
    #[arg()]
    port_id: String,
}

/// Bindings list command
pub struct BindingsCmd {
    pub args: BindingsArgs,
}
/// Bindings response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The hostname of the system the agent is running on.
    #[serde()]
    #[structable(optional)]
    host: Option<String>,

    /// The type of which mechanism is used for the port.
    /// An API consumer like nova can use this to determine an appropriate way
    /// to
    /// attach a device (for example an interface of a virtual server) to the
    /// port.
    /// Available values currently defined includes
    /// `ovs`, `bridge`, `macvtap`, `hw\_veb`, `hostdev\_physical`,
    /// `vhostuser`, `distributed` and `other`.
    /// There are also special values: `unbound` and `binding\_failed`.
    /// `unbound` means the port is
    /// not bound to a networking back-end. `binding\_failed` means an error
    /// that the port failed to be bound to a networking back-end.
    #[serde()]
    #[structable(optional)]
    vif_type: Option<String>,

    /// A dictionary which contains additional information on the port.
    /// Currently the following fields are defined: `port\_filter` and
    /// `ovs\_hybrid\_plug`.
    /// `port\_filter` is a boolean indicating the networking service
    /// provides port filtering features such as security group and/or
    /// anti MAC/IP spoofing.
    /// `ovs\_hybrid\_plug` is a boolean used to inform an API consumer
    /// like nova that the hybrid plugging strategy for OVS should be used.
    #[serde()]
    #[structable(optional)]
    vif_details: Option<String>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port.
    /// The valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic`
    /// and `remote-managed`.
    /// What type of vNIC is actually available depends on deployments.
    #[serde()]
    #[structable(optional)]
    vnic_type: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to
    /// pass and receive vif port information specific to the networking back-
    /// end.
    /// The networking API does not define a specific format of this field.
    /// If the update request is null this response field will be {}.
    #[serde()]
    #[structable(optional)]
    profile: Option<HashMapStringValue>,

    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[async_trait]
impl Command for BindingsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Bindings with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = list::Request::builder();
        // Set path parameters
        ep_builder.port_id(&self.args.path.port_id);
        // Set query parameters
        if let Some(val) = &self.args.query.host {
            ep_builder.host(val);
        }
        if let Some(val) = &self.args.query.vif_type {
            ep_builder.vif_type(val);
        }
        if let Some(val) = &self.args.query.vnic_type {
            ep_builder.vnic_type(val);
        }
        if let Some(val) = &self.args.query.status {
            ep_builder.status(val);
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
