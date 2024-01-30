//! Shows information for a floating IP port forwarding.
//!
//! Use the `fields` query parameter to control which fields are returned in
//! the response body.
//! For information, see [Filtering and Column Selection](https://wiki.openstac
//! k.org/wiki/Neutron/APIv2-specification#Filtering_and_Column_Selection).
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 404
//!
use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::floatingip::port_forwarding::find;
use openstack_sdk::api::network::v2::floatingip::port_forwarding::get;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct PortForwardingArgs {
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
    /// floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id}
    /// API
    #[arg()]
    floatingip_id: String,

    /// id parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API
    #[arg()]
    id: String,
}

/// PortForwarding show command
pub struct PortForwardingCmd {
    pub args: PortForwardingArgs,
}
/// PortForwarding response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the floating IP port forwarding.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP
    /// address.
    #[serde()]
    #[structable(optional)]
    external_port: Option<f32>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde()]
    #[structable(optional)]
    internal_port: Option<f32>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP
    /// port forwarding.
    #[serde()]
    #[structable(optional)]
    internal_ip_address: Option<String>,

    /// The IP protocol used in the floating IP port forwarding.
    #[serde()]
    #[structable(optional)]
    protocol: Option<String>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    #[serde()]
    #[structable(optional)]
    internal_port_id: Option<String>,

    /// A text describing the rule, which helps users to
    /// manage/find easily theirs rules.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP
    /// address.
    #[serde()]
    #[structable(optional)]
    external_port_range: Option<f32>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde()]
    #[structable(optional)]
    internal_port_range: Option<f32>,
}

#[async_trait]
impl OSCCommand for PortForwardingCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show PortForwarding with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.floatingip_id(&self.args.path.floatingip_id);
        find_builder.id(&self.args.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
