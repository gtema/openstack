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

//! Create L7Policy command
//!
//! Wraps invoking of the `v2/lbaas/l7policies` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use crate::common::parse_json;
use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::load_balancer::v2::l7policy::create;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a L7 policy.
///
/// This operation provisions a new L7 policy by using the configuration that
/// you define in the request object. After the API validates the request and
/// starts the provisioning process, the API returns a response object that
/// contains a unique ID and the status of provisioning the L7 policy.
///
/// In the response, the L7 policy [provisioning status](#prov-status) is
/// `ACTIVE`, `PENDING_CREATE`, or `ERROR`.
///
/// If the status is `PENDING_CREATE`, issue GET
/// `/v2/lbaas/l7policies/{l7policy_id}` to view the progress of the
/// provisioning operation. When the L7 policy status changes to `ACTIVE`, the
/// L7 policy is successfully provisioned and is ready for further
/// configuration.
///
/// If the API cannot fulfill the request due to insufficient data or data that
/// is not valid, the service returns the HTTP `Bad Request (400)` response
/// code with information about the failure in the response body. Validation
/// errors require that you correct the error and submit the request again.
///
/// All the rules associated with a given policy are logically ANDead together.
/// A request must match all the policy’s rules to match the policy.
///
/// If you need to express a logical OR operation between rules, then do this
/// by creating multiple policies with the same action.
///
/// If a new policy is created with a position that matches that of an existing
/// policy, then the new policy is inserted at the given position.
///
/// L7 policies with `action` of `REDIRECT_TO_URL` will return the default HTTP
/// `Found (302)` response code with the `redirect_url`. Also, specify
/// `redirect_http_code` to configure the needed HTTP response code, such as,
/// 301, 302, 303, 307 and 308.
///
/// L7 policies with `action` of `REJECT` will return a `Forbidden (403)`
/// response code to the requester.
///
#[derive(Args)]
#[command(about = "Create an L7 Policy")]
pub struct L7PolicyCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Defines mandatory and optional attributes of a POST request.
    ///
    #[command(flatten)]
    l7policy: L7policy,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Action {
    RedirectPrefix,
    RedirectToPool,
    RedirectToUrl,
    Reject,
}

/// L7policy Body data
#[derive(Args, Clone)]
struct L7policy {
    /// The L7 policy action. One of `REDIRECT_PREFIX`, `REDIRECT_TO_POOL`,
    /// `REDIRECT_TO_URL`, or `REJECT`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    action: Action,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    admin_state_up: Option<bool>,

    /// A human-readable description for the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The ID of the listener.
    ///
    #[arg(help_heading = "Body parameters", long)]
    listener_id: String,

    /// Human-readable name of the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The position of this policy on the listener. Positions start at 1.
    ///
    #[arg(help_heading = "Body parameters", long)]
    position: Option<i32>,

    /// The ID of the project owning this resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    /// Requests matching this policy will be redirected to the specified URL
    /// or Prefix URL with the HTTP response code. Valid if `action` is
    /// `REDIRECT_TO_URL` or `REDIRECT_PREFIX`. Valid options are: 301, 302,
    /// 303, 307, or 308. Default is 302.
    ///
    /// **New in version 2.9**
    ///
    #[arg(help_heading = "Body parameters", long)]
    redirect_http_code: Option<i32>,

    /// Requests matching this policy will be redirected to the pool with this
    /// ID. Only valid if `action` is `REDIRECT_TO_POOL`. The pool has some
    /// restrictions, See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    ///
    #[arg(help_heading = "Body parameters", long)]
    redirect_pool_id: Option<String>,

    /// Requests matching this policy will be redirected to this Prefix URL.
    /// Only valid if `action` is `REDIRECT_PREFIX`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    redirect_prefix: Option<String>,

