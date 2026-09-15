# Project Cleanup Subsystem — Design

## Context

The python `openstacksdk` implements `OpenStackCloud.project_cleanup()`
(`openstack/cloud/openstackcloud.py`) as follows:

- Each service proxy may implement `_get_cleanup_dependencies()` (returns
  `{before: [...], after: [...]}` service names) and `_service_cleanup(...)`.
- `project_cleanup` builds a service-level DAG (`utils.TinyDAG`) from these
  dependency hints and walks it with a thread pool, invoking each service's
  `_service_cleanup` in dependency order.
- Each service's `_service_cleanup` is a single large imperative function
  that lists, filters, and deletes its own resources. Resources discovered
  as "to delete" are pushed into a shared `identified_resources` dict so
  other services' cleanup functions can consult what's already marked,
  and into a `status_queue` for caller visibility.
- `dry_run` gates whether `del_fn` actually gets called, but the same flag
  is reused mid-function as a probe (network proxy calls
  `_service_cleanup_del_res(..., dry_run=True)` to *check* whether a
  network needs deleting, before making its real delete decision) — this
  conflates "user asked for dry run" with "internal evaluation call."

Problems this design causes, confirmed by reading
`openstack/network/v2/_proxy.py::_service_cleanup` (lines ~9837–10108):

1. **Dependency graph is service-level only.** Any ordering that depends on
   actual resource relationships (a specific port belongs to a specific
   network; a router is attached via a specific interface port) can't be
   expressed as a graph edge. The network proxy instead hand-codes this as
   ~270 lines of imperative logic: list networks, list ports per network,
   classify port by `device_owner`, decide if the network "has ports
   allocated," detach router interfaces, delete ports, delete subnets,
   delete network, delete orphaned routers — all inline, untestable in
   isolation, and specific to network.
