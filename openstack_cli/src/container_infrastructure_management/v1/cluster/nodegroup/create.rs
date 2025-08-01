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

//! Create Nodegroup command
//!
//! Wraps invoking of the `v1/clusters/nodegroups` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use crate::common::parse_key_val;
use openstack_sdk::api::container_infrastructure_management::v1::cluster::nodegroup::create;
use openstack_sdk::api::QueryAsync;
use openstack_types::container_infrastructure_management::v1::cluster::nodegroup::response::create::NodegroupResponse;
use serde_json::Value;

/// Retrieve a list of nodegroups.
///
/// | param cluster_id: | | | --- | --- | | | the cluster id or name | | param
/// marker: | pagination marker for large data sets. | | param limit: | maximum
/// number of resources to return in a single result. | | param sort_key: |
/// column to sort results by. Default: id. | | param sort_dir: | direction to
/// sort. "asc" or "desc". Default: asc. | | param role: | list all nodegroups
/// with the specified role. |
#[derive(Args)]
pub struct NodegroupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long)]
    cluster_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    created_at: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    docker_volume_size: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    flavor_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    id: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    image_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    is_default: Option<String>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    labels: Option<Vec<(String, String)>>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    labels_added: Option<Vec<(String, String)>>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    labels_overridden: Option<Vec<(String, String)>>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    labels_skipped: Option<Vec<(String, String)>>,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=crate::common::parse_json)]
    links: Option<Vec<Value>>,

    #[arg(help_heading = "Body parameters", long)]
    max_node_count: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    merge_labels: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    min_node_count: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    node_addresses: Option<Vec<String>>,

    #[arg(help_heading = "Body parameters", long)]
    node_count: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    role: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    stack_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    status: Option<Status>,

    #[arg(help_heading = "Body parameters", long)]
    status_reason: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    updated_at: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    uuid: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    version: Option<String>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Status {
    AdoptComplete,
    CheckComplete,
    CreateComplete,
    CreateFailed,
    CreateInProgress,
    DeleteComplete,
    DeleteFailed,
    DeleteInProgress,
    RestoreComplete,
    ResumeComplete,
    ResumeFailed,
    RollbackComplete,
    RollbackFailed,
    RollbackInProgress,
    SnapshotComplete,
    UpdateComplete,
    UpdateFailed,
    UpdateInProgress,
}

impl NodegroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Nodegroup");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("container-infrastructure-management.cluster/nodegroup"),
            Some("create"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set body parameters
        // Set Request.cluster_id data
        if let Some(arg) = &self.cluster_id {
            ep_builder.cluster_id(arg);
        }

        // Set Request.created_at data
        if let Some(arg) = &self.created_at {
            ep_builder.created_at(arg);
        }

        // Set Request.docker_volume_size data
        if let Some(arg) = &self.docker_volume_size {
            ep_builder.docker_volume_size(*arg);
        }

        // Set Request.flavor_id data
        if let Some(arg) = &self.flavor_id {
            ep_builder.flavor_id(arg);
        }

        // Set Request.id data
        if let Some(arg) = &self.id {
            ep_builder.id(*arg);
        }

        // Set Request.image_id data
        if let Some(arg) = &self.image_id {
            ep_builder.image_id(arg);
        }

        // Set Request.is_default data
        if let Some(arg) = &self.is_default {
            ep_builder.is_default(arg);
        }

        // Set Request.labels data
        if let Some(arg) = &self.labels {
            ep_builder.labels(arg.iter().cloned());
        }

        // Set Request.labels_added data
        if let Some(arg) = &self.labels_added {
            ep_builder.labels_added(arg.iter().cloned());
        }

        // Set Request.labels_overridden data
        if let Some(arg) = &self.labels_overridden {
            ep_builder.labels_overridden(arg.iter().cloned());
        }

        // Set Request.labels_skipped data
        if let Some(arg) = &self.labels_skipped {
            ep_builder.labels_skipped(arg.iter().cloned());
        }

        // Set Request.links data
        if let Some(arg) = &self.links {
            let links_builder: Vec<create::Links> = arg
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Links>(v.to_owned()))
                .collect::<Vec<create::Links>>();
            ep_builder.links(links_builder);
        }

        // Set Request.max_node_count data
        if let Some(arg) = &self.max_node_count {
            ep_builder.max_node_count(*arg);
        }

        // Set Request.merge_labels data
        if let Some(arg) = &self.merge_labels {
            ep_builder.merge_labels(arg);
        }

        // Set Request.min_node_count data
        if let Some(arg) = &self.min_node_count {
            ep_builder.min_node_count(*arg);
        }

        // Set Request.name data
        if let Some(arg) = &self.name {
            ep_builder.name(arg);
        }

        // Set Request.node_addresses data
        if let Some(arg) = &self.node_addresses {
            ep_builder.node_addresses(arg.iter().map(Into::into).collect::<Vec<_>>());
        }

        // Set Request.node_count data
        if let Some(arg) = &self.node_count {
            ep_builder.node_count(*arg);
        }

        // Set Request.project_id data
        if let Some(arg) = &self.project_id {
            ep_builder.project_id(arg);
        }

        // Set Request.role data
        if let Some(arg) = &self.role {
            ep_builder.role(arg);
        }

        // Set Request.stack_id data
        if let Some(arg) = &self.stack_id {
            ep_builder.stack_id(arg);
        }

        // Set Request.status data
        if let Some(arg) = &self.status {
            let tmp = match arg {
                Status::AdoptComplete => create::Status::AdoptComplete,
                Status::CheckComplete => create::Status::CheckComplete,
                Status::CreateComplete => create::Status::CreateComplete,
                Status::CreateFailed => create::Status::CreateFailed,
                Status::CreateInProgress => create::Status::CreateInProgress,
                Status::DeleteComplete => create::Status::DeleteComplete,
                Status::DeleteFailed => create::Status::DeleteFailed,
                Status::DeleteInProgress => create::Status::DeleteInProgress,
                Status::RestoreComplete => create::Status::RestoreComplete,
                Status::ResumeComplete => create::Status::ResumeComplete,
                Status::ResumeFailed => create::Status::ResumeFailed,
                Status::RollbackComplete => create::Status::RollbackComplete,
                Status::RollbackFailed => create::Status::RollbackFailed,
                Status::RollbackInProgress => create::Status::RollbackInProgress,
                Status::SnapshotComplete => create::Status::SnapshotComplete,
                Status::UpdateComplete => create::Status::UpdateComplete,
                Status::UpdateFailed => create::Status::UpdateFailed,
                Status::UpdateInProgress => create::Status::UpdateInProgress,
            };
            ep_builder.status(tmp);
        }

        // Set Request.status_reason data
        if let Some(arg) = &self.status_reason {
            ep_builder.status_reason(arg);
        }

        // Set Request.updated_at data
        if let Some(arg) = &self.updated_at {
            ep_builder.updated_at(arg);
        }

        // Set Request.uuid data
        if let Some(arg) = &self.uuid {
            ep_builder.uuid(arg);
        }

        // Set Request.version data
        if let Some(arg) = &self.version {
            ep_builder.version(arg);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<NodegroupResponse>(data)?;
        Ok(())
    }
}
