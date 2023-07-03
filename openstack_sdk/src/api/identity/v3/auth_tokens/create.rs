use derive_builder::Builder;
use std::collections::HashMap;
use std::convert::From;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeSet;
use tracing::{debug, trace};

use anyhow::anyhow;
use thiserror::Error;

use http::HeaderMap;

use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;
use crate::api::{ApiError, Client, Query};
use crate::types::NameOrId;

/// Identity method
#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IdentityMethod {
    Password,
    Token,
}

impl IdentityMethod {
    fn as_str(self) -> &'static str {
        match self {
            IdentityMethod::Password => "password",
            IdentityMethod::Token => "token",
        }
    }
}

impl TryFrom<String> for IdentityMethod {
    type Error = anyhow::Error;
    fn try_from(val: String) -> Result<Self, Self::Error> {
        match val.as_str() {
            "password" => Ok(IdentityMethod::Password),
            "token" => Ok(IdentityMethod::Token),
            _ => Err(anyhow!("auth_type {} is not supported", val)),
        }
    }
}

// impl ParamValue<'static> for IdentityMethod {
//     fn as_value(&self) -> Cow<'static, str> {
//         self.as_str().into()
//     }
// }

/// Auth placeholder for Token
/// Token authorization data
#[derive(Debug, Clone, Serialize)]
pub struct TokenAuthData<'a> {
    pub id: Cow<'a, str>,
}

/// UserWithPassword authorization data
#[derive(Debug, Clone, Serialize)]
pub struct UserWithPassword<'a> {
    #[serde(flatten)]
    pub user: NameOrId,
    pub password: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub domain: Option<NameOrId>,
}

/// Auth placeholder for UserWithPassword
#[derive(Debug, Clone, Serialize)]
pub struct PasswordAuthData<'a> {
    pub user: UserWithPassword<'a>,
}

//impl<'a> From<UserWithPassword<'a>> for PasswordAuthData<'a> {
//    fn from(user: UserWithPassword<'a>) -> PasswordAuthData {
//        PasswordAuthData { user: user }
//    }
//}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthData<'a> {
    Password(PasswordAuthData<'a>),
    Token(TokenAuthData<'a>),
    None,
}

impl<'a> From<UserWithPassword<'a>> for AuthData<'a> {
    fn from(user: UserWithPassword<'a>) -> AuthData {
        AuthData::Password(PasswordAuthData { user })
    }
}

impl<'a> From<TokenAuthData<'a>> for AuthData<'a> {
    fn from(token: TokenAuthData<'a>) -> AuthData {
        AuthData::Token(token)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Identity<'a> {
    methods: BTreeSet<IdentityMethod>,
    #[serde(flatten)]
    auth_data: AuthData<'a>,
}

impl<'a> Default for Identity<'a> {
    fn default() -> Self {
        Identity {
            methods: BTreeSet::new(),
            auth_data: AuthData::None,
        }
    }
}
///
/// A reference to a project in a domain.
#[derive(Clone, Debug, Serialize)]
pub struct Project {
    #[serde(flatten)]
    pub project: NameOrId,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub domain: Option<NameOrId>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Scope {
    #[serde(rename = "project")]
    Project(Project),
    /// Domain scope.
    #[serde(rename = "domain")]
    Domain(NameOrId),
}

#[derive(Builder, Debug, Clone, Serialize)]
pub struct Auth<'a> {
    #[builder(setter(into), default)]
    identity: Identity<'a>,
    #[builder(setter(into), default)]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    scope: Option<Scope>,
}

impl<'a> Auth<'a> {
    pub fn builder() -> AuthBuilder<'a> {
        AuthBuilder::default()
    }
}

impl<'a> AuthBuilder<'a> {
    /// Use UsernamePassword authorization
    pub fn with_user(&mut self, user: UserWithPassword<'a>) -> &mut Self {
        let mut identity = self.identity.get_or_insert(Identity::default());
        // TODO: insert or replace - this is a list?
        identity.methods.insert(IdentityMethod::Password);
        identity.auth_data = user.into();
        self
    }

    /// Use Token based authorization
    pub fn with_token(&mut self, token: TokenAuthData<'a>) -> &mut Self {
        let mut identity = self.identity.get_or_insert(Identity::default());
        // TODO: insert or replace - this is a list?
        identity.methods.insert(IdentityMethod::Token);
        identity.auth_data = token.into();
        self
    }

