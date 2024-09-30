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
use assert_cmd::prelude::*;
use std::process::Command;

mod create_20_autogen;
mod create_215_autogen;
mod create_264_autogen;
mod delete_autogen;
mod list_autogen;
mod show_autogen;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.arg("compute").arg("server-group").arg("--help");
    cmd.assert().success();

    Ok(())
}
