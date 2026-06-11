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
/// Utilities for testing `openstack_sdk` or resources based on the `openstack_sdk` in external
/// crates.
pub use openstack_sdk_core::test::*;

#[cfg(test)]
mod tests {
    //! End-to-end tests with mock identity/catalog service
    //!
    //! Tests exercise the full auth flow and corner cases including:
    //! - `new()` -> discovery -> auth -> catalog -> client
    //! - 401 re-authentication
    //! - Clone sharing after auth
    //! - Constructor failure paths

    #[cfg(all(feature = "sync", feature = "async"))]
    use httpmock::MockServer;
    #[cfg(all(feature = "sync", feature = "async"))]
    use reqwest::StatusCode;
    #[cfg(all(feature = "sync", feature = "async"))]
    use secrecy::ExposeSecret;

    #[cfg(all(feature = "sync", feature = "async"))]
    use openstack_sdk_core::api::{AsyncClient, Client};
    #[cfg(all(feature = "sync", feature = "async"))]
    use openstack_sdk_core::config::{Auth as ConfigAuth, CloudConfig};
    #[cfg(all(feature = "sync", feature = "async"))]
    use openstack_sdk_core::types::ServiceType;

    #[cfg(all(feature = "sync", feature = "async"))]
    use crate::AsyncOpenStack;
    #[cfg(all(feature = "sync", feature = "async"))]
    use crate::OpenStack;

    /// Shared helpers for mock identity setup.
    #[cfg(all(feature = "sync", feature = "async"))]
    mod helpers {
        use httpmock::MockServer;
        use openstack_sdk_core::config::{Auth as ConfigAuth, CloudConfig};
        use reqwest::StatusCode;
        use serde_json::json;
        pub fn mock_identity_catalog(server: &MockServer) {
            mock_identity_catalog_with_token(server, "test-token-from-catalog");
        }
        pub fn mock_identity_catalog_with_token(server: &MockServer, token: &str) {
            let base_url = server.base_url();
            server.mock(|when, then| {
                when.method(httpmock::Method::GET).path("/");
                then.status(StatusCode::OK).json_body(json!({
                    "versions": [{
                        "id": "v3", "status": "SUPPORTED",
                        "links": [{ "rel": "self", "href": format!("{base_url}/v3/") }]
                    }]
                }));
            });
            server.mock(|when, then| {
                when.method(httpmock::Method::GET).path("/v3/");
                then.status(StatusCode::OK).json_body(json!({
                    "versions": [{
                        "id": "v3", "status": "SUPPORTED",
                        "links": [{ "rel": "self", "href": format!("{base_url}/v3/") }]
                    }]
                }));
            });
            let expires = (chrono::Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
            server.mock(|when, then| {
                when.method(httpmock::Method::POST).path("/v3/auth/tokens");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", token)
                    .json_body(json!({
                        "token": {
                            "id": "token-id", "expires_at": expires,
                            "project": { "id": "test-project", "name": "TestProject" },
                            "user": { "id": "test-user", "name": "test-user" },
                            "methods": ["password"], "audit_ids": ["audit-1"],
                            "catalog": [
                                { "type": "identity", "name": "keystone", "endpoints": [{
                                    "id": "identity-1", "url": format!("{}/v3", base_url),
                                    "region": "RegionOne", "interface": "public"
                                }] },
                                { "type": "compute", "name": "nova", "endpoints": [{
                                    "id": "compute-1", "url": format!("{}/v2.1", base_url),
                                    "region": "RegionOne", "interface": "public"
                                }] },
                                { "type": "network", "name": "neutron", "endpoints": [{
                                    "id": "network-1", "url": format!("{}/v2.0", base_url),
                                    "region": "RegionOne", "interface": "public"
                                }] }
                            ]
                        }
                    }));
            });
        }
        pub fn create_test_cloud_config(server: &MockServer) -> CloudConfig {
            let base_url = server.base_url();
            CloudConfig {
                auth: Some(ConfigAuth {
                    auth_url: Some(format!("{}/v3", base_url)),
                    username: Some("test-user".into()),
                    user_domain_name: Some("Default".into()),
                    password: Some("test-password".into()),
                    project_id: Some("test-project".into()),
                    ..Default::default()
                }),
                region_name: Some("RegionOne".into()),
                interface: Some("public".into()),
                ..Default::default()
            }
        }
    }

    #[cfg(all(feature = "sync", feature = "async"))]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_async_client_full_auth_flow() {
        let server = MockServer::start_async().await;

        helpers::mock_identity_catalog(&server);

        let config = helpers::create_test_cloud_config(&server);

        let client = AsyncOpenStack::new_with_authentication_helper(
            &config,
            crate::auth::auth_helper::Noop::default(),
            false,
        )
        .await
        .expect("AsyncOpenStack client creation failed");

