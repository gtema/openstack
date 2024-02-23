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

//! Helper methods to deal with OpenStack authentication with TOTP token
//!
//! ```json
//! {
//!     "auth": {
//!         "identity": {
//!             "methods": [
//!                 "totp"
//!             ],
//!             "totp": {
//!                 "user": {
//!                     "id": "ee4dfb6e5540447cb3741905149d9b6e",
//!                     "passcode": "123456"
//!                 }
//!             }
//!         },
//!     }
//! }
//! ```

use thiserror::Error;

use dialoguer::Input;

use crate::api::identity::v3::auth::token::create as token_v3;
use crate::config;

/// TOTP related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TotpError {
    /// UserID is required
    #[error("User id is missing")]
    MissingUserId,

    /// Passcode is required
    #[error("Auth token is missing")]
    MissingPasscode,

    /// TotpUser builder
    #[error("Cannot construct TOTP user information: {}", source)]
    UserBuilder {
        /// The error source
        #[from]
        source: token_v3::TotpUserBuilderError,
    },

    /// TotpUserDomain builder
    #[error("Cannot construct TOTP user domain information: {}", source)]
    UserDomainBuilder {
        /// The error source
        #[from]
        source: token_v3::UserDomainStructBuilderError,
    },

    /// Totp builder
    #[error("Cannot construct TOTP auth information: {}", source)]
    TotpBuilder {
        /// The error source
        #[from]
        source: token_v3::TotpBuilderError,
    },
}

/// Fill [`IdentityBuilder`][`token_v3::IdentityBuilder`] with MFA passcode
pub fn fill_identity(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    interactive: bool,
) -> Result<(), TotpError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Totp]));
    let mut user = token_v3::TotpUserBuilder::default();
    if let Some(val) = &auth_data.user_id {
        user.id(val.clone());
    } else if let Some(val) = &auth_data.username {
        user.name(val.clone());
    } else if interactive {
        // Or ask user for username in interactive mode
        let name: String = Input::new()
            .with_prompt("Please provide the username:")
            .interact_text()
            .unwrap();
        user.name(name);
    } else {
        return Err(TotpError::MissingUserId);
    }
    // Process user domain information
    if auth_data.user_domain_id.is_some() || auth_data.user_domain_name.is_some() {
        let mut user_domain = token_v3::UserDomainStructBuilder::default();
        if let Some(val) = &auth_data.user_domain_id {
            user_domain.id(val.clone());
        }
        if let Some(val) = &auth_data.user_domain_name {
            user_domain.name(val.clone());
        }
        user.domain(user_domain.build()?);
    }

    if let Some(passcode) = &auth_data.passcode {
        user.passcode(passcode.clone());
    } else if interactive {
        // Or ask user for username in interactive mode
        let name: String = Input::new()
            .with_prompt("Please provide the MFA passcode:")
            .interact_text()
            .unwrap();
        user.passcode(name);
    } else {
        return Err(TotpError::MissingPasscode);
    }
    identity_builder.totp(
        token_v3::TotpBuilder::default()
            .user(user.build()?)
            .build()?,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::api::identity::v3::auth::token::create as token_v3;
    use crate::config;

    #[test]
    fn test_fill_raise_no_user_id() {
        let config = config::Auth {
            passcode: Some("pass".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config, false);
        match res.unwrap_err() {
            TotpError::MissingUserId => {}
            other => {
                panic!("Unexpected error: {}", other)
            }
        }
    }

    #[test]
    fn test_fill_raise_no_user_passcode() {
        let config = config::Auth {
            user_id: Some("uid".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config, false);
        match res.unwrap_err() {
            TotpError::MissingPasscode => {}
            other => {
                panic!("Unexpected error: {}", other)
            }
        }
    }

    #[test]
    fn test_fill() {
        let config = config::Auth {
            user_id: Some("uid".to_string()),
            username: Some("un".to_string()),
            user_domain_id: Some("udi".to_string()),
            user_domain_name: Some("udn".to_string()),
            passcode: Some("pass".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(&mut identity, &config, false).unwrap();
        assert_eq!(
            serde_json::to_value(identity.build().unwrap()).unwrap(),
            json!({
                "methods": ["totp"],
                "totp": {
                    "user": {
                        "id": "uid",
                        "domain": {
                            "id": "udi",
                            "name": "udn"
                        },
                        "passcode": "pass"
                    },
                }
            })
        );
    }
}
