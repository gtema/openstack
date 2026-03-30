// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//! CLI top level command and processing
use clap::Parser;

use openstack_cli_core::cli::{CliArgs, CompletionCommand, GlobalOpts, parse_config, styles};
use openstack_sdk::AsyncOpenStack;

use crate::error::OpenStackCliError;

use crate::api;
use crate::auth;
use crate::block_storage::v3 as block_storage;
use crate::catalog;
use crate::compute::v2 as compute;
use crate::config::Config;
use crate::container_infrastructure_management::v1 as container_infra;
use crate::dns::v2 as dns;
use crate::identity::v3 as identity;
#[cfg(feature = "keystone_ng")]
use crate::identity::v4 as identity_v4;
use crate::image::v2 as image;
use crate::load_balancer::v2 as load_balancer;
use crate::network::v2 as network;
use crate::object_store::v1 as object_store;
use crate::placement::v1 as placement;

/// OpenStack command line interface.
///
/// ## Configuration
///
/// As all OpenStack tools it fully supports `clouds.yaml`.
///
/// Certain aspects of the CLI itself (not the cloud connection) can be configured using
/// `$XDG_CONFIG_HOME/osc/config.yaml` file. With it it is possible, for example, to configure
/// which resource fields are returned when no other output controlling parameters has be passed.
///
/// Example:
///
/// ```yaml
/// views:
///   compute.server:
///     # Listing compute servers will only return ID, NAME and IMAGE columns unless `-o wide` or
///     `-f XXX` parameters are being passed
///     fields: [id, name, image]
///   dns.zone/recordset:
///     # DNS zone recordsets are listed in the wide mode by default.
///     wide: true
/// ```
///
/// ## Features
///
///  * `osc api` as an API wrapper allowing user to perform any direct API call specifying service
///    type, url, method and payload. This can be used for example when certain resource is not
///    currently implemented natively.
///
///  * `osc auth` with subcommands for dealing explicitly with authentication (showing current auth
///    info, renewing auth, MFA/SSO support)
///
///  * Every resource is having a service type in the command solving confusions like user groups
///    vs volume groups
///
///  * Every multi-word resource name is "-" separated (i.e. floating-ip, access-rule)
///
/// ## Output
///
///  * `osc ... -o json` as an explicit machine readable format output. It allows seeing raw
///    resource json representation as send by the API without any processing on the client side.
///
///    **Note:** the result is not the raw json response, but the raw json resource information found
///    underneath expected resource key. This mode can be used i.e. to see fields that are not
///    expected by the `osc` and allows further easy machine processing with tools like `jq`
///
///  * `osc ... -o wide` for list operations to return all known fields. By default list operation
///    will only return a subset of known generic resource fields to prevent multiline tables. This
///    mode (together with not specifying `-o` parameter at all) is considered as an output for
///    humans. Field names are not generally renamed and are names as the API returns them.
///
/// ## Shell autocompletion
///
/// `osc` supports generation of the completion file for diverse shells. This can be enabled i.e.
/// by executing
///
/// ```bash
/// echo 'source <(osc completion bash)' >>~/.bashrc
/// ```
#[derive(Parser)]
#[command(name="osc", author, version, about, long_about, styles = styles())]
#[command(propagate_version = true)]
pub struct Cli {
    /// Global cli arguments and options
    #[command(flatten)]
    pub global_opts: GlobalOpts,

    /// subcommand
    #[command(subcommand)]
    pub command: TopLevelCommands,

    /// CLI configuration
    ///
    /// This does not accept parameters at the moment and will always get config from default
    /// location.
    #[arg(hide = true, long("cli-config"), value_parser = parse_config, default_value_t = Config::new().expect("invalid config"))]
    pub config: Config,
}

impl CliArgs for Cli {
    fn global_opts(&self) -> &GlobalOpts {
        &self.global_opts
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

/// Supported Top Level commands (services).
#[allow(missing_docs)]
#[derive(Parser)]
pub enum TopLevelCommands {
    Api(api::ApiCommand),
    Auth(auth::AuthCommand),
    BlockStorage(block_storage::BlockStorageCommand),
    Catalog(catalog::CatalogCommand),
    Compute(compute::ComputeCommand),
    #[command(aliases = ["container-infrastructure-management", "container"])]
    ContainerInfrastructure(container_infra::ContainerInfrastructureCommand),
    Dns(dns::DnsCommand),
    Identity(identity::IdentityCommand),
    #[cfg(feature = "keystone_ng")]
    Identity4(identity_v4::IdentityCommand),
    Image(image::ImageCommand),
    LoadBalancer(load_balancer::LoadBalancerCommand),
    Network(network::NetworkCommand),
    ObjectStore(object_store::ObjectStoreCommand),
    Placement(placement::PlacementCommand),
    Completion(CompletionCommand),
}

impl Cli {
    /// Perform command action.
    pub async fn take_action(&self, client: &mut AsyncOpenStack) -> Result<(), OpenStackCliError> {
        match &self.command {
            TopLevelCommands::Api(args) => args.take_action(self, client).await,
            TopLevelCommands::Auth(args) => args.take_action(self, client).await,
            TopLevelCommands::BlockStorage(args) => args.take_action(self, client).await,
            TopLevelCommands::Catalog(args) => args.take_action(self, client).await,
            TopLevelCommands::Compute(args) => args.take_action(self, client).await,
            TopLevelCommands::ContainerInfrastructure(args) => args.take_action(self, client).await,
            TopLevelCommands::Dns(args) => args.take_action(self, client).await,
            TopLevelCommands::Identity(args) => args.take_action(self, client).await,
            #[cfg(feature = "keystone_ng")]
            TopLevelCommands::Identity4(args) => args.take_action(self, client).await,
            TopLevelCommands::Image(args) => args.take_action(self, client).await,
            TopLevelCommands::LoadBalancer(args) => args.take_action(self, client).await,
            TopLevelCommands::Network(args) => args.take_action(self, client).await,
            TopLevelCommands::ObjectStore(args) => args.take_action(self, client).await,
            TopLevelCommands::Placement(args) => args.take_action(self, client).await,
            TopLevelCommands::Completion(_) => unimplemented!(),
        }
    }
}
