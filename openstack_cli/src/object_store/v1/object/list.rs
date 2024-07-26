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

//! Shows details for a container and lists objects, sorted by name, in the
//! container.
//! Specify query parameters in the request to filter the list and return a
//! subset of objects. Omit query parameters to return a list of objects that
//! are stored in the container, up to 10,000 names. The 10,000 maximum value
//! is configurable. To view the value for the cluster, issue a GET /info
//! request.
use clap::Args;

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use structable_derive::StructTable;

use openstack_sdk::{
    api::RestClient,
    types::{ApiVersion, ServiceType},
    AsyncOpenStack,
};

use openstack_sdk::api::object_store::v1::container::get::Request;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Shows details for a container and lists objects, sorted by name, in the
/// container.
/// Specify query parameters in the request to filter the list and return a
/// subset of objects. Omit query parameters to return a list of objects that
/// are stored in the container, up to 10,000 names. The 10,000 maximum value
/// is configurable. To view the value for the cluster, issue a GET /info
/// request.
#[derive(Args, Clone, Debug)]
pub struct ObjectsCommand {
    /// The unique (within an account) name for the container. The container
    /// name must be from 1 to 256 characters long and can start with any
    /// character and contain any pattern. Character set must be UTF-8. The
    /// container name cannot contain a slash (/) character because this
    /// character delimits the container and object name. For example, the path
    /// /v1/account/www/pages specifies the www container, not the www/pages
    /// container.
    #[arg()]
    container: String,

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

/// Objects
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Objects {
    /// The name of the container.
    #[structable(optional)]
    name: Option<String>,

    /// The content type of the object.
    #[structable(optional, wide)]
    content_type: Option<String>,

    /// The total number of bytes that are stored in Object Storage for the
    /// container.
    #[structable(optional, wide)]
    bytes: Option<u64>,

    /// The MD5 checksum value of the object content.
    #[structable(optional, wide)]
    hash: Option<String>,

    /// The date and time when the object was last modified. The date and time
    /// stamp format is ISO 8601
    #[structable(optional, wide)]
    last_modified: Option<String>,

    /// This field exists only when the object is symlink. This is the target
    /// path of the symlink object.
    #[structable(optional, wide)]
    symlink_path: Option<String>,
}

impl ObjectsCommand {
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Objects with {:?}", self);

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
            .last();
        if let Some(account) = account {
            ep_builder.account(account);
        }
        ep_builder.container(&self.container);
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

        op.output_list::<Objects>(data)?;
        Ok(())
    }
}
