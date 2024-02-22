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

use crate::api::identity::v3::auth::token::create as token_v3;
use crate::auth::authtoken::AuthTokenError;
use crate::config;

/// Fill Auth Request Identity with user token
pub fn fill_identity_using_token(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    _interactive: bool,
) -> Result<(), AuthTokenError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Token]));
    let token = token_v3::TokenBuilder::default()
        .id(auth_data
            .token
            .clone()
            .ok_or(AuthTokenError::MissingToken)?)
        .build()?;
    identity_builder.token(token);
    Ok(())
}
