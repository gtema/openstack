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
use async_trait::async_trait;
use clap::Args;
use tracing::info;

use crate::output::{self, OutputProcessor};
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::types::identity::v3::AuthResponse;
use openstack_sdk::AsyncOpenStack;

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
#[derive(Args, Clone, Debug)]
pub struct AuthArgs {}

/// show command
pub struct AuthCmd {
    pub args: AuthArgs,
}

impl StructTable for AuthResponse {
    fn build(&self, _: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Field".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        if let Some(issued_at) = self.token.issued_at {
            rows.push(Vec::from(["issued_at".to_string(), issued_at.to_string()]));
        }
        rows.push(Vec::from([
            "expires_at".to_string(),
            self.token.expires_at.to_string(),
        ]));
        rows.push(Vec::from([
            "user".to_string(),
            serde_json::to_string(&self.token.user).expect("Should never happen"),
        ]));
        if let Some(data) = &self.token.roles {
            rows.push(Vec::from([
                "roles".to_string(),
                serde_json::to_string(&data).expect("Should never happen"),
            ]));
        }
        if let Some(data) = &self.token.project {
            rows.push(Vec::from([
                "project".to_string(),
                serde_json::to_string(&data).expect("Should never happen"),
            ]));
        }
        if let Some(data) = &self.token.domain {
            rows.push(Vec::from([
                "domain".to_string(),
                serde_json::to_string(&data).expect("Should never happen"),
            ]));
        }
        (headers, rows)
    }
}

#[async_trait]
impl OSCCommand for AuthCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show auth info");

        let op = OutputProcessor::from_args(parsed_args);

        if let Some(auth_info) = client.get_auth_info() {
            match op.target {
                output::OutputFor::Human => {
                    op.output_human(&auth_info)?;
                }
                _ => {
                    op.output_machine(serde_json::to_value(auth_info)?)?;
                }
            }
        }
        Ok(())
    }
}
