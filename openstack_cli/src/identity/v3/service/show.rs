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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Show Service command
//!
//! Wraps invoking of the `v3/services/{service_id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::service::find;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Shows details for a service.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/service`
///
#[derive(Args)]
#[command(about = "Show service details")]
pub struct ServiceCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// service_id parameter for /v3/services/{service_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Service response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The service description.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// Defines whether the service and its endpoints appear in the service
    /// catalog: - `false`. The service and its endpoints do not appear in the
    /// service catalog. - `true`. The service and its endpoints appear in the
    /// service catalog. Default is `true`.
    ///
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// The UUID of the service to which the endpoint belongs.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The service name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The service type, which describes the API implemented by the service.
    /// Value is `compute`, `ec2`, `identity`, `image`, `network`, or `volume`.
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,
}

impl ServiceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Service");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
