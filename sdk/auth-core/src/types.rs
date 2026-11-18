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

//! Types of the SDK authentication methods
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::BuilderError;

/// Point in time used by authentication data (token and receipt expiration/issue time).
///
/// The type hides the date/time library used internally, so that consumers are not forced to depend
/// on it and the library can be replaced without a breaking change. Use the conversions to
/// [`std::time::SystemTime`] (or the RFC3339 string form provided by [`Display`](std::fmt::Display)
/// and [`FromStr`](std::str::FromStr)) to interoperate with any date/time crate.
///
/// Serialization is the RFC3339 string (i.e. `2026-03-04T05:06:07.123456Z`). Offsets in the input
/// are accepted and normalized to UTC.
///
/// # Converting to other date/time libraries
///
/// The conversion goes through [`std::time::SystemTime`], which every date/time crate understands.
///
/// ```
/// # use std::time::SystemTime;
/// # use openstack_sdk_auth_core::types::TokenTimestamp;
/// let ts: TokenTimestamp = "2026-03-04T05:06:07Z".parse().unwrap();
///
/// // jiff
/// let jiff_ts = jiff::Timestamp::try_from(SystemTime::from(ts)).unwrap();
/// assert_eq!(jiff_ts.to_string(), "2026-03-04T05:06:07Z");
///
/// // chrono
/// // let chrono_ts = chrono::DateTime::<chrono::Utc>::from(SystemTime::from(ts));
///
/// // time
/// // let time_ts = time::OffsetDateTime::from(SystemTime::from(ts));
///
/// // Without any library: seconds since the UNIX epoch
/// assert_eq!(ts.as_second(), 1_772_600_767);
/// ```
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct TokenTimestamp(jiff::Timestamp);

impl TokenTimestamp {
    /// Current time.
    pub fn now() -> Self {
        Self(jiff::Timestamp::now())
    }

    /// Seconds since the UNIX epoch (rounded towards the past).
    pub fn as_second(&self) -> i64 {
        self.0.as_second()
    }

    /// Add [`Duration`](std::time::Duration). Returns [`None`] when the result is not representable.
    pub fn checked_add(&self, delta: std::time::Duration) -> Option<Self> {
        self.0.checked_add(delta).ok().map(Self)
    }

    /// Time left until this timestamp, or zero when it is already in the past.
    pub fn remaining(&self) -> std::time::Duration {
        std::time::Duration::try_from(self.0.duration_since(jiff::Timestamp::now()))
            .unwrap_or(std::time::Duration::ZERO)
    }
}

impl std::fmt::Display for TokenTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/// Error parsing [`TokenTimestamp`] from a string.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("invalid RFC3339 timestamp: {0}")]
pub struct ParseTokenTimestampError(String);

impl std::str::FromStr for TokenTimestamp {
    type Err = ParseTokenTimestampError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<jiff::Timestamp>()
            .map(Self)
            .map_err(|x| ParseTokenTimestampError(x.to_string()))
    }
}

impl From<std::time::SystemTime> for TokenTimestamp {
    /// Out of range values saturate to the minimum/maximum representable time.
    fn from(value: std::time::SystemTime) -> Self {
        Self(jiff::Timestamp::try_from(value).unwrap_or_else(|_| {
            if value < std::time::UNIX_EPOCH {
                jiff::Timestamp::MIN
            } else {
                jiff::Timestamp::MAX
            }
        }))
    }
}

impl From<TokenTimestamp> for std::time::SystemTime {
    fn from(value: TokenTimestamp) -> Self {
        let secs = value.0.as_second();
        let nanos = value.0.subsec_nanosecond();
        // nanos carry the sign of the timestamp
        let base = if secs >= 0 {
            std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs.unsigned_abs())
        } else {
            std::time::UNIX_EPOCH - std::time::Duration::from_secs(secs.unsigned_abs())
        };
        if nanos >= 0 {
            base + std::time::Duration::from_nanos(nanos.unsigned_abs().into())
        } else {
            base - std::time::Duration::from_nanos(nanos.unsigned_abs().into())
        }
    }
}

