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

//! Action Server command [microversion = 2.91]
//!
//! Wraps invoking of the `v2.1/servers/{id}/action` with `POST` method

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

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::server::unshelve_291;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Command without description in OpenAPI
#[derive(Args)]
#[command(about = "Unshelve (Restore) Shelved Server (unshelve Action) (microversion = 2.91)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    unshelve: Option<Unshelve>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Unshelve Body data
#[derive(Args)]
struct Unshelve {
    #[arg(long)]
    availability_zone: Option<String>,

    #[arg(long)]
    host: Option<String>,
}

/// Server response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = unshelve_291::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.91");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.unshelve data
        let args = &self.unshelve;

        if let Some(lunshelve) = &args {
            let mut unshelve_builder = unshelve_291::UnshelveBuilder::default();
            if let Some(val) = &lunshelve.availability_zone {
                unshelve_builder.availability_zone(Some(val.into()));
            }
            if let Some(val) = &lunshelve.host {
                unshelve_builder.host(val.clone());
            }
            ep_builder.unshelve(unshelve_builder.build().expect("A valid object"));
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
