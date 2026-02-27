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

//! JWT login handling
//!
//! This module implements login using the JWT token by exchanging it for a regular Keystone token.

use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue, header};
use secrecy::{ExposeSecret, SecretString};
use std::borrow::Cow;
use thiserror::Error;

use crate::api::RestEndpoint;
use crate::api::rest_endpoint_prelude::*;
use crate::auth::auth_helper::AuthHelper;
use crate::config;
use crate::types::{ApiVersion, ServiceType};

/// JWT related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JwtError {
    /// Auth data is missing.
    #[error("auth data is missing")]
    MissingAuthData,

    /// Identity provider id is missing.
    #[error("identity provider id is missing")]
    MissingIdentityProvider,

    /// Attribute mapping name is missing.
    #[error("attribute mapping name is missing")]
    MissingAttributeMapping,

    /// JWT is missing.
    #[error("JWT is missing")]
    MissingJwt,

    /// Jwt Auth builder.
    #[error("error preparing auth request: {}", source)]
    JwtBuilder {
        /// The error source.
        #[from]
        source: JwtRequestBuilderError,
    },

    /// HeaderValue error.
    #[error("invalid value for the header: {}", source)]
    HeaderValue {
        /// The error source.
        #[from]
        source: http::header::InvalidHeaderValue,
    },
}

/// Endpoint for the JWT authorization
#[derive(Builder, Debug, Clone)]
#[builder(setter(into, strip_option))]
pub struct JwtRequest<'a> {
    /// idp_id that issued the JWT.
    #[builder(setter(into))]
    idp_id: Cow<'a, str>,
    /// Attribute mapping name.

    #[builder(default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> JwtRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> JwtRequestBuilder<'a> {
        JwtRequestBuilder::default()
    }
}

impl<'a> JwtRequestBuilder<'a> {
    /// Set attribute mapping name.
    pub fn mapping_name<S: AsRef<str>>(&mut self, mapping_name: S) -> Result<(), JwtError> {
        let val = HeaderValue::from_str(mapping_name.as_ref())?;
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(HeaderName::from_static("openstack-mapping"), val);
        Ok(())
    }

    /// Set the JWT token.
    pub fn token(&mut self, token: &SecretString) -> Result<(), JwtError> {
        let mut val = HeaderValue::from_str(format!("bearer {}", token.expose_secret()).as_str())?;
        val.set_sensitive(true);
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header::AUTHORIZATION, val);
        Ok(())
    }
}

impl RestEndpoint for JwtRequest<'_> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "federation/identity_providers/{idp_id}/jwt",
            idp_id = self.idp_id.as_ref(),
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        JsonBodyParams::default().into_body()
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
        Some(ApiVersion::new(4, 0))
    }
}

/// Get [`RestEndpoint`] for initializing the JWT authentication.
pub async fn get_auth_ep<A>(
    config: &config::CloudConfig,
    auth_helper: &mut A,
) -> Result<impl RestEndpoint + use<A>, JwtError>
where
    A: AuthHelper,
{
    if let Some(auth_data) = &config.auth {
        let connection_name = config.name.as_ref();
        let mut ep = JwtRequest::builder();
        if let Some(val) = &auth_data.identity_provider {
            ep.idp_id(val.clone());
        } else {
            // Or ask user for idp_id in interactive mode
            let idp = auth_helper
                .get("identity_provider".into(), connection_name.cloned())
                .await
                .map_err(|_| JwtError::MissingIdentityProvider)?
                .to_owned();
            ep.idp_id(idp);
        }
        if let Some(val) = &auth_data.attribute_mapping_name {
            ep.mapping_name(val)?;
        } else {
            // Or ask user for mapping name in interactive mode
            ep.mapping_name(
                auth_helper
                    .get("mapping name".into(), connection_name.cloned())
                    .await
                    .map_err(|_| JwtError::MissingAttributeMapping)?,
            )?;
        }
        if let Some(val) = &auth_data.jwt {
            ep.token(val)?;
        } else {
            // Or ask user for token in interactive mode
            ep.token(
                &auth_helper
                    .get_secret("JWT".into(), connection_name.cloned())
                    .await
                    .map_err(|_| JwtError::MissingJwt)?,
            )?;
        }
        return Ok(ep.build()?);
    }
    Err(JwtError::MissingAuthData)
}
