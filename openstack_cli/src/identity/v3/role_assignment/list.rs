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

//! List RoleAssignments command
//!
//! Wraps invoking of the `v3/role_assignments` with `GET` method

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

use openstack_sdk::api::identity::v3::role_assignment::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Get a list of role assignments.
///
/// If no query parameters are specified, then this API will return a list of
/// all role assignments.
///
/// Since this list is likely to be very long, this API would typically always
/// be used with one of more of the filter queries. Some typical examples are:
///
/// `GET /v3/role_assignments?user.id={user_id}` would list all role
/// assignments involving the specified user.
///
/// `GET /v3/role_assignments?scope.project.id={project_id}` would list all
/// role assignments involving the specified project.
///
/// It is also possible to list all role assignments within a tree of projects:
/// `GET /v3/role_assignments?scope.project.id={project_id}&include_subtree=true`
/// would list all role assignments involving the specified project and all
/// sub-projects. `include_subtree=true` can only be specified in conjunction
/// with `scope.project.id`, specifying it without this will result in an HTTP
/// 400 Bad Request being returned.
///
/// Each role assignment entity in the collection contains a link to the
/// assignment that gave rise to this entity.
///
/// The scope section in the list response is extended to allow the
/// representation of role assignments that are inherited to projects.
///
/// The query filter `scope.OS-INHERIT:inherited_to` can be used to filter
/// based on role assignments that are inherited. The only value of
/// `scope.OS-INHERIT:inherited_to` that is currently supported is `projects`,
/// indicating that this role is inherited to all projects of the owning domain
/// or parent project.
///
/// If the query parameter `effective` is specified, rather than simply
/// returning a list of role assignments that have been made, the API returns a
/// list of effective assignments at the user, project and domain level, having
/// allowed for the effects of group membership, role inference rules as well
/// as inheritance from the parent domain or project. Since the effects of
/// group membership have already been allowed for, the group role assignment
/// entities themselves will not be returned in the collection. Likewise, since
/// the effects of inheritance have already been allowed for, the role
/// assignment entities themselves that specify the inheritance will also not
/// be returned in the collection. This represents the effective role
/// assignments that would be included in a scoped token. The same set of query
/// parameters can also be used in combination with the `effective` parameter.
///
/// For example:
///
/// `GET /v3/role_assignments?user.id={user_id}&effective` would, in other
/// words, answer the question “what can this user actually do?”.
///
/// `GET /v3/role_assignments?user.id={user_id}&scope.project.id={project_id}&effective`
/// would return the equivalent set of role assignments that would be included
/// in the token response of a project scoped token.
///
/// An example response for an API call with the query parameter `effective`
/// specified is given below:
///
/// The entity `links` section of a response using the `effective` query
/// parameter also contains, for entities that are included by virtue of group
/// membership, a url that can be used to access the membership of the group.
///
/// If the query parameter `include_names` is specified, rather than simply
/// returning the entity IDs in the role assignments, the collection will
/// additionally include the names of the entities. For example:
///
/// `GET /v3/role_assignments?user.id={user_id}&effective&include_names=true`
/// would return:
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/role_assignments`
///
#[derive(Args)]
#[command(about = "List role assignments")]
pub struct RoleAssignmentsCommand {
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
    /// Returns the effective assignments, including any assignments gained by
    /// virtue of group membership.
    ///
    #[arg(action=clap::ArgAction::SetTrue, help_heading = "Query parameters", long)]
    effective: Option<bool>,

    /// Filters the response by a group ID.
    ///
    #[arg(help_heading = "Query parameters", long)]
    group_id: Option<String>,

    /// If set, then the names of any entities returned will be include as well
    /// as their IDs. Any value other than 0 (including no value) will be
    /// interpreted as true.
    ///
    #[arg(action=clap::ArgAction::SetTrue, help_heading = "Query parameters", long)]
    include_names: Option<bool>,

    /// If set, then relevant assignments in the project hierarchy below the
    /// project specified in the scope.project_id query parameter are also
    /// included in the response. Any value other than 0 (including no value)
    /// for include_subtree will be interpreted as true.
    ///
    #[arg(action=clap::ArgAction::SetTrue, help_heading = "Query parameters", long)]
    include_subtree: Option<bool>,

    /// Filters the response by a role ID.
    ///
    #[arg(help_heading = "Query parameters", long)]
    role_id: Option<String>,

    /// Filters the response by a domain ID.
    ///
    #[arg(help_heading = "Query parameters", long)]
    scope_domain_id: Option<String>,

    /// Filters based on role assignments that are inherited. The only value of
    /// inherited_to that is currently supported is projects.
    ///
    #[arg(help_heading = "Query parameters", long)]
    scope_os_inherit_inherited_to: Option<String>,

    /// Filters the response by a project ID.
    ///
    #[arg(help_heading = "Query parameters", long)]
    scope_project_id: Option<String>,

    /// Filters the response by a user ID.
    ///
    #[arg(help_heading = "Query parameters", long)]
    user_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// RoleAssignments response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional, pretty)]
    group: Option<Value>,

    /// A prior role object.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    role: Option<Value>,

    /// The authorization scope, including the system (Since v3.10), a project,
    /// or a domain (Since v3.4). If multiple scopes are specified in the same
    /// request (e.g. project and domain or domain and system) an HTTP 400 Bad
    /// Request will be returned, as a token cannot be simultaneously scoped to
    /// multiple authorization targets. An ID is sufficient to uniquely
    /// identify a project but if a project is specified by name, then the
    /// domain of the project must also be specified in order to uniquely
    /// identify the project by name. A domain scope may be specified by either
    /// the domain’s ID or name with equivalent results.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    scope: Option<Value>,

    /// A user object
    ///
    #[serde()]
    #[structable(optional, pretty)]
    user: Option<Value>,
}

impl RoleAssignmentsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List RoleAssignments");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.group_id {
            ep_builder.group_id(val);
        }
        if let Some(val) = &self.query.role_id {
            ep_builder.role_id(val);
        }
        if let Some(val) = &self.query.user_id {
            ep_builder.user_id(val);
        }
        if let Some(val) = &self.query.scope_domain_id {
            ep_builder.scope_domain_id(val);
        }
        if let Some(val) = &self.query.scope_project_id {
            ep_builder.scope_project_id(val);
        }
        if let Some(val) = &self.query.scope_os_inherit_inherited_to {
            ep_builder.scope_os_inherit_inherited_to(val);
        }
        if let Some(true) = self.query.effective {
            ep_builder.effective(serde_json::Value::Null);
        }
        if let Some(true) = self.query.include_names {
            ep_builder.include_names(serde_json::Value::Null);
        }
        if let Some(true) = self.query.include_subtree {
            ep_builder.include_subtree(serde_json::Value::Null);
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
