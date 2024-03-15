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

mod create_30_autogen;
mod create_313_autogen;
mod create_347_autogen;
mod create_353_autogen;
mod delete_autogen;
mod list;
mod list_autogen;
mod metadata;
// mod os_attach_autogen;
// mod os_begin_detaching_autogen;
// mod os_detach_autogen;
mod os_extend_autogen;
// mod os_force_delete_autogen;
// mod os_force_detach_autogen;
// mod os_initialize_connection_autogen;
// mod os_migrate_volume_30_autogen;
// mod os_migrate_volume_316_autogen;
// mod os_migrate_volume_completion_autogen;
// mod os_reimage_368_autogen;
// mod os_reserve_autogen;
// mod os_reset_status_autogen;
// mod os_retype_autogen;
// mod os_roll_detaching_autogen;
// mod os_set_bootable_autogen;
// mod os_set_image_metadata_autogen;
// mod os_show_image_metadata_autogen;
// mod os_terminate_connection_autogen;
// mod os_unmanage_autogen;
// mod os_unreserve_autogen;
// mod os_unset_image_metadata_autogen;
// mod os_update_readonly_flag_autogen;
// mod os_volume_upload_image_30_autogen;
// mod os_volume_upload_image_31_autogen;
// mod revert_autogen;
mod set_30_autogen;
mod set_353_autogen;
mod show_autogen;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.arg("block-storage").arg("volume").arg("--help");
    cmd.assert().success();

    Ok(())
}
