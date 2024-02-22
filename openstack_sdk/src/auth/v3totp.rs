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

use dialoguer::Input;

use crate::api::identity::v3::auth::token::create as token_v3;
use crate::auth::authtoken::AuthTokenError;
use crate::config;

/// Fill Auth Request Identity with MFA passcode
pub fn fill_identity_using_totp(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    interactive: bool,
) -> Result<(), AuthTokenError> {
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
        return Err(AuthTokenError::MissingUserId);
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
        return Err(AuthTokenError::MissingPasscode);
    }
    identity_builder.totp(
        token_v3::TotpBuilder::default()
            .user(user.build()?)
            .build()?,
    );
    Ok(())
}
