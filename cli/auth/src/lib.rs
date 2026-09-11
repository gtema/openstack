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

//! Authorization operations

use clap::{Parser, Subcommand};

use openstack_cli_core::{
    cli::{CliArgs, ConnectionRequirements, ConnectionRequirementsProvider},
    error::OpenStackCliError,
};
use openstack_sdk::AsyncOpenStack;

pub mod cache;
pub mod login;
pub mod logout;
pub mod show;
pub mod status;

/// Cloud Authentication operations
///
/// This command provides various authorization
/// operations (login, show, status, etc)
#[derive(Parser)]
pub struct AuthCommand {
    /// Authentication commands
    #[command(subcommand)]
    pub command: AuthCommands,
}

#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AuthCommands {
    Cache(cache::CacheCommand),
    Login(login::LoginCommand),
    Logout(logout::LogoutCommand),
    Show(show::ShowCommand),
    Status(status::StatusCommand),
}

impl ConnectionRequirementsProvider for AuthCommands {
    fn connection_requirements(&self) -> ConnectionRequirements {
        match self {
            AuthCommands::Login(login::LoginCommand { renew: true, .. }) => {
                ConnectionRequirements {
                    needs_auth: true,
                    renew: true,
                }
            }
            AuthCommands::Status(_) => ConnectionRequirements {
                needs_auth: false,
                renew: false,
            },
            AuthCommands::Logout(_) => ConnectionRequirements {
                needs_auth: false,
                renew: false,
            },
            AuthCommands::Cache(cache) => cache.command.connection_requirements(),
            _ => ConnectionRequirements::connected(),
        }
    }
}

impl AuthCommand {
    /// Returns `Some(&ClearCommand)` only when this is `auth cache clear
    /// --all`, which must run before any cloud is resolved. See
    /// `openstack_cli`'s entry point.
    pub fn as_offline_cache_clear_all(&self) -> Option<&cache::clear::ClearCommand> {
        match &self.command {
            AuthCommands::Cache(cache) => cache.as_offline_cache_clear_all(),
            _ => None,
        }
    }

    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AuthCommands::Cache(cmd) => cmd.take_action(parsed_args, client).await,
            AuthCommands::Show(cmd) => cmd.take_action(parsed_args, client).await,
            AuthCommands::Login(cmd) => cmd.take_action(parsed_args, client).await,
            AuthCommands::Logout(cmd) => cmd.take_action(parsed_args, client).await,
            AuthCommands::Status(cmd) => cmd.take_action(parsed_args, client).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> AuthCommand {
        let mut full = vec!["auth"];
        full.extend_from_slice(args);
        AuthCommand::try_parse_from(full).expect("should parse")
    }

    #[test]
    fn logout_parses() {
        let cmd = parse(&["logout"]);
        assert!(matches!(
            cmd.command,
            AuthCommands::Logout(logout::LogoutCommand { local: false })
        ));
        let req = cmd.command.connection_requirements();
        assert!(!req.needs_auth);
        assert!(!req.renew);
    }

    #[test]
    fn logout_local_parses() {
        let cmd = parse(&["logout", "--local"]);
        assert!(matches!(
            cmd.command,
            AuthCommands::Logout(logout::LogoutCommand { local: true })
        ));
    }

    #[test]
    fn cache_clear_parses() {
        let cmd = parse(&["cache", "clear"]);
        let req = cmd.command.connection_requirements();
        assert!(!req.needs_auth);
        assert!(!req.renew);
        assert!(cmd.as_offline_cache_clear_all().is_none());
    }

    #[test]
    fn cache_clear_all_parses() {
        let cmd = parse(&["cache", "clear", "--all"]);
        assert!(cmd.as_offline_cache_clear_all().is_some());
        let req = cmd.command.connection_requirements();
        assert!(!req.needs_auth);
        assert!(!req.renew);
    }

    #[test]
    fn cache_clear_all_yes_parses() {
        let cmd = parse(&["cache", "clear", "--all", "--yes"]);
        let clear = cmd.as_offline_cache_clear_all().expect("is --all");
        assert!(clear.all);
        assert!(clear.yes);
    }
}
