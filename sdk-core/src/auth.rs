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

//! OpenStack API authentication
//!
//! Currently there are only 2 types of auth supported:
//!
//! - AuthToken (X-Auth-Token header)
//! - None (unauthenticated)

use std::collections::HashMap;

use secrecy::SecretString;

pub use openstack_sdk_auth_core::{Auth, AuthError, AuthState};

pub mod auth_helper;
mod auth_token_endpoint;
pub mod authtoken;
pub mod authtoken_scope;
pub mod v3_token_info;
pub mod v3password;
pub mod v3token;
pub mod v3totp;

use crate::auth::auth_helper::AuthHelper;
use crate::config::CloudConfig;
use crate::error::OpenStackError;
use authtoken::AuthToken;

pub(crate) async fn gather_auth_data<A>(
    requirements: &serde_json::Value,
    config: &CloudConfig,
    auth_helper: &mut A,
) -> Result<HashMap<String, SecretString>, OpenStackError>
where
    A: AuthHelper,
{
    let config_values = serde_json::to_value(&config.auth)?;
    let mut res = HashMap::new();
    let required: Vec<String> =
        serde_json::from_value(requirements["required"].clone()).unwrap_or_default();
    for (field, metadata) in requirements["properties"]
        .as_object()
        .ok_or(AuthError::PluginMalformedRequirement)?
    {
        let is_secret = metadata["format"].as_str() == Some("password")
            || metadata["writeOnly"].as_bool().unwrap_or(false);
        if let Some(val) = config_values[field].as_str() {
            res.insert(field.to_string(), SecretString::from(val));
        } else {
            if required.contains(field) {
                let data = if is_secret {
                    auth_helper
                        .get_secret(field.to_string(), config.name.clone())
                        .await?
                } else {
                    SecretString::from(
                        auth_helper
                            .get(field.to_string(), config.name.clone())
                            .await?,
                    )
                };
                res.insert(field.to_string(), data);
            }
        };
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::auth::auth_helper::Noop;
    use crate::config;

    #[tokio::test]
    async fn test_required() {
        let mut auth_helper = Noop::default();
        let auth = config::Auth {
            application_credential_secret: Some("foo".into()),
            application_credential_name: Some("bar".into()),
            ..Default::default()
        };
        gather_auth_data(
            &json!({"properties": {"application_credential_name": {}}}),
            &CloudConfig {
                auth: Some(auth),
                ..Default::default()
            },
            &mut auth_helper,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_available() {
        let mut auth_helper = Noop::default();
        let auth = config::Auth {
            application_credential_secret: Some("foo".into()),
            application_credential_name: Some("bar".into()),
            ..Default::default()
        };
        let vals = gather_auth_data(
            &json!({
                "required": ["application_credential_secret"],
                "properties": {
                    "application_credential_name": {},
                    "application_credential_secret": {},
                    "application_credential_id": {}
                }
            }),
            &CloudConfig {
                auth: Some(auth),
                ..Default::default()
            },
            &mut auth_helper,
        )
        .await
        .unwrap();
        assert!(vals.contains_key("application_credential_name"));
        assert!(vals.contains_key("application_credential_secret"));
        assert!(!vals.contains_key("application_credential_id"));
    }
}
