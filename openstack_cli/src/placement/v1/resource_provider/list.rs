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

//! List ResourceProviders command
//!
//! Wraps invoking of the `resource_providers` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::placement::v1::resource_provider::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// List an optionally filtered collection of resource providers.
///
/// Normal Response Codes: 200
///
/// Error response codes: badRequest(400)
///
/// A 400 BadRequest response code will be returned if a resource class
/// specified in `resources` request parameter does not exist.
///
#[derive(Args)]
#[command(about = "List resource providers")]
pub struct ResourceProvidersCommand {
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
    /// A string representing a resource provider uuid. When supplied, it will
    /// filter the returned allocation candidates to only those resource
    /// providers that are in the same tree with the given resource provider.
    ///
    #[arg(help_heading = "Query parameters", long)]
    in_tree: Option<String>,

    /// A string representing an aggregate uuid; or the prefix in: followed by
    /// a comma-separated list of strings representing aggregate uuids. The
    /// resource providers in the allocation request in the response must
    /// directly or via the root provider be associated with the aggregate or
    /// aggregates identified by uuid:
    /// `member_of=5e08ea53-c4c6-448e-9334-ac4953de3cfa`,
    /// `member_of=in:42896e0d-205d-4fe3-bd1e-100924931787,5e08ea53-c4c6-448e-9334-ac4953de3cfa`
    /// Starting from microversion 1.24 specifying multiple member_of query
    /// string parameters is possible. Multiple member_of parameters will
    /// result in filtering providers that are directly or via root provider
    /// associated with aggregates listed in all of the member_of query string
    /// values. For example, to get the providers that are associated with
    /// aggregate A as well as associated with any of aggregates B or C, the
    /// user could issue the following query:
    /// `member_of=AGGA_UUID&member_of=in:AGGB_UUID,AGGC_UUID` Starting from
    /// microversion 1.32 specifying forbidden aggregates is supported in the
    /// member_of query string parameter. Forbidden aggregates are prefixed
    /// with a !. This negative expression can also be used in multiple
    /// member_of parameters: `member_of=AGGA_UUID&member_of=!AGGB_UUID` would
    /// translate logically to “Candidate resource providers must be in AGGA
    /// and not in AGGB.” We do NOT support ! on the values within in:, but we
    /// support !in:. Both of the following two example queries return
    /// candidate resource providers that are NOT in AGGA, AGGB, or AGGC:
    /// `member_of=!in:AGGA_UUID,AGGB_UUID,AGGC_UUID`,
    /// `member_of=!AGGA_UUID&member_of=!AGGB_UUID&member_of=!AGGC_UUID` We do
    /// not check if the same aggregate uuid is in both positive and negative
    /// expression to return 400 BadRequest. We still return 200 for such
    /// cases. For example: `member_of=AGGA_UUID&member_of=!AGGA_UUID` would
    /// return empty allocation_requests and provider_summaries, while:
    /// `member_of=in:AGGA_UUID,AGGB_UUID&member_of=!AGGA_UUID` would return
    /// resource providers that are NOT in AGGA but in AGGB.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    member_of: Option<Vec<String>>,

    /// The name of a resource provider to filter the list.
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// A comma-separated list of traits that a provider must have:
    /// `required=HW_CPU_X86_AVX,HW_CPU_X86_SSE` Allocation requests in the
    /// response will be for resource providers that have capacity for all
    /// requested resources and the set of those resource providers will
    /// collectively contain all of the required traits. These traits may be
    /// satisfied by any provider in the same non-sharing tree or associated
    /// via aggregate as far as that provider also contributes resource to the
    /// request. Starting from microversion 1.22 traits which are forbidden
    /// from any resource provider contributing resources to the request may be
    /// expressed by prefixing a trait with a `!`. Starting from microversion
    /// 1.39 the required query parameter can be repeated. The trait lists from
    /// the repeated parameters are AND-ed together. So:
    /// `required=T1,!T2&required=T3` means T1 and not T2 and T3. Also starting
    /// from microversion 1.39 the required parameter supports the syntax:
    /// `required=in:T1,T2,T3` which means T1 or T2 or T3. Mixing forbidden
    /// traits into an in: prefixed value is not supported and rejected. But
    /// mixing a normal trait list and an in: prefixed trait list in two query
    /// params within the same request is supported. So:
    /// `required=in:T3,T4&required=T1,!T2` is supported and it means T1 and
    /// not T2 and (T3 or T4).
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    required: Option<Vec<String>>,

    /// A comma-separated list of strings indicating an amount of resource of a
    /// specified class that providers in each allocation request must
    /// collectively have the capacity and availability to serve:
    /// `resources=VCPU:4,DISK_GB:64,MEMORY_MB:2048` These resources may be
    /// satisfied by any provider in the same non-sharing tree or associated
    /// via aggregate.
    ///
    #[arg(help_heading = "Query parameters", long)]
    resources: Option<String>,

    /// The uuid of a resource provider.
    ///
    #[arg(help_heading = "Query parameters", long)]
    uuid: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// ResourceProviders response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A consistent view marker that assists with the management of concurrent
    /// resource provider updates.
    ///
    #[serde()]
    #[structable()]
    generation: i32,

    /// The name of one resource provider.
    ///
    #[serde()]
    #[structable()]
    name: String,

    /// The UUID of the immediate parent of the resource provider.
    ///
    /// **New in version 1.14**
    ///
    #[serde()]
    #[structable(optional)]
    parent_provider_uuid: Option<String>,

    /// Read-only UUID of the top-most provider in this provider tree.
    ///
    /// **New in version 1.14**
    ///
    #[serde()]
    #[structable(optional)]
    root_provider_uuid: Option<String>,

    /// The uuid of a resource provider.
    ///
    #[serde()]
    #[structable()]
    uuid: String,
}

impl ResourceProvidersCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List ResourceProviders");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.uuid {
            ep_builder.uuid(val);
        }
        if let Some(val) = &self.query.resources {
            ep_builder.resources(val);
        }
        if let Some(val) = &self.query.required {
            ep_builder.required(val.iter());
        }
        if let Some(val) = &self.query.member_of {
            ep_builder.member_of(val.iter());
        }
        if let Some(val) = &self.query.in_tree {
            ep_builder.in_tree(val);
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
