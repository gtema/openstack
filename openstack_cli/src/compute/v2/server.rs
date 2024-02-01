//! Compute Server commands
#![deny(missing_docs)]
use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod add_fixed_ip_21;
pub mod add_floating_ip_21;
pub mod add_security_group;
pub mod change_password;
pub mod confirm_resize;
pub mod create_20;
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
pub mod os_get_rdpconsole_21;
pub mod os_get_serial_console_21;
pub mod os_get_spiceconsole_21;
pub mod os_get_vncconsole_21;
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
pub mod security_group {
    pub mod list;
}
pub mod server_password;
pub mod set_20;
pub mod set_21;
pub mod set_219;
pub mod set_290;
pub mod set_294;
pub mod shelve;
pub mod shelve_offload;
pub mod show;
pub mod suspend;
pub mod tag;
pub mod topology {
    pub mod list;
}
pub mod trigger_crash_dump_217;
pub mod unlock;
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
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
#[command(about = "Servers")]
pub struct ServerArgs {
    #[command(subcommand)]
    command: ServerCommands,
}

#[derive(Subcommand, Clone)]
pub enum ServerCommands {
    AddFixedIP(Box<add_fixed_ip_21::ServerArgs>),
    AddFloatingIP(Box<add_floating_ip_21::ServerArgs>),
    AddSecurityGroup(Box<add_security_group::ServerArgs>),
    ChangePassword(Box<change_password::ServerArgs>),
    ConfirmResize(Box<confirm_resize::ServerArgs>),
    #[command(visible_alias = "create")]
    Create294(Box<create_294::ServerArgs>),
    Create290(Box<create_290::ServerArgs>),
    Create274(Box<create_274::ServerArgs>),
    Create267(Box<create_267::ServerArgs>),
    Create263(Box<create_263::ServerArgs>),
    Create257(Box<create_257::ServerArgs>),
    Create252(Box<create_252::ServerArgs>),
    Create242(Box<create_242::ServerArgs>),
    Create237(Box<create_237::ServerArgs>),
    Create233(Box<create_233::ServerArgs>),
    Create232(Box<create_232::ServerArgs>),
    Create219(Box<create_219::ServerArgs>),
    Create21(Box<create_21::ServerArgs>),
    CreateBackup(Box<create_backup_21::ServerArgs>),
    CreateImage(Box<create_image_21::ServerArgs>),
    Delete(Box<delete::ServerArgs>),
    Diagnostic(Box<diagnostic::get::DiagnosticArgs>),
    Evacuate214(Box<evacuate_214::ServerArgs>),
    Evacuate229(Box<evacuate_229::ServerArgs>),
    Evacuate268(Box<evacuate_268::ServerArgs>),
    #[command(visible_alias = "evacuate")]
    Evacuate295(Box<evacuate_295::ServerArgs>),
    ForceDelete(Box<force_delete::ServerArgs>),
    GetConsoleOutput(Box<os_get_console_output::ServerArgs>),
    InstanceAction(Box<instance_action::InstanceActionArgs>),
    Interface(Box<interface::InterfaceArgs>),
    InjectNetworkInfo(Box<inject_network_info::ServerArgs>),
    Ip(Box<ip::IpArgs>),
    List(Box<list::ServersArgs>),
    LiveMigrate20(Box<os_migrate_live_20::ServerArgs>),
    LiveMigrate225(Box<os_migrate_live_225::ServerArgs>),
    LiveMigrate230(Box<os_migrate_live_230::ServerArgs>),
    #[command(visible_alias = "live-migrate")]
    LiveMigrate268(Box<os_migrate_live_268::ServerArgs>),
    Lock(Box<lock_273::ServerArgs>),
    Metadata(Box<metadata::MetadataArgs>),
    Migrate(Box<migrate_256::ServerArgs>),
    Migration(Box<migration::MigrationArgs>),
    Password(Box<server_password::PasswordArgs>),
    Pause(Box<pause::ServerArgs>),
    ResetState(Box<os_reset_state::ServerArgs>),
    Reboot(Box<reboot::ServerArgs>),
    Rebuild21(Box<rebuild_21::ServerArgs>),
    Rebuild219(Box<rebuild_219::ServerArgs>),
    Rebuild254(Box<rebuild_254::ServerArgs>),
    Rebuild257(Box<rebuild_257::ServerArgs>),
    Rebuild263(Box<rebuild_263::ServerArgs>),
    Rebuild290(Box<rebuild_290::ServerArgs>),
    #[command(visible_alias = "rebuild")]
    Rebuild294(Box<rebuild_294::ServerArgs>),
    RemoteConsole(Box<remote_console::RemoteConsoleArgs>),
    RemoveFixedIP(Box<remove_fixed_ip_21::ServerArgs>),
    RemoveFloatingIP(Box<remove_floating_ip_21::ServerArgs>),
    RemoveSecurityGroup(Box<remove_security_group::ServerArgs>),
    Rescue(Box<rescue::ServerArgs>),
    ResetNetwork(Box<reset_network::ServerArgs>),
    Resize(Box<resize::ServerArgs>),
    Restore(Box<restore::ServerArgs>),
    Resume(Box<resume::ServerArgs>),
    RevertResize(Box<revert_resize::ServerArgs>),
    SecurityGroups(Box<security_group::list::SecurityGroupsArgs>),
    Set21(Box<set_21::ServerArgs>),
    Set219(Box<set_219::ServerArgs>),
    Set290(Box<set_290::ServerArgs>),
    #[command(visible_alias = "set")]
    Set294(Box<set_294::ServerArgs>),
    Shelve(Box<shelve::ServerArgs>),
    ShelveOffload(Box<shelve_offload::ServerArgs>),
    Show(Box<show::ServerArgs>),
    Start(Box<os_start::ServerArgs>),
    Stop(Box<os_stop::ServerArgs>),
    Suspend(Box<suspend::ServerArgs>),
    Tag(Box<tag::TagArgs>),
    Topology(Box<topology::list::TopologiesArgs>),
    TriggerCrashDump(Box<trigger_crash_dump_217::ServerArgs>),
    Unlock(Box<unlock::ServerArgs>),
    Unpause(Box<unpause::ServerArgs>),
    Unrescue(Box<unrescue::ServerArgs>),
    Unshelve277(Box<unshelve_277::ServerArgs>),
    #[command(visible_alias = "unshelve")]
    Unshelve291(Box<unshelve_291::ServerArgs>),
    VolumeAttachment(Box<volume_attachment::VolumeAttachmentArgs>),
}

