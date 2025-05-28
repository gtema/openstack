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

use clap::Parser;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::fmt;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;
use structable::StructTable;
use structable::StructTableOptions;

/// Shows current catalog information
#[derive(Parser)]
pub struct ListCommand {}

/// Catalog entries
#[derive(Deserialize, Serialize)]
pub struct VecCatalogEndpoints(pub Vec<CatalogEndpoint>);

/// Catalog
#[derive(Deserialize, Serialize, StructTable)]
pub struct Catalog {
    /// Service type
    #[structable(title = "service_type")]
    #[serde(rename = "type")]
    service_type: String,

    /// Service name
    #[structable(title = "service_name")]
    name: String,

    /// Service endpoints
    endpoints: VecCatalogEndpoints,
}

/// Catalog entry representation
#[derive(Deserialize, Serialize, StructTable)]
pub struct CatalogEndpoint {
    /// id
    id: String,
    /// Interface
    interface: String,
    ///Region
    region: String,
    /// URL
    url: String,
}

impl fmt::Display for CatalogEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "interface: {}, region: {}, url: {}",
            self.interface, self.region, self.url
        )
    }
}

impl fmt::Display for VecCatalogEndpoints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl ListCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Catalog");

        let op = OutputProcessor::from_args(parsed_args, Some("catalog"), Some("list"));
        op.validate_args(parsed_args)?;

        let data: Vec<Value> = client
            .get_token_catalog()
            .unwrap_or_default()
            .into_iter()
            .map(|x| serde_json::to_value(x).unwrap())
            .collect();

        op.output_list::<Catalog>(data).unwrap();
        op.show_command_hint()?;
        Ok(())
    }
}
