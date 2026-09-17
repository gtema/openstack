# Cleanup Provider Gap Coverage Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the resource-coverage gaps identified in the existing `network`, `compute`, and `block-storage` cleanup providers: floating IPs and security groups (network), keypairs (compute), and volume backups (block-storage).

**Architecture:** Each gap is a new `ResourceKind` added to an *existing* provider file (no new provider structs, no new files) — same `discover()`/`delete()`/`relations()` shape already used by every kind in that file. New kinds slot into `discover()`'s existing `Vec<PlannedResource>` and into `delete()`'s existing `if resource.kind == X` chain. Relations follow the two patterns already established: same-service `Blocks`/`CascadeGroup` pairs (like `VOLUME`/`SNAPSHOT`), and `#[cfg(feature = "...")]`-gated cross-provider rules (like `PORT Blocks SERVER`).

**Tech Stack:** Rust, `async_trait`, `serde_json::Value` as the untyped resource payload, `httpmock` for provider unit tests, existing `paged`/`raw`/`QueryAsync` SDK plumbing.

## Global Constraints

- Every new `ResourceKind` uses `ResourceKind::new(<service_type>, <kind>)` matching the file's existing `service_type()` string (`"network"`, `"compute"`, `"block-storage"`) — never invent a new service_type string for these.
- Every new resource kind gets: a `pub const` `ResourceKind`, a `discover()` list call appended to the existing function, a `delete()` branch appended to the existing `if/else if` chain, and provider unit test(s) using the existing `mock_client` helper already in that file — do not duplicate `mock_client`.
- Match the exact request/response module paths and field names verified against the generated SDK crates in this repo (`sdk/network/src/v2/...`, `sdk/compute/src/v2/...`, `sdk/block-storage/src/v3/...`) — every path below has been confirmed to exist. Do not guess at alternate names.
- Security group default-SG exclusion is mandatory: Neutron auto-creates one `security_group` per project named exactly `"default"` and refuses to delete it (400 `SecurityGroupCannotRemoveDefault`). `discover()` must never emit it as a node.
- Nova's keypair APIs (`sdk/compute/src/v2/keypair/{list_20,delete_20}.rs`) key on **id** in this codebase's generated `Request` builder (`.id(...)`), even though the underlying Nova endpoint is name-keyed (`/os-keypairs/{id}` where the path segment happens to be the keypair's name) — pass the keypair's `id` field (which equals its `name` in the API response) exactly as the existing generated builder expects, the same way every other provider already does (`.id(resource.id.clone())`).
- No new Cargo features. Floating IP and security group live behind the existing `network` feature; keypair behind `compute`; backup behind `block_storage` (note the crate's feature is spelled `block_storage`, underscore — confirm against `openstack_sdk/Cargo.toml` before writing any `#[cfg(feature = "...")]`, never assume the hyphenated `service_type()` string is also the feature name).
- Every task ends by running `cargo build -p openstack_sdk --all-features --tests` and `cargo test -p openstack_sdk --all-features --lib cleanup::` — both must be clean before moving to the next task.
- Do not touch `openstack_sdk/tests/cleanup/full_scenario.rs` or `helpers.rs` in this plan — the functional test is out of scope; these are unit-test-covered provider additions only.

---

### Task 1: Floating IP in network provider

**Files:**
- Modify: `openstack_sdk/src/cleanup/providers/network.rs`

**Interfaces:**
- Produces: `pub const FLOATINGIP: ResourceKind = ResourceKind::new("network", "floatingip");` — later tasks (none in this plan) could reference it via `crate::cleanup::providers::network::FLOATINGIP`.
- Consumes: existing `value_str`, `to_planned`, `list_err` closure pattern, `paged`/`raw`/`QueryAsync` imports already present in this file.

Floating IPs have no delete-blocking relation in this codebase: Neutron allows deleting a floating IP regardless of whether it's still associated to a port (it simply gets disassociated first internally). So no new `RelationRule` — same "no relations" reasoning `image.rs` already documents for images.

- [ ] **Step 1: Add the `FLOATINGIP` resource kind constant**

Add directly below the existing kind constants (after `openstack_sdk/src/cleanup/providers/network.rs:64`, the `PORT` constant):

```rust
pub const FLOATINGIP: ResourceKind = ResourceKind::new("network", "floatingip");
```

- [ ] **Step 2: Add the failing discover test**

Add to the `mod tests` block (after the existing `discover_classifies_ports_correctly` test):

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn discover_maps_floating_ips_to_planned_resources() {
    let server = MockServer::start_async().await;
    let client = mock_client(&server).await;

    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/networks");
        then.status(200)
            .json_body(serde_json::json!({"networks": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/subnets");
        then.status(200)
            .json_body(serde_json::json!({"subnets": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/routers");
        then.status(200)
            .json_body(serde_json::json!({"routers": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/ports");
        then.status(200)
            .json_body(serde_json::json!({"ports": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/floatingips");
        then.status(200).json_body(serde_json::json!({"floatingips": [
            {"id": "fip-1", "floating_ip_address": "203.0.113.5", "name": null}
        ]}));
    });

    let cleanup = ProjectCleanupBuilder::new(&client)
        .with_provider(NetworkCleanupProvider)
        .build();

    let plan = cleanup
        .discover(HashMap::new(), None)
        .await
        .expect("discover failed");

    let node = plan.nodes.iter().find(|n| n.id == "fip-1").unwrap();
    assert_eq!(node.kind, FLOATINGIP);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delete_issues_floatingip_delete_request() {
    let server = MockServer::start_async().await;
    let client = mock_client(&server).await;

    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/networks");
        then.status(200)
            .json_body(serde_json::json!({"networks": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/subnets");
        then.status(200)
            .json_body(serde_json::json!({"subnets": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/routers");
        then.status(200)
            .json_body(serde_json::json!({"routers": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/ports");
        then.status(200)
            .json_body(serde_json::json!({"ports": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/floatingips");
        then.status(200).json_body(serde_json::json!({"floatingips": [
            {"id": "fip-1", "floating_ip_address": "203.0.113.5", "name": null}
        ]}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::DELETE)
            .path("/v2.0/floatingips/fip-1");
        then.status(204);
    });

    let cleanup = ProjectCleanupBuilder::new(&client)
        .with_provider(NetworkCleanupProvider)
        .build();

    let eval: crate::cleanup::provider::EvaluationFn =
        std::sync::Arc::new(|r: &PlannedResource| r.kind == FLOATINGIP);
    let plan = cleanup
        .discover(HashMap::new(), Some(eval))
        .await
        .expect("discover failed");
    let result = cleanup.apply(plan).await.expect("apply failed");

    assert!(
        result.errors.is_empty(),
        "unexpected errors: {:?}",
        result.errors
    );
    assert!(result.deleted_ids.contains(&"fip-1".to_string()));
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p openstack_sdk --all-features --lib cleanup::providers::network -- discover_maps_floating_ips_to_planned_resources delete_issues_floatingip_delete_request`
Expected: FAIL (compile error — `FLOATINGIP` undefined, `/v2.0/floatingips` GET mock never called since `discover()` doesn't list it).

- [ ] **Step 4: Add the `FLOATINGIP` constant (from Step 1) and wire discover/delete**

In `discover()`, after the existing ports loop (ends around the closing `}` following the `nodes.push(to_planned(PORT, port_v));` block, i.e. right before the final `Ok(nodes)` in this function), add:

```rust
let floatingips: Vec<Value> = paged(
    floatingip::list::Request::builder().build().map_err(|e| {
        CleanupError::Engine(format!("failed to build floating ip list request: {e}"))
    })?,
    Pagination::All,
)
.query_async(ctx.client)
.await
.map_err(|e| list_err(FLOATINGIP)(e.into()))?;
for v in floatingips {
    nodes.push(to_planned(FLOATINGIP, v));
}
```

Add the import alongside the other `network::v2` imports near the top of the file:

```rust
use crate::api::network::v2::floatingip;
```

In `delete()`, add a new branch before the final `else` (after the existing `else if resource.kind == PORT { ... }` block):

```rust
} else if resource.kind == FLOATINGIP {
    let req = floatingip::delete::Request::builder()
        .id(resource.id.clone())
        .build()
        .map_err(|e| {
            CleanupError::Engine(format!("failed to build floating ip delete request: {e}"))
        })?;
    raw(req)
        .query_async(ctx.client)
        .await
        .map_err(|e| err(e.into()))?;
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p openstack_sdk --all-features --lib cleanup::providers::network`
Expected: PASS, all tests in the module including the two new ones and the pre-existing four.

- [ ] **Step 6: Commit**

```bash
git add openstack_sdk/src/cleanup/providers/network.rs
git commit -m "feat(cleanup): add floating IP support to network provider"
```

---

### Task 2: Security groups + rules in network provider

**Files:**
- Modify: `openstack_sdk/src/cleanup/providers/network.rs`

**Interfaces:**
- Produces: `pub const SECURITY_GROUP: ResourceKind = ResourceKind::new("network", "security_group");` and `pub const SECURITY_GROUP_RULE: ResourceKind = ResourceKind::new("network", "security_group_rule");`
- Consumes: `FLOATINGIP` const and `floatingip` import added in Task 1 are unrelated — this task only needs the file's pre-existing shared helpers (`value_str`, `to_planned`) and the `RelationEffect`/`RelationRule` imports already present.

Relation shape: a security group rule references its parent security group via `security_group_id`. Neutron blocks deleting a security group while it still has rules? No — Neutron actually allows deleting a security group directly; it cascades its own rules server-side. But cleanup must still delete `SECURITY_GROUP_RULE` nodes if independently selected, and if a group is selected, its rules should be cascaded so they don't linger as orphaned plan nodes reported as "still there." Use the same `Blocks` + `CascadeGroup` pair already established for `VOLUME`/`SNAPSHOT` and `NETWORK`/`SUBNET`.

Security groups also block deletion while still referenced by a live `PORT`'s `security_groups` array — add a `SECURITY_GROUP Blocks PORT` rule (mirroring the existing `NETWORK Blocks PORT` shape) so a port's security groups aren't deleted out from under it. Do not add a `CascadeGroup` for this pair (deleting a port must not imply deleting its security groups — same reasoning `block_storage.rs` gives for not cascading `VOLUME`→`SERVER`).

**The default security group must never become a node.** Filter it out of `discover()` by `name == "default"`, exactly as documented below.

- [ ] **Step 1: Add the two resource kind constants**

After the `FLOATINGIP` constant added in Task 1:

```rust
pub const SECURITY_GROUP: ResourceKind = ResourceKind::new("network", "security_group");
pub const SECURITY_GROUP_RULE: ResourceKind = ResourceKind::new("network", "security_group_rule");
```

- [ ] **Step 2: Add the failing tests**

Add to `mod tests`:

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn discover_excludes_default_security_group_and_cascades_rules() {
    let server = MockServer::start_async().await;
    let client = mock_client(&server).await;

    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/networks");
        then.status(200)
            .json_body(serde_json::json!({"networks": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/subnets");
        then.status(200)
            .json_body(serde_json::json!({"subnets": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/routers");
        then.status(200)
            .json_body(serde_json::json!({"routers": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/ports");
        then.status(200)
            .json_body(serde_json::json!({"ports": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/floatingips");
        then.status(200)
            .json_body(serde_json::json!({"floatingips": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/security-groups");
        then.status(200).json_body(serde_json::json!({"security_groups": [
            {"id": "sg-default", "name": "default"},
            {"id": "sg-custom", "name": "web"}
        ]}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/security-group-rules");
        then.status(200).json_body(serde_json::json!({"security_group_rules": [
            {"id": "rule-1", "security_group_id": "sg-custom"}
        ]}));
    });

    let cleanup = ProjectCleanupBuilder::new(&client)
        .with_provider(NetworkCleanupProvider)
        .build();

    let eval: crate::cleanup::provider::EvaluationFn =
        std::sync::Arc::new(|r: &PlannedResource| {
            r.kind == SECURITY_GROUP && r.name.as_deref() == Some("web")
        });

    let plan = cleanup
        .discover(HashMap::new(), Some(eval))
        .await
        .expect("discover failed");

    assert!(
        plan.nodes.iter().all(|n| n.id != "sg-default"),
        "default security group must never be a node"
    );
    let sg = plan.nodes.iter().find(|n| n.id == "sg-custom").unwrap();
    let rule = plan.nodes.iter().find(|n| n.id == "rule-1").unwrap();
    assert!(sg.selected);
    assert!(rule.selected, "rule must be pulled in by the cascade group");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delete_issues_security_group_and_rule_delete_requests() {
    let server = MockServer::start_async().await;
    let client = mock_client(&server).await;

    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/networks");
        then.status(200)
            .json_body(serde_json::json!({"networks": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/subnets");
        then.status(200)
            .json_body(serde_json::json!({"subnets": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/routers");
        then.status(200)
            .json_body(serde_json::json!({"routers": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/ports");
        then.status(200)
            .json_body(serde_json::json!({"ports": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/floatingips");
        then.status(200)
            .json_body(serde_json::json!({"floatingips": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/security-groups");
        then.status(200).json_body(serde_json::json!({"security_groups": [
            {"id": "sg-custom", "name": "web"}
        ]}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.0/security-group-rules");
        then.status(200).json_body(serde_json::json!({"security_group_rules": [
            {"id": "rule-1", "security_group_id": "sg-custom"}
        ]}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::DELETE)
            .path("/v2.0/security-group-rules/rule-1");
        then.status(204);
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::DELETE)
            .path("/v2.0/security-groups/sg-custom");
        then.status(204);
    });

    let cleanup = ProjectCleanupBuilder::new(&client)
        .with_provider(NetworkCleanupProvider)
        .build();

    let eval: crate::cleanup::provider::EvaluationFn =
        std::sync::Arc::new(|r: &PlannedResource| r.kind == SECURITY_GROUP);
    let plan = cleanup
        .discover(HashMap::new(), Some(eval))
        .await
        .expect("discover failed");
    let result = cleanup.apply(plan).await.expect("apply failed");

    assert!(
        result.errors.is_empty(),
        "unexpected errors: {:?}",
        result.errors
    );
    let rule_pos = result
        .deleted_ids
        .iter()
        .position(|id| id == "rule-1")
        .unwrap();
    let sg_pos = result
        .deleted_ids
        .iter()
        .position(|id| id == "sg-custom")
        .unwrap();
    assert!(rule_pos < sg_pos, "rule must delete before its security group");
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p openstack_sdk --all-features --lib cleanup::providers::network -- discover_excludes_default_security_group_and_cascades_rules delete_issues_security_group_and_rule_delete_requests`
Expected: FAIL to compile (`SECURITY_GROUP`/`SECURITY_GROUP_RULE` undefined).

- [ ] **Step 4: Implement**

Add imports near the top, alongside the other `network::v2` imports:

```rust
use crate::api::network::v2::security_group;
use crate::api::network::v2::security_group_rule;
```

In `relations()`, append after the existing `PORT` `CascadeGroup` rule (the last element of the current `vec![...]`):

```rust
RelationRule {
    parent_kind: SECURITY_GROUP,
    child_kind: SECURITY_GROUP_RULE,
    matches: |child, parent| {
        value_str(&child.raw, "security_group_id") == value_str(&parent.raw, "id")
    },
    effect: RelationEffect::Blocks,
},
RelationRule {
    parent_kind: SECURITY_GROUP,
    child_kind: SECURITY_GROUP_RULE,
    matches: |child, parent| {
        value_str(&child.raw, "security_group_id") == value_str(&parent.raw, "id")
    },
    effect: RelationEffect::CascadeGroup,
},
RelationRule {
    parent_kind: SECURITY_GROUP,
    child_kind: PORT,
    matches: |child, parent| {
        child
            .raw
            .get("security_groups")
            .and_then(|v| v.as_array())
            .is_some_and(|groups| {
                groups
                    .iter()
                    .any(|g| g.as_str() == value_str(&parent.raw, "id"))
            })
    },
    effect: RelationEffect::Blocks,
},
```

In `discover()`, after the floating-ip block added in Task 1, add:

```rust
// Neutron auto-creates exactly one security group per project named
// "default" and refuses to delete it (400 SecurityGroupCannotRemoveDefault)
// — it must never become a cleanup candidate.
let security_groups: Vec<Value> = paged(
    security_group::list::Request::builder().build().map_err(|e| {
        CleanupError::Engine(format!("failed to build security group list request: {e}"))
    })?,
    Pagination::All,
)
.query_async(ctx.client)
.await
.map_err(|e| list_err(SECURITY_GROUP)(e.into()))?;
for v in security_groups {
    if value_str(&v, "name") == Some("default") {
        continue;
    }
    nodes.push(to_planned(SECURITY_GROUP, v));
}

let security_group_rules: Vec<Value> = paged(
    security_group_rule::list::Request::builder()
        .build()
        .map_err(|e| {
            CleanupError::Engine(format!(
                "failed to build security group rule list request: {e}"
            ))
        })?,
    Pagination::All,
)
.query_async(ctx.client)
.await
.map_err(|e| list_err(SECURITY_GROUP_RULE)(e.into()))?;
for v in security_group_rules {
    nodes.push(to_planned(SECURITY_GROUP_RULE, v));
}
```

In `delete()`, add before the final `else`:

```rust
} else if resource.kind == SECURITY_GROUP_RULE {
    let req = security_group_rule::delete::Request::builder()
        .id(resource.id.clone())
        .build()
        .map_err(|e| {
            CleanupError::Engine(format!(
                "failed to build security group rule delete request: {e}"
            ))
        })?;
    raw(req)
        .query_async(ctx.client)
        .await
        .map_err(|e| err(e.into()))?;
} else if resource.kind == SECURITY_GROUP {
    let req = security_group::delete::Request::builder()
        .id(resource.id.clone())
        .build()
        .map_err(|e| {
            CleanupError::Engine(format!("failed to build security group delete request: {e}"))
        })?;
    raw(req)
        .query_async(ctx.client)
        .await
        .map_err(|e| err(e.into()))?;
```

(Insert these two branches together, before the pre-existing `FLOATINGIP` branch or after it — order among branches doesn't matter, only that all are before the final `else`.)

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p openstack_sdk --all-features --lib cleanup::providers::network`
Expected: PASS, all tests including the two new ones.

- [ ] **Step 6: Commit**

```bash
git add openstack_sdk/src/cleanup/providers/network.rs
git commit -m "feat(cleanup): add security group and rule support to network provider"
```

---

### Task 3: Keypairs in compute provider

**Files:**
- Modify: `openstack_sdk/src/cleanup/providers/compute.rs`

**Interfaces:**
- Produces: `pub const KEYPAIR: ResourceKind = ResourceKind::new("compute", "keypair");`
- Consumes: existing `value_str`, `to_planned`, `list_err`, imports already in this file.

Keypairs have no delete-blocking relation: Nova allows deleting a keypair while servers still reference it by name (it only matters at boot time). No new `RelationRule` needed — `relations()`'s existing `#[cfg(feature = "network")]`-gated `Vec` is untouched.

- [ ] **Step 1: Add the resource kind constant**

After the existing `pub const SERVER: ResourceKind = ...` (`openstack_sdk/src/cleanup/providers/compute.rs:46`):

```rust
pub const KEYPAIR: ResourceKind = ResourceKind::new("compute", "keypair");
```

- [ ] **Step 2: Add the failing tests**

Add to `mod tests`:

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn discover_maps_keypairs_to_planned_resources() {
    let server = MockServer::start_async().await;
    let client = mock_client(&server).await;

    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.1/servers");
        then.status(200)
            .json_body(serde_json::json!({"servers": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.1/os-keypairs");
        then.status(200).json_body(serde_json::json!({"keypairs": [
            {"keypair": {"id": "test-key", "name": "test-key"}}
        ]}));
    });

    let cleanup = ProjectCleanupBuilder::new(&client)
        .with_provider(ComputeCleanupProvider)
        .build();

    let plan = cleanup
        .discover(HashMap::new(), None)
        .await
        .expect("discover failed");

    let node = plan.nodes.iter().find(|n| n.id == "test-key").unwrap();
    assert_eq!(node.kind, KEYPAIR);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delete_issues_keypair_delete_request() {
    let server = MockServer::start_async().await;
    let client = mock_client(&server).await;

    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.1/servers");
        then.status(200)
            .json_body(serde_json::json!({"servers": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/v2.1/os-keypairs");
        then.status(200).json_body(serde_json::json!({"keypairs": [
            {"keypair": {"id": "test-key", "name": "test-key"}}
        ]}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::DELETE)
            .path("/v2.1/os-keypairs/test-key");
        then.status(202);
    });

    let cleanup = ProjectCleanupBuilder::new(&client)
        .with_provider(ComputeCleanupProvider)
        .build();

    let eval: crate::cleanup::provider::EvaluationFn =
        std::sync::Arc::new(|r: &PlannedResource| r.kind == KEYPAIR);
    let plan = cleanup
        .discover(HashMap::new(), Some(eval))
        .await
        .expect("discover failed");
    let result = cleanup.apply(plan).await.expect("apply failed");

    assert!(
        result.errors.is_empty(),
        "unexpected errors: {:?}",
        result.errors
    );
    assert!(result.deleted_ids.contains(&"test-key".to_string()));
}
```

**Note on the mock response shape:** Nova's keypair list endpoint wraps each entry in a `{"keypair": {...}}` envelope (`GET /os-keypairs` → `{"keypairs": [{"keypair": {...}}, ...]}`), unlike every other resource in this codebase which is a flat array of objects. Verify this against `sdk/compute/src/v2/keypair/list_20.rs`'s response type / existing SDK tests before writing `discover()` in Step 4 — if the generated response deserializes to a flat `Vec<Value>` of un-enveloped keypair objects instead (i.e., the SDK's paged response-key handling already unwraps the outer `"keypair"` envelope per element), adjust `discover()` in Step 4 accordingly and simplify these two mocks' JSON bodies to match (drop the inner `"keypair"` wrapper). Confirm by running Step 3 first — if it fails with a deserialization/field-not-found symptom rather than the expected "field not found on empty node" symptom, that confirms the envelope needs unwrapping in `discover()`.

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p openstack_sdk --all-features --lib cleanup::providers::compute -- discover_maps_keypairs_to_planned_resources delete_issues_keypair_delete_request`
Expected: FAIL to compile (`KEYPAIR` undefined).

- [ ] **Step 4: Implement**

Add the import alongside the existing `use crate::api::compute::v2::server;`:

```rust
use crate::api::compute::v2::keypair;
```

In `discover()`, after the existing servers loop, add:

```rust
let keypairs: Vec<Value> = paged(
    keypair::list_20::Request::builder().build().map_err(|e| {
        CleanupError::Engine(format!("failed to build keypair list request: {e}"))
    })?,
    Pagination::All,
)
.query_async(ctx.client)
.await
.map_err(|e| list_err(KEYPAIR)(e.into()))?;
for v in keypairs {
    // See the note on this test's mock shape: unwrap the `"keypair"`
    // envelope here if the generated response type doesn't already do it.
    let entry = v.get("keypair").cloned().unwrap_or(v);
    nodes.push(to_planned(KEYPAIR, entry));
}
```

In `delete()`, add before the final `else`:

```rust
} else if resource.kind == KEYPAIR {
    let req = keypair::delete_20::Request::builder()
        .id(resource.id.clone())
        .build()
        .map_err(|e| {
            CleanupError::Engine(format!("failed to build keypair delete request: {e}"))
        })?;
    raw(req)
        .query_async(ctx.client)
        .await
        .map_err(|e| err(e.into()))?;
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p openstack_sdk --all-features --lib cleanup::providers::compute`
Expected: PASS, all tests including the two new ones and the pre-existing three.

- [ ] **Step 6: Commit**

```bash
git add openstack_sdk/src/cleanup/providers/compute.rs
git commit -m "feat(cleanup): add keypair support to compute provider"
```

---

### Task 4: Backups in block-storage provider

**Files:**
- Modify: `openstack_sdk/src/cleanup/providers/block_storage.rs`

**Interfaces:**
- Produces: `pub const BACKUP: ResourceKind = ResourceKind::new("block-storage", "backup");`
- Consumes: existing `value_str`, `to_planned`, `list_err`, `RelationEffect`/`RelationRule` imports already in this file.

Relation shape: identical to the existing `VOLUME`/`SNAPSHOT` pair — a backup references its source volume via `volume_id` and blocks that volume's deletion until the backup is itself selected, cascading automatically when the volume is selected.

- [ ] **Step 1: Add the resource kind constant**

After the existing `pub const SNAPSHOT: ResourceKind = ...` (`openstack_sdk/src/cleanup/providers/block_storage.rs:39`):

```rust
pub const BACKUP: ResourceKind = ResourceKind::new("block-storage", "backup");
```

- [ ] **Step 2: Add the failing tests**

Add to `mod tests`:

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn discover_marks_volume_with_backup_blocked_until_backup_selected() {
    let server = MockServer::start_async().await;
    let client = mock_client(&server).await;

    server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/v3/test-project/volumes");
        then.status(200).json_body(serde_json::json!({"volumes": [
            {"id": "vol-1", "name": "data"}
        ]}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/v3/test-project/snapshots");
        then.status(200)
            .json_body(serde_json::json!({"snapshots": []}));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/v3/test-project/backups");
        then.status(200).json_body(serde_json::json!({"backups": [
            {"id": "backup-1", "name": "data-backup", "volume_id": "vol-1"}
        ]}));
    });

    let cleanup = ProjectCleanupBuilder::new(&client)
        .with_provider(BlockStorageCleanupProvider)
        .build();

    let eval: crate::cleanup::provider::EvaluationFn =
        std::sync::Arc::new(|r: &PlannedResource| {
            r.kind == VOLUME && r.name.as_deref() == Some("data")
        });

    let plan = cleanup
        .discover(HashMap::new(), Some(eval))
        .await
        .expect("discover failed");

    let vol = plan.nodes.iter().find(|n| n.id == "vol-1").unwrap();
    let backup = plan.nodes.iter().find(|n| n.id == "backup-1").unwrap();
    assert!(vol.selected);
    assert!(
        backup.selected,
        "backup must be pulled in by the cascade group"
    );

    server.mock(|when, then| {
        when.method(httpmock::Method::DELETE)
            .path("/v3/test-project/backups/backup-1");
        then.status(202);
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::DELETE)
            .path("/v3/test-project/volumes/vol-1");
        then.status(202);
    });

    let result = cleanup.apply(plan).await.expect("apply failed");
    assert!(
        result.errors.is_empty(),
        "unexpected errors: {:?}",
        result.errors
    );
    let backup_pos = result
        .deleted_ids
        .iter()
        .position(|id| id == "backup-1")
        .unwrap();
    let vol_pos = result
        .deleted_ids
        .iter()
        .position(|id| id == "vol-1")
        .unwrap();
    assert!(backup_pos < vol_pos, "backup must delete before its volume");
}
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cargo test -p openstack_sdk --all-features --lib cleanup::providers::block_storage -- discover_marks_volume_with_backup_blocked_until_backup_selected`
Expected: FAIL to compile (`BACKUP` undefined).

- [ ] **Step 4: Implement**

Add the import alongside the existing `use crate::api::block_storage::v3::snapshot;` / `use crate::api::block_storage::v3::volume;`:

```rust
use crate::api::block_storage::v3::backup;
```

In `relations()`, append to the existing `vec![...]` (before the `#[cfg(feature = "compute")]` push):

```rust
RelationRule {
    parent_kind: VOLUME,
    child_kind: BACKUP,
    matches: |child, parent| {
        value_str(&child.raw, "volume_id") == value_str(&parent.raw, "id")
    },
    effect: RelationEffect::Blocks,
},
RelationRule {
    parent_kind: VOLUME,
    child_kind: BACKUP,
    matches: |child, parent| {
        value_str(&child.raw, "volume_id") == value_str(&parent.raw, "id")
    },
    effect: RelationEffect::CascadeGroup,
},
```

In `discover()`, after the existing snapshots loop, add:

```rust
let backups: Vec<Value> = paged(
    backup::list::Request::builder().build().map_err(|e| {
        CleanupError::Engine(format!("failed to build backup list request: {e}"))
    })?,
    Pagination::All,
)
.query_async(ctx.client)
.await
.map_err(|e| list_err(BACKUP)(e.into()))?;
for v in backups {
    nodes.push(to_planned(BACKUP, v));
}
```

In `delete()`, add before the final `else`:

```rust
} else if resource.kind == BACKUP {
    let req = backup::delete::Request::builder()
        .id(resource.id.clone())
        .build()
        .map_err(|e| {
            CleanupError::Engine(format!("failed to build backup delete request: {e}"))
        })?;
    raw(req)
        .query_async(ctx.client)
        .await
        .map_err(|e| err(e.into()))?;
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p openstack_sdk --all-features --lib cleanup::providers::block_storage`
Expected: PASS, all tests including the new one and the pre-existing two.

- [ ] **Step 6: Commit**

```bash
git add openstack_sdk/src/cleanup/providers/block_storage.rs
git commit -m "feat(cleanup): add backup support to block-storage provider"
```

---

### Task 5: Whole-branch verification

**Files:** none (verification only)

- [ ] **Step 1: Full workspace build**

Run: `cargo build -p openstack_sdk --all-features --tests`
Expected: clean, no new warnings beyond the two pre-existing ones already present before this plan (deprecated `RawQuery`/`RawQueryAsync` re-exports in `openstack_sdk/src/api.rs`, dead-code on `Paged` in `sdk/core/src/api/paged/pagination.rs:103`).

- [ ] **Step 2: Full lib test suite**

Run: `cargo test -p openstack_sdk --all-features --lib`
Expected: all tests pass (87 pre-existing + 2 floating-ip + 2 security-group + 2 keypair + 1 backup = 94).

- [ ] **Step 3: Feature-matrix sanity check**

The new rules in Task 2 (`SECURITY_GROUP Blocks PORT`) and the pre-existing cross-provider rules are all same-provider (network-internal) or already `#[cfg(feature = ...)]`-gated from earlier work — this plan adds no new cross-provider `#[cfg]` rules, so no additional feature-matrix build is required beyond `--all-features`. Confirm this by grepping the diff for `#[cfg(feature`:

Run: `git diff main --stat -- openstack_sdk/src/cleanup/providers/`
Expected: only the four provider files touched, no new files.
