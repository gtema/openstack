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

use thiserror::Error;

use crate::auth::auth_token_endpoint as token_v3;
use crate::auth::AuthToken;
use crate::config;

/// User name/pass related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TokenError {
    /// Token is missing
    #[error("Auth token is missing")]
    MissingToken,

    /// Token Identity builder
    #[error("Cannot construct token auth information from config: {}", source)]
    TokenBuilder {
        /// The error source
        #[from]
        source: token_v3::TokenBuilderError,
    },

    /// Identity builder
    #[error("Cannot construct identity auth information from config: {}", source)]
    Identity {
        /// The error source
        #[from]
        source: token_v3::IdentityBuilderError,
    },
}

/// Fill [`IdentityBuilder`][`token_v3::IdentityBuilder`] with user token
pub fn fill_identity(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    _interactive: bool,
) -> Result<(), TokenError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Token]));
    let token = token_v3::TokenBuilder::default()
        .id(auth_data.token.clone().ok_or(TokenError::MissingToken)?)
        .build()?;
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
                    .id(auth.token.clone())
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
    use crate::config;

    #[test]
    fn test_fill_raise_no_token() {
        let config = config::Auth::default();
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config, false);
        match res.unwrap_err() {
            TokenError::MissingToken => {}
            other => {
                panic!("Unexpected error: {other}")
            }
        }
    }

    #[test]
    fn test_fill() {
        let config = config::Auth {
            token: Some("foo".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(&mut identity, &config, false).unwrap();
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

    #[test]
    #[traced_test]
    fn test_token_not_in_log() {
        let config = config::Auth {
            token: Some("secret".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(&mut identity, &config, false).unwrap();
        let identity = identity.build().unwrap();
        info!("Auth is {:?}", identity);
        assert!(!logs_contain("secret"));
    }
}
