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

//! List Images command
//!
//! Wraps invoking of the `v2/images` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::image::v2::image::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use structable_derive::StructTable;

/// Lists public virtual machine (VM) images. *(Since Image API v2.0)*
///
/// **Pagination**
///
/// Returns a subset of the larger collection of images and a link that you can
/// use to get the next set of images. You should always check for the presence
/// of a `next` link and use it as the URI in a subsequent HTTP GET request.
/// You should follow this pattern until a `next` link is no longer provided.
///
/// The `next` link preserves any query parameters that you send in your
/// initial request. You can use the `first` link to jump back to the first
/// page of the collection. If you prefer to paginate through images manually,
/// use the `limit` and `marker` parameters.
///
/// **Query Filters**
///
/// The list operation accepts query parameters to filter the response.
///
/// A client can provide direct comparison filters by using most image
/// attributes, such as `name=Ubuntu`, `visibility=public`, and so on.
///
/// To filter using image tags, use the filter `tag` (note the singular). To
/// filter on multiple tags, include each tag separately in the query. For
/// example, to find images with the tag **ready**, include `tag=ready` in your
/// query string. To find images tagged with **ready** and **approved**,
/// include `tag=ready&tag=approved` in your query string. (Note that only
/// images containing *both* tags will be included in the response.)
///
/// A client cannot use any `link` in the json-schema, such as self, file, or
/// schema, to filter the response.
///
/// You can list VM images that have a status of `active`, `queued`, or
/// `saving`.
///
/// **The** `in` **Operator**
///
/// As a convenience, you may specify several values for any of the following
/// fields by using the `in` operator:
///
/// For most of these, usage is straight forward. For example, to list images
/// in queued or saving status, use:
///
/// `GET /v2/images?status=in:saving,queued`
///
/// To find images in a particular list of image IDs, use:
///
/// `GET /v2/images?id=in:3afb79c1-131a-4c38-a87c-bc4b801d14e6,2e011209-660f-44b5-baf2-2eb4babae53d`
///
/// Using the `in` operator with the `name` property of images can be a bit
/// trickier, depending upon how creatively you have named your images. The
/// general rule is that if an image name contains a comma (`,`), you must
/// enclose the entire name in quotation marks (`"`). As usual, you must URL
/// encode any characters that require it.
///
/// For example, to find images named `glass, darkly` or `share me`, you would
/// use the following filter specification:
///
/// `GET v2/images?name=in:"glass,%20darkly",share%20me`
///
/// As with regular filtering by name, you must specify the complete name you
/// are looking for. Thus, for example, the query string `name=in:glass,share`
/// will only match images with the exact name `glass` or the exact name
/// `share`. It will not find an image named `glass, darkly` or an image named
/// `share me`.
///
/// **Size Comparison Filters**
///
/// You can use the `size_min` and `size_max` query parameters to filter images
/// that are greater than or less than the image size. The size, in bytes, is
/// the size of an image on disk.
///
/// For example, to filter the container to include only images that are from 1
/// to 4 MB, set the `size_min` query parameter to `1048576` and the `size_max`
/// query parameter to `4194304`.
///
/// **Time Comparison Filters**
///
/// You can use a *comparison operator* along with the `created_at` or
/// `updated_at` fields to filter your results. Specify the operator first, a
/// colon (`:`) as a separator, and then the time in
/// [ISO 8601 Format](https://en.wikipedia.org/wiki/ISO_8601). Available
/// comparison operators are:
///
/// For example:
///
/// **Sorting**
///
/// You can use query parameters to sort the results of this operation.
///
/// To sort the response, use the `sort_key` and `sort_dir` query parameters:
///
/// Alternatively, specify the `sort` query parameter:
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 403
///
#[derive(Args)]
#[command(about = "List images")]
pub struct ImagesCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// Specify a comparison filter based on the date and time when the
    /// resource was created.
    ///
    #[arg(help_heading = "Query parameters", long)]
    created_at: Option<String>,

    /// id filter parameter
    ///
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    ///
    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    ///
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// Filters the response by a member status. A valid value is accepted,
    /// pending, rejected, or all. Default is accepted.
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["accepted","all","pending","rejected"])]
    member_status: Option<String>,

    /// Filters the response by a name, as a string. A valid value is the name
    /// of an image.
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// When true, filters the response to display only "hidden" images. By
    /// default, "hidden" images are not included in the image-list response.
    /// (Since Image API v2.7)
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    os_hidden: Option<bool>,

    /// Filters the response by a project (also called a “tenant”) ID. Shows
    /// only images that are shared with you by the specified owner.
    ///
    #[arg(help_heading = "Query parameters", long)]
    owner: Option<String>,

    /// Filters the response by the ‘protected’ image property. A valid value
    /// is one of ‘true’, ‘false’ (must be all lowercase). Any other value will
    /// result in a 400 response.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    protected: Option<bool>,

    /// Filters the response by a maximum image size, in bytes.
    ///
    #[arg(help_heading = "Query parameters", long)]
    size_max: Option<String>,

    /// Filters the response by a minimum image size, in bytes.
    ///
    #[arg(help_heading = "Query parameters", long)]
    size_min: Option<String>,

    /// Sorts the response by one or more attribute and sort direction
    /// combinations. You can also set multiple sort keys and directions.
    /// Default direction is desc. Use the comma (,) character to separate
    /// multiple values. For example: `sort=name:asc,status:desc`
    ///
    #[arg(help_heading = "Query parameters", long)]
    sort: Option<String>,

    /// Sorts the response by a set of one or more sort direction and attribute
    /// (sort_key) combinations. A valid value for the sort direction is asc
    /// (ascending) or desc (descending). If you omit the sort direction in a
    /// set, the default is desc.
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["asc","desc"])]
    sort_dir: Option<String>,

    /// Sorts the response by an attribute, such as name, id, or updated_at.
    /// Default is created_at. The API uses the natural sorting direction of
    /// the sort_key image attribute.
    ///
    #[arg(help_heading = "Query parameters", long)]
    sort_key: Option<String>,

    /// Filters the response by an image status.
    ///
    #[arg(help_heading = "Query parameters", long)]
    status: Option<String>,

    /// Filters the response by the specified tag value. May be repeated, but
    /// keep in mind that you're making a conjunctive query, so only images
    /// containing all the tags specified will appear in the response.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tag: Option<Vec<String>>,

    /// Specify a comparison filter based on the date and time when the
    /// resource was most recently modified.
    ///
    #[arg(help_heading = "Query parameters", long)]
    updated_at: Option<String>,

    /// Filters the response by an image visibility value. A valid value is
    /// public, private, community, shared, or all. (Note that if you filter on
    /// shared, the images included in the response will only be those where
    /// your member status is accepted unless you explicitly include a
    /// member_status filter in the request.) If you omit this parameter, the
    /// response shows public, private, and those shared images with a member
    /// status of accepted.
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["all","community","private","public","shared"])]
    visibility: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Images response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// md5 hash of image contents.
    ///
    #[serde()]
    #[structable(optional, wide)]
    checksum: Option<String>,

    /// Format of the container
    ///
    #[serde()]
    #[structable(optional, wide)]
    container_format: Option<String>,

    /// Date and time of image registration
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// URL to access the image file kept in external store
    ///
    #[serde()]
    #[structable(optional, wide)]
    direct_url: Option<String>,

    /// Format of the disk
    ///
    #[serde()]
    #[structable(optional, wide)]
    disk_format: Option<String>,

    /// An image file url
    ///
    #[serde()]
    #[structable(optional, wide)]
    file: Option<String>,

    /// An identifier for the image
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// A set of URLs to access the image file kept in external store
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    locations: Option<Value>,

    /// Amount of disk space (in GB) required to boot image.
    ///
    #[serde()]
    #[structable(optional, wide)]
    min_disk: Option<i32>,

    /// Amount of ram (in MB) required to boot image.
    ///
    #[serde()]
    #[structable(optional, wide)]
    min_ram: Option<i32>,

    /// Descriptive name for the image
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// Algorithm to calculate the os_hash_value
    ///
    #[serde()]
    #[structable(optional, wide)]
    os_hash_algo: Option<String>,

    /// Hexdigest of the image contents using the algorithm specified by the
    /// os_hash_algo
    ///
    #[serde()]
    #[structable(optional, wide)]
    os_hash_value: Option<String>,

    /// If true, image will not appear in default image list response.
    ///
    #[serde()]
    #[structable(optional, wide)]
    os_hidden: Option<bool>,

    /// Owner of the image
    ///
    #[serde()]
    #[structable(optional, wide)]
    owner: Option<String>,

    /// If true, image will not be deletable.
    ///
    #[serde()]
    #[structable(optional, wide)]
    protected: Option<bool>,

    /// An image schema url
    ///
    #[serde()]
    #[structable(optional, wide)]
    schema: Option<String>,

    /// An image self url
    ///
    #[serde(rename = "self")]
    #[structable(optional, title = "self", wide)]
    _self: Option<String>,

    /// Size of image file in bytes
    ///
    #[serde()]
    #[structable(optional, wide)]
    size: Option<i64>,

    /// Status of the image
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// Store in which image data resides. Only present when the operator has
    /// enabled multiple stores. May be a comma-separated list of store
    /// identifiers.
    ///
    #[serde()]
    #[structable(optional, wide)]
    stores: Option<String>,

    /// List of strings related to the image
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    tags: Option<Value>,

    /// Date and time of the last image modification
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// Virtual size of image in bytes
    ///
    #[serde()]
    #[structable(optional, wide)]
    virtual_size: Option<i64>,

    /// Scope of image accessibility
    ///
    #[serde()]
    #[structable(optional, wide)]
    visibility: Option<String>,
}

impl ImagesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Images");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.owner {
            ep_builder.owner(val);
        }
        if let Some(val) = &self.query.protected {
            ep_builder.protected(*val);
        }
        if let Some(val) = &self.query.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.query.tag {
            ep_builder.tag(val.iter());
        }
        if let Some(val) = &self.query.visibility {
            ep_builder.visibility(val);
        }
        if let Some(val) = &self.query.os_hidden {
            ep_builder.os_hidden(*val);
        }
        if let Some(val) = &self.query.member_status {
            ep_builder.member_status(val);
        }
        if let Some(val) = &self.query.size_max {
            ep_builder.size_max(val);
        }
        if let Some(val) = &self.query.size_min {
            ep_builder.size_min(val);
        }
        if let Some(val) = &self.query.created_at {
            ep_builder.created_at(val);
        }
        if let Some(val) = &self.query.updated_at {
            ep_builder.updated_at(val);
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val);
        }
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val);
        }
        if let Some(val) = &self.query.sort {
            ep_builder.sort(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
