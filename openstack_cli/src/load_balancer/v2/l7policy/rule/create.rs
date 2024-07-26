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

//! Create Rule command
//!
//! Wraps invoking of the `v2/lbaas/l7policies/{l7policy_id}/rules` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use clap::ValueEnum;
use openstack_sdk::api::load_balancer::v2::l7policy::rule::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a L7 rule.
///
/// This operation provisions a new L7 rule by using the configuration that you
/// define in the request object. After the API validates the request and
/// starts the provisioning process, the API returns a response object that
/// contains a unique ID and the status of provisioning the L7 rule.
///
/// In the response, the L7 rule [provisioning status](#prov-status) is
/// `ACTIVE`, `PENDING_CREATE`, or `ERROR`.
///
/// If the status is `PENDING_CREATE`, issue GET
/// `/v2/lbaas/l7policies/{l7policy_id}/rules/{l7rule_id}` to view the progress
/// of the provisioning operation. When the L7 rule status changes to `ACTIVE`,
/// the L7 rule is successfully provisioned and is ready for further
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
#[derive(Args)]
#[command(about = "Create an L7 Rule")]
pub struct RuleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Defines mandatory and optional attributes of a POST request.
    ///
    #[command(flatten)]
    rule: Rule,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// l7policy_id parameter for
    /// /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_l7policy_id",
        value_name = "L7POLICY_ID"
    )]
    l7policy_id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Type {
    Cookie,
    FileType,
    Header,
    HostName,
    Path,
    SslConnHasCert,
    SslDnField,
    SslVerifyResult,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum CompareType {
    Contains,
    EndsWith,
    EqualTo,
    Regex,
    StartsWith,
}

/// Rule Body data
#[derive(Args, Clone)]
struct Rule {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    admin_state_up: Option<bool>,

    /// The comparison type for the L7 rule. One of `CONTAINS`, `ENDS_WITH`,
    /// `EQUAL_TO`, `REGEX`, or `STARTS_WITH`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    compare_type: CompareType,

    /// When `true` the logic of the rule is inverted. For example, with invert
    /// `true`, equal to would become not equal to. Default is `false`.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    invert: Option<bool>,

    /// The key to use for the comparison. For example, the name of the cookie
    /// to evaluate.
    ///
    #[arg(help_heading = "Body parameters", long)]
    key: Option<String>,

    /// The ID of the project owning this resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,

    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,

    /// The L7 rule type. One of `COOKIE`, `FILE_TYPE`, `HEADER`, `HOST_NAME`,
    /// `PATH`, `SSL_CONN_HAS_CERT`, `SSL_VERIFY_RESULT`, or `SSL_DN_FIELD`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    _type: Type,

    /// The value to use for the comparison. For example, the file type to
    /// compare.
    ///
    #[arg(help_heading = "Body parameters", long)]
    value: String,
}

/// Rule response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<bool>,

    /// The comparison type for the L7 rule. One of `CONTAINS`, `ENDS_WITH`,
    /// `EQUAL_TO`, `REGEX`, or `STARTS_WITH`.
    ///
    #[serde()]
    #[structable(optional)]
    compare_type: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The ID of the L7 rule.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// When `true` the logic of the rule is inverted. For example, with invert
    /// `true`, equal to would become not equal to.
    ///
    #[serde()]
    #[structable(optional)]
    invert: Option<bool>,

    /// The key to use for the comparison. For example, the name of the cookie
    /// to evaluate.
    ///
    #[serde()]
    #[structable(optional)]
    key: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde()]
    #[structable(optional)]
    operating_status: Option<String>,

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

    /// The L7 rule type. One of `COOKIE`, `FILE_TYPE`, `HEADER`, `HOST_NAME`,
    /// `PATH`, `SSL_CONN_HAS_CERT`, `SSL_VERIFY_RESULT`, or `SSL_DN_FIELD`.
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The value to use for the comparison. For example, the file type to
    /// compare.
    ///
    #[serde()]
    #[structable(optional)]
    value: Option<String>,
}

impl RuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Rule");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.l7policy_id(&self.path.l7policy_id);
        // Set query parameters
        // Set body parameters
        // Set Request.rule data
        let args = &self.rule;
        let mut rule_builder = create::RuleBuilder::default();

        let tmp = match &args._type {
            Type::Cookie => create::Type::Cookie,
            Type::FileType => create::Type::FileType,
            Type::Header => create::Type::Header,
            Type::HostName => create::Type::HostName,
            Type::Path => create::Type::Path,
            Type::SslConnHasCert => create::Type::SslConnHasCert,
            Type::SslDnField => create::Type::SslDnField,
            Type::SslVerifyResult => create::Type::SslVerifyResult,
        };
        rule_builder._type(tmp);

        let tmp = match &args.compare_type {
            CompareType::Contains => create::CompareType::Contains,
            CompareType::EndsWith => create::CompareType::EndsWith,
            CompareType::EqualTo => create::CompareType::EqualTo,
            CompareType::Regex => create::CompareType::Regex,
            CompareType::StartsWith => create::CompareType::StartsWith,
        };
        rule_builder.compare_type(tmp);

        if let Some(val) = &args.key {
            rule_builder.key(val);
        }

        rule_builder.value(&args.value);

        if let Some(val) = &args.invert {
            rule_builder.invert(*val);
        }

        if let Some(val) = &args.admin_state_up {
            rule_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.project_id {
            rule_builder.project_id(val);
        }

        if let Some(val) = &args.tags {
            rule_builder.tags(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.tenant_id {
            rule_builder.tenant_id(val);
        }

        ep_builder.rule(rule_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
