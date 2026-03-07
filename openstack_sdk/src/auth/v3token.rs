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

//! Helper methods to deal with OpenStack authentication with Auth Token
//!
//! ```json
//! {
//!     "auth": {
//!         "identity": {
//!             "methods": [
//!                 "token"
//!             ],
//!             "token": {
//!                 "id": "'$OS_TOKEN'"
//!             }
//!         },
//!     }
//! }
use secrecy::ExposeSecret;
use thiserror::Error;

use openstack_sdk_auth_core::AuthToken;
use openstack_sdk_core::auth::auth_helper::{AuthHelper, AuthHelperError};

use crate::auth::auth_token_endpoint as token_v3;
use crate::config;

/// User name/pass related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TokenError {
    /// Authentication helper error
    #[error(transparent)]
    AuthHelper {
        /// The error source
        #[from]
        source: AuthHelperError,
    },

    /// Token is missing
    #[error("Auth token is missing")]
    MissingToken,

    /// Token Identity builder
    #[error("Cannot construct token auth information: {}", source)]
    Builder {
        /// The error source
        #[from]
        source: crate::BuilderError,
    },
}

/// Fill [`IdentityBuilder`][`token_v3::IdentityBuilder`] with user token
pub async fn fill_identity<S: AsRef<str>>(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    connection_name: Option<S>,
    auth_helper: &mut (dyn AuthHelper + Send),
) -> Result<(), TokenError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Token]));
    let token_val = if let Some(val) = &auth_data.token {
        val.clone()
    } else {
        auth_helper
            .get_secret(
                "token".into(),
                connection_name.as_ref().map(|x| x.as_ref().to_string()),
            )
            .await
            .map_err(|_| TokenError::MissingToken)?
    };
    let token = token_v3::TokenBuilder::default().id(token_val).build()?;
    identity_builder.token(token);
    Ok(())
}

/// Build Auth [`Identity`][`token_v3::Identity`] from existing [`AuthToken`] using current token
impl TryFrom<&AuthToken> for token_v3::Identity<'_> {
    type Error = TokenError;

    fn try_from(auth: &AuthToken) -> Result<Self, Self::Error> {
        Ok(token_v3::IdentityBuilder::default()
            .methods(Vec::from([token_v3::Methods::Token]))
            .token(
                token_v3::TokenBuilder::default()
                    .id(auth.token.expose_secret())
                    .build()?,
            )
            .build()?)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tracing::info;
    use tracing_test::traced_test;

    use super::*;
    use crate::auth::auth_helper::Noop;
    use crate::config;

    #[tokio::test]
    async fn test_fill_raise_no_token() {
        let config = config::Auth::default();
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config, None::<&str>, &mut Noop::default()).await;
        match res.unwrap_err() {
            TokenError::MissingToken => {}
            other => {
                panic!("Unexpected error: {other}")
            }
        }
    }

    #[tokio::test]
    async fn test_fill() {
        let config = config::Auth {
            token: Some("foo".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let _res = fill_identity(&mut identity, &config, None::<&str>, &mut Noop::default()).await;
        assert_eq!(
            serde_json::to_value(identity.build().unwrap()).unwrap(),
            json!({
                "methods": ["token"],
                "token": {
                    "id": "foo",
                }
            })
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_token_not_in_log() {
        let config = config::Auth {
            token: Some("secret".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let _res = fill_identity(&mut identity, &config, None::<&str>, &mut Noop::default()).await;
        let identity = identity.build().unwrap();
        info!("Auth is {:?}", identity);
        assert!(!logs_contain("secret"));
    }
}
