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

#[test]
fn show() -> Result<(), Box<dyn std::error::Error>> {
    skip_without_service!("object-store");

    let mut cmd = Command::cargo_bin("osc")?;

    cmd.args(["object-store", "account", "show"]);
    cmd.assert().success();

    Ok(())
}