    /// Requests matching this policy will be redirected to this URL. Only
    /// valid if `action` is `REDIRECT_TO_URL`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    redirect_url: Option<String>,

    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    rules: Option<Vec<Value>>,

    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,

    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,
}

/// L7Policy response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The L7 policy action. One of `REDIRECT_PREFIX`, `REDIRECT_TO_POOL`,
    /// `REDIRECT_TO_URL`, or `REJECT`.
    ///
    #[serde()]
    #[structable(optional)]
    action: Option<String>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<bool>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the L7 policy.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the listener.
    ///
    #[serde()]
    #[structable(optional)]
    listener_id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde()]
    #[structable(optional)]
    operating_status: Option<String>,

    /// The position of this policy on the listener. Positions start at 1.
    ///
    #[serde()]
    #[structable(optional)]
    position: Option<i32>,

    /// The ID of the project owning this resource.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[serde()]
    #[structable(optional)]
    provisioning_status: Option<String>,

    /// Requests matching this policy will be redirected to the specified URL
    /// or Prefix URL with the HTTP response code. Valid if `action` is
    /// `REDIRECT_TO_URL` or `REDIRECT_PREFIX`. Valid options are: 301, 302,
    /// 303, 307, or 308. Default is 302.
    ///
    /// **New in version 2.9**
    ///
    #[serde()]
    #[structable(optional)]
    redirect_http_code: Option<i32>,

    /// Requests matching this policy will be redirected to the pool with this
    /// ID. Only valid if `action` is `REDIRECT_TO_POOL`. The pool has some
    /// restrictions, See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    ///
    #[serde()]
    #[structable(optional)]
    redirect_pool_id: Option<String>,

    /// Requests matching this policy will be redirected to this Prefix URL.
    /// Only valid if `action` is `REDIRECT_PREFIX`.
    ///
    #[serde()]
    #[structable(optional)]
    redirect_prefix: Option<String>,

    /// Requests matching this policy will be redirected to this URL. Only
    /// valid if `action` is `REDIRECT_TO_URL`.
    ///
    #[serde()]
    #[structable(optional)]
    redirect_url: Option<String>,

    /// List of associated L7 rule IDs.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    rules: Option<Value>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl L7PolicyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create L7Policy");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.l7policy data
        let args = &self.l7policy;
        let mut l7policy_builder = create::L7policyBuilder::default();
        if let Some(val) = &args.name {
            l7policy_builder.name(val);
        }

        if let Some(val) = &args.description {
            l7policy_builder.description(val);
        }

        if let Some(val) = &args.admin_state_up {
            l7policy_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.project_id {
            l7policy_builder.project_id(val);
        }

        let tmp = match &args.action {
            Action::RedirectPrefix => create::Action::RedirectPrefix,
            Action::RedirectToPool => create::Action::RedirectToPool,
            Action::RedirectToUrl => create::Action::RedirectToUrl,
            Action::Reject => create::Action::Reject,
        };
        l7policy_builder.action(tmp);

        if let Some(val) = &args.redirect_pool_id {
            l7policy_builder.redirect_pool_id(val);
        }

        if let Some(val) = &args.redirect_url {
            l7policy_builder.redirect_url(val);
        }

        if let Some(val) = &args.redirect_prefix {
            l7policy_builder.redirect_prefix(val);
        }

        if let Some(val) = &args.position {
            l7policy_builder.position(*val);
        }

        l7policy_builder.listener_id(&args.listener_id);

        if let Some(val) = &args.rules {
            let rules_builder: Vec<create::Rules> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Rules>(v.to_owned()))
                .collect::<Vec<create::Rules>>();
            l7policy_builder.rules(rules_builder);
        }

        if let Some(val) = &args.tags {
            l7policy_builder.tags(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.redirect_http_code {
            l7policy_builder.redirect_http_code(*val);
        }

        if let Some(val) = &args.tenant_id {
            l7policy_builder.tenant_id(val);
        }

        ep_builder.l7policy(l7policy_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
