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

//! Shows details for an account and lists containers, sorted by name, in the
//! account.
use clap::Args;

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;
use structable::{StructTable, StructTableOptions};

use openstack_sdk::{
    AsyncOpenStack,
    api::RestClient,
    types::{ApiVersion, ServiceType},
};

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::object_store::v1::account::get::Request;
use openstack_sdk::api::{Pagination, paged};

/// Shows details for an account and lists containers, sorted by name, in the
/// account.
#[derive(Args, Clone, Debug)]
pub struct ContainersCommand {
    /// For an integer value n, limits the number of results to n.
    #[arg(long)]
    limit: Option<i32>,

    /// For a string value, x, constrains the list to items whose names are
    /// greater than x.
    #[arg(long)]
    marker: Option<String>,

    /// For a string value, x, constrains the list to items whose names are
    /// less than x.
    #[arg(long)]
    end_marker: Option<String>,

    /// The response format. Valid values are json, xml, or plain. The default
    /// is plain. If you append the format=xml or format=json query parameter
    /// to the storage account URL, the response shows extended container
    /// information serialized in that format. If you append the format=plain
    /// query parameter, the response lists the container names separated by
    /// newlines.
    #[arg(long)]
    format: Option<String>,

    /// Only objects with this prefix will be returned. When combined with a
    /// delimiter query, this enables API users to simulate and traverse the
    /// objects in a container as if they were in a directory tree.
    #[arg(long)]
    prefix: Option<String>,

    /// The delimiter is a single character used to split object names to
    /// present a pseudo-directory hierarchy of objects. When combined with a
    /// prefix query, this enables API users to simulate and traverse the
    /// objects in a container as if they were in a directory tree.
    #[arg(long)]
    delimiter: Option<String>,

    /// By default, listings are returned sorted by name, ascending. If you
    /// include the reverse=true query parameter, the listing will be returned
    /// sorted by name, descending.
    #[arg(long, action=clap::ArgAction::Set)]
    reverse: Option<bool>,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Containers
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Containers {
    /// The number of objects in the container.
    #[structable(optional, wide)]
    count: Option<u64>,

    /// The total number of bytes that are stored in Object Storage for the
    /// account.
    #[structable(optional, wide)]
    bytes: Option<u64>,

    /// The name of the container.
    #[structable(optional)]
    name: Option<String>,

    /// Last modification date of the container
    #[structable(optional, wide)]
    last_modified: Option<String>,
}

impl ContainersCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Containers with {:?}", self);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = Request::builder();
        // Set path parameters
        let ep = client.get_service_endpoint(
            &ServiceType::ObjectStore,
            Some(ApiVersion::new(1, 0)).as_ref(),
        )?;
        let account = ep
            .url()
            .path_segments()
            .expect("Object Store endpoint must not point to a bare domain")
            .filter(|x| !x.is_empty())
            .next_back();
        if let Some(account) = account {
            ep_builder.account(account);
        }
        // Set query parameters
        if let Some(val) = &self.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.end_marker {
            ep_builder.end_marker(val);
        }
        if let Some(val) = &self.format {
            ep_builder.format(val);
        }
        if let Some(val) = &self.prefix {
            ep_builder.prefix(val);
        }
        if let Some(val) = &self.delimiter {
            ep_builder.delimiter(val);
        }
        if let Some(val) = &self.reverse {
            ep_builder.reverse(*val);
        }
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;

        op.output_list::<Containers>(data)?;
        Ok(())
    }
}