2. **No real plan/approve mode.** Selection ("should this resource be
   deleted") and deletion happen interleaved in the same imperative pass,
   with mutable shared state (`identified_resources`) mutated across
   threads as services run concurrently. There's no point where a
   complete, stable "here's what will be deleted and why" object exists
   that a caller could inspect, edit, and then apply. `dry_run=True` only
   suppresses the delete call; it doesn't produce an artifact.
3. **Not extensible.** Only services shipped in openstacksdk itself can
   participate, by defining these two dunder-ish methods on their proxy.
   A caller can't inject a cleanup hook for a service the SDK doesn't
   support, or override/augment built-in behavior, without subclassing
   the proxy classes.

## Goals

Design a project-cleanup subsystem for the rust `openstack_sdk` crate that:

- Expresses dependencies at both the service level (coarse ordering hints)
  and the resource level (relationships between actual discovered
  resources), so no service needs to hand-code cascade/ordering logic.
- Produces a real two-phase plan/approve flow: a discovery phase builds a
  complete, inspectable `CleanupPlan`; a separate apply phase executes
  only what's selected in that plan.
- Lets callers inject their own cleanup providers (for services the SDK
  doesn't support, or to customize/override built-in behavior) through
  the same interface used by built-in providers — no special-casing.

## Non-goals (v1)

- Sync execution support (SDK's `sync` feature). This subsystem targets
  the `async` feature only.
- Full parity with every service python covers. v1 ships compute,
  network, block-storage, image, and identity-scoped resources; the
  extension mechanism is designed so other services attach later with no
  core changes.
- Automatic re-validation of plan freshness (re-listing resources between
  discover and apply). Apply attempts deletes and tolerates 404s from
  resources that vanished in the interim; it does not re-run discovery.

## Architecture

### Resource envelope

Cleanup logic must be able to reason generically about resources without
being generic over every SDK resource type, mirroring python's untyped
`resource.Resource` handling in the cleanup path:

```rust
pub struct PlannedResource {
    pub kind: ResourceKind,          // e.g. ResourceKind::new("network", "network")
    pub id: String,
    pub name: Option<String>,
    pub raw: serde_json::Value,      // full resource body, for relation matching/filters
    pub selected: bool,              // discovery's filter verdict; caller may flip before apply
    pub reason: Option<String>,      // why selected/skipped, for plan display
}

pub struct ResourceKind {
    pub service_type: &'static str,  // "network", "compute", ...
    pub resource_type: &'static str, // "port", "server", ...
}
```

### Two dependency layers

**Service-level (`CleanupDependency`)** — same shape as python's
`{before, after}`, used only for ordering that isn't about specific
resource relationships (e.g. identity-scoped project resources should be
handled after everything else that lives inside the project).

**Resource-level (`RelationRule`)** — declarative edges between resource
*kinds*, evaluated against actually discovered `PlannedResource`s during
the discovery phase:

```rust
pub struct RelationRule {
    pub parent_kind: ResourceKind,
    pub child_kind: ResourceKind,
    pub matches: fn(child: &PlannedResource, parent: &PlannedResource) -> bool,
    pub effect: RelationEffect,
}

pub enum RelationEffect {
    /// Parent cannot be deleted while a matching, still-selected-or-existing
    /// child exists. Generic replacement for network's
    /// `network_has_ports_allocated` check.
    Blocks,
    /// Selecting any member of the group selects every member; the group
    /// has its own internal sub-order. Generic replacement for network's
    /// "networks are crazy, delete router+net+subnet together" cascade.
    CascadeGroup { order: fn(&[PlannedResource]) -> Vec<usize> },
    /// Before the parent is deleted, run this action to sever the
    /// relationship (does not delete the child). Generic replacement for
    /// `remove_interface_from_router`.
    Detach(fn(&CleanupContext, child: &PlannedResource) -> BoxFuture<'_, Result<(), CleanupError>>),
}
```

This is the direct fix for the network proxy's hacks: what's currently
270 lines of one-off imperative code becomes three `RelationRule` values
declared by the network provider, using primitives every other provider
can reuse.

### Two-phase execution

**Discover phase.** Every registered `CleanupProvider` lists its resources
concurrently (tokio tasks respecting only service-level `CleanupDependency`
ordering where a provider genuinely needs another service's data to list
its own — e.g. needing a project-scoped list). Each provider's discovered
resources are merged into one node set; `RelationRule`s are evaluated
against the merged set to compute edges and apply `Blocks`/`CascadeGroup`
effects. The per-resource filter/evaluation callback (equivalent of
python's `resource_evaluation_fn` and built-in filters like
`created_at`/`updated_at`) runs here too, setting `selected`. Output is a
`CleanupPlan { nodes: Vec<PlannedResource>, edges: Vec<(NodeIdx, NodeIdx, RelationEffect)> }`,
which is `serde`-serializable — it can be printed as a table/tree, diffed,
or handed back after a caller/CLI lets the user toggle `selected` flags.
No deletions happen in this phase.

**Apply phase.** Takes a `CleanupPlan` (possibly edited) and walks it as a
DAG: for `Blocks` edges, children delete before parents; for
`CascadeGroup`s, members delete in the group's declared internal order;
`Detach` actions run immediately before their parent's delete call. Only
`selected` nodes are touched. Deletes run concurrently across independent
subgraphs (tokio tasks + a shared "node done" signal, the same shape as
python's `TinyDAG.walk`/`node_done`, implemented with `petgraph` for graph
structure and topological walking). A delete returning "not found" is
treated as success (plan may be stale relative to real state).

### Extensibility

```rust
#[async_trait]
pub trait CleanupProvider: Send + Sync {
    fn service_type(&self) -> &'static str;
    fn dependencies(&self) -> CleanupDependency { CleanupDependency::default() }
    fn relations(&self) -> Vec<RelationRule> { vec![] }
    async fn discover(&self, ctx: &CleanupContext) -> Result<Vec<PlannedResource>, CleanupError>;
    async fn delete(&self, ctx: &CleanupContext, r: &PlannedResource) -> Result<(), CleanupError>;
}
```

A `ProjectCleanupBuilder` registers providers:

```rust
let cleanup = ProjectCleanupBuilder::new(session)
    .with_provider(NetworkCleanupProvider::default())   // built-in
    .with_provider(ComputeCleanupProvider::default())    // built-in
    .with_provider(MyOrgCustomCleanupProvider::new(...)) // caller-injected, same trait
    .build();

let plan = cleanup.discover(&filters).await?;
// caller inspects/edits plan.nodes[*].selected
let result = cleanup.apply(plan).await?;
```

Built-in and caller-supplied providers are indistinguishable to the
engine — this directly satisfies the extensibility requirement without
subclassing or special-casing.

### Placement

New module `openstack_sdk::cleanup`, gated behind the existing `async`
feature (matches the crate's current `#[cfg(feature = "async")]`
structure in `lib.rs`). Reusable from `openstack_cli`/`openstack_tui`
without duplicating logic.

### Error handling

Provider `discover`/`delete` errors are collected per-`PlannedResource`
into the plan/apply result (not per-service, as python does) and don't
abort the overall run — one resource failing to delete doesn't block
unrelated subgraphs. This is a strict improvement over python's per-service
`try/except` + log, since python's per-service scope hides which
individual resource failed inside a service that touches many resource
types.

### Testing

`RelationRule` evaluation, `Blocks`/`CascadeGroup` resolution, and DAG
ordering are unit-testable against synthetic `PlannedResource` sets with
no live cloud connection required — this was not possible in python,
where the equivalent logic is inline in one large imperative
`_service_cleanup` method per service.

## v1 scope

Providers: compute, network (proves `Blocks`/`CascadeGroup`/`Detach`),
block-storage, image, identity-scoped resources. `petgraph` added as a new
dependency for graph structure/topo-walk. Sync support and full
service-parity with python are deferred; the trait-based extension point
means later services need no core changes.
