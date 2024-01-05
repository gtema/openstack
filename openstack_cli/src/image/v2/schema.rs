use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod image;
mod images;
mod member;
mod members;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct SchemaArgs {
    #[command(subcommand)]
    command: SchemaCommands,
}

#[derive(Subcommand, Clone)]
pub enum SchemaCommands {
    /// Show Image Schema
    Image(image::ImageArgs),
    /// Show Images Schema
    Images(images::ImagesArgs),
    /// Show Member Schema
    Member(member::MemberArgs),
    /// Show Members Schema
    Members(members::MembersArgs),
}

pub struct SchemaCommand {
    pub args: SchemaArgs,
}

impl ResourceCommands for SchemaCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            SchemaCommands::Image(args) => {
                image::ImageCommand { args: args.clone() }.get_command(session)
            }
            SchemaCommands::Images(args) => {
                images::ImagesCommand { args: args.clone() }.get_command(session)
            }
            SchemaCommands::Member(args) => {
                member::MemberCommand { args: args.clone() }.get_command(session)
            }
            SchemaCommands::Members(args) => {
                members::MembersCommand { args: args.clone() }.get_command(session)
            }
        }
    }
}
