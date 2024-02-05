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

//! Action Volume command [microversion = 3.68]
//!
//! Wraps invoking of the `v3/volumes/{id}/action` with `POST` method

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

use openstack_sdk::api::block_storage::v3::volume::os_reimage_368;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct VolumeCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    os_reimage: OsReimage,
}

/// Query parameters
#[derive(Args)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args)]
pub struct PathParameters {
    /// id parameter for /v3/volumes/{id} API
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// OsReimage Body data
#[derive(Args)]
struct OsReimage {
    #[arg(long)]
    image_id: String,

    #[arg(action=clap::ArgAction::Set, long)]
    reimage_reserved: Option<bool>,
}

/// Response data as HashMap type
#[derive(Deserialize, Serialize)]
pub struct ResponseData(HashMap<String, serde_json::Value>);

impl StructTable for ResponseData {
    fn build(&self, _options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Name".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.extend(self.0.iter().map(|(k, v)| {
            Vec::from([
                k.clone(),
                serde_json::to_string(&v).expect("Is a valid data"),
            ])
        }));
        (headers, rows)
    }
}

impl VolumeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Volume");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = os_reimage_368::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.68");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_reimage data
        let args = &self.os_reimage;
        let mut os_reimage_builder = os_reimage_368::OsReimageBuilder::default();

        os_reimage_builder.image_id(args.image_id.clone());

        if let Some(val) = &args.reimage_reserved {
            os_reimage_builder.reimage_reserved(*val);
        }

        ep_builder.os_reimage(os_reimage_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
