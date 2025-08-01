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

//! Set QuotaClassSet command [microversion = 2.50]
//!
//! Wraps invoking of the `v2.1/os-quota-class-sets/{id}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::quota_class_set::set_250;
use openstack_types::compute::v2::quota_class_set::response::set::QuotaClassSetResponse;

/// Update the quotas for the Quota Class.
///
/// If the requested Quota Class is not found in the DB, then the API will
/// create the one. Only ‘default’ quota class is valid and used to set the
/// default quotas, all other quota class would not be used anywhere.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403)
#[derive(Args)]
#[command(about = "Create or Update Quotas for Quota Class (microversion = 2.50)")]
pub struct QuotaClassSetCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `quota_class_set` object.
    #[command(flatten)]
    quota_class_set: QuotaClassSet,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/os-quota-class-sets/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// QuotaClassSet Body data
#[derive(Args, Clone)]
struct QuotaClassSet {
    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    cores: Option<i32>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    injected_file_content_bytes: Option<i32>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    injected_file_path_bytes: Option<i32>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    injected_files: Option<i32>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    instances: Option<i32>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    key_pairs: Option<i32>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    metadata_items: Option<i32>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    ram: Option<i32>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    server_group_members: Option<i32>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    #[arg(help_heading = "Body parameters", long)]
    server_groups: Option<i32>,
}

impl QuotaClassSetCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set QuotaClassSet");

        let op =
            OutputProcessor::from_args(parsed_args, Some("compute.quota_class_set"), Some("set"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_250::Request::builder();
        ep_builder.header(
            http::header::HeaderName::from_static("openstack-api-version"),
            http::header::HeaderValue::from_static("compute 2.50"),
        );

        ep_builder.id(&self.path.id);

        // Set body parameters
        // Set Request.quota_class_set data
        let args = &self.quota_class_set;
        let mut quota_class_set_builder = set_250::QuotaClassSetBuilder::default();
        if let Some(val) = &args.cores {
            quota_class_set_builder.cores(*val);
        }

        if let Some(val) = &args.injected_file_content_bytes {
            quota_class_set_builder.injected_file_content_bytes(*val);
        }

        if let Some(val) = &args.injected_file_path_bytes {
            quota_class_set_builder.injected_file_path_bytes(*val);
        }

        if let Some(val) = &args.injected_files {
            quota_class_set_builder.injected_files(*val);
        }

        if let Some(val) = &args.instances {
            quota_class_set_builder.instances(*val);
        }

        if let Some(val) = &args.key_pairs {
            quota_class_set_builder.key_pairs(*val);
        }

        if let Some(val) = &args.metadata_items {
            quota_class_set_builder.metadata_items(*val);
        }

        if let Some(val) = &args.ram {
            quota_class_set_builder.ram(*val);
        }

        if let Some(val) = &args.server_group_members {
            quota_class_set_builder.server_group_members(*val);
        }

        if let Some(val) = &args.server_groups {
            quota_class_set_builder.server_groups(*val);
        }

        ep_builder.quota_class_set(quota_class_set_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<QuotaClassSetResponse>(data)?;
        Ok(())
    }
}