        // Verify auth token
        let token = client.get_auth_token();
        assert!(token.is_some(), "Auth token should be set");
        assert_eq!(token.unwrap().expose_secret(), "test-token-from-catalog");

        // Verify endpoint lookup for compute
        let ep = client
            .get_service_endpoint(&ServiceType::Compute, None)
            .await
            .expect("Compute endpoint lookup failed");

        assert!(
            ep.url_str().contains("v2.1"),
            "Compute endpoint should use v2.1"
        );

        // Verify endpoint lookup for network
        let net_ep = client
            .get_service_endpoint(&ServiceType::Network, None)
            .await
            .expect("Network endpoint lookup failed");

        assert!(
            net_ep.url_str().contains("v2.0"),
            "Network endpoint should use v2.0"
        );
    }

    #[cfg(all(feature = "sync", feature = "async"))]
    #[test]
    fn test_sync_client_full_auth_flow() {
        let server = MockServer::start();

        helpers::mock_identity_catalog(&server);

        let config = helpers::create_test_cloud_config(&server);

        let client = OpenStack::new(&config).expect("OpenStack client creation failed");

        // Verify auth token
        let token = client.get_auth_token();
        assert!(token.is_some(), "Auth token should be set");
        assert_eq!(token.unwrap().expose_secret(), "test-token-from-catalog");

        // Verify endpoint lookup for compute
        let ep = client
            .get_service_endpoint(&ServiceType::Compute, None)
            .expect("Compute endpoint lookup failed");

        assert!(
            ep.url_str().contains("v2.1"),
            "Compute endpoint should use v2.1"
        );
    }

    #[cfg(all(feature = "sync", feature = "async"))]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_concurrent_requests_after_full_auth() {
        let server = MockServer::start_async().await;
        let base_url = server.base_url();

        helpers::mock_identity_catalog(&server);

