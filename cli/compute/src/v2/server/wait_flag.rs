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

//! Shared `--wait` flag and server-status-wait helpers for the `create`/`delete` commands.
//! Hand-written — not generated. Kept as thin, non-generated glue so the generated `create_294`/
//! `delete` files only need a one-field, one-call patch each; codegen support for emitting this
//! automatically per-resource is tracked separately.
//!
//! Not yet wired into any generated command (that happens in a later task), so these items are
//! currently unused from the compiler's point of view.

use clap::Args;
use std::time::Duration;

use openstack_cli_core::error::OpenStackCliError;
use openstack_sdk::AsyncOpenStack;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::get;
use openstack_sdk::api::{wait_deleted, wait_for_status_typed};

const SERVER_READY: &[&str] = &["ACTIVE"];
const SERVER_FAILED: &[&str] = &["ERROR"];

/// Wait-related CLI flags, flattened into `create`/`delete` commands.
#[derive(Args, Debug)]
pub struct WaitParameters {
    /// Wait for the server to reach its target status (or, for delete, to disappear) before
    /// returning. Uses server-side status polling.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub wait: bool,
    /// Maximum time to wait, in seconds. Only meaningful with `--wait`.
    #[arg(long, default_value_t = 600)]
    pub wait_timeout: u64,
}

/// Poll `id` until it reaches `ACTIVE` (or `ERROR`, which is a failure), returning the final
/// observed body so the caller can re-render it.
///
/// Not yet wired into any generated command (that happens in a later task).
#[allow(dead_code)]
pub async fn wait_for_server_active(
    client: &mut AsyncOpenStack,
    id: &str,
    timeout_secs: u64,
) -> Result<serde_json::Value, OpenStackCliError> {
    let ep = get::Request::builder()
        .id(id)
        .build()
        .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
    let outcome = wait_for_status_typed(ep, SERVER_READY, SERVER_FAILED)
        .timeout(Duration::from_secs(timeout_secs))
        .query_async(client)
        .await?;
    Ok(outcome.into_present().unwrap_or(serde_json::Value::Null))
}

/// Poll `id` until it disappears (404).
///
/// Not yet wired into any generated command (that happens in a later task).
#[allow(dead_code)]
pub async fn wait_for_server_deleted(
    client: &mut AsyncOpenStack,
    id: &str,
    timeout_secs: u64,
) -> Result<(), OpenStackCliError> {
    let ep = get::Request::builder()
        .id(id)
        .build()
        .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
    wait_deleted(ep)
        .timeout(Duration::from_secs(timeout_secs))
        .query_async(client)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser)]
    struct TestCli {
        #[command(flatten)]
        wait: WaitParameters,
    }

    #[test]
    fn wait_flag_defaults_to_false_and_600s() {
        let cli = TestCli::parse_from(["prog"]);
        assert!(!cli.wait.wait);
        assert_eq!(cli.wait.wait_timeout, 600);
    }

    #[test]
    fn wait_flag_parses_explicit_values() {
        let cli = TestCli::parse_from(["prog", "--wait", "--wait-timeout", "30"]);
        assert!(cli.wait.wait);
        assert_eq!(cli.wait.wait_timeout, 30);
    }
}
