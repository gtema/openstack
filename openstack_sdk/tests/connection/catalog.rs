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
// limitations of the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Tests that verify catalog refresh behavior with a live OpenStack cloud.
//! These tests require `OS_CLOUD` environment variable to be set and are
//! marked `#[ignore]` to opt-in via `--ignored` flag.

use std::env;
use std::time::Instant;

use openstack_sdk::api::identity::v3::endpoint::create;
use openstack_sdk::api::identity::v3::endpoint::delete;
use openstack_sdk::api::identity::v3::service::create as service_create;
use openstack_sdk::api::identity::v3::service::delete as service_delete;
use openstack_sdk::api::{AsyncClient, Client, Query, QueryAsync, raw};
use openstack_sdk::types::ServiceType;
use openstack_sdk::{AsyncOpenStack, OpenStack, config::ConfigFile};
use std::borrow::Cow;

fn unique_suffix() -> String {
    format!(
        "{}-{}",
        Instant::now().elapsed().as_millis(),
        std::process::id()
    )
}

#[test]
fn sync_catalog_refresh_with_new_service() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigFile::new()?
        .get_cloud_config(&env::var("OS_CLOUD").unwrap_or_default())?
        .unwrap();
    let session = OpenStack::new(&config)?;

    if !session.get_auth_info().is_some_and(|auth| {
        auth.token
            .roles
            .is_some_and(|roles| roles.iter().any(|role| role.name == "admin"))
    }) {
        eprintln!("No admin role — skipping catalog refresh test");
        eprintln!(
            "Following roles are there {:?}",
            session
                .get_auth_info()
                .map(|auth| auth.token.roles.map(|roles| roles
                    .iter()
                    .map(|role| role.name.clone())
                    .collect::<Vec<_>>()))
        );
        return Ok(());
    }

    let suffix = unique_suffix();
    let svc_type = "search";
    let ep_url = format!("https://test-catalog-{}.openstack.local/", suffix);

    // Create service
    let svc_req = service_create::Request::builder()
        .service(
            service_create::ServiceBuilder::default()
                ._type(svc_type)
                .name("test-catalog")
                .build()
                .expect("service builder"),
        )
        .build()
        .expect("request builder");
    let svc_resp: serde_json::Value = svc_req.query(&session)?;
    let service_id = svc_resp["id"].as_str().expect("service id");

    // Create endpoint
    let ep_req = create::Request::builder()
        .endpoint(
            create::EndpointBuilder::default()
                .interface(create::Interface::Public)
                .service_id(service_id)
                .region_id(config.region_name.map(Into::into))
                .url(ep_url.clone())
                .build()
                .expect("endpoint builder"),
        )
        .build()
        .expect("request builder");
    let ep_resp: serde_json::Value = ep_req.query(&session)?;
    let endpoint_id = ep_resp["id"].as_str().expect("endpoint id");

    // Verify the new service is discoverable via catalog refresh (auto-refresh in get_service_endpoint)
    let found = session
        .get_service_endpoint(&ServiceType::Other(svc_type.to_string()), None)
        .is_ok();
    assert!(
        found,
        "test service endpoint should be found in catalog after auto-refresh"
    );

    // Cleanup
    let _del_ep = delete::Request::builder()
        .id(endpoint_id)
        .build()
        .expect("delete request builder");
    let _del_ep = raw(_del_ep).query(&session)?;

    let _del_svc = service_delete::Request::builder()
        .id(service_id)
        .build()
        .expect("delete request builder");
    let _del_svc = raw(_del_svc).query(&session)?;

    Ok(())
}

#[tracing_test::traced_test]
#[tokio::test]
async fn async_catalog_refresh_with_new_service() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigFile::new()?
        .get_cloud_config(&env::var("OS_CLOUD").unwrap_or_default())?
        .unwrap();
    let session = AsyncOpenStack::new(&config).await?;

    if !session.get_auth_info().is_some_and(|auth| {
        auth.token
            .roles
            .is_some_and(|roles| roles.iter().any(|role| role.name == "admin"))
    }) {
        eprintln!("No admin role — skipping catalog refresh test");
        return Ok(());
    }

    let suffix = unique_suffix();
    let svc_type = "search";
    let ep_url = format!("https://test-catalog-{}.openstack.local/", suffix);

    // Create service
    let svc_req = service_create::Request::builder()
        .service(
            service_create::ServiceBuilder::default()
                ._type(svc_type)
                .name("test-catalog")
                .build()
                .expect("service builder"),
        )
        .build()
        .expect("request builder");
    let svc_resp: serde_json::Value = svc_req.query_async(&session).await?;
    let service_id = svc_resp["id"].as_str().expect("service id");

    // Create endpoint
    let ep_req = create::Request::builder()
        .endpoint(
            create::EndpointBuilder::default()
                .interface(create::Interface::Public)
                .service_id(service_id)
                .url(ep_url.clone())
                .region_id(config.region_name.map(Into::into))
                .build()
                .expect("endpoint builder"),
        )
        .build()
        .expect("request builder");
    let ep_resp: serde_json::Value = ep_req.query_async(&session).await?;
    let endpoint_id = ep_resp["id"].as_str().expect("endpoint id");

    // Verify the new service is discoverable via catalog refresh (auto-refresh in get_service_endpoint)
    let found = session
        .get_service_endpoint(&ServiceType::Other(svc_type.to_string()), None)
        .await
        .is_ok();

    // Cleanup
    let del_ep = delete::Request::builder()
        .id(endpoint_id)
        .build()
        .expect("delete request builder");
    let _del_ep = raw(del_ep).query_async(&session).await?;

    let del_svc = service_delete::Request::builder()
        .id(service_id)
        .build()
        .expect("delete request builder");
    let _del_svc = raw(del_svc).query_async(&session).await?;

    assert!(
        found,
        "test service endpoint should be found in catalog after auto-refresh"
    );

    Ok(())
}
