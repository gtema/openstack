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

mod add_fixed_ip_21_autogen;
mod add_floating_ip_21_autogen;
mod add_security_group_autogen;
mod change_password_autogen;
mod confirm_resize_autogen;
// mod create_20_autogen;
mod create_219_autogen;
mod create_21_autogen;
mod create_232_autogen;
mod create_233_autogen;
mod create_237_autogen;
mod create_242_autogen;
mod create_252_autogen;
mod create_257_autogen;
mod create_263_autogen;
mod create_267_autogen;
mod create_274_autogen;
mod create_290_autogen;
mod create_294_autogen;
// mod create_backup_20_autogen;
mod create_backup_21_autogen;
// mod create_image_20_autogen;
mod create_image_21_autogen;
mod delete_autogen;
mod diagnostic;
// mod evacuate_20_autogen;
mod evacuate_214_autogen;
mod evacuate_229_autogen;
mod evacuate_268_autogen;
mod evacuate_295_autogen;
mod force_delete_autogen;
mod inject_network_info_autogen;
mod instance_action;
mod interface;
mod ip;
mod list;
mod list_autogen;
mod lock_273_autogen;
mod metadata;
mod migrate_256_autogen;
mod migration;
mod os_get_console_output_autogen;
// mod os_get_rdpconsole_21_autogen;
// mod os_get_serial_console_21_autogen;
// mod os_get_spiceconsole_21_autogen;
// mod os_get_vncconsole_21_autogen;
mod os_migrate_live_20_autogen;
mod os_migrate_live_225_autogen;
mod os_migrate_live_230_autogen;
mod os_migrate_live_268_autogen;
mod os_reset_state_autogen;
mod os_start_autogen;
mod os_stop_autogen;
mod pause_autogen;
mod reboot_autogen;
// mod rebuild_20_autogen;
mod rebuild_219_autogen;
mod rebuild_21_autogen;
mod rebuild_254_autogen;
mod rebuild_257_autogen;
mod rebuild_263_autogen;
mod rebuild_290_autogen;
mod rebuild_294_autogen;
mod remote_console;
mod remove_fixed_ip_21_autogen;
mod remove_floating_ip_21_autogen;
mod remove_security_group_autogen;
mod rescue_autogen;
mod reset_network_autogen;
mod resize_autogen;
mod restore_autogen;
mod resume_autogen;
mod revert_resize_autogen;
mod security_group;
mod server_password;
// mod set_20_autogen;
mod set_219_autogen;
mod set_21_autogen;
mod set_290_autogen;
mod set_294_autogen;
mod shelve_autogen;
mod shelve_offload_autogen;
mod show_autogen;
mod suspend_autogen;
mod tag;
mod topology;
mod trigger_crash_dump_217_autogen;
mod unlock_autogen;
mod unpause_autogen;
mod unrescue_autogen;
mod unshelve_277_autogen;
mod unshelve_291_autogen;
mod volume_attachment;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.arg("compute").arg("server").arg("--help");
    cmd.assert().success();

    Ok(())
}
