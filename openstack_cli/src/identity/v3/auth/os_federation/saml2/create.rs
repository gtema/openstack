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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Create Saml2 command
//!
//! Wraps invoking of the `v3/auth/OS-FEDERATION/saml2` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_json;
use bytes::Bytes;
use clap::ValueEnum;
use dialoguer::Password;
use http::Response;
use openstack_sdk::api::identity::v3::auth::os_federation::saml2::create;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// A user may generate a SAML assertion document based on the scoped token
/// that is used in the request.
///
/// Request Parameters:
///
/// To generate a SAML assertion, a user must provides a scoped token ID and
/// Service Provider ID in the request body.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/ext/OS-FEDERATION/1.0/rel/saml2`
///
#[derive(Args)]
#[command(about = "Generate a SAML assertion")]
pub struct Saml2Command {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Auth data with user’s identity and Service Provider scope information
    ///
    #[command(flatten)]
    auth: Auth,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Methods {
    ApplicationCredential,
    Password,
    Token,
    Totp,
}

/// User Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct User {
    /// A `domain` object
    ///
    #[arg(help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    domain: Option<Value>,

    /// The ID of the user. Required if you do not specify the user name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// The user name. Required if you do not specify the ID of the user. If
    /// you specify the user name, you must also specify the domain, by ID or
    /// name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// User Password
    ///
    #[arg(help_heading = "Body parameters", long)]
    password: Option<String>,
}

/// Password Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct Password {
    /// A `user` object.
    ///
    #[command(flatten)]
    user: Option<User>,
}

/// Token Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct Token {
    /// Authorization Token value
    ///
    #[arg(help_heading = "Body parameters", long, required = false)]
    id: Option<String>,
}

/// TotpUser Body data
#[derive(Args, Clone)]
#[group(required = true, multiple = true)]
struct TotpUser {
    /// A `domain` object
    ///
    #[arg(help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    domain: Option<Value>,

    /// The user ID
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// The user name
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// MFA passcode
    ///
    #[arg(help_heading = "Body parameters", long, required = false)]
    passcode: Option<String>,
}

/// Totp Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct Totp {
    #[command(flatten)]
    user: TotpUser,
}

/// ApplicationCredential Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct ApplicationCredential {
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The secret for authenticating the application credential.
    ///
    #[arg(help_heading = "Body parameters", long, required = false)]
    secret: Option<String>,

    /// A user object, required if an application credential is identified by
    /// name and not ID.
    ///
    #[arg(help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    user: Option<Value>,
}

/// Identity Body data
#[derive(Args, Clone)]
#[group(required = true, multiple = true)]
struct Identity {
    /// An application credential object.
    ///
    #[command(flatten)]
    application_credential: Option<ApplicationCredential>,

    /// The authentication method. For password authentication, specify
    /// `password`.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, required=false)]
    methods: Vec<Methods>,

    /// The `password` object, contains the authentication information.
    ///
    #[command(flatten)]
    password: Option<Password>,

    /// A `token` object
    ///
    #[command(flatten)]
    token: Option<Token>,

    /// Multi Factor Authentication information
    ///
    #[command(flatten)]
    totp: Option<Totp>,
}

/// ScopeDomain Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct ScopeDomain {
    /// Domain id
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// Domain name
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// OsTrustTrust Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct OsTrustTrust {
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,
}

/// System Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct System {
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    all: Option<bool>,
}

/// Scope Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct Scope {
    #[command(flatten)]
    domain: Option<ScopeDomain>,

    #[command(flatten)]
    os_trust_trust: Option<OsTrustTrust>,

    #[arg(help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    project: Option<Value>,

    #[command(flatten)]
    system: Option<System>,
}

/// Auth Body data
#[derive(Args, Clone)]
struct Auth {
    /// An `identity` object.
    ///
    #[command(flatten)]
    identity: Identity,

    /// The authorization scope, including the system (Since v3.10), a project,
    /// or a domain (Since v3.4). If multiple scopes are specified in the same
    /// request (e.g. project and domain or domain and system) an HTTP 400 Bad
    /// Request will be returned, as a token cannot be simultaneously scoped to
    /// multiple authorization targets. An ID is sufficient to uniquely
    /// identify a project but if a project is specified by name, then the
    /// domain of the project must also be specified in order to uniquely
    /// identify the project by name. A domain scope may be specified by either
    /// the domain’s ID or name with equivalent results.
    ///
    #[command(flatten)]
    scope: Option<Scope>,
}

