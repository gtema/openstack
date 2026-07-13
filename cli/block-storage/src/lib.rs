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

//! Block storage commands.
use clap::error::{Error, ErrorKind};
use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches};

use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk::AsyncOpenStack;

pub mod v3;

const API_VERSION_ARG_ID: &str = "os_block_storage_api_version";
const API_VERSION_LONG: &str = "os-block-storage-api-version";
const API_VERSION_ENV: &str = "OS_BLOCK_STORAGE_API_VERSION";
const DEFAULT_BLOCK_STORAGE_API_VERSION: &str = "3";

/// Block Storage (Volume) service (Cinder) commands
pub enum BlockStorageCommand {
    V3(v3::BlockStorageCommand),
}

impl Args for BlockStorageCommand {
    fn augment_args(cmd: Command) -> Command {
        // Only one version currently exists, so there's nothing to branch
        // on here; `from_arg_matches` below still validates the flag's
        // actual value and reports a proper "unsupported version" error
        // for anything else.
        let cmd = cmd.arg(
            Arg::new(API_VERSION_ARG_ID)
                .long(API_VERSION_LONG)
                .env(API_VERSION_ENV)
                .global(true)
                .action(ArgAction::Set)
                .default_value(DEFAULT_BLOCK_STORAGE_API_VERSION)
                .help("Block Storage API version to use (default: 3)"),
        );
        v3::BlockStorageCommand::augment_args(cmd)
    }

    fn augment_args_for_update(cmd: Command) -> Command {
        Self::augment_args(cmd)
    }
}

impl FromArgMatches for BlockStorageCommand {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let version = matches
            .get_one::<String>(API_VERSION_ARG_ID)
            .map(String::as_str)
            .unwrap_or(DEFAULT_BLOCK_STORAGE_API_VERSION);
        match version {
            "3" => Ok(Self::V3(v3::BlockStorageCommand::from_arg_matches(
                matches,
            )?)),
            other => Err(Error::raw(
                ErrorKind::InvalidValue,
                format!("unsupported Block Storage API version: {other}. Supported: 3\n"),
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

impl BlockStorageCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match self {
            Self::V3(cmd) => cmd.take_action(parsed_args, client).await,
        }
    }
}
