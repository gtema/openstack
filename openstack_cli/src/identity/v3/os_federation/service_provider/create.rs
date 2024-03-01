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

//! Create ServiceProvider command
//!
//! Wraps invoking of the `v3/OS-FEDERATION/service_providers/{sp_id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::identity::v3::os_federation::service_provider::create;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Create a service provider.
///
/// PUT /OS-FEDERATION/service_providers/{sp_id}
///
#[derive(Args)]
pub struct ServiceProviderCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    service_provider: ServiceProvider,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// sp_id parameter for /v3/OS-FEDERATION/service_providers/{sp_id} API
    ///
    #[arg(id = "path_param_sp_id", value_name = "SP_ID")]
    sp_id: String,
}
/// ServiceProvider Body data
#[derive(Args)]
struct ServiceProvider {
    #[arg(long)]
    auth_url: String,

    #[arg(long)]
    sp_url: String,

    #[arg(long)]
    description: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    enabled: Option<bool>,

    #[arg(long)]
    relay_state_prefix: Option<String>,
}

/// ServiceProvider response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The URL to authenticate against
    ///
    #[serde()]
    #[structable()]
    auth_url: String,

    /// The description of the Service Provider
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The Service Provider unique ID
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Whether the Service Provider is enabled or not
    ///
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// The prefix of the RelayState SAML attribute
    ///
    #[serde()]
    #[structable(optional)]
    relay_state_prefix: Option<String>,

    /// The Service Provider’s URL
    ///
    #[serde()]
    #[structable()]
    sp_url: String,
}

impl ServiceProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ServiceProvider");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.sp_id(&self.path.sp_id);
        // Set query parameters
        // Set body parameters
        // Set Request.service_provider data
        let args = &self.service_provider;
        let mut service_provider_builder = create::ServiceProviderBuilder::default();

        service_provider_builder.auth_url(&args.auth_url);

        service_provider_builder.sp_url(&args.sp_url);

        if let Some(val) = &args.description {
            service_provider_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.enabled {
            service_provider_builder.enabled(*val);
        }

        if let Some(val) = &args.relay_state_prefix {
            service_provider_builder.relay_state_prefix(Some(val.into()));
        }

        ep_builder.service_provider(service_provider_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
