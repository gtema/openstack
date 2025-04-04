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

mod attachment;
mod backup;
mod cluster;
mod default_type;
mod extension;
mod group;
mod group_snapshot;
mod group_type;
mod host;
mod limit;
mod message;
mod qos_spec;
mod resource_filter;
mod service;
mod snapshot;
mod snapshot_manage;
mod r#type;
mod volume;
mod volume_manage;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.arg("block-storage").arg("--help");
    cmd.assert().success();

    Ok(())
}
