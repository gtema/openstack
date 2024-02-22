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

//! Helper methods to deal with OpenStack authentication with user name/password
//!
//! ```json
//! {
//!     "auth": {
//!         "identity": {
//!             "methods": [
//!                 "password"
//!             ],
//!             "password": {
//!                 "user": {
//!                     "name": "admin",
//!                     "domain": {
//!                         "name": "Default"
//!                     },
//!                     "password": "devstacker"
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use thiserror::Error;

use dialoguer::{Input, Password};

use crate::api::identity::v3::auth::token::create as token_v3;
use crate::config;

/// User name/pass related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PasswordError {
    #[error("User password is missing")]
    MissingPassword,
    #[error("User id/name is missing")]
    MissingUserId,
    #[error("Cannot construct password auth information from config: {}", source)]
    PasswordBuilder {
        #[from]
        source: token_v3::PasswordBuilderError,
    },
    #[error("Cannot construct user auth information from config: {}", source)]
    UserBuilder {
        #[from]
        source: token_v3::UserBuilderError,
    },
    #[error("Cannot construct user domain information from config: {}", source)]
    UserDomainBuilder {
        #[from]
        source: token_v3::DomainBuilderError,
    },
}

/// Fill Auth Request Identity with user password data
pub fn fill_identity(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    interactive: bool,
) -> Result<(), PasswordError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Password]));
    let mut user = token_v3::UserBuilder::default();
    // Set user_id or name
    if let Some(val) = &auth_data.user_id {
        user.id(val.clone());
    }
    if let Some(val) = &auth_data.username {
        user.name(val.clone());
    }
    if auth_data.user_id.is_none() && auth_data.username.is_none() {
        if interactive {
            // Or ask user for username in interactive mode
            let name: String = Input::new()
                .with_prompt("Username:")
                .interact_text()
                .unwrap();
            user.name(name);
        } else {
            return Err(PasswordError::MissingUserId);
        }
    }
    // Fill password
    if let Some(val) = &auth_data.password {
        user.password(val.clone());
    } else if interactive {
        // Or ask user for password
        let password = Password::new()
            .with_prompt("User Password")
            .interact()
            .unwrap();
        user.password(password.to_string());
    } else {
        return Err(PasswordError::MissingPassword);
    }

    // Process user domain information
    if auth_data.user_domain_id.is_some() || auth_data.user_domain_name.is_some() {
        let mut user_domain = token_v3::DomainBuilder::default();
        if let Some(val) = &auth_data.user_domain_id {
            user_domain.id(val.clone());
        }
        if let Some(val) = &auth_data.user_domain_name {
            user_domain.name(val.clone());
        }
        user.domain(user_domain.build()?);
    }

    let password = token_v3::PasswordBuilder::default()
        .user(user.build()?)
        .build()?;
    identity_builder.password(password);
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::api::identity::v3::auth::token::create as token_v3;
    use crate::config;

    #[test]
    fn test_fill_raise_no_user_id_or_name() {
        let config = config::Auth {
            password: Some("pass".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config, false);
        match res.unwrap_err() {
            PasswordError::MissingUserId => {}
            other => {
                panic!("Unexpected error: {}", other)
            }
        }
    }

    #[test]
    fn test_fill_raise_no_password() {
        let config = config::Auth {
            user_id: Some("uid".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config, false);
        match res.unwrap_err() {
            PasswordError::MissingPassword => {}
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
            password: Some("pass".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(&mut identity, &config, false).unwrap();
        assert_eq!(
            serde_json::to_value(identity.build().unwrap()).unwrap(),
            json!({
                "methods": ["password"],
                "password": {
                    "user": {
                        "id": "uid",
                        "name": "un",
                        "domain": {
                            "id": "udi",
                            "name": "udn"
                        },
                        "password": "pass"
                    },
                }
            })
        );
    }
}