        // Mock generic REST endpoint
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path_prefix("/v2.1/");
            then.status(StatusCode::OK)
                .body(r#"{"servers": [{"id": "server1", "name": "test-server"}]}"#);
        });

        let config = helpers::create_test_cloud_config(&server);

        let client = AsyncOpenStack::new_with_authentication_helper(
            &config,
            crate::auth::auth_helper::Noop::default(),
            false,
        )
        .await
        .expect("AsyncOpenStack client creation failed");

        // Spawn 5 concurrent requests
        let futures = (0..5)
            .map(|i| {
                let client = client.clone();
                let base_url = base_url.clone();
                async move {
                    // Verify auth token is shared
                    assert_eq!(
                        client.get_auth_token().unwrap().expose_secret(),
                        "test-token-from-catalog"
                    );

                    // Verify endpoint lookup
                    let ep = client
                        .get_service_endpoint(&ServiceType::Compute, None)
                        .await
                        .unwrap();
                    assert!(ep.url_str().contains("v2.1"));

                    // Make REST call
                    let request = http::Request::builder()
                        .method(http::Method::GET)
                        .uri(format!("{}/v2.1/servers?i={}", base_url, i));

                    let result = client.rest_async(request, Vec::new()).await;
                    assert!(result.is_ok(), "REST call failed: {:?}", result);
                    assert_eq!(result.unwrap().status(), StatusCode::OK);
                }
            })
            .collect::<Vec<_>>();

        futures::future::join_all(futures).await;
    }

    #[cfg(all(feature = "sync", feature = "async"))]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_auth_cache_reuse() {
        let server = MockServer::start_async().await;

        helpers::mock_identity_catalog(&server);

        let config = helpers::create_test_cloud_config(&server);

        let client = AsyncOpenStack::new_with_authentication_helper(
            &config,
            crate::auth::auth_helper::Noop::default(),
            false,
        )
        .await
        .expect("AsyncOpenStack client creation failed");

        // First endpoint lookup
        let ep1 = client
            .get_service_endpoint(&ServiceType::Compute, None)
            .await
            .unwrap();

        // Second lookup should return same result
        let ep2 = client
            .get_service_endpoint(&ServiceType::Compute, None)
            .await
            .unwrap();

        assert_eq!(ep1.url_str(), ep2.url_str());

        // Auth token should remain consistent
        assert_eq!(
            client.get_auth_token().unwrap().expose_secret(),
            "test-token-from-catalog"
        );
    }

    /// Test 401 re-auth through the full  flow.
    ///
    /// Simulates a real-world scenario: token is valid at  time, but a subsequent
    /// API call returns 401, triggering automatic re-authentication.
    #[cfg(all(feature = "sync", feature = "async"))]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_401_reauth_with_full_flow_async() {
        let server = MockServer::start_async().await;
        let base_url = server.base_url();

        helpers::mock_identity_catalog(&server);

        // Compute API: first call returns 401, second with new token returns 200.
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.1/resource")
                .header("X-Auth-Token", "test-token-from-catalog");
            then.status(StatusCode::UNAUTHORIZED)
                .body(r#"{"error": {"message": "Unauthorized"}, "message": "Unauthorized"}"#);
        });

        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.1/resource")
                .header("X-Auth-Token", "test-token-from-catalog");
            then.status(StatusCode::OK).body(r#"{"result": "ok"}"#);
        });

        let config = helpers::create_test_cloud_config(&server);
        let client = AsyncOpenStack::new_with_authentication_helper(
            &config,
            crate::auth::auth_helper::Noop::default(),
            false,
        )
        .await
        .expect("AsyncOpenStack client creation failed");

        let auth_token_before = client.get_auth_token();
        assert!(auth_token_before.is_some());

        let _result = client
            .rest_async(
                http::Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("{}/v2.1/resource", base_url)),
                Vec::new(),
            )
            .await;

        let token_after = client.get_auth_token();
        assert!(token_after.is_some(), "Token should be set after re-auth");
        assert_eq!(
            token_after.unwrap().expose_secret(),
            "test-token-from-catalog"
        );
    }

    /// Test missing auth_url in constructor.
    #[cfg(all(feature = "sync", feature = "async"))]
    #[test]
    fn test_missing_auth_url_fails() {
        let config = CloudConfig {
            auth: Some(ConfigAuth {
                ..Default::default()
            }),
            region_name: Some("RegionOne".into()),
            ..Default::default()
        };
        let result = OpenStack::new(&config);
        assert!(result.is_err(), "Missing auth_url should fail");
    }

    /// Test missing auth data in constructor.
    #[cfg(all(feature = "sync", feature = "async"))]
    #[test]
    fn test_missing_auth_fails() {
        let config = CloudConfig {
            region_name: Some("RegionOne".into()),
            ..Default::default()
        };
        let result = OpenStack::new(&config);
        assert!(result.is_err(), "Missing auth should fail");
    }

    /// Test version discovery failure — identity server returns 404 on version discovery.
    #[cfg(all(feature = "sync", feature = "async"))]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_version_discovery_404_fails() {
        let server = MockServer::start_async().await;
        let base_url = server.base_url();

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/");
            then.status(StatusCode::NOT_FOUND)
                .body(r#"{"error": {"message": "Not Found"}, "message": "Not Found"}"#);
        });

        let config = CloudConfig {
            auth: Some(ConfigAuth {
                auth_url: Some(format!("{}/v3", base_url)),
                username: Some("test-user".into()),
                user_domain_name: Some("Default".into()),
                password: Some("test-password".into()),
                project_id: Some("test-project".into()),
                ..Default::default()
            }),
            region_name: Some("RegionOne".into()),
            interface: Some("public".into()),
            ..Default::default()
        };

        let result = AsyncOpenStack::new(&config).await;
        assert!(result.is_err(), "Version discovery 404 should fail");
    }

    /// Test that cloning the client after  produces a working copy.
    #[cfg(all(feature = "sync", feature = "async"))]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_clone_after_full_auth() {
        let server = MockServer::start_async().await;

        helpers::mock_identity_catalog(&server);

        let config = helpers::create_test_cloud_config(&server);
        let client = AsyncOpenStack::new(&config)
            .await
            .expect("AsyncOpenStack client creation failed");

        let clone = client.clone();

        let ep1 = client
            .get_service_endpoint(&ServiceType::Compute, None)
            .await
            .unwrap();
        let ep2 = clone
            .get_service_endpoint(&ServiceType::Compute, None)
            .await
            .unwrap();

        assert_eq!(ep1.url_str(), ep2.url_str());
        assert_eq!(
            client.get_auth_token().unwrap().expose_secret(),
            clone.get_auth_token().unwrap().expose_secret()
        );
    }

    /// Test that discover_service_endpoint + get_service_endpoint work without
    /// version discovery when auth catalog already provides the endpoint.
    #[cfg(all(feature = "sync", feature = "async"))]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_endpoint_from_catalog_only() {
        let server = MockServer::start_async().await;

        helpers::mock_identity_catalog(&server);

        let config = helpers::create_test_cloud_config(&server);
        let client = AsyncOpenStack::new(&config)
            .await
            .expect("AsyncOpenStack client creation failed");

        let ep = client
            .get_service_endpoint(&ServiceType::Compute, None)
            .await
            .unwrap();
        assert!(ep.url_str().contains("v2.1"));

        let result = client
            .get_service_endpoint(&ServiceType::from("nonexistent"), None)
            .await;
        assert!(result.is_err(), "Unknown service type should fail");
    }
}
