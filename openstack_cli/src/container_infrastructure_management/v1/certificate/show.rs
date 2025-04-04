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

//! Show Certificate command
//!
//! Wraps invoking of the `v1/certificates/{certificate_id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::container_infrastructure_management::v1::certificate::get;
use serde_json::Value;
use structable_derive::StructTable;

/// Show CA certificate details that are associated with the created cluster
/// based on the given CA certificate type.
///
#[derive(Args)]
#[command(about = "Show details about the CA certificate for a cluster")]
pub struct CertificateCommand {
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
    /// certificate_id parameter for /v1/certificates/{certificate_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Certificate response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional)]
    ca_cert_type: Option<String>,

    /// The UUID of the cluster.
    ///
    #[serde()]
    #[structable(optional)]
    cluster_uuid: Option<String>,

    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    csr: Option<String>,

    /// Links to the resources in question.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    /// CA certificate for the cluster.
    ///
    #[serde()]
    #[structable(optional)]
    pem: Option<String>,

    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl CertificateCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Certificate");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
