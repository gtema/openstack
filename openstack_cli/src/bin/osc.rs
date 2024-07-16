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

//! OpenStack CLI
//!
//! The binary of the CLI
#![deny(missing_docs)]

use color_eyre::eyre::{Report, Result};

use openstack_cli::error::OpenStackCliError;

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::config::HookBuilder::default()
        .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
        .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
        .issue_filter(|kind| match kind {
            color_eyre::ErrorKind::NonRecoverable(_) => true,
            color_eyre::ErrorKind::Recoverable(error) => {
                match error.downcast_ref::<OpenStackCliError>() {
                    Some(OpenStackCliError::OpenStackApi { .. }) => false,
                    Some(OpenStackCliError::Auth { .. }) => false,
                    Some(OpenStackCliError::ReScope { .. }) => false,
                    Some(OpenStackCliError::ConnectionNotFound { .. }) => false,
                    _ => true,
                }
            }
        })
        .install()?;

    openstack_cli::entry_point().await?;
    Ok(())
}
