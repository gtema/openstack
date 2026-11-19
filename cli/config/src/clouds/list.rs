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

//! List clouds of clouds.yaml/secure.yaml.

use clap::Args;
use eyre::eyre;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

use openstack_cli_core::cli::CliArgs;
use openstack_cli_core::error::OpenStackCliError;
use openstack_cli_core::output::OutputProcessor;
use openstack_sdk_core::config::ConfigFile;

/// List the clouds configured in clouds.yaml/secure.yaml.
#[derive(Debug, Args)]
pub struct ListCommand {}

/// A cloud as shown by the list.
#[derive(Clone, Debug, Default, Deserialize, Serialize, StructTable)]
pub struct CloudView {
    /// Cloud name.
    #[structable(title = "Name")]
    pub name: String,
    /// Authentication URL.
    #[structable(title = "Auth URL", optional)]
    pub auth_url: Option<String>,
    /// Region name.
    #[structable(title = "Region", optional)]
    pub region_name: Option<String>,
    /// Authentication type.
    #[structable(title = "Auth type", optional)]
    pub auth_type: Option<String>,
    /// User name.
    #[structable(title = "User", optional)]
    pub username: Option<String>,
    /// Project name.
    #[structable(title = "Project", optional)]
    pub project_name: Option<String>,
}

impl ListCommand {
    /// Perform command action
    pub fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        let connection = &parsed_args.global_opts().connection;
        let cfg = ConfigFile::new_with_user_specified_configs(
            connection.os_client_config_file.as_deref(),
            connection.os_client_secure_file.as_deref(),
        )?;
        let mut names = cfg.get_available_clouds();
        names.sort();

        let mut data = Vec::new();
        for name in names {
            let Some(cloud) = cfg.get_cloud_config(&name)? else {
                continue;
            };
            let auth = cloud.auth.as_ref();
            data.push(
                serde_json::to_value(CloudView {
                    name,
                    auth_url: auth.and_then(|a| a.auth_url.clone()),
                    region_name: cloud.region_name.clone(),
                    auth_type: cloud.auth_type.clone(),
                    username: auth.and_then(|a| a.username.clone()),
                    project_name: auth.and_then(|a| a.project_name.clone()),
                })
                .map_err(|err| eyre!("{err}"))?,
            );
        }

        let op = OutputProcessor::from_args(parsed_args, Some("config"), Some("clouds list"));
        op.output_list::<CloudView>(data)?;
        op.show_command_hint()?;
        Ok(())
    }
}
