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

use std::collections::{HashMap, HashSet};

use secrecy::SecretString;

pub use openstack_sdk_auth_core::{Auth, AuthError, AuthState, find_auth_plugin};

pub mod auth_helper;
pub mod authtoken_scope;

use crate::auth::auth_helper::AuthHelper;
use crate::config::CloudConfig;
use crate::error::OpenStackError;

/// Whether the plugin requirements property (json schema) describes a secret value.
fn is_secret_property(metadata: &serde_json::Value) -> bool {
    metadata["format"].as_str() == Some("password")
        || metadata["writeOnly"].as_bool().unwrap_or(false)
}

/// Names of the properties which the plugin requirements (json schema) mark as secrets.
pub fn sensitive_fields_from_schema(requirements: &serde_json::Value) -> HashSet<String> {
    requirements["properties"]
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(_, metadata)| is_secret_property(metadata))
        .map(|(field, _)| field.to_string())
        .collect()
}

/// Names of the config properties which the compiled-in authentication plugin for the
/// connection declares as secrets.
///
/// An unknown `auth_type` (i.e. a plugin which is not linked in, or a wasm one) or a failing
/// `requirements` call result in an empty set: it is up to the caller to combine the result with
/// the baseline knowledge about sensitive fields.
pub fn plugin_sensitive_fields(config: &CloudConfig) -> HashSet<String> {
    let auth_type = config.auth_type.as_deref().unwrap_or("v3password");
    let hints = config
        .auth_methods
        .as_ref()
        .map(|methods| serde_json::json!({"auth_methods": methods}));
    find_auth_plugin(auth_type)
        .and_then(|plugin| plugin.requirements(hints.as_ref()).ok())
        .map(|requirements| sensitive_fields_from_schema(&requirements))
        .unwrap_or_default()
}

pub async fn gather_auth_data<A>(
    requirements: &serde_json::Value,
    config: &CloudConfig,
    auth_helper: &A,
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
        let is_secret = is_secret_property(metadata);
        // Plugin specific settings are not part of the typed `auth` section: fall
        // back to the (flattened) cloud level `options`. Scalars of any type are
        // stringified so that i.e. an integer `callback_port` is not dropped.
        let option_val = config
            .options
            .get(field)
            .and_then(|v| v.clone().into_string().ok());
        if let Some(val) = config_values[field]
            .as_str()
            .map(str::to_string)
            .or(option_val)
        {
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

    #[test]
    fn test_sensitive_fields_from_schema() {
        assert_eq!(
            sensitive_fields_from_schema(&json!({"properties": {
                "password": {"type": "string", "format": "password"},
                "secret": {"type": "string", "writeOnly": true},
                "username": {"type": "string"},
                "other": {"format": "uri"},
            }})),
            HashSet::from(["password".to_string(), "secret".to_string()])
        );
        assert!(sensitive_fields_from_schema(&json!({})).is_empty());
    }

    #[tokio::test]
    async fn test_required() {
        let auth_helper = Noop::default();
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
            &auth_helper,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_available() {
        let auth_helper = Noop::default();
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
            &auth_helper,
        )
        .await
        .unwrap();
        assert!(vals.contains_key("application_credential_name"));
        assert!(vals.contains_key("application_credential_secret"));
        assert!(!vals.contains_key("application_credential_id"));
    }

    #[tokio::test]
    async fn test_options_fallback() {
        let auth_helper = Noop::default();
        let mut options = std::collections::HashMap::new();
        options.insert("callback_port".to_string(), ::config::Value::from(8080_i64));
        options.insert(
            "oidc_endpoint".to_string(),
            ::config::Value::from("https://idp"),
        );
        options.insert(
            "client_id".to_string(),
            ::config::Value::from("from-options"),
        );
        let auth = config::Auth {
            client_id: Some("from-auth".into()),
            ..Default::default()
        };
        let vals = gather_auth_data(
            &json!({"properties": {
                "callback_port": {},
                "oidc_endpoint": {},
                "client_id": {},
                "missing": {}
            }}),
            &CloudConfig {
                auth: Some(auth),
                options,
                ..Default::default()
            },
            &auth_helper,
        )
        .await
        .unwrap();
        use secrecy::ExposeSecret;
        assert_eq!(vals["callback_port"].expose_secret(), "8080");
        assert_eq!(vals["oidc_endpoint"].expose_secret(), "https://idp");
        // `auth` section wins over `options`
        assert_eq!(vals["client_id"].expose_secret(), "from-auth");
        assert!(!vals.contains_key("missing"));
    }
}
