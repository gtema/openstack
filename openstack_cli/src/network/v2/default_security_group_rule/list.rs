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

//! List DefaultSecurityGroupRules command
//!
//! Wraps invoking of the `v2.0/default-security-group-rules` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::default_security_group_rule::list;
use openstack_sdk::api::{Pagination, paged};
use openstack_types::network::v2::default_security_group_rule::response::list::DefaultSecurityGroupRuleResponse;

/// Lists a summary of all OpenStack Networking security group rules that are
/// used for every newly created Security Group.
///
/// The list provides the ID for each security group default rule.
///
/// Standard query parameters are supported on the URI. For more information,
/// see [Filtering and Column Selection](#filtering).
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. For more information, see [Fields](#fields).
///
/// Pagination query parameters are supported if Neutron configuration supports
/// it by overriding `allow_pagination=false`. For more information, see
/// [Pagination](#pagination).
///
/// Sorting query parameters are supported if Neutron configuration supports it
/// with `allow_sorting=true`. For more information, see [Sorting](#sorting).
///
/// Normal response codes: 200
///
/// Error response codes: 401
#[derive(Args)]
#[command(about = "List security group default rules")]
pub struct DefaultSecurityGroupRulesCommand {
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
    /// description query parameter for /v2.0/default-security-group-rules API
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// direction query parameter for /v2.0/default-security-group-rules API
    #[arg(help_heading = "Query parameters", long, value_parser = ["egress","ingress"])]
    direction: Option<String>,

    /// ethertype query parameter for /v2.0/default-security-group-rules API
    #[arg(help_heading = "Query parameters", long, value_parser = ["IPv4","IPv6"])]
    ethertype: Option<String>,

    /// id query parameter for /v2.0/default-security-group-rules API
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[arg(
        help_heading = "Query parameters",
        long("page-size"),
        visible_alias("limit")
    )]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// Reverse the page direction
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    /// port_range_max query parameter for /v2.0/default-security-group-rules
    /// API
    #[arg(help_heading = "Query parameters", long)]
    port_range_max: Option<i32>,

    /// port_range_min query parameter for /v2.0/default-security-group-rules
    /// API
    #[arg(help_heading = "Query parameters", long)]
    port_range_min: Option<i32>,

    /// protocol query parameter for /v2.0/default-security-group-rules API
    #[arg(help_heading = "Query parameters", long)]
    protocol: Option<String>,

    /// remote_address_group_id query parameter for
    /// /v2.0/default-security-group-rules API
    #[arg(help_heading = "Query parameters", long)]
    remote_address_group_id: Option<String>,

    /// Filter the security group rule list result by the ID of the remote
    /// group that associates with this security group rule. This field can
    /// contains uuid of the security group or special word `PARENT` which
    /// means that in the real rule created from this template, uuid of the
    /// owner Security Group will be put as `remote_group_id`.
    #[arg(help_heading = "Query parameters", long)]
    remote_group_id: Option<String>,

    /// remote_ip_prefix query parameter for /v2.0/default-security-group-rules
    /// API
    #[arg(help_heading = "Query parameters", long)]
    remote_ip_prefix: Option<String>,

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_dir: Option<Vec<String>>,

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_key: Option<Vec<String>>,

    /// used_in_default_sg query parameter for
    /// /v2.0/default-security-group-rules API
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    used_in_default_sg: Option<bool>,

    /// used_in_non_default_sg query parameter for
    /// /v2.0/default-security-group-rules API
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    used_in_non_default_sg: Option<bool>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

impl DefaultSecurityGroupRulesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List DefaultSecurityGroupRules");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("network.default_security_group_rule"),
            Some("list"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set query parameters
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.query.direction {
            ep_builder.direction(val);
        }
        if let Some(val) = &self.query.ethertype {
            ep_builder.ethertype(val);
        }
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.port_range_max {
            ep_builder.port_range_max(*val);
        }
        if let Some(val) = &self.query.port_range_min {
            ep_builder.port_range_min(*val);
        }
        if let Some(val) = &self.query.protocol {
            ep_builder.protocol(val);
        }
        if let Some(val) = &self.query.remote_address_group_id {
            ep_builder.remote_address_group_id(val);
        }
        if let Some(val) = &self.query.remote_group_id {
            ep_builder.remote_group_id(val);
        }
        if let Some(val) = &self.query.remote_ip_prefix {
            ep_builder.remote_ip_prefix(val);
        }
        if let Some(val) = &self.query.used_in_default_sg {
            ep_builder.used_in_default_sg(*val);
        }
        if let Some(val) = &self.query.used_in_non_default_sg {
            ep_builder.used_in_non_default_sg(*val);
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.page_reverse {
            ep_builder.page_reverse(*val);
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val.iter());
        }
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val.iter());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;
        op.output_list::<DefaultSecurityGroupRuleResponse>(data)?;
        Ok(())
    }
}
