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

//! Federated (OAUTH2/OIDC) authentication using the OIDC access token
//!

use derive_builder::Builder;
use http::{header, HeaderMap, HeaderName, HeaderValue};
use secrecy::ExposeSecret;
use std::borrow::Cow;
use thiserror::Error;
use tracing::error;

use crate::api::RestEndpoint;
use crate::auth::auth_helper::AuthHelper;
use crate::config;
use crate::types::{ApiVersion, ServiceType};

/// OidcAccessToken related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OidcAccessTokenError {
    /// Auth data is missing
    #[error("auth data is missing")]
    MissingAuthData,

    /// Identity provider ID is missing
    #[error("identity_provider ID is missing")]
    MissingIdpId,

    /// Federation protocol ID is missing
    #[error("federation protocol ID is missing")]
    MissingProtocolId,

    /// Access token is missing
    #[error("access token is missing")]
    MissingAccessToken,

    /// OidcAccessToken Auth builder
    #[error("error preparing auth request: {}", source)]
    AuthRequestBuilder {
        /// The error source
        #[from]
        source: OidcAccessTokenRequestBuilderError,
    },

    /// Invalid http header value
    #[error("invalid http header: {}", source)]
    HeaderValue {
        /// The error source
        #[from]
        source: header::InvalidHeaderValue,
    },
}

/// Endpoint for initializing oauth2 authorization
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct OidcAccessTokenRequest<'a> {
    /// IDP id
    #[builder(setter(into))]
    idp_id: Cow<'a, str>,

    /// OIDC protocol ID (typically oidc)
    #[builder(setter(into))]
    protocol: Cow<'a, str>,

    /// Request headers
    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> OidcAccessTokenRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> OidcAccessTokenRequestBuilder<'a> {
        OidcAccessTokenRequestBuilder::default()
    }
}

impl<'a> OidcAccessTokenRequestBuilder<'a> {
    /// Add a single header to the Service.
    pub fn header<K, V>(&mut self, header_name: K, header_value: V) -> &mut Self
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header_name.into(), header_value.into());
        self
    }

    /// Add multiple headers.
    pub fn headers<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<(Option<HeaderName>, HeaderValue)>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl RestEndpoint for OidcAccessTokenRequest<'_> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol}/auth",
            idp_id = self.idp_id.as_ref(),
            protocol = self.protocol.as_ref()
        )
        .into()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(3, 0))
    }
}

/// Get [`RestEndpoint`] for initializing the OIDC authentication
pub async fn get_auth_ep<A: AuthHelper>(
    config: &config::CloudConfig,
    auth_helper: &mut A,
) -> Result<impl RestEndpoint, OidcAccessTokenError> {
    if let Some(auth) = &config.auth {
        let mut ep = OidcAccessTokenRequest::builder();

        if let Some(identity_provider) = &auth.identity_provider {
            ep.idp_id(identity_provider.clone());
        } else {
            let idp_id = auth_helper
                .get("idp_id".into(), config.name.clone())
                .await
                .map_err(|_| OidcAccessTokenError::MissingIdpId)?
                .to_owned();
            ep.idp_id(idp_id);
        }

        if let Some(protocol) = &auth.protocol {
            ep.protocol(protocol.clone());
        } else {
            let protocol = auth_helper
                .get("protocol".into(), config.name.clone())
                .await
                .map_err(|_| OidcAccessTokenError::MissingProtocolId)?
                .to_owned();
            ep.protocol(protocol);
        }
        if let Some(access_token) = &auth.access_token {
            let mut token_header_value =
                HeaderValue::from_str(format!("Bearer {}", access_token.expose_secret()).as_str())?;
            token_header_value.set_sensitive(true);
            ep.header(header::AUTHORIZATION, token_header_value);
        } else {
            let access_token = auth_helper
                .get_secret("access_token".into(), config.name.clone())
                .await
                .map_err(|_| OidcAccessTokenError::MissingAccessToken)?
                .to_owned();
            let mut token_header_value =
                HeaderValue::from_str(format!("Bearer {}", access_token.expose_secret()).as_str())?;
            token_header_value.set_sensitive(true);
            ep.header(header::AUTHORIZATION, token_header_value);
        }
        return Ok(ep.build()?);
    }
    Err(OidcAccessTokenError::MissingAuthData)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::auth_helper::Noop;
    use crate::config;

    #[tokio::test]
    async fn test_get() {
        let config = config::CloudConfig {
            auth: Some(config::Auth {
                identity_provider: Some("foo".into()),
                protocol: Some("bar".into()),
                access_token: Some("baz".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let ep = get_auth_ep(&config, &mut Noop::default()).await.unwrap();
        assert_eq!(
            "OS-FEDERATION/identity_providers/foo/protocols/bar/auth",
            ep.endpoint()
        );
        assert_eq!(
            HeaderValue::from_static("Bearer baz"),
            ep.request_headers()
                .unwrap()
                .get(header::AUTHORIZATION)
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_fill_raise_no_secret() {
        let config = config::CloudConfig {
            auth: Some(config::Auth {
                identity_provider: Some("foo".into()),
                protocol: Some("bar".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let res = get_auth_ep(&config, &mut Noop::default()).await;
        match res {
            Err(OidcAccessTokenError::MissingAccessToken) => {}
            _ => {
                panic!("Should raise an error")
            }
        }
    }

    #[tokio::test]
    async fn test_fill_raise_no_idp_id() {
        let config = config::CloudConfig {
            auth: Some(config::Auth {
                protocol: Some("bar".into()),
                access_token: Some("baz".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let res = get_auth_ep(&config, &mut Noop::default()).await;
        match res {
            Err(OidcAccessTokenError::MissingIdpId) => {}
            _ => {
                panic!("Should raise an error")
            }
        }
    }

    #[tokio::test]
    async fn test_fill_raise_no_mapping() {
        let config = config::CloudConfig {
            auth: Some(config::Auth {
                identity_provider: Some("foo".into()),
                access_token: Some("baz".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let res = get_auth_ep(&config, &mut Noop::default()).await;
        match res {
            Err(OidcAccessTokenError::MissingProtocolId) => {}
            _ => {
                panic!("Should raise an error")
            }
        }
    }
}
