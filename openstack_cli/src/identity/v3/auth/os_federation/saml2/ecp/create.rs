//! Exchange a scoped token for an ECP assertion.
//!
//! POST /v3/auth/OS-FEDERATION/saml2/ecp
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use clap::ValueEnum;
use dialoguer::Password;
use openstack_sdk::api::identity::v3::auth::os_federation::saml2::ecp::create;
use openstack_sdk::api::RawQueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct EcpArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    auth: Auth,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Methods {
    Password,
    Token,
    Totp,
}

/// Domain Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct Domain {
    /// User Domain ID
    #[arg(long)]
    id: Option<String>,

    /// User Domain Name
    #[arg(long)]
    name: Option<String>,
}

/// User Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct User {
    /// The ID of the application credential used for authentication. If not
    /// provided, the application credential must be identified by its name and
    /// its owning user.
    #[arg(long)]
    id: Option<String>,

    /// The name of the application credential used for authentication. If
    /// provided, must be accompanied by a user object.
    #[arg(long)]
    name: Option<String>,

    /// User Password
    #[arg(long)]
    password: Option<String>,

    /// A `domain` object
    #[command(flatten)]
    domain: Option<Domain>,
}

/// Password Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct Password {
    /// A `user` object, required if an application credential is identified by
    /// name and not ID.
    #[command(flatten)]
    user: Option<User>,
}

/// Token Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct Token {
    /// Authorization Token value
    #[arg(long, required = false)]
    id: Option<String>,
}

/// UserDomain Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct UserDomain {
    #[arg(long)]
    id: Option<String>,

    #[arg(long)]
    name: Option<String>,
}

/// TotpUser Body data
#[derive(Args, Debug, Clone)]
#[group(required = true, multiple = true)]
struct TotpUser {
    /// The user ID
    #[arg(long)]
    id: Option<String>,

    /// The user name
    #[arg(long)]
    name: Option<String>,

    #[command(flatten)]
    domain: Option<UserDomain>,

    /// MFA passcode
    #[arg(long, required = false)]
    passcode: Option<String>,
}

/// Totp Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct Totp {
    #[command(flatten)]
    user: TotpUser,
}

/// Identity Body data
#[derive(Args, Debug, Clone)]
#[group(required = true, multiple = true)]
struct Identity {
    /// The authentication method. To authenticate with an application
    /// credential,
    /// specify `application\_credential`.
    #[arg(action=clap::ArgAction::Append, long, required=false)]
    methods: Vec<Methods>,

    /// The `password` object, contains the authentication information.
    #[command(flatten)]
    password: Option<Password>,

    /// A `token` object. The token authentication
    /// method is used. This method is typically used in combination with
    /// a request to change authorization scope.
    #[command(flatten)]
    token: Option<Token>,

    /// Multi Factor Authentication information
    #[command(flatten)]
    totp: Option<Totp>,
}

/// ProjectDomain Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct ProjectDomain {
    /// Project domain Id
    #[arg(long)]
    id: Option<String>,

    /// Project domain name
    #[arg(long)]
    name: Option<String>,
}

/// Project Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct Project {
    /// Project Name
    #[arg(long)]
    name: Option<String>,

    /// Project Id
    #[arg(long)]
    id: Option<String>,

    #[command(flatten)]
    domain: Option<ProjectDomain>,
}

/// ScopeDomain Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct ScopeDomain {
    /// Domain id
    #[arg(long)]
    id: Option<String>,

    /// Domain name
    #[arg(long)]
    name: Option<String>,
}

/// OsTrustTrust Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct OsTrustTrust {
    #[arg(long)]
    id: Option<String>,
}

/// System Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct System {
    #[arg(action=clap::ArgAction::Set, long)]
    all: Option<bool>,
}

/// Scope Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct Scope {
    #[command(flatten)]
    project: Option<Project>,

    #[command(flatten)]
    domain: Option<ScopeDomain>,

    #[command(flatten)]
    os_trust_trust: Option<OsTrustTrust>,

    #[command(flatten)]
    system: Option<System>,
}

/// Auth Body data
#[derive(Args, Debug, Clone)]
struct Auth {
    /// An `identity` object.
    #[command(flatten)]
    identity: Identity,

