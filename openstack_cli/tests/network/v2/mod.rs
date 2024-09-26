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

mod address_group;
mod address_scope;
mod agent;
mod auto_allocated_topology;
mod availability_zone;
mod default_security_group_rule;
mod extension;
mod floatingip;
mod network;
mod port;
mod rbac_policy;
mod router;
mod security_group;
mod security_group_rule;
mod subnet;
mod subnetpool;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.arg("network").arg("--help");
    cmd.assert().success();

    Ok(())
}