impl std::ops::Add<std::time::Duration> for TokenTimestamp {
    type Output = Self;

    /// Saturates to the maximum representable time.
    fn add(self, rhs: std::time::Duration) -> Self {
        self.checked_add(rhs).unwrap_or(Self(jiff::Timestamp::MAX))
    }
}

impl std::ops::Sub<std::time::Duration> for TokenTimestamp {
    type Output = Self;

    /// Saturates to the minimum representable time.
    fn sub(self, rhs: std::time::Duration) -> Self {
        self.0
            .checked_sub(rhs)
            .map(Self)
            .unwrap_or(Self(jiff::Timestamp::MIN))
    }
}

/// A reference to a resource by both its name and ID.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Serialize)]
pub struct IdAndName {
    /// The UID for the entity.
    pub id: String,
    /// The name of the entity.
    pub name: String,
}

/// A reference to a resource by either its Name or ID.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum NameOrId {
    /// Resource ID.
    #[serde(rename = "id")]
    Id(String),
    /// Resource name.
    #[serde(rename = "name")]
    Name(String),
}

/// Authentication response structure returned by token calls.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthResponse {
    /// Token information.
    pub token: TokenInfo,
}

/// AuthToken response information.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TokenInfo {
    /// Application credential information.
    pub application_credential: Option<ApplicationCredential>,
    /// Catalog of available services.
    pub catalog: Option<Vec<ServiceEndpoints>>,
    /// Domain in which the token was issued.
    pub domain: Option<Domain>,
    /// Token expiration time.
    pub expires_at: TokenTimestamp,
    /// Token issue time.
    pub issued_at: Option<TokenTimestamp>,
    /// Project in which the token was issued.
    pub project: Option<Project>,
    /// Roles assigned to the token.
    pub roles: Option<Vec<IdAndName>>,
    /// System scope of the token.
    pub system: Option<System>,
    /// User who obtained the token.
    pub user: User,
}

/// Service endpoint catalog entries.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServiceEndpoints {
    /// List of available endpoints for this service.
    pub endpoints: Vec<CatalogEndpoint>,
    /// Human-readable service name.
    pub name: String,
    #[serde(rename = "type")]
    /// Service type identifier (e.g., "compute", "network").
    pub service_type: String,
}

/// Service catalog endpoint information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogEndpoint {
    /// Endpoint unique identifier.
    pub id: String,
    /// Interface type (public, internal, admin).
    pub interface: String,
    /// Region identifier.
    pub region: Option<String>,
    /// Endpoint URL.
    pub url: String,
}

/// User identity information.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct User {
    /// Domain the user belongs to.
    pub domain: Option<Domain>,
    /// User unique identifier.
    pub id: String,
    /// User name.
    pub name: String,
    // Note(gtema): some clouds return empty string instead of null when
    // password does not expire. It is technically possible to use
    // deserialize_with to capture errors, but that leads bincode to fail
    // when deserializing. For now just leave it as optional string instead
    // of DateTime
    // #[serde(deserialize_with = "deser_ok_or_default")]
    /// Optional password expiration date.
    pub password_expires_at: Option<String>,
}