pub struct ServerCommand {
    pub args: ServerArgs,
}

impl OSCCommand for ServerCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ServerCommands::AddFixedIP(args) => Ok(Box::new(add_fixed_ip_21::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::AddFloatingIP(args) => Ok(Box::new(add_floating_ip_21::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::AddSecurityGroup(args) => Ok(Box::new(add_security_group::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::ChangePassword(args) => Ok(Box::new(change_password::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::ConfirmResize(args) => Ok(Box::new(confirm_resize::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create294(args) => Ok(Box::new(create_294::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create290(args) => Ok(Box::new(create_290::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create274(args) => Ok(Box::new(create_274::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create267(args) => Ok(Box::new(create_267::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create263(args) => Ok(Box::new(create_263::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create257(args) => Ok(Box::new(create_257::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create252(args) => Ok(Box::new(create_252::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create242(args) => Ok(Box::new(create_242::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create237(args) => Ok(Box::new(create_237::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create233(args) => Ok(Box::new(create_233::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create232(args) => Ok(Box::new(create_232::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create219(args) => Ok(Box::new(create_219::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Create21(args) => Ok(Box::new(create_21::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::CreateBackup(args) => Ok(Box::new(create_backup_21::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::CreateImage(args) => Ok(Box::new(create_image_21::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Delete(args) => Ok(Box::new(delete::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Diagnostic(args) => Ok(Box::new(diagnostic::get::DiagnosticCmd {
                args: *args.clone(),
            })),
            ServerCommands::Evacuate214(args) => Ok(Box::new(evacuate_214::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Evacuate229(args) => Ok(Box::new(evacuate_229::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Evacuate268(args) => Ok(Box::new(evacuate_268::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Evacuate295(args) => Ok(Box::new(evacuate_295::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::ForceDelete(args) => Ok(Box::new(force_delete::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::GetConsoleOutput(args) => {
                Ok(Box::new(os_get_console_output::ServerCmd {
                    args: *args.clone(),
                }))
            }
            ServerCommands::InstanceAction(args) => instance_action::InstanceActionCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ServerCommands::Interface(args) => interface::InterfaceCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ServerCommands::InjectNetworkInfo(args) => {
                Ok(Box::new(inject_network_info::ServerCmd {
                    args: *args.clone(),
                }))
            }
            ServerCommands::Ip(args) => ip::IpCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ServerCommands::List(args) => Ok(Box::new(list::ServersCmd {
                args: *args.clone(),
            })),
            ServerCommands::LiveMigrate20(args) => Ok(Box::new(os_migrate_live_20::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::LiveMigrate225(args) => Ok(Box::new(os_migrate_live_225::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::LiveMigrate230(args) => Ok(Box::new(os_migrate_live_230::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::LiveMigrate268(args) => Ok(Box::new(os_migrate_live_268::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Lock(args) => Ok(Box::new(lock_273::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Metadata(args) => metadata::MetadataCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ServerCommands::Migrate(args) => Ok(Box::new(migrate_256::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Migration(args) => migration::MigrationCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ServerCommands::Password(args) => server_password::PasswordCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ServerCommands::Pause(args) => Ok(Box::new(pause::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::ResetState(args) => Ok(Box::new(os_reset_state::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Reboot(args) => Ok(Box::new(reboot::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Rebuild21(args) => Ok(Box::new(rebuild_21::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Rebuild219(args) => Ok(Box::new(rebuild_219::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Rebuild254(args) => Ok(Box::new(rebuild_254::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Rebuild257(args) => Ok(Box::new(rebuild_257::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Rebuild263(args) => Ok(Box::new(rebuild_263::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Rebuild290(args) => Ok(Box::new(rebuild_290::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Rebuild294(args) => Ok(Box::new(rebuild_294::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::RemoveFixedIP(args) => Ok(Box::new(remove_fixed_ip_21::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::RemoveFloatingIP(args) => {
                Ok(Box::new(remove_floating_ip_21::ServerCmd {
                    args: *args.clone(),
                }))
            }
            ServerCommands::RemoveSecurityGroup(args) => {
                Ok(Box::new(remove_security_group::ServerCmd {
                    args: *args.clone(),
                }))
            }
            ServerCommands::RemoteConsole(args) => remote_console::RemoteConsoleCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ServerCommands::Rescue(args) => Ok(Box::new(rescue::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::ResetNetwork(args) => Ok(Box::new(reset_network::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Resize(args) => Ok(Box::new(resize::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Restore(args) => Ok(Box::new(restore::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Resume(args) => Ok(Box::new(resume::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::RevertResize(args) => Ok(Box::new(revert_resize::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::SecurityGroups(args) => {
                Ok(Box::new(security_group::list::SecurityGroupsCmd {
                    args: *args.clone(),
                }))
            }
            ServerCommands::Set21(args) => Ok(Box::new(set_21::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Set219(args) => Ok(Box::new(set_219::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Set290(args) => Ok(Box::new(set_290::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Set294(args) => Ok(Box::new(set_294::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Shelve(args) => Ok(Box::new(shelve::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::ShelveOffload(args) => Ok(Box::new(shelve_offload::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Show(args) => Ok(Box::new(show::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Start(args) => Ok(Box::new(os_start::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Stop(args) => Ok(Box::new(os_stop::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Suspend(args) => Ok(Box::new(suspend::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Tag(args) => tag::TagCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ServerCommands::Topology(args) => Ok(Box::new(topology::list::TopologiesCmd {
                args: *args.clone(),
            })),
            ServerCommands::TriggerCrashDump(args) => {
                Ok(Box::new(trigger_crash_dump_217::ServerCmd {
                    args: *args.clone(),
                }))
            }
            ServerCommands::Unlock(args) => Ok(Box::new(unlock::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Unpause(args) => Ok(Box::new(unpause::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Unrescue(args) => Ok(Box::new(unrescue::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Unshelve277(args) => Ok(Box::new(unshelve_277::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Unshelve291(args) => Ok(Box::new(unshelve_291::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::VolumeAttachment(args) => volume_attachment::VolumeAttachmentCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
        }
    }
}
