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

//! Create RbacPolicy command
//!
//! Wraps invoking of the `v2.0/rbac-policies` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::rbac_policy::create;
use openstack_types::network::v2::rbac_policy::response::create::RbacPolicyResponse;

/// Create RBAC policy for given tenant.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401
#[derive(Args)]
#[command(about = "Create RBAC policy")]
pub struct RbacPolicyCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    rbac_policy: RbacPolicy,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// RbacPolicy Body data
#[derive(Args, Clone)]
struct RbacPolicy {
    /// Action for the RBAC policy which is `access_as_external` or
    /// `access_as_shared`.
    #[arg(help_heading = "Body parameters", long)]
    action: Option<String>,

    /// The ID of the `object_type` resource. An `object_type` of `network`
    /// returns a network ID, an `object_type` of `qos-policy` returns a QoS
    /// policy ID, an `object_type` of `security-group` returns a security
    /// group ID, an `object_type` of `address-scope` returns a address scope
    /// ID, an `object_type` of `subnetpool` returns a subnetpool ID and an
    /// `object_type` of `address-group` returns an address group ID.
    #[arg(help_heading = "Body parameters", long)]
    object_id: Option<String>,

    /// The type of the object that the RBAC policy affects. Types include
    /// `qos-policy`, `network`, `security-group`, `address-scope`,
    /// `subnetpool` or `address-group`.
    #[arg(help_heading = "Body parameters", long)]
    object_type: Option<String>,

    /// The ID of the tenant to which the RBAC policy will be enforced. Please
    /// note that Neutron does not perform any type of validation that the
    /// value provided is actually the ID of the existing project. If, for
    /// example, the name of the project is provided here, it will be accepted
    /// by the Neutron API, but the RBAC rule created will not work as
    /// expected.
    #[arg(help_heading = "Body parameters", long)]
    target_tenant: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,
}

impl RbacPolicyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create RbacPolicy");

        let op =
            OutputProcessor::from_args(parsed_args, Some("network.rbac_policy"), Some("create"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set body parameters
        // Set Request.rbac_policy data
        let args = &self.rbac_policy;
        let mut rbac_policy_builder = create::RbacPolicyBuilder::default();
        if let Some(val) = &args.action {
            rbac_policy_builder.action(val);
        }

        if let Some(val) = &args.object_id {
            rbac_policy_builder.object_id(val);
        }

        if let Some(val) = &args.object_type {
            rbac_policy_builder.object_type(val);
        }

        if let Some(val) = &args.target_tenant {
            rbac_policy_builder.target_tenant(val);
        }

        if let Some(val) = &args.tenant_id {
            rbac_policy_builder.tenant_id(val);
        }

        ep_builder.rbac_policy(rbac_policy_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<RbacPolicyResponse>(data)?;
        Ok(())
    }
}
