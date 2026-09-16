# Project Cleanup: Extended Providers (compute, block-storage, image) + Real Functional Test

## Context

`openstack_sdk/src/cleanup` (merged in `32461f9fc`, see
`docs/superpowers/specs/2026-09-15-project-cleanup-design.md`) ships the
engine — declarative `RelationRule`-based dependency edges, a
discover/apply engine that toposorts and layers the resulting graph,
concurrent per-layer discovery/deletion, and the `CleanupProvider`
extension point — proven out by a single provider: network (network,
subnet, router, router_interface, port). Everything else in the original
design doc's v1 scope (compute, block-storage, image, identity-scoped
resources) was deliberately deferred as "separate, additive follow-up
tasks" (see `docs/superpowers/plans/2026-09-15-project-cleanup-core-engine.md:1294`).

All cleanup engine tests so far are unit tests against `httpmock` — there
is no test that exercises the engine against a real OpenStack control
plane. This project closes both gaps for three of the four deferred
services (compute, block-storage, image — identity-scoped resources
remain out of scope, per explicit decision this round) and adds a real
functional test.

## Goal

1. Add `ComputeCleanupProvider` (servers), `BlockStorageCleanupProvider`
   (volumes, snapshots), and `ImageCleanupProvider` (images), each
   following the existing `NetworkCleanupProvider` shape.
2. Declare the cross-service `RelationRule`s and service-level
   `CleanupDependency` hints needed for correct deletion ordering across
   these services.
3. Add one real, devstack-backed functional test
   (`openstack_sdk/tests/cleanup/`) that creates related resources across
   all four services, runs a real `discover()` + `apply()`, and asserts
   everything is gone in a valid order.

## Non-goals

- Identity-scoped resources (excluded this round; a later follow-up).
- Floating IPs, security groups, VPN resources (already-noted network
  follow-ups, untouched here).
- Any CLI/TUI surface (`osc project cleanup`) — SDK-only, per the original
  design doc.
- New engine primitives — the merged engine already evaluates
  `RelationRule`s against the full merged node set from every registered
  provider, so cross-service relations need no engine change.

## Providers

Each new provider follows `NetworkCleanupProvider`'s exact discover/delete
shape: `discover()` pages the service's list endpoint via
`crate::api::{service}::paged(...).query_async(ctx.client)`, converts each
`serde_json::Value` to a `PlannedResource` via a local `to_planned`
helper (id/name extraction, full body kept as `raw` for relation
matching); `delete()` matches on `resource.kind` and issues the
corresponding delete request via `raw(req).query_async(ctx.client)`.

### `ComputeCleanupProvider` (`openstack_sdk/src/cleanup/providers/compute.rs`)

- `pub const SERVER: ResourceKind = ResourceKind::new("compute", "server");`
- `discover()`: `crate::api::compute::v2::server::list::Request` (paged,
  detailed enough to carry `id`/`name`; use the plain `list` endpoint, not
  `list_detailed`, since cleanup only needs id/name/attachments-adjacent
  fields already present on the plain list response — if attachment data
  the block-storage relation needs turns out to live only on the volume
  side (it does — see below), the server side needs no extra fields).
- `delete()`: `crate::api::compute::v2::server::delete::Request`.
- `dependencies()`: `CleanupDependency { before: vec!["network"], after:
  vec![] }` — ensures compute's servers are deleted before network's
  ports are deleted, avoiding a race where a `compute:nova`-owned port is
  deleted out from under a still-live instance. (Cinder volume ordering
  needs no service-level hint — see the `VOLUME Blocks SERVER`
  `RelationRule` below, which is resource-level and sufficient on its
  own.)
- `relations()`: none declared by this provider (the cross-service rule
  involving `SERVER` is declared by `BlockStorageCleanupProvider`, since
  `RelationRule`s are just data evaluated globally regardless of which
  provider declared them — see `relations.rs::evaluate_edges`, which
  takes the full node set and full rule set with no per-provider
  scoping).

### `BlockStorageCleanupProvider` (`openstack_sdk/src/cleanup/providers/block_storage.rs`)

- `pub const VOLUME: ResourceKind = ResourceKind::new("block-storage", "volume");`
- `pub const SNAPSHOT: ResourceKind = ResourceKind::new("block-storage", "snapshot");`
- `discover()`: `crate::api::block_storage::v3::volume::list::Request` and
  `crate::api::block_storage::v3::snapshot::list::Request`, both paged.
  The volume list response's `attachments` array (list of objects with a
  `server_id` field) is kept verbatim in `raw` for the cross-service
  relation match below.
