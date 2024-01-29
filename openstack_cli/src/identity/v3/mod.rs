pub mod project;
pub mod user;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::identity::v3::project::{ProjectArgs, ProjectCommand};
use crate::identity::v3::user::access_rule::{AccessRuleArgs, AccessRuleCommand};
use crate::identity::v3::user::application_credential::{
    ApplicationCredentialArgs, ApplicationCredentialCommand,
};
use crate::identity::v3::user::{UserArgs, UserCommand};
use crate::{OSCCommand, OpenStackCliError};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct IdentitySrvArgs {
    /// Identity service resource
    #[command(subcommand)]
    command: IdentitySrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum IdentitySrvCommands {
    /// **Application Credentials**
    ///
    /// Application credentials provide a way to delegate a
    /// user’s authorization to an application without
    /// sharing the user’s password authentication. This is
    /// a useful security measure, especially for
    /// situations where the user’s identification is
    /// provided by an external source, such as LDAP or a
    /// single-sign-on service. Instead of storing user
    /// passwords in config files, a user creates an
    /// application credential for a specific project, with
    /// all or a subset of the role assignments they have
    /// on that project, and then stores the application
    /// credential identifier and secret in the config
    /// file.
    ///
    /// Multiple application credentials may be active at
    /// once, so you can easily rotate application
    /// credentials by creating a second one, converting
    /// your applications to use it one by one, and finally
    /// deleting the first one.
    ///
    /// Application credentials are limited by the lifespan
    /// of the user that created them. If the user is
    /// deleted, disabled, or loses a role assignment on a
    /// project, the application credential is deleted.
    ///
    /// Application credentials can have their privileges
    /// limited in two ways. First, the owner may specify a
    /// subset of their own roles that the application
    /// credential may assume when getting a token for a
    /// project. For example, if a user has the member role
    /// on a project, they also have the implied role
    /// reader and can grant the application credential
    /// only the reader role for the project:
    ///
    /// "roles": [ {"name": "reader"} ]
    ///
    /// Users also have the option of delegating more
    /// fine-grained access control to their application
    /// credentials by using access rules. For example, to
    /// create an application credential that is
    /// constricted to creating servers in nova, the user
    /// can add the following access rules:
    ///
    /// "access_rules": [ { "path": "/v2.1/servers",
    /// "method": "POST", "service": "compute" } ]
    ///
    /// The "path" attribute of application credential
    /// access rules uses a wildcard syntax to make it more
    /// flexible. For example, to create an application
    /// credential that is constricted to listing server IP
    /// addresses, you could use either of the following
    /// access rules:
    ///
    /// "access_rules": [ { "path": "/v2.1/servers/*/ips",
    /// "method": "GET", "service": "compute" } ]
    ///
    /// or equivalently:
    ///
    /// "access_rules": [ { "path":
    /// "/v2.1/servers/{server_id}/ips", "method": "GET",
    /// "service": "compute" } ]
    ///
    /// In both cases, a request path containing any server
    /// ID will match the access rule. For even more
    /// flexibility, the recursive wildcard ** indicates
    /// that request paths containing any number of / will
    /// be matched. For example:
    ///
    /// "access_rules": [ { "path": "/v2.1/**", "method":
    /// "GET", "service": "compute" } ]
    ///
    /// will match any nova API for version 2.1.
    ///
    /// An access rule created for one application
    /// credential can be re-used by providing its ID to
    /// another application credential, for example:
    ///
    /// "access_rules": [ { "id": "abcdef" } ]
    ///
    ///
    ApplicationCredential(ApplicationCredentialArgs),
    /// **Application Credentials - Access Rules**
    ///
    /// Users also have the option of delegating more fine-grained access
    /// control to their application credentials by using access rules. For
    /// example, to create an application credential that is constricted to
    /// creating servers in nova, the user can add the following access
    /// rules:
    ///
    /// ```json { "access_rules": [{ "path": "/v2.1/servers", "method":
    /// "POST", "service": "compute" }] } ```
    ///
    /// The "path" attribute of application credential access rules uses a
    /// wildcard syntax to make it more flexible. For example, to create an
    /// application credential that is constricted to listing server IP
    /// addresses, you could use either of the following access rules:
    ///
    /// ```json { "access_rules": [ { "path": "/v2.1/servers/*/ips",
    /// "method": "GET", "service": "compute" } ] } ```
    ///
    /// or equivalently:
    ///
    /// ```json { "access_rules": [ { "path":
    /// "/v2.1/servers/{server_id}/ips", "method": "GET", "service":
    /// "compute" } ] } ```
    ///
    /// In both cases, a request path containing any server ID will match the
    /// access rule. For even more flexibility, the recursive wildcard **
    /// indicates that request paths containing any number of / will be
    /// matched. For example:
    ///
    /// ```json { "access_rules": [ { "path": "/v2.1/**", "method": "GET",
    /// "service": "compute" } ] } ```
    ///
    /// will match any nova API for version 2.1.
    ///
    /// An access rule created for one application credential can be re-used
    /// by providing its ID to another application credential, for example:
    ///
    /// ```json { "access_rules": [ { "id": "abcdef" } ] } ```
    AccessRule(AccessRuleArgs),
    /// Project commands
    Project(ProjectArgs),
    /// User commands
    ///
    /// A user is an individual API consumer that is owned by a domain. A
    /// role explicitly associates a user with projects or domains. A user
    /// with no assigned roles has no access to OpenStack resources.
    ///
    /// You can list, create, show details for, update, delete, and change
    /// the password for users.
    ///
    /// You can also list groups, projects, and role assignments for a
    /// specified user.
    User(UserArgs),
}

pub struct IdentitySrvCommand {
    pub args: IdentitySrvArgs,
}

impl OSCCommand for IdentitySrvCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            IdentitySrvCommands::AccessRule(args) => {
                AccessRuleCommand { args: args.clone() }.get_subcommand(session)
            }
            IdentitySrvCommands::ApplicationCredential(args) => {
                ApplicationCredentialCommand { args: args.clone() }.get_subcommand(session)
            }
            IdentitySrvCommands::Project(args) => {
                ProjectCommand { args: args.clone() }.get_subcommand(session)
            }
            IdentitySrvCommands::User(args) => {
                UserCommand { args: args.clone() }.get_subcommand(session)
            }
        }
    }
}
