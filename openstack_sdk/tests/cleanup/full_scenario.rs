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

//! Exercises the project-cleanup engine against a real OpenStack cloud
//! across all four built-in providers (network, compute, block-storage,
//! image) in one related scenario, proving the engine's relation-aware
//! cross-resource ordering with real resources — not just per-provider
//! CRUD. Run against devstack in CI (`.github/workflows/functional.yml`)
//! and manually against a real cloud during development (`OS_CLOUD` +
//! optionally `TEST_IMAGE_NAME`).

use std::env;

use openstack_sdk::cleanup::{
    BlockStorageCleanupProvider, ComputeCleanupProvider, ImageCleanupProvider,
    NetworkCleanupProvider, PlannedResource, ProjectCleanupBuilder,
};
use openstack_sdk::types::ServiceType;
use openstack_sdk::{AsyncOpenStack, config::ConfigFile};

use super::helpers::{
    CreatedResources, create_network, create_router_with_interface, create_server_from_volume,
    create_snapshot, create_subnet, create_volume_from_image, first_flavor_id,
    resolve_test_image_id, wait_for_server_active, wait_for_volume_available,
};

#[tokio::test(flavor = "multi_thread")]
async fn cleanup_engine_removes_related_resources_across_all_services()
-> Result<(), Box<dyn std::error::Error>> {
    let cfg = ConfigFile::new().unwrap();
    let profile = cfg
        .get_cloud_config(env::var("OS_CLOUD").expect("OS_CLOUD variable set"))
        .unwrap()
        .unwrap();
    let client = AsyncOpenStack::new(&profile).await?;
    // Per-service endpoint version discovery is not automatic (identity is
    // the only exception, handled during `connect()`) - every service this
    // scenario touches needs it done once up front, same as generated CLI
    // modules do (e.g. `cli/network/src/v2.rs`) and as
    // `tests/connection/async.rs` demonstrates for a single service. Skipping
    // this works against devstack (whose catalog endpoints are already
    // version-prefixed) but 404s against a real cloud whose catalog
    // endpoints are unversioned roots.
    client
        .discover_service_endpoint(&ServiceType::Network)
        .await?;
    client
        .discover_service_endpoint(&ServiceType::Compute)
        .await?;
    client
        .discover_service_endpoint(&ServiceType::BlockStorage)
        .await?;
    client
        .discover_service_endpoint(&ServiceType::Image)
        .await?;

    let marker = format!("osc-cleanup-test-{}", uuid::Uuid::new_v4());
    let mut created = CreatedResources::new(&client);

    // 1. Network + subnet + router-with-interface.
    let network_id = create_network(&client, &format!("{marker}-net")).await?;
    created.track("network", network_id.clone());
    let subnet_id = create_subnet(
        &client,
        &format!("{marker}-subnet"),
        &network_id,
        "10.250.0.0/24",
    )
    .await?;
    let router_id =
        create_router_with_interface(&client, &format!("{marker}-router"), &subnet_id).await?;
    created.track("network-router", format!("{router_id}:{subnet_id}"));

    // 2. Resolve the pre-existing boot image (never created/deleted by
    //    this test - see `resolve_test_image_id`'s doc comment).
    let image_id = resolve_test_image_id(&client).await?;

    // 3. Volume from that image, then a snapshot of it.
    let volume_id =
        create_volume_from_image(&client, &format!("{marker}-vol"), &image_id, 10).await?;
    created.track("block-storage-volume", volume_id.clone());
    wait_for_volume_available(&client, &volume_id).await?;
    let snapshot_id = create_snapshot(&client, &format!("{marker}-snap"), &volume_id).await?;
    created.track("block-storage-snapshot", snapshot_id.clone());

    // 4. Server booted from that volume, attached to the created network.
    let flavor_id = first_flavor_id(&client).await?;
    let server_id = create_server_from_volume(
        &client,
        &format!("{marker}-server"),
        &flavor_id,
        &volume_id,
        &network_id,
    )
    .await?;
    created.track("compute", server_id.clone());
    wait_for_server_active(&client, &server_id).await?;

    // 5. Run the real cleanup engine, selecting everything whose name
    //    carries this test's marker. The network's subnet and
    //    router-interface have no `name` field of their own but are
    //    pulled in by the engine's existing `CascadeGroup` rules once
    //    the network itself is selected (same mechanism the network
    //    provider's own unit tests already exercise against httpmock).
    //    The pre-existing boot image is never selected: its name carries
    //    no marker.
    let cleanup = ProjectCleanupBuilder::new(&client)
        .with_provider(NetworkCleanupProvider)
        .with_provider(ComputeCleanupProvider)
        .with_provider(BlockStorageCleanupProvider)
        .with_provider(ImageCleanupProvider)
        .build();

    let marker_for_eval = marker.clone();
    let eval: openstack_sdk::cleanup::provider::EvaluationFn =
        std::sync::Arc::new(move |r: &PlannedResource| {
            r.name
                .as_deref()
                .is_some_and(|n| n.starts_with(&marker_for_eval))
        });

    let plan = cleanup
        .discover(std::collections::HashMap::new(), Some(eval))
        .await
        .expect("discover failed");
    let result = cleanup.apply(plan).await.expect("apply failed");

    assert!(
        result.errors.is_empty(),
        "unexpected cleanup errors: {:?}",
        result.errors
    );

    for id in [
        &network_id,
        &subnet_id,
        &router_id,
        &volume_id,
        &snapshot_id,
        &server_id,
    ] {
        assert!(
            result.deleted_ids.contains(id),
            "expected {id} to be deleted, deleted_ids = {:?}",
            result.deleted_ids
        );
    }

    let pos = |id: &str| {
        result
            .deleted_ids
            .iter()
            .position(|d| d == id)
            .unwrap_or_else(|| panic!("{id} missing from deleted_ids"))
    };
    assert!(
        pos(&snapshot_id) < pos(&volume_id),
        "snapshot must delete before its volume"
    );
    assert!(
        pos(&server_id) < pos(&volume_id),
        "server must delete before its attached volume"
    );
    assert!(
        pos(&subnet_id) < pos(&network_id),
        "subnet must delete before its network"
    );
    assert!(
        pos(&router_id) < pos(&network_id),
        "router interface must delete (and detach the router) before its network"
    );

    // Everything above was deleted by the engine under test; `created`'s
    // `Drop` runs now and its deletes are expected to no-op against
    // already-gone resources.
    drop(created);

    Ok(())
}