/// Authorization project details.
///
/// While in the response `id` and `name` and mandatorily set this type is
/// also reused to manage authentications where at least one of them must be
/// present.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct Project {
    /// Associated domain for the project.
    #[builder(default)]
    pub domain: Option<Domain>,

    /// Project unique identifier.
    #[builder(default)]
    pub id: Option<String>,

    /// Project name.
    #[builder(default)]
    pub name: Option<String>,
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            #[derive(Serialize)]
            struct ProjectJson<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                domain: Option<&'a Domain>,
                #[serde(skip_serializing_if = "Option::is_none")]
                id: Option<&'a str>,
                #[serde(skip_serializing_if = "Option::is_none")]
                name: Option<&'a str>,
            }
            let helper = ProjectJson {
                domain: self.domain.as_ref(),
                id: self.id.as_deref(),
                name: self.name.as_deref(),
            };
            helper.serialize(serializer)
        } else {
            #[derive(Serialize)]
            struct ProjectRaw<'a> {
                domain: &'a Option<Domain>,
                id: &'a Option<String>,
                name: &'a Option<String>,
            }
            let helper = ProjectRaw {
                domain: &self.domain,
                id: &self.id,
                name: &self.name,
            };
            helper.serialize(serializer)
        }
    }
}

/// Domain identity information.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
#[serde(default)]
pub struct Domain {
    /// Domain unique identifier.
    #[builder(default)]
    pub id: Option<String>,

    /// Domain name.
    #[builder(default)]
    pub name: Option<String>,
}

impl Serialize for Domain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            #[derive(Serialize)]
            struct DomainJson<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: Option<&'a str>,
                #[serde(skip_serializing_if = "Option::is_none")]
                name: Option<&'a str>,
            }
            let helper = DomainJson {
                id: self.id.as_deref(),
                name: self.name.as_deref(),
            };
            helper.serialize(serializer)
        } else {
            #[derive(Serialize)]
            struct DomainRaw<'a> {
                id: &'a Option<String>,
                name: &'a Option<String>,
            }
            let helper = DomainRaw {
                id: &self.id,
                name: &self.name,
            };
            helper.serialize(serializer)
        }
    }
}

/// System Scope.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct System {
    /// Flag indicating if the system scope is all.
    #[builder(default)]
    pub all: Option<bool>,
}

/// Trust scope information.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct OsTrustTrust {
    /// Trust unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub id: Option<String>,
}

/// Multimodal authentication receipt response from the identity service.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuthReceiptResponse {
    /// The actual auth receipt data.
    pub receipt: AuthReceipt,
    /// Required authentication methods for the receipt.
    pub required_auth_methods: Vec<Vec<String>>,
    /// Token associated with this receipt.
    pub token: Option<String>,
}

/// Authentication receipt data returned when additional authentication methods are required.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuthReceipt {
    /// Catalog of available services.
    pub catalog: Option<Vec<ServiceEndpoints>>,
    /// Receipt expiration time.
    pub expires_at: TokenTimestamp,
    /// Receipt issue time.
    pub issued_at: Option<TokenTimestamp>,
    /// Authentication methods already completed.
    pub methods: Vec<String>,
    /// Roles assigned to the receipt.
    pub roles: Option<Vec<IdAndName>>,
    /// User information.
    pub user: User,
}

/// Application Credential information from the token
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct ApplicationCredential {
    /// The ID of the application credential.
    pub id: Option<String>,
    /// The name of the application credential.
    pub name: Option<String>,
    /// A flag indicating whether the application credential may be used for creation or destruction
    /// of other application credentials or trusts.
    pub restricted: Option<bool>,
}

/// Build [`AuthTokenScope`] from [`AuthResponse`]
impl From<&AuthResponse> for crate::authtoken_scope::AuthTokenScope {
    fn from(auth: &AuthResponse) -> Self {
        if let Some(project) = &auth.token.project {
            Self::Project(project.clone())
        } else if let Some(domain) = &auth.token.domain {
            Self::Domain(domain.clone())
        } else if let Some(system) = &auth.token.system {
            Self::System(system.clone())
        } else {
            Self::Unscoped
        }
    }
}

/// Authentication error response.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthErrorResponse {
    /// Error object.
    pub error: IdentityError,
}