/// Saml2 response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl Saml2Command {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Saml2");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.auth data
        let args = &self.auth;
        let mut auth_builder = create::AuthBuilder::default();

        let mut identity_builder = create::IdentityBuilder::default();

        identity_builder.methods(
            args.identity
                .methods
                .iter()
                .map(|v| v.into())
                .collect::<Vec<_>>(),
        );
        if let Some(val) = &&args.identity.password {
            let mut password_builder = create::PasswordBuilder::default();
            if let Some(val) = &val.user {
                let mut user_builder = create::UserBuilder::default();
                if let Some(val) = &val.id {
                    user_builder.id(val);
                }
                if let Some(val) = &val.name {
                    user_builder.name(val);
                }
                if let Some(val) = &val.password {
                    user_builder.password(val);
                }
                if let Some(val) = &val.domain {
                    user_builder.domain(serde_json::from_value::<create::Domain>(val.to_owned())?);
                }
                password_builder.user(user_builder.build().expect("A valid object"));
            }
            identity_builder.password(password_builder.build().expect("A valid object"));
        }
        if let Some(val) = &&args.identity.token {
            let mut token_builder = create::TokenBuilder::default();

            token_builder.id(&val.id);
            identity_builder.token(token_builder.build().expect("A valid object"));
        }
        if let Some(val) = &&args.identity.totp {
            let mut totp_builder = create::TotpBuilder::default();

            let mut user_builder = create::TotpUserBuilder::default();
            if let Some(val) = &&val.user.id {
                user_builder.id(val);
            }
            if let Some(val) = &&val.user.name {
                user_builder.name(val);
            }
            if let Some(val) = &&val.user.domain {
                user_builder.domain(serde_json::from_value::<create::Domain>(val.to_owned())?);
            }

            user_builder.passcode(&val.user.passcode);
            totp_builder.user(user_builder.build().expect("A valid object"));
            identity_builder.totp(totp_builder.build().expect("A valid object"));
        }
        if let Some(val) = &&args.identity.application_credential {
            let mut application_credential_builder =
                create::ApplicationCredentialBuilder::default();
            if let Some(val) = &val.id {
                application_credential_builder.id(val);
            }
            if let Some(val) = &val.name {
                application_credential_builder.name(val);
            }

            application_credential_builder.secret(&val.secret);
            if let Some(val) = &val.user {
                application_credential_builder.user(serde_json::from_value::<
                    create::ApplicationCredentialUser,
                >(val.to_owned())?);
            }
            identity_builder.application_credential(
                application_credential_builder
                    .build()
                    .expect("A valid object"),
            );
        }
        auth_builder.identity(identity_builder.build().expect("A valid object"));

        if let Some(val) = &args.scope {
            let mut scope_builder = create::ScopeBuilder::default();
            if let Some(val) = &val.project {
                scope_builder.project(serde_json::from_value::<create::Project>(val.to_owned())?);
            }
            if let Some(val) = &val.domain {
                let mut domain_builder = create::ScopeDomainBuilder::default();
                if let Some(val) = &val.id {
                    domain_builder.id(val);
                }
                if let Some(val) = &val.name {
                    domain_builder.name(val);
                }
                scope_builder.domain(domain_builder.build().expect("A valid object"));
            }
            if let Some(val) = &val.os_trust_trust {
                let mut os_trust_trust_builder = create::OsTrustTrustBuilder::default();
                if let Some(val) = &val.id {
                    os_trust_trust_builder.id(val);
                }
                scope_builder
                    .os_trust_trust(os_trust_trust_builder.build().expect("A valid object"));
            }
            if let Some(val) = &val.system {
                let mut system_builder = create::SystemBuilder::default();
                if let Some(val) = &val.all {
                    system_builder.all(*val);
                }
                scope_builder.system(system_builder.build().expect("A valid object"));
            }
            auth_builder.scope(scope_builder.build().expect("A valid object"));
        }

        ep_builder.auth(auth_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