    /// The authorization scope (Since v3.4). Specify
    /// `unscoped` to make an explicit unscoped token request, which
    /// returns an unscoped response without any authorization. This
    /// request behaves the same as a token request with no scope where
    /// the user has no default project defined. If an explicit,
    /// `unscoped` token request is not made and the user has
    /// authorization to their default project, then the response will
    /// return a project-scoped token. If a default project is not defined,
    /// a token is issued without an explicit scope of authorization,
    /// which is the same as asking for an explicit unscoped token.
    #[command(flatten)]
    scope: Option<Scope>,
}

/// Ecp create command
pub struct EcpCmd {
    pub args: EcpArgs,
}
/// Ecp response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for EcpCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Ecp with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.auth data
        let args = &self.args.auth;
        let mut auth_builder = create::AuthBuilder::default();

        let mut sub = create::IdentityBuilder::default();

        sub.methods(&args.methods.iter().map(|v| v.into()).collect::<Vec<_>>());
        if let Some(val) = &&args.identity.password {
            let mut sub = create::PasswordBuilder::default();
            if let Some(val) = &val.user {
                let mut sub = create::UserBuilder::default();
                if let Some(val) = &val.id {
                    sub.id(val);
                }
                if let Some(val) = &val.name {
                    sub.name(val);
                }
                if let Some(val) = &val.password {
                    sub.password(val);
                }
                if let Some(val) = &val.domain {
                    let mut sub = create::DomainBuilder::default();
                    if let Some(val) = &val.id {
                        sub.id(val);
                    }
                    if let Some(val) = &val.name {
                        sub.name(val);
                    }
                    sub.domain(sub.build().expect("A valid object"));
                }
                sub.user(sub.build().expect("A valid object"));
            }
            sub.password(sub.build().expect("A valid object"));
        }
        if let Some(val) = &&args.identity.token {
            let mut sub = create::TokenBuilder::default();

            sub.id(&args.id);
            sub.token(sub.build().expect("A valid object"));
        }
        if let Some(val) = &&args.identity.totp {
            let mut sub = create::TotpBuilder::default();

            let mut sub = create::TotpUserBuilder::default();
            if let Some(val) = &&args.user.id {
                sub.id(val);
            }
            if let Some(val) = &&args.user.name {
                sub.name(val);
            }
            if let Some(val) = &&args.user.domain {
                let mut sub = create::UserDomainBuilder::default();
                if let Some(val) = &val.id {
                    sub.id(val);
                }
                if let Some(val) = &val.name {
                    sub.name(val);
                }
                sub.domain(sub.build().expect("A valid object"));
            }

            sub.passcode(&args.passcode);
            sub.user(sub.build().expect("A valid object"));
            sub.totp(sub.build().expect("A valid object"));
        }
        auth_builder.identity(sub.build().expect("A valid object"));

        if let Some(val) = &args.scope {
            let mut sub = create::ScopeBuilder::default();
            if let Some(val) = &val.project {
                let mut sub = create::ProjectBuilder::default();
                if let Some(val) = &val.name {
                    sub.name(val);
                }
                if let Some(val) = &val.id {
                    sub.id(val);
                }
                if let Some(val) = &val.domain {
                    let mut sub = create::ProjectDomainBuilder::default();
                    if let Some(val) = &val.id {
                        sub.id(val);
                    }
                    if let Some(val) = &val.name {
                        sub.name(val);
                    }
                    sub.domain(sub.build().expect("A valid object"));
                }
                sub.project(sub.build().expect("A valid object"));
            }
            if let Some(val) = &val.domain {
                let mut sub = create::ScopeDomainBuilder::default();
                if let Some(val) = &val.id {
                    sub.id(val);
                }
                if let Some(val) = &val.name {
                    sub.name(val);
                }
                sub.domain(sub.build().expect("A valid object"));
            }
            if let Some(val) = &val.os_trust_trust {
                let mut sub = create::OsTrustTrustBuilder::default();
                if let Some(val) = &val.id {
                    sub.id(val);
                }
                sub.os_trust_trust(sub.build().expect("A valid object"));
            }
            if let Some(val) = &val.system {
                let mut sub = create::SystemBuilder::default();
                if let Some(val) = &val.all {
                    sub.all(*val);
                }
                sub.system(sub.build().expect("A valid object"));
            }
            auth_builder.scope(sub.build().expect("A valid object"));
        }

        ep_builder.auth(auth_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
