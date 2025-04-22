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

//! Integration tests of OpenStackCLI

mod api;
#[cfg(feature = "block_storage")]
mod block_storage;
mod catalog;
#[cfg(feature = "compute")]
mod compute;
#[cfg(feature = "container_infra")]
mod container_infrastructure_management;
#[cfg(feature = "dns")]
mod dns;
#[cfg(feature = "identity")]
mod identity;
#[cfg(feature = "image")]
mod image;
#[cfg(feature = "load_balancer")]
mod load_balancer;
#[macro_use]
mod macros;
#[cfg(feature = "network")]
mod network;
#[cfg(feature = "object_store")]
mod object_store;
#[cfg(feature = "placement")]
mod placement;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.arg("--help");
    cmd.assert().success();

    Ok(())
}
