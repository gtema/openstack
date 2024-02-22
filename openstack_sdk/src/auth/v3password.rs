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

use dialoguer::{Input, Password};

use crate::api::identity::v3::auth::token::create as token_v3;
use crate::auth::authtoken::AuthTokenError;
use crate::config;

/// Fill Auth Request Identity with user password data
pub fn fill_identity_using_password(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    interactive: bool,
) -> Result<(), AuthTokenError> {
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
            return Err(AuthTokenError::MissingUserId);
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
        return Err(AuthTokenError::MissingPassword);
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
