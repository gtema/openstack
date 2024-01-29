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

use crate::common::parse_json;
use crate::common::parse_key_val;
use clap::ValueEnum;
use openstack_sdk::api::network::v2::port::binding::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct BindingArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    binding: Binding,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs
    /// API
    #[arg()]
    port_id: String,

    /// id parameter for /v2.0/ports/{port_id}/bindings/{id} API
    #[arg()]
    id: String,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum VnicType {
    AcceleratorDirect,
    AcceleratorDirectPhysical,
    Baremetal,
    Direct,
    DirectPhysical,
    Macvtap,
    Normal,
    RemoteManaged,
    SmartNic,
    Vdpa,
    VirtioForwarder,
}

/// Binding Body data
#[derive(Args, Debug, Clone)]
struct Binding {
    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    vnic_type: Option<VnicType>,

    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    profile: Option<Vec<(String, Value)>>,
}

/// Binding set command
pub struct BindingCmd {
    pub args: BindingArgs,
}
/// Binding response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    #[serde()]
    #[structable(optional)]
    host: Option<String>,

    #[serde()]
    #[structable(optional)]
    vif_type: Option<String>,

    #[serde()]
    #[structable(optional)]
    vif_details: Option<String>,

    #[serde()]
    #[structable(optional)]
    vnic_type: Option<String>,

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
impl OSCCommand for BindingCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Binding with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.port_id(&self.args.path.port_id);
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.binding data
        let args = &self.args.binding;
        let mut binding_builder = set::BindingBuilder::default();
        if let Some(val) = &args.host {
            binding_builder.host(val.clone());
        }

        if let Some(val) = &args.vnic_type {
            let tmp = match val {
                VnicType::AcceleratorDirect => set::VnicType::AcceleratorDirect,
                VnicType::AcceleratorDirectPhysical => set::VnicType::AcceleratorDirectPhysical,
                VnicType::Baremetal => set::VnicType::Baremetal,
                VnicType::Direct => set::VnicType::Direct,
                VnicType::DirectPhysical => set::VnicType::DirectPhysical,
                VnicType::Macvtap => set::VnicType::Macvtap,
                VnicType::Normal => set::VnicType::Normal,
                VnicType::RemoteManaged => set::VnicType::RemoteManaged,
                VnicType::SmartNic => set::VnicType::SmartNic,
                VnicType::Vdpa => set::VnicType::Vdpa,
                VnicType::VirtioForwarder => set::VnicType::VirtioForwarder,
            };
            binding_builder.vnic_type(tmp);
        }

        if let Some(val) = &args.profile {
            binding_builder.profile(val.iter().cloned());
        }

        ep_builder.binding(binding_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
