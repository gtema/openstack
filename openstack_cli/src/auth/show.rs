//! Show current auth information
use async_trait::async_trait;
use clap::Args;
use tracing::info;

use crate::output::{self, OutputProcessor};
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::types::identity::v3::AuthResponse;
use openstack_sdk::AsyncOpenStack;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct AuthArgs {}

/// show command
pub struct AuthCmd {
    pub args: AuthArgs,
}

impl StructTable for AuthResponse {
    fn build(&self, options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
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
impl Command for AuthCmd {
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
