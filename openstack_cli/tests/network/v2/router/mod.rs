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

mod add_external_gateways_autogen;
mod add_extraroutes_autogen;
mod add_router_interface_autogen;
mod conntrack_helper;
mod create_autogen;
mod delete_autogen;
mod l3_agent;
mod list;
mod list_autogen;
mod remove_external_gateways_autogen;
mod remove_extraroutes_autogen;
mod remove_router_interface_autogen;
mod set_autogen;
mod show_autogen;
mod tag;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.arg("network").arg("router").arg("--help");
    cmd.assert().success();

    Ok(())
}
