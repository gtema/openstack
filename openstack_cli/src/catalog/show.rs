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

//! Catalog list command

use clap::Args;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;
use structable::StructTable;
use structable::StructTableOptions;

/// Shows current catalog information
#[derive(Args)]
pub struct ShowCommand {
    /// Service type
    #[arg(long, alias = "type")]
    service_type: String,
}

/// Catalog entry representation
#[derive(Deserialize, Serialize, StructTable)]
pub struct CatalogEndpoint {
    /// Endpoint Id
    id: String,
    /// Interface
    interface: String,
    /// Region
    region: String,
    /// URL
    url: String,
}

impl ShowCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show service endpoint catalog configuration");

        let op = OutputProcessor::from_args(parsed_args, Some("catalog"), Some("show"));
        op.validate_args(parsed_args)?;

        let data: Vec<Value> = client
            .get_token_catalog()
            .unwrap_or_default()
            .into_iter()
            .filter(|x| x.service_type == self.service_type)
            .flat_map(|x| {
                let mut eps = x.endpoints;
                eps.sort_by_key(|x| format!("{}{}", x.region, x.interface));
                eps
            })
            .map(|x| serde_json::to_value(x).unwrap())
            .collect();

        if data.is_empty() {
            return Err(openstack_sdk::catalog::CatalogError::ServiceNotConfigured(
                self.service_type.clone(),
            )
            .into());
        }

        op.output_list::<CatalogEndpoint>(data).unwrap();
        op.show_command_hint()?;
        Ok(())
    }
}
