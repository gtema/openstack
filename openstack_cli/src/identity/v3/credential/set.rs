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

//! Set Credential command
//!
//! Wraps invoking of the `v3/credentials/{credential_id}` with `PATCH` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::credential::set;
use openstack_types::identity::v3::credential::response::set::CredentialResponse;

/// Updates a credential.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/credential`
#[derive(Args)]
#[command(about = "Update credential")]
pub struct CredentialCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `credential` object.
    #[command(flatten)]
    credential: Credential,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// credential_id parameter for /v3/credentials/{credential_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Credential Body data
#[derive(Args, Clone)]
struct Credential {
    /// The credential itself, as a serialized blob.
    #[arg(help_heading = "Body parameters", long)]
    blob: Option<String>,

    /// The ID for the project.
    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    /// The credential type, such as `ec2` or `cert`. The implementation
    /// determines the list of supported types.
    #[arg(help_heading = "Body parameters", long)]
    _type: Option<String>,

    /// The ID of the user who owns the credential.
    #[arg(help_heading = "Body parameters", long)]
    user_id: Option<String>,
}

impl CredentialCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Credential");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.credential data
        let args = &self.credential;
        let mut credential_builder = set::CredentialBuilder::default();
        if let Some(val) = &args.blob {
            credential_builder.blob(val);
        }

        if let Some(val) = &args.project_id {
            credential_builder.project_id(Some(val.into()));
        }

        if let Some(val) = &args._type {
            credential_builder._type(val);
        }

        if let Some(val) = &args.user_id {
            credential_builder.user_id(val);
        }

        ep_builder.credential(credential_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<CredentialResponse>(data)?;
        Ok(())
    }
}