    //pub fn with_project_scope(&mut self, project: Project) -> &mut Self {
    //    self.scope = Some(Scope::Project(project));
    //    self
    //}
}

/// Create authorization token
#[derive(Debug, Builder, Clone, Serialize)]
pub struct CreateAuthToken<'a> {
    #[builder(setter(into))]
    auth: Auth<'a>,
}

impl<'a> CreateAuthToken<'a> {
    pub fn builder() -> CreateAuthTokenBuilder<'a> {
        CreateAuthTokenBuilder::default()
    }
}

// impl<'a> CreateAuthTokenBuilder<'a> {
//    pub fn with_user(&mut self, user: &UserWithPassword) -> &mut Self {
//        self.auth.with_user(user);
//        // identity.methods.insert(IdentityMethod::Password);
//        self
//    }
// }

impl<'a> RestEndpoint for CreateAuthToken<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "auth/tokens".into()
    }

    fn service_type(&self) -> Cow<'static, str> {
        "identity".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let data = serde_json::to_string(self)?;
        Ok(Some(("application/json", data.into_bytes())))
    }

    fn response_headers(&self) -> HashMap<&str, &str> {
        HashMap::from([("X-Subject-Token", "x_auth_token")])
    }
}

// impl<'a> Query for CreateAuthToken<'a> {
// }

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Method;
    use serde_json::json;

    use crate::api::identity::v3::auth_tokens::create::*;
    use crate::api::{self, Query};
    use crate::test::client::MockServerClient;

    #[test]
    fn test_auth_builder_with_pwd() -> Result<(), String> {
        let user: UserWithPassword = UserWithPassword {
            user: NameOrId::Id("id".into()),
            password: "password".into(),
            domain: None,
        };
        let auth = Auth::builder().with_user(user).build().unwrap();
        match auth.identity.auth_data {
            AuthData::Password(data) => {
                assert_eq!("password", data.user.password);
                Ok(())
            }
            _ => Err(String::from("UserPassword is expected")),
        }
    }

    #[test]
    fn test_auth_builder_with_token() -> Result<(), String> {
        let token = TokenAuthData { id: "id".into() };
        let auth = Auth::builder().with_token(token).build().unwrap();
        match auth.identity.auth_data {
            AuthData::Token(data) => {
                assert_eq!("id", data.id);
                Ok(())
            }
            _ => Err(String::from("Token is expected")),
        }
    }

    #[test]
    #[should_panic]
    fn default() {
        CreateAuthToken::builder().build().unwrap();
    }

    #[test]
    fn request_with_token() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/auth/tokens")
                .json_body(json!({ "auth": {"identity": {"methods": ["token"], "token":{"id": "token_id"}}}}));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({}));
        });
        let token: TokenAuthData = TokenAuthData {
            id: "token_id".into(),
        };
        let auth = Auth::builder().with_token(token).build().unwrap();

        let endpoint = CreateAuthToken::builder().auth(auth).build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();

        mock.assert();
    }

    #[test]
    fn request_with_password() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/auth/tokens")
                .json_body(json!({ "auth": {"identity": {"methods": ["password"], "password": {"user": {"id": "user_id", "password": "password"}}}} }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({}));
        });

        let user: UserWithPassword = UserWithPassword {
            user: NameOrId::Id("user_id".into()),
            password: "password".into(),
            domain: None,
        };
        let auth = Auth::builder().with_user(user).build().unwrap();
        let endpoint = CreateAuthToken::builder().auth(auth).build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();

        mock.assert();
    }

    #[test]
    fn request_with_project_scope() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/auth/tokens")
                .json_body(json!({ "auth": {"identity": {"methods": ["token"], "token": {"id": "token_id"}}, "scope": {"project": {"id": "project_id"}}}}))
                ;
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({}));
        });
        let token: TokenAuthData = TokenAuthData {
            id: "token_id".into(),
        };
        let project = Project {
            project: NameOrId::Id("project_id".into()),
            domain: None,
        };
        let scope = Scope::Project(project);
        let auth = Auth::builder()
            .with_token(token)
            .scope(scope)
            .build()
            .unwrap();
        let endpoint = CreateAuthToken::builder().auth(auth).build().unwrap();

        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
