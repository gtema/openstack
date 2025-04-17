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

//! Show current auth information
use chrono::prelude::*;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::{self, OutputProcessor};
use structable::{StructTable, StructTableOptions};

use openstack_sdk::AsyncOpenStack;
use openstack_sdk::types::identity::v3::AuthResponse;

/// Show current authorization information for the cloud
///
/// This command returns authentication and authorization
/// information for the currently active connection. It includes
/// issue and expiration information, user data, list of granted
/// roles and project/domain information.
///
/// **NOTE**: The command does not support selecting individual
/// fields in the output, but it supports `-o json` command and
/// returns full available information in json format what allows
/// further processing with `jq`
#[derive(Debug, Parser)]
pub struct ShowCommand {}

/// Authentication info
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct Auth {
    /// Auth roles
    #[structable(optional, serialize)]
    pub roles: Option<Value>,

    /// Authenticated user
    #[structable(serialize)]
    pub user: Value,

    /// Project scope information
    #[structable(optional, serialize)]
    pub project: Option<Value>,

    /// Domain scope information
    #[structable(optional, serialize)]
    pub domain: Option<Value>,

    /// System scope information
    #[structable(optional, serialize)]
    pub system: Option<Value>,

    /// Issued at of the token
    #[structable(optional, serialize)]
    pub issued_at: Option<DateTime<Utc>>,

    /// Token expiration time
    #[structable(serialize)]
    pub expires_at: DateTime<Utc>,
}

impl TryFrom<&AuthResponse> for Auth {
    type Error = eyre::Report;
    fn try_from(value: &AuthResponse) -> Result<Self, Self::Error> {
        let roles: Option<Value> = if let Some(val) = &value.token.roles {
            Some(serde_json::to_value(val)?)
        } else {
            None
        };
        let project: Option<Value> = if let Some(val) = &value.token.project {
            Some(serde_json::to_value(val)?)
        } else {
            None
        };
        let domain: Option<Value> = if let Some(val) = &value.token.domain {
            Some(serde_json::to_value(val)?)
        } else {
            None
        };
        let system: Option<Value> = if let Some(val) = &value.token.system {
            Some(serde_json::to_value(val)?)
        } else {
            None
        };
        Ok(Self {
            roles,
            user: serde_json::to_value(&value.token.user)?,
            project,
            domain,
            system,
            issued_at: value.token.issued_at,
            expires_at: value.token.expires_at,
        })
    }
}

impl ShowCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show auth info");

        let op = OutputProcessor::from_args(parsed_args);

        if let Some(auth_info) = client.get_auth_info() {
            match op.target {
                output::OutputFor::Human => {
                    op.output_human(&Auth::try_from(&auth_info)?)?;
                }
                _ => {
                    op.output_machine(serde_json::to_value(auth_info)?)?;
                }
            }
        }
        Ok(())
    }
}
