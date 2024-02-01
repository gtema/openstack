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

mod cli {
    mod image {
        use assert_cmd::prelude::*;
        use std::process::Command;

        #[test]
        #[ignore]
        fn image_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("image").arg("list");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn schema_image_show() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("schema").arg("image").arg("show");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn schema_images_show() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("schema").arg("images").arg("show");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn schema_member_show() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("schema").arg("member").arg("show");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn schema_members_show() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("schema").arg("members").arg("show");
            cmd.assert().success();

            Ok(())
        }
    }
}
