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

//! Compute Server commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod add_fixed_ip_21;
pub mod add_floating_ip_21;
pub mod add_security_group;
pub mod change_password;
pub mod confirm_resize;
// mod create_20;
pub mod create_21;
pub mod create_219;
pub mod create_232;
pub mod create_233;
pub mod create_237;
pub mod create_242;
pub mod create_252;
pub mod create_257;
pub mod create_263;
pub mod create_267;
pub mod create_274;
pub mod create_290;
pub mod create_294;
pub mod create_backup_21;
pub mod create_image_21;
pub mod delete;
/// Server diagnostics
pub mod diagnostic {
    pub mod get;
}
pub mod evacuate_214;
pub mod evacuate_229;
pub mod evacuate_268;
pub mod evacuate_295;
pub mod force_delete;
pub mod inject_network_info;
pub mod instance_action;
pub mod interface;
pub mod ip;
pub mod list;
pub mod lock_273;
pub mod metadata;
pub mod migrate_256;
pub mod migration;
pub mod os_get_console_output;
// Comment out deprecated APIs
// mod os_get_rdpconsole_21;
// mod os_get_serial_console_21;
// mod os_get_spiceconsole_21;
// mod os_get_vncconsole_21;
pub mod os_migrate_live_20;
pub mod os_migrate_live_225;
pub mod os_migrate_live_230;
pub mod os_migrate_live_268;
pub mod os_reset_state;
pub mod os_start;
pub mod os_stop;
pub mod pause;
pub mod reboot;
pub mod rebuild_21;
pub mod rebuild_219;
pub mod rebuild_254;
pub mod rebuild_257;
pub mod rebuild_263;
pub mod rebuild_290;
pub mod rebuild_294;
pub mod remote_console;
pub mod remove_fixed_ip_21;
pub mod remove_floating_ip_21;
pub mod remove_security_group;
pub mod rescue;
pub mod reset_network;
pub mod resize;
pub mod restore;
pub mod resume;
pub mod revert_resize;
/// Server security group
pub mod security_group {
    pub mod list;
}
pub mod server_password;
// mod set_20;
pub mod set_21;
pub mod set_219;
pub mod set_290;
pub mod set_294;
pub mod shelve;
pub mod shelve_offload;
pub mod show;
pub mod suspend;
pub mod tag;
/// Server topology
pub mod topology {
    pub mod list;
}
pub mod trigger_crash_dump_217;
pub mod unlock_21;
pub mod unpause;
pub mod unrescue;
pub mod unshelve_277;
pub mod unshelve_291;
pub mod volume_attachment;