- `delete()`: `crate::api::block_storage::v3::volume::delete::Request` /
  `crate::api::block_storage::v3::snapshot::delete::Request`, matched on
  `resource.kind`.
- `dependencies()`: default (no service-level hint needed).
- `relations()`:
  ```rust
  vec![
      RelationRule {
          parent_kind: VOLUME,
          child_kind: SNAPSHOT,
          matches: |child, parent| {
              value_str(&child.raw, "volume_id") == value_str(&parent.raw, "id")
          },
          effect: RelationEffect::Blocks,
      },
      RelationRule {
          parent_kind: VOLUME,
          child_kind: SNAPSHOT,
          matches: |child, parent| {
              value_str(&child.raw, "volume_id") == value_str(&parent.raw, "id")
          },
          effect: RelationEffect::CascadeGroup,
      },
      RelationRule {
          parent_kind: VOLUME,
          child_kind: crate::cleanup::providers::compute::SERVER,
          matches: |child, parent| {
              parent.raw.get("attachments")
                  .and_then(|v| v.as_array())
                  .is_some_and(|attachments| {
                      attachments.iter().any(|a| {
                          value_str(a, "server_id") == Some(child.id.as_str())
                      })
                  })
          },
          effect: RelationEffect::Blocks,
      },
  ]
  ```
  Semantics: a snapshot blocks its volume from deleting until the
  snapshot is also selected (and gets pulled in automatically via
  `CascadeGroup` — same pattern as network's `NETWORK`/`SUBNET` pair); a
  volume attached to a still-live, unselected server is blocked from
  deleting until that server is gone. No `CascadeGroup` on the
  volume/server pair — deleting a server must not imply deleting its
  volumes.

### `ImageCleanupProvider` (`openstack_sdk/src/cleanup/providers/image.rs`)

- `pub const IMAGE: ResourceKind = ResourceKind::new("image", "image");`
- `discover()`: `crate::api::image::v2::image::list::Request`, paged.
- `delete()`: `crate::api::image::v2::image::delete::Request`.
- `dependencies()` / `relations()`: both default/empty. Glance permits
  deleting an image regardless of what still references it (booted
  servers, volumes created from it) — no ordering constraint to express.

### Registration

`ProjectCleanupBuilder` callers (and the functional test) register all
four built-in providers:
```rust
ProjectCleanupBuilder::new(&client)
    .with_provider(NetworkCleanupProvider)
    .with_provider(ComputeCleanupProvider)
    .with_provider(BlockStorageCleanupProvider)
    .with_provider(ImageCleanupProvider)
    .build()
```

## Functional test

**Location:** `openstack_sdk/tests/cleanup/mod.rs` +
`openstack_sdk/tests/cleanup/full_scenario.rs`, wired in via `mod cleanup;`
in `openstack_sdk/tests/main.rs` (alongside the existing `mod connection;`).
Follows `openstack_sdk/tests/connection/async.rs`'s pattern exactly:
`#[tokio::test]`, `ConfigFile::new()` + `get_cloud_config(OS_CLOUD)`,
real `AsyncOpenStack::new(&profile)`. Runs in CI via the existing
`.github/workflows/functional.yml` (`cargo nextest run --test functional`
against a real devstack, `OS_CLOUD=devstack`) — no CI config changes
needed since that workflow already runs everything under
`openstack_sdk/tests/`.

**Scenario** (`full_scenario.rs`, single `#[tokio::test]`):

1. Generate a unique marker, e.g. `format!("cleanup-test-{}",
   uuid::Uuid::new_v4())` (or reuse whatever unique-name helper the repo
   already has for functional tests — check `tests/connection/` first; if
   none, a random suffix on a fixed prefix is sufficient), used as a name
   prefix on every resource this test creates, so `evaluation_fn` can
   select precisely this test's resources and nothing else a concurrent
   CI matrix leg or prior failed run left behind.
2. Create, in dependency order:
   - a network + subnet + router with the subnet attached as a router
     interface (reuses the same shape the existing network unit tests
     exercise, now for real);
   - an image is *not* created — resolve the boot image by name via
     `env::var("TEST_IMAGE_NAME").unwrap_or_else(|_| "cirros-0.6.2-x86_64-disk".into())`
     (mirrors python openstacksdk's functional-test convention of the
     same name/default), list images, find the one matching that name,
     use its id; do not delete this pre-existing image (see step 5). This
     also means the test is not devstack-specific: pointing `OS_CLOUD` at
     any real cloud and setting `TEST_IMAGE_NAME` to an image that exists
     there is enough to run it;
   - a volume (small, e.g. 1 GiB) built from that image;
   - a snapshot of that volume;
   - a server booted from the volume (`block_device_mapping_v2`), attached
     to the created network.
3. Wait (poll with a short timeout, e.g. via existing SDK helpers if any
   exist for "wait until ACTIVE" — check `sdk/compute` for a waiter
   before hand-rolling one) for the server to reach `ACTIVE` so the
   engine's discovery sees stable state.
4. Build a `ProjectCleanup` with all four built-in providers registered,
   an `evaluation_fn` matching the marker prefix on `name` (falling back
   to matching the known created ids directly for resources without a
   `name` field, e.g. the snapshot if it wasn't given one, or the
   router-interface pseudo-resource — match by parent network id via the
   marker instead, same as how the plan's `CascadeGroup` already pulls
   those in once the network is selected).
5. **Do not select the pre-existing boot image** — the `evaluation_fn`
   only matches names carrying the test's marker prefix, and the
   pre-existing (cirros-by-default, or `TEST_IMAGE_NAME`) image has no
   such name, so it is never selected regardless of provider
   registration.
6. Run `cleanup.discover(...)` then `cleanup.apply(plan)`. Assert:
   - `result.errors.is_empty()`.
   - Every created resource's id appears in `result.deleted_ids`.
   - Ordering: snapshot's `deleted_ids` position < volume's; server's
     position < volume's; subnet's and the router-interface's positions <
     network's (mirrors the existing network unit test's ordering
     assertions).
