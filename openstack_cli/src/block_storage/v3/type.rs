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

//! Block storage VolumeType Type commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod add_project_access;
pub mod create;
pub mod delete;
pub mod encryption;
pub mod extra_spec;
pub mod list;
pub mod remove_project_access;
pub mod set;
pub mod show;

/// Block Storage VolumeType type commands
///
/// To create an environment with multiple-storage back ends, you must specify a volume type. The
/// API spawns Block Storage volume back ends as children to cinder-volume, and keys them from a
/// unique queue. The API names the back ends cinder-volume.HOST.BACKEND. For example,
/// cinder-volume.ubuntu.lvmdriver. When you create a volume, the scheduler chooses an appropriate
/// back end for the volume type to handle the request.
///
/// For information about how to use volume types to create multiple- storage back ends, see
/// [Configure multiple-storage back
/// ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html).
#[derive(Parser)]
pub struct VolumeTypeCommand {
    /// subcommand
    #[command(subcommand)]
    command: VolumeTypeCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum VolumeTypeCommands {
    AddProjectAccess(Box<add_project_access::TypeCommand>),
    Create(Box<create::TypeCommand>),
    Delete(Box<delete::TypeCommand>),
    Encryption(Box<encryption::EncryptionCommand>),
    Extraspecs(Box<extra_spec::ExtraSpecsCommand>),
    List(Box<list::TypesCommand>),
    RemoveProjectAccess(Box<remove_project_access::TypeCommand>),
    Set(Box<set::TypeCommand>),
    Show(Box<show::TypeCommand>),
}

impl VolumeTypeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            VolumeTypeCommands::AddProjectAccess(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            VolumeTypeCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTypeCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTypeCommands::Encryption(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTypeCommands::Extraspecs(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTypeCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTypeCommands::RemoveProjectAccess(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            VolumeTypeCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTypeCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