/// Error object.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct IdentityError {
    /// Error code.
    pub code: u32,
    /// Error message.
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    fn ts(s: &str) -> TokenTimestamp {
        s.parse().expect("valid timestamp")
    }

    /// Timestamps as written by the chrono based implementation (and by
    /// Keystone) must keep deserializing into the same instant.
    #[test]
    fn token_info_accepts_legacy_chrono_timestamps() {
        let expected = ts("2026-03-04T05:06:07Z");
        for raw in [
            // Keystone style (microseconds, `Z`)
            "2026-03-04T05:06:07.000000Z",
            // chrono `DateTime<Utc>` serialization (variable precision, `Z`)
            "2026-03-04T05:06:07Z",
            "2026-03-04T05:06:07.000Z",
            // chrono `DateTime<Local>` / `to_rfc3339()` style offsets
            "2026-03-04T05:06:07+00:00",
            "2026-03-04T07:06:07+02:00",
        ] {
            let json = format!(
                r#"{{"expires_at":"{raw}","issued_at":"{raw}","user":{{"id":"u","name":"n"}}}}"#
            );
            let info: TokenInfo =
                serde_json::from_str(&json).unwrap_or_else(|e| panic!("{raw}: {e}"));
            assert_eq!(info.expires_at, expected, "{raw}");
            assert_eq!(info.issued_at, Some(expected), "{raw}");
        }
    }

    #[test]
    fn token_info_keeps_subsecond_precision() {
        let json =
            r#"{"expires_at":"2026-03-04T05:06:07.123456789Z","user":{"id":"u","name":"n"}}"#;
        let info: TokenInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.expires_at, ts("2026-03-04T05:06:07.123456789Z"));
        // And survives a serialize/deserialize cycle.
        let again: TokenInfo =
            serde_json::from_str(&serde_json::to_string(&info).unwrap()).unwrap();
        assert_eq!(again.expires_at, info.expires_at);
    }

    #[test]
    fn auth_receipt_accepts_legacy_local_offsets() {
        let json = r#"{"expires_at":"2026-03-04T07:06:07.5+02:00","issued_at":null,
            "methods":["password"],"user":{"id":"u","name":"n"}}"#;
        let receipt: AuthReceipt = serde_json::from_str(json).unwrap();
        assert_eq!(receipt.expires_at, ts("2026-03-04T05:06:07.5Z"));
        assert!(receipt.issued_at.is_none());
    }

    /// Conversion into other date/time libraries goes through [`SystemTime`] and is lossless.
    #[test]
    fn token_timestamp_converts_to_jiff() {
        let t = ts("2026-03-04T05:06:07.123456Z");
        let expected: jiff::Timestamp = "2026-03-04T05:06:07.123456Z".parse().unwrap();

        let via_system_time = jiff::Timestamp::try_from(SystemTime::from(t)).unwrap();
        assert_eq!(via_system_time, expected);

        let via_string: jiff::Timestamp = t.to_string().parse().unwrap();
        assert_eq!(via_string, expected);

        let via_seconds = jiff::Timestamp::from_second(t.as_second()).unwrap();
        assert_eq!(
            via_seconds,
            jiff::Timestamp::from_second(expected.as_second()).unwrap()
        );

        // and back
        let back = TokenTimestamp::from(SystemTime::from(t));
        assert_eq!(SystemTime::from(back), SystemTime::from(t));
        assert_eq!(back, t);
    }

    #[test]
    fn token_timestamp_helpers() {
        let future = TokenTimestamp::from(SystemTime::now() + Duration::from_secs(3600));
        assert!(future.remaining() > Duration::from_secs(3500));
        let past = TokenTimestamp::from(SystemTime::now() - Duration::from_secs(3600));
        assert_eq!(past.remaining(), Duration::ZERO);
        assert_eq!(
            ts("2026-03-04T05:06:07Z").checked_add(Duration::from_secs(60)),
            Some(ts("2026-03-04T05:07:07Z"))
        );
        assert!("garbage".parse::<TokenTimestamp>().is_err());
    }
}
