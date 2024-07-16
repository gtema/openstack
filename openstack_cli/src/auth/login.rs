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

//! Perform cloud login
use clap::Parser;
use eyre::eyre;
use std::io::{self, Write};
use tracing::info;

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

/// Fetch a new valid authorization token for the cloud.
///
/// This command writes token to the stdout
#[derive(Parser)]
#[command(about = "Login to the cloud and get a valid authorization token")]
pub struct LoginCommand {
    /// Require token renewal
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub renew: bool,
}

impl LoginCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        _parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show auth info");

        // TODO(gtema): here would be the Webbrowser based login
        // implementation

        if let Some(token) = client.get_auth_token() {
            let mut stdout = io::stdout().lock();

            stdout.write_all(&token.into_bytes())?;
            return Ok(());
        }
        Err(eyre!("Authorization information missing").into())
    }
}
