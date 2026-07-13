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

//! Identity (Keystone) API bindings
use clap::error::{Error, ErrorKind};
use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches};

use openstack_cli_core::{
    cli::{CliArgs, resolve_api_version},
    error::OpenStackCliError,
};
use openstack_sdk::AsyncOpenStack;

pub mod v3;
#[cfg(feature = "keystone_ng")]
pub mod v4;

const API_VERSION_ARG_ID: &str = "os_identity_api_version";
const API_VERSION_LONG: &str = "os-identity-api-version";
const API_VERSION_ENV: &str = "OS_IDENTITY_API_VERSION";
const DEFAULT_IDENTITY_API_VERSION: &str = "3";

/// Identity (Keystone) commands
pub enum IdentityCommand {
    V3(Box<v3::IdentityCommand>),
    #[cfg(feature = "keystone_ng")]
    V4(Box<v4::IdentityCommand>),
}

impl Args for IdentityCommand {
    fn augment_args(cmd: Command) -> Command {
        let version = resolve_api_version(
            API_VERSION_LONG,
            API_VERSION_ENV,
            DEFAULT_IDENTITY_API_VERSION,
        );
        let cmd = cmd.arg(
            Arg::new(API_VERSION_ARG_ID)
                .long(API_VERSION_LONG)
                .env(API_VERSION_ENV)
                .global(true)
                .action(ArgAction::Set)
                .default_value(DEFAULT_IDENTITY_API_VERSION)
                .help("Identity API version to use (default: 3)"),
        );
        match version.as_str() {
            #[cfg(feature = "keystone_ng")]
            "4" => v4::IdentityCommand::augment_args(cmd),
            // Any other value (including the default "3") falls back to
            // v3's tree, so an invalid version still parses cleanly and
            // `from_arg_matches` below reports a proper "unsupported
            // version" error instead of clap's generic "unexpected
            // argument" (which would fire first if no subcommand tree
            // were attached at all).
            _ => v3::IdentityCommand::augment_args(cmd),
        }
    }

    fn augment_args_for_update(cmd: Command) -> Command {
        Self::augment_args(cmd)
    }
}

impl FromArgMatches for IdentityCommand {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let version = matches
            .get_one::<String>(API_VERSION_ARG_ID)
            .map(String::as_str)
            .unwrap_or(DEFAULT_IDENTITY_API_VERSION);
        match version {
            "3" => Ok(Self::V3(Box::new(v3::IdentityCommand::from_arg_matches(
                matches,
            )?))),
            #[cfg(feature = "keystone_ng")]
            "4" => Ok(Self::V4(Box::new(v4::IdentityCommand::from_arg_matches(
                matches,
            )?))),
            other => Err(Error::raw(
                ErrorKind::InvalidValue,
                format!(
                    "unsupported Identity API version: {other}. Supported: 3{}\n",
                    if cfg!(feature = "keystone_ng") {
                        ", 4"
                    } else {
                        ""
                    }
                ),
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

impl IdentityCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match self {
            Self::V3(cmd) => cmd.take_action(parsed_args, client).await,
            #[cfg(feature = "keystone_ng")]
            Self::V4(cmd) => cmd.take_action(parsed_args, client).await,
        }
    }
}
