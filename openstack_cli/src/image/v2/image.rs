use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod create;
mod deactivate;
mod delete;
mod download;
mod list;
mod reactivate;
mod set;
mod show;
mod upload;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImageArgs {
    #[command(subcommand)]
    command: ImageCommands,
}

#[derive(Subcommand, Clone)]
pub enum ImageCommands {
    /// List Images
    List(list::ImagesArgs),
    /// Show single Image
    Show(show::ImageArgs),
    /// Create Image
    Create(create::ImageArgs),
    /// Update Image
    Set(set::ImageArgs),
    /// Download Image
    Download(download::ImageArgs),
    /// Upload Image
    Upload(upload::ImageArgs),
    /// Delete Image
    Delete(delete::ImageArgs),
    /// Deactivate Image
    Deactivate(deactivate::ImageArgs),
    /// Reactivate Image
    Reactivate(reactivate::ImageArgs),
}

pub struct ImageCommand {
    pub args: ImageArgs,
}

impl ResourceCommands for ImageCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ImageCommands::List(args) => Box::new(list::ImagesCmd { args: args.clone() }),
            ImageCommands::Show(args) => Box::new(show::ImageCmd { args: args.clone() }),
            ImageCommands::Set(args) => Box::new(set::ImageCmd { args: args.clone() }),
            ImageCommands::Download(args) => Box::new(download::ImageCmd { args: args.clone() }),
            ImageCommands::Upload(args) => Box::new(upload::ImageCmd { args: args.clone() }),
            ImageCommands::Create(args) => Box::new(create::ImageCmd { args: args.clone() }),
            ImageCommands::Delete(args) => Box::new(delete::ImageCmd { args: args.clone() }),
            ImageCommands::Deactivate(args) => {
                Box::new(deactivate::ImageCmd { args: args.clone() })
            }
            ImageCommands::Reactivate(args) => {
                Box::new(reactivate::ImageCmd { args: args.clone() })
            }
        }
    }
}
