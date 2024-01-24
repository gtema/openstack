//! To authenticate with an application credential, specify
//! “application\_credential” as the auth method. You are not allowed to
//! request a
//! scope, as the scope is retrieved from the application credential.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/auth\_tokens`
//!
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
use openstack_sdk::api::identity::v3::auth::token::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct TokenArgs {
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
    ApplicationCredential,
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

/// UserDomainStructInput Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct UserDomainStructInput {
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
    domain: Option<UserDomainStructInput>,

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

/// ApplicationCredentialUser Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct ApplicationCredentialUser {
    /// The user ID
    #[arg(long)]
    id: Option<String>,

    /// The user name
    #[arg(long)]
    name: Option<String>,

    #[command(flatten)]
    domain: Option<UserDomainStructInput>,
}

/// ApplicationCredential Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct ApplicationCredential {
    #[arg(long)]
    id: Option<String>,

    #[arg(long)]
    name: Option<String>,

    /// The secret for authenticating the application credential.
    #[arg(long, required = false)]
    secret: Option<String>,

    /// A user object, required if an application credential is identified by
    /// name and not ID.
    #[command(flatten)]
    user: Option<ApplicationCredentialUser>,
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

    /// An application credential object.
    #[command(flatten)]
    application_credential: Option<ApplicationCredential>,
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

/// Token create command
pub struct TokenCmd {
    pub args: TokenArgs,
}
/// Token response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// A list of one or two audit IDs. An audit ID is a
    /// unique, randomly generated, URL-safe string that you can use to
    /// track a token. The first audit ID is the current audit ID for the
    /// token. The second audit ID is present for only re-scoped tokens
    /// and is the audit ID from the token before it was re-scoped. A re-
    /// scoped token is one that was exchanged for another token of the
    /// same or different scope. You can use these audit IDs to track the
    /// use of a token or chain of tokens across multiple requests and
    /// endpoints without exposing the token ID to non-privileged users.
    #[serde()]
    #[structable(optional, wide)]
    audit_ids: Option<VecString>,

    /// A `catalog` object.
    #[serde()]
    #[structable(optional)]
    catalog: Option<VecResponseCatalog>,

    /// The date and time when the token expires.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss.sssZ
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58.000000Z`.
    ///
    ///
    /// A `null` value indicates that the token never expires.
    #[serde()]
    #[structable(optional)]
    expires_at: Option<String>,

    /// The date and time when the token was issued.
    #[serde()]
    #[structable(optional)]
    issues_at: Option<String>,

    /// The authentication methods, which are commonly `password`,
    /// `token`, or other methods. Indicates the accumulated set of
    /// authentication methods that were used to obtain the token. For
    /// example, if the token was obtained by password authentication, it
    /// contains `password`. Later, if the token is exchanged by using
    /// the token authentication method one or more times, the
    /// subsequently created tokens contain both `password` and
    /// `token` in their `methods` attribute. Unlike multi-factor
    /// authentication, the `methods` attribute merely indicates the
    /// methods that were used to authenticate the user in exchange for a
    /// token. The client is responsible for determining the total number
    /// of authentication factors.
    #[serde()]
    #[structable(optional)]
    methods: Option<VecString>,

    /// A `user` object.
    #[serde()]
    #[structable(optional)]
    user: Option<ResponseUser>,

    #[serde()]
    #[structable(optional)]
    is_domain: Option<bool>,

    /// A domain object including the id and name representing the domain the
    /// token is scoped to. This is only included in tokens that are scoped to
    /// a domain.
    #[serde()]
    #[structable(optional)]
    domain: Option<ResponseDomainStructResponse>,

    /// A `project` object
    #[serde()]
    #[structable(optional)]
    project: Option<ResponseProject>,

    /// A list of `role` objects
    #[serde()]
    #[structable(optional)]
    roles: Option<VecResponseRoles>,

    /// A `system` object containing information about which parts of the
    /// system
    /// the token is scoped to. If the token is scoped to the entire deployment
    /// system, the `system` object will consist of `{"all": true}`. This is
    /// only included in tokens that are scoped to the system.
    #[serde()]
    #[structable(optional)]
    system: Option<HashMapStringbool>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseEndpoints {
    id: Option<String>,
    interface: Option<String>,
    region: Option<String>,
    url: Option<String>,
}

impl fmt::Display for ResponseEndpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "interface={}",
                self.interface
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "region={}",
                self.region
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "url={}",
                self.url
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseEndpoints(Vec<ResponseEndpoints>);
impl fmt::Display for VecResponseEndpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseCatalog {
    endpoints: Option<VecResponseEndpoints>,
    id: Option<String>,
    _type: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "endpoints={}",
                self.endpoints
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "_type={}",
                self._type
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseCatalog(Vec<ResponseCatalog>);
impl fmt::Display for VecResponseCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseDomain {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseUser {
    id: Option<String>,
    name: Option<String>,
    domain: Option<ResponseDomain>,
    password_expires_at: Option<String>,
    os_federation: Option<HashMapStringValue>,
}

impl fmt::Display for ResponseUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "domain={}",
                self.domain
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "password_expires_at={}",
                self.password_expires_at
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "os_federation={}",
                self.os_federation
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseDomainStructResponse {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseDomainStructResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseProject {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseRoles {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseRoles(Vec<ResponseRoles>);
impl fmt::Display for VecResponseRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringbool(HashMap<String, bool>);
impl fmt::Display for HashMapStringbool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[async_trait]
impl Command for TokenCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Token with {:?}", self.args);

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
                let mut sub = create::UserDomainStructInputBuilder::default();
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
        if let Some(val) = &&args.identity.application_credential {
            let mut sub = create::ApplicationCredentialBuilder::default();
            if let Some(val) = &val.id {
                sub.id(val);
            }
            if let Some(val) = &val.name {
                sub.name(val);
            }

            sub.secret(&args.secret);
            if let Some(val) = &val.user {
                let mut sub = create::ApplicationCredentialUserBuilder::default();
                if let Some(val) = &val.id {
                    sub.id(val);
                }
                if let Some(val) = &val.name {
                    sub.name(val);
                }
                if let Some(val) = &val.domain {
                    let mut sub = create::UserDomainStructInputBuilder::default();
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
            sub.application_credential(sub.build().expect("A valid object"));
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

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
