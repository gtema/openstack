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

//! List Endpoints command
//!
//! Wraps invoking of the `v3/endpoints` with `GET` method

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

use openstack_sdk::api::identity::v3::endpoint::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists all available endpoints.
///
/// Relationship: `https://docs.openstack.org/api/openstack-
/// identity/3/rel/endpoints`
#[derive(Args)]
#[command(about = "List endpoints")]
pub struct EndpointsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// Filters the response by a service ID.
    #[arg(long)]
    service_id: Option<String>,

    /// Filters the response by a region ID.
    #[arg(long)]
    region: Option<String>,

    /// Filters the response by an interface.
    #[arg(long, value_parser = ["admin","internal","public"])]
    interface: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Endpoints response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Indicates whether the endpoint appears in the
    /// service catalog: - `false`. The endpoint does not appear in the
    /// service catalog. - `true`. The endpoint appears in the service
    /// catalog.
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    /// The endpoint ID.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The interface type, which describes the
    /// visibility of the endpoint. Value is: - `public`. Visible by
    /// end users on a publicly available network interface. -
    /// `internal`. Visible by end users on an unmetered internal
    /// network interface. - `admin`. Visible by administrative users
    /// on a secure network interface.
    #[serde()]
    #[structable(optional, wide)]
    interface: Option<String>,

    /// (Deprecated in v3.2) The geographic location of
    /// the service endpoint.
    #[serde()]
    #[structable(optional, wide)]
    region: Option<String>,

    /// (Since v3.2) The ID of the region that contains
    /// the service endpoint.
    #[serde()]
    #[structable(optional, wide)]
    region_id: Option<String>,

    /// The UUID of the service to which the endpoint
    /// belongs.
    #[serde()]
    #[structable(optional, wide)]
    service_id: Option<String>,

    /// The endpoint URL.
    #[serde()]
    #[structable(optional, wide)]
    url: Option<String>,
}

impl EndpointsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Endpoints");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.service_id {
            ep_builder.service_id(val.clone());
        }
        if let Some(val) = &self.query.region {
            ep_builder.region(val.clone());
        }
        if let Some(val) = &self.query.interface {
            ep_builder.interface(val.clone());
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}