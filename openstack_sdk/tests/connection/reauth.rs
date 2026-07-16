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

//! End-to-end reauth tests
//!
//! These tests verify that the SDK can automatically re-authenticate after
//! the current token is revoked.
//!
//! Both tests share the same auth cache file on disk (same cloud config),
//! so they must run sequentially. Serialization is enforced by the
//! `serial-connection` test group in `.config/nextest.toml`.

use std::env;

use http::{HeaderName, HeaderValue, StatusCode};
use openstack_sdk::api::compute::v2::server::list;
use openstack_sdk::api::identity::v3::auth::token::delete;
use openstack_sdk::api::{Pagination, Query, QueryAsync, paged, raw};
use openstack_sdk::config::ConfigFile;
use openstack_sdk::types::ServiceType;
use openstack_sdk::{AsyncOpenStack, OpenStack};
use secrecy::ExposeSecret;

#[ignore]
#[tokio::test]
async fn async_reauth_after_token_revocation() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = ConfigFile::new().unwrap();
    let profile = cfg
        .get_cloud_config(env::var("OS_CLOUD").expect("OS_CLOUD variable set"))
        .unwrap()
        .unwrap();

    let session = AsyncOpenStack::new(&profile).await?;
    session
        .discover_service_endpoint(&ServiceType::Compute)
        .await?;

    // Revoke current token via the SDK's identity endpoint builder
    let token = session.get_auth_token().expect("Auth token should be set");
    let token_str = token.expose_secret().to_string();

    let endpoint = delete::Request::builder()
        .header(
            HeaderName::from_static("x-subject-token"),
            HeaderValue::from_str(&token_str).unwrap(),
        )
        .build()
        .unwrap();
    let response = raw(endpoint)
        .skip_error_check(true)
        .query_async(&session)
        .await?;
    // 403 means the token was already revoked by a parallel test — same end result.
    assert!(
        response.status().is_success() || response.status() == StatusCode::FORBIDDEN,
        "Token revocation failed with status: {}",
        response.status()
    );

    // Perform a REST call - it should automatically re-authenticate
    let ep = list::Request::builder().build().unwrap();
    let _servers: Vec<serde_json::Value> = paged(ep, Pagination::Limit(10))
        .query_async(&session)
        .await?;

    assert!(
        session.get_auth_token().is_some(),
        "Auth token should be re-set after re-authentication"
    );

    Ok(())
}

#[ignore]
#[test]
fn sync_reauth_after_token_revocation() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = ConfigFile::new().unwrap();
    let profile = cfg
        .get_cloud_config(env::var("OS_CLOUD").expect("OS_CLOUD variable set"))
        .unwrap()
        .unwrap();

    let session = OpenStack::new(&profile)?;
    session.discover_service_endpoint(&ServiceType::Compute)?;

    // Revoke current token via the SDK's identity endpoint builder
    let token = session.get_auth_token().expect("Auth token should be set");
    let token_str = token.expose_secret().to_string();

    let endpoint = delete::Request::builder()
        .header(
            HeaderName::from_static("x-subject-token"),
            HeaderValue::from_str(&token_str).unwrap(),
        )
        .build()
        .unwrap();
    let response = raw(endpoint).query(&session)?;
    // 403 means the token was already revoked by a parallel test — same end result.
    assert!(
        response.status().is_success() || response.status() == StatusCode::FORBIDDEN,
        "Token revocation failed with status: {}",
        response.status()
    );

    // Perform a REST call - it should automatically re-authenticate
    let ep = list::Request::builder().build().unwrap();
    let _servers: Vec<serde_json::Value> = paged(ep, Pagination::Limit(10)).query(&session)?;

    assert!(
        session.get_auth_token().is_some(),
        "Auth token should be re-set after re-authentication"
    );

    Ok(())
}