7. Defensive cleanup: wrap resource creation in a helper that records
   every created (kind, id) pair as it goes, and on any early return
   (assertion failure, `?` propagation) attempt best-effort direct deletes
   of everything recorded, in reverse creation order, swallowing errors
   (already-deleted-by-the-thing-under-test is the success path, not a
   failure). Implement via a small RAII guard struct
   (`struct CreatedResources { client: &AsyncOpenStack, created: Vec<(ResourceKindLike, String)> }` with a
   `Drop` impl) rather than manual `?`-path bookkeeping, since the test
   has many fallible creation steps.

## Testing

- Existing unit-test pattern (httpmock-backed, in each new provider's
  `#[cfg(test)] mod tests`) for discover/delete/relation behavior per
  provider — mirrors `providers/network.rs`'s existing tests
  (`discover_marks_*_blocked_until_*_selected`,
  `discover_classifies_*_correctly`) adapted to each service's shape:
  at minimum, one test per provider proving discover→to_planned mapping,
  and one test per new cross-service `RelationRule` proving the edge is
  produced only for a matching pair (mirrors
  `relations.rs`'s own `blocks_edge_created_only_for_matching_pair`
  pattern, but exercised through the provider's `relations()` output
  against a synthetic merged node set spanning two providers' kinds).
- The new functional test, run manually against a real OpenStack cloud
  (not devstack) during implementation — this sandbox cannot reach any
  live OpenStack anyway (loopback networking is blocked here, a known,
  pre-existing limitation), and the test is written to be
  cloud-agnostic (`OS_CLOUD` + `TEST_IMAGE_NAME`), so verifying against a
  real cloud during development is a strictly stronger check than
  devstack. CI continues to run it against devstack on every PR via
  `.github/workflows/functional.yml`.
- `cargo clippy -p openstack_sdk --lib --tests --all-features` clean, as
  with the original core-engine PR.

## Open questions resolved during brainstorming

- Build order: one combined spec/plan covering all three providers plus
  the functional test (not split into separate PRs).
- Service scope: compute, block-storage, image. Identity-scoped resources
  explicitly excluded this round.
- Functional test shape: one combined cross-service scenario (not
  per-service smoke tests), since the engine's distinguishing value over
  python's `project_cleanup` is relation-aware cross-resource ordering,
  which only a cross-service scenario actually exercises with real
  resources.