/// **Servers (servers)**
///
/// Lists, creates, shows details for, updates, and deletes
/// servers.
///
/// **Passwords**
///
/// When you create a server, you can specify a password through
/// the optional adminPass attribute. The password must meet the
/// complexity requirements set by your OpenStack Compute provider.
/// The server might enter an ERROR state if the complexity
/// requirements are not met. In this case, a client might issue a
/// change password action to reset the server password.
///
/// If you do not specify a password, the API generates and assigns
/// a random password that it returns in the response object. This
/// password meets the security requirements set by the compute
/// provider. For security reasons, subsequent GET calls do not
/// require this password.
///
/// **Server metadata**
///
/// You can specify custom server metadata at server launch time.
/// The maximum size for each metadata key-value pair is 255 bytes.
/// The compute provider determines the maximum number of key-value
/// pairs for each server. You can query this value through the
/// maxServerMeta absolute limit.
#[derive(Parser)]
#[command(about = "Servers")]
pub struct ServerCommand {
    /// sucommand
    #[command(subcommand)]
    command: ServerCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ServerCommands {
    #[command(visible_alias = "add-fixed-ip")]
    AddFixedIP21(Box<add_fixed_ip_21::ServerCommand>),
    #[command(visible_alias = "add-floating-ip")]
    AddFloatingIP21(Box<add_floating_ip_21::ServerCommand>),
    AddSecurityGroup(Box<add_security_group::ServerCommand>),
    ChangePassword(Box<change_password::ServerCommand>),
    ConfirmResize(Box<confirm_resize::ServerCommand>),
    #[command(visible_alias = "create")]
    Create294(Box<create_294::ServerCommand>),
    Create290(Box<create_290::ServerCommand>),
    Create274(Box<create_274::ServerCommand>),
    Create267(Box<create_267::ServerCommand>),
    Create263(Box<create_263::ServerCommand>),
    Create257(Box<create_257::ServerCommand>),
    Create252(Box<create_252::ServerCommand>),
    Create242(Box<create_242::ServerCommand>),
    Create237(Box<create_237::ServerCommand>),
    Create233(Box<create_233::ServerCommand>),
    Create232(Box<create_232::ServerCommand>),
    Create219(Box<create_219::ServerCommand>),
    Create21(Box<create_21::ServerCommand>),
    #[command(visible_alias = "create-backup")]
    CreateBackup21(Box<create_backup_21::ServerCommand>),
    #[command(visible_alias = "create-image")]
    CreateImage21(Box<create_image_21::ServerCommand>),
    Delete(Box<delete::ServerCommand>),
    Diagnostic(Box<diagnostic::get::DiagnosticCommand>),
    Evacuate214(Box<evacuate_214::ServerCommand>),
    Evacuate229(Box<evacuate_229::ServerCommand>),
    Evacuate268(Box<evacuate_268::ServerCommand>),
    #[command(visible_alias = "evacuate")]
    Evacuate295(Box<evacuate_295::ServerCommand>),
    ForceDelete(Box<force_delete::ServerCommand>),
    GetConsoleOutput(Box<os_get_console_output::ServerCommand>),
    InstanceAction(Box<instance_action::InstanceActionCommand>),
    Interface(Box<interface::InterfaceCommand>),
    InjectNetworkInfo(Box<inject_network_info::ServerCommand>),
    Ip(Box<ip::IpCommand>),
    List(Box<list::ServersCommand>),
    LiveMigrate20(Box<os_migrate_live_20::ServerCommand>),
    LiveMigrate225(Box<os_migrate_live_225::ServerCommand>),
    LiveMigrate230(Box<os_migrate_live_230::ServerCommand>),
    #[command(visible_alias = "live-migrate")]
    LiveMigrate268(Box<os_migrate_live_268::ServerCommand>),
    #[command(visible_alias = "lock")]
    Lock273(Box<lock_273::ServerCommand>),
    Metadata(Box<metadata::MetadataCommand>),
    #[command(visible_alias = "migrate")]
    Migrate256(Box<migrate_256::ServerCommand>),
    Migration(Box<migration::MigrationCommand>),
    Password(Box<server_password::PasswordCommand>),
    Pause(Box<pause::ServerCommand>),
    ResetState(Box<os_reset_state::ServerCommand>),
    Reboot(Box<reboot::ServerCommand>),
    Rebuild21(Box<rebuild_21::ServerCommand>),
    Rebuild219(Box<rebuild_219::ServerCommand>),
    Rebuild254(Box<rebuild_254::ServerCommand>),
    Rebuild257(Box<rebuild_257::ServerCommand>),
    Rebuild263(Box<rebuild_263::ServerCommand>),
    Rebuild290(Box<rebuild_290::ServerCommand>),
    #[command(visible_alias = "rebuild")]
    Rebuild294(Box<rebuild_294::ServerCommand>),
    RemoteConsole(Box<remote_console::RemoteConsoleCommand>),
    #[command(visible_alias = "remove-fixed-ip")]
    RemoveFixedIP21(Box<remove_fixed_ip_21::ServerCommand>),
    #[command(visible_alias = "remove-floating-ip")]
    RemoveFloatingIP21(Box<remove_floating_ip_21::ServerCommand>),
    RemoveSecurityGroup(Box<remove_security_group::ServerCommand>),
    Rescue(Box<rescue::ServerCommand>),
    ResetNetwork(Box<reset_network::ServerCommand>),
    Resize(Box<resize::ServerCommand>),
    Restore(Box<restore::ServerCommand>),
    Resume(Box<resume::ServerCommand>),
    RevertResize(Box<revert_resize::ServerCommand>),
    SecurityGroups(Box<security_group::list::SecurityGroupsCommand>),
    Set21(Box<set_21::ServerCommand>),
    Set219(Box<set_219::ServerCommand>),
    Set290(Box<set_290::ServerCommand>),
    #[command(visible_alias = "set")]
    Set294(Box<set_294::ServerCommand>),
    Shelve(Box<shelve::ServerCommand>),
    ShelveOffload(Box<shelve_offload::ServerCommand>),
    Show(Box<show::ServerCommand>),
    Start(Box<os_start::ServerCommand>),
    Stop(Box<os_stop::ServerCommand>),
    Suspend(Box<suspend::ServerCommand>),
    Tag(Box<tag::TagCommand>),
    Topology(Box<topology::list::TopologiesCommand>),
    #[command(visible_alias = "trigger-crash-dump")]
    TriggerCrashDump217(Box<trigger_crash_dump_217::ServerCommand>),
    #[command(visible_alias = "unlock")]
    Unlock21(Box<unlock_21::ServerCommand>),
    Unpause(Box<unpause::ServerCommand>),
    Unrescue(Box<unrescue::ServerCommand>),
    Unshelve277(Box<unshelve_277::ServerCommand>),
    #[command(visible_alias = "unshelve")]
    Unshelve291(Box<unshelve_291::ServerCommand>),
    VolumeAttachment(Box<volume_attachment::VolumeAttachmentCommand>),
}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ServerCommands::AddFixedIP21(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::AddFloatingIP21(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::AddSecurityGroup(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::ChangePassword(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::ConfirmResize(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create294(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create290(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create274(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create267(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create263(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create257(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create252(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create242(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create237(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create233(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Create232(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Create219(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Create21(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::CreateBackup21(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::CreateImage21(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Diagnostic(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Evacuate214(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Evacuate229(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Evacuate268(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Evacuate295(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::ForceDelete(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::GetConsoleOutput(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::InstanceAction(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Interface(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::InjectNetworkInfo(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Ip(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::List(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::LiveMigrate20(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::LiveMigrate225(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::LiveMigrate230(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::LiveMigrate268(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Lock273(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Metadata(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Migrate256(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Migration(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Password(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Pause(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::ResetState(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Reboot(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Rebuild21(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Rebuild219(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Rebuild254(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Rebuild257(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Rebuild263(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Rebuild290(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Rebuild294(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::RemoveFixedIP21(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::RemoveFloatingIP21(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::RemoveSecurityGroup(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::RemoteConsole(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Rescue(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::ResetNetwork(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Resize(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Restore(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Resume(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::RevertResize(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::SecurityGroups(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Set21(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Set219(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Set290(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Set294(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Shelve(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::ShelveOffload(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Start(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Stop(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Suspend(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,

            ServerCommands::Topology(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::TriggerCrashDump217(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Unlock21(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Unpause(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Unrescue(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Unshelve277(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::Unshelve291(cmd) => cmd.take_action(parsed_args, session).await,
            ServerCommands::VolumeAttachment(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
