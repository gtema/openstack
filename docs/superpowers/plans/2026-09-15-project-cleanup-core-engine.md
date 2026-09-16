# Project Cleanup Core Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the resource-level dependency graph engine and provider extension point for project cleanup in `openstack_sdk`, proven against one real provider (network) that exercises the two hard cases python hand-codes today: "don't delete a parent while an unselected child still points at it" and "deleting one member of a connected group must delete the whole group."

**Architecture:** A `CleanupProvider` trait (one impl per service) discovers resources as untyped `PlannedResource` envelopes and declares `RelationRule`s between resource kinds. A pure `relations::evaluate_edges` function turns discovered nodes + rules into a graph. `discover()` merges all providers' nodes, computes edges, propagates cascade-group selection, and returns a serializable `CleanupPlan` — this is the plan/approve artifact. `apply()` takes a (possibly caller-edited) plan and walks it as a DAG, deleting children before parents, skipping any parent still blocked by a kept child.

**Tech Stack:** Rust, `openstack_sdk` crate (async feature), `petgraph` (new dependency) for graph/topological walk, `tokio` for concurrent provider discovery, `async-trait`, `serde`/`serde_json`, `httpmock` (existing dev-dependency) for the network provider's integration test.

## Global Constraints

- Async-only (`#[cfg(feature = "async")]`); no sync-feature support in this plan.
- New code lives entirely under `openstack_sdk/src/cleanup/`.
- No live-cloud calls in unit tests for the engine (Tasks 1-5); only the network provider task (Task 6) talks to a mocked HTTP server via `httpmock`, following the existing pattern in `openstack_sdk/src/test.rs`.
- `RelationEffect` has exactly two variants, `Blocks` and `CascadeGroup` — no third "Detach" primitive. A resource that needs to be *detached* rather than deleted (e.g. a router interface) is modeled as its own `ResourceKind` whose `CleanupProvider::delete` performs the detach call. This is a deliberate simplification versus the design doc's three-variant sketch, made because an async detach action can't be carried as a plain `fn` pointer without boxed-future ceremony, and the two-variant + dedicated-kind approach covers the same cases with less machinery. Record this as the implemented behavior; the design doc's "Detach" bullet is superseded by this note.
- Every provider's `delete` must treat `OpenStackError` whose underlying `ApiError::is_not_found()` is `true` as success (plan may be stale).

---

### Task 1: Core types (`ResourceKind`, `PlannedResource`) and crate wiring

**Files:**
- Create: `openstack_sdk/src/cleanup/mod.rs`
- Create: `openstack_sdk/src/cleanup/types.rs`
- Modify: `openstack_sdk/src/lib.rs` (add `#[cfg(feature = "async")] pub mod cleanup;` after the existing `#[cfg(feature = "async")] mod openstack_async;` block)
- Modify: `openstack_sdk/Cargo.toml` (add `petgraph` dependency)
- Test: `openstack_sdk/src/cleanup/types.rs` (inline `#[cfg(test)] mod tests`)

**Interfaces:**
- Produces: `pub struct ResourceKind { pub service_type: &'static str, pub resource_type: &'static str }` with `pub const fn new(service_type: &'static str, resource_type: &'static str) -> Self`, deriving `Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize`.
- Produces: `pub struct PlannedResource { pub kind: ResourceKind, pub id: String, pub name: Option<String>, pub raw: serde_json::Value, pub selected: bool, pub reason: Option<String> }` deriving `Debug, Clone, serde::Serialize, serde::Deserialize`.

- [ ] **Step 1: Add the `petgraph` dependency**

Add to the workspace's shared dependency table if one exists, otherwise directly under `[dependencies]` in `openstack_sdk/Cargo.toml`, alphabetically next to `parking_lot`:

```toml
petgraph = "0.6"
```

- [ ] **Step 2: Write the failing test for `ResourceKind` and `PlannedResource`**

Create `openstack_sdk/src/cleanup/types.rs`:

```rust
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

//! Core envelope types shared by every cleanup provider and by the
//! discover/apply engine.

use serde::{Deserialize, Serialize};

/// Identifies a resource type across services without requiring the engine
/// to be generic over every SDK resource struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceKind {
    pub service_type: &'static str,
    pub resource_type: &'static str,
}

impl ResourceKind {
    pub const fn new(service_type: &'static str, resource_type: &'static str) -> Self {
        Self {
            service_type,
            resource_type,
        }
    }
}

/// A single resource discovered by a [`crate::cleanup::provider::CleanupProvider`],
/// carried through discovery, plan inspection/editing, and apply.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedResource {
    pub kind: ResourceKind,
    pub id: String,
    pub name: Option<String>,
    /// Full resource body as returned by the API, used for relation
    /// matching and caller-supplied filters.
    pub raw: serde_json::Value,
    /// Whether this resource is currently slated for deletion. Discovery
    /// sets this from filters/cascade rules; a caller may flip it before
    /// calling `apply()`.
    pub selected: bool,
    /// Human-readable reason `selected` has its current value, for plan
    /// display (e.g. "matched filter", "cascade: network net-123").
    pub reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_kind_equality_and_hash() {
        let a = ResourceKind::new("network", "network");
        let b = ResourceKind::new("network", "network");
        let c = ResourceKind::new("network", "port");
        assert_eq!(a, b);
        assert_ne!(a, c);

        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(a);
        set.insert(b);
        set.insert(c);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn planned_resource_serde_roundtrip() {
        let node = PlannedResource {
            kind: ResourceKind::new("network", "network"),
            id: "net-1".into(),
            name: Some("private".into()),
            raw: serde_json::json!({"id": "net-1", "name": "private"}),
            selected: true,
            reason: Some("matched filter".into()),
        };
        let json = serde_json::to_string(&node).unwrap();
        let back: PlannedResource = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, "net-1");
        assert_eq!(back.kind, node.kind);
        assert!(back.selected);
    }
}
```

- [ ] **Step 3: Wire the module and run to verify it fails to build (module not yet registered)**

Create `openstack_sdk/src/cleanup/mod.rs`:

```rust
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

//! Project cleanup: resource-level dependency graph, discover/apply engine,
//! and the [`provider::CleanupProvider`] extension point.

pub mod types;

pub use types::{PlannedResource, ResourceKind};
```

In `openstack_sdk/src/lib.rs`, find:

```rust
#[cfg(feature = "async")]
mod openstack_async;
#[cfg(feature = "async")]
pub use openstack_async::{AsyncOpenStack, AsyncOpenStackBuilder, RenewHandle};
```

and add immediately after it:

```rust
#[cfg(feature = "async")]
pub mod cleanup;
```

Run: `cargo test -p openstack_sdk --lib cleanup:: 2>&1 | tail -20`
Expected: compiles and runs the two tests in `cleanup::types::tests`, both PASS. (There is no "fails first" step here since this task only adds new, self-contained types — there's no existing behavior to regress. Confirm PASS, not a pre-existing FAIL.)

- [ ] **Step 4: Commit**

```bash
git add openstack_sdk/Cargo.toml openstack_sdk/src/lib.rs openstack_sdk/src/cleanup/mod.rs openstack_sdk/src/cleanup/types.rs
git commit -m "feat(sdk): add cleanup module with PlannedResource/ResourceKind types"
```

---

### Task 2: `RelationRule` and pure edge evaluation

**Files:**
- Create: `openstack_sdk/src/cleanup/relations.rs`
- Modify: `openstack_sdk/src/cleanup/mod.rs` (add `pub mod relations;` and re-exports)

**Interfaces:**
- Consumes: `PlannedResource`, `ResourceKind` from Task 1 (`crate::cleanup::types`).
- Produces:
  - `pub enum RelationEffect { Blocks, CascadeGroup }` deriving `Debug, Clone, Copy, PartialEq, Eq`.
  - `pub struct RelationRule { pub parent_kind: ResourceKind, pub child_kind: ResourceKind, pub matches: fn(child: &PlannedResource, parent: &PlannedResource) -> bool, pub effect: RelationEffect }`.
  - `pub struct Edge { pub child: usize, pub parent: usize, pub effect: RelationEffect }` (indices into the node slice passed to `evaluate_edges`).
  - `pub fn evaluate_edges(nodes: &[PlannedResource], rules: &[RelationRule]) -> Vec<Edge>`.

- [ ] **Step 1: Write the failing test**

Create `openstack_sdk/src/cleanup/relations.rs`:

```rust
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

//! Declarative, resource-level dependency rules between resource kinds,
//! and their evaluation against a concrete set of discovered resources.
//!
//! This replaces the imperative, per-service ordering logic (e.g. the
//! python network proxy's inline "does this network still have ports"
//! check) with a rule every provider can reuse.

use crate::cleanup::types::{PlannedResource, ResourceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationEffect {
    /// The parent cannot be deleted while a matching child still exists
    /// and is not itself selected for deletion.
    Blocks,
    /// If either side of a matching pair is selected for deletion, both
    /// become selected (and, transitively, every node reachable through
    /// other `CascadeGroup` edges).
    CascadeGroup,
}

pub struct RelationRule {
    pub parent_kind: ResourceKind,
    pub child_kind: ResourceKind,
    pub matches: fn(child: &PlannedResource, parent: &PlannedResource) -> bool,
    pub effect: RelationEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub child: usize,
    pub parent: usize,
    pub effect: RelationEffect,
}

/// Evaluate every rule against every (child, parent) pair of matching kind
/// in `nodes`, returning the resulting edges as node indices.
pub fn evaluate_edges(nodes: &[PlannedResource], rules: &[RelationRule]) -> Vec<Edge> {
    let mut edges = Vec::new();
    for rule in rules {
        for (child_idx, child) in nodes.iter().enumerate() {
            if child.kind != rule.child_kind {
                continue;
            }
            for (parent_idx, parent) in nodes.iter().enumerate() {
                if parent.kind != rule.parent_kind {
                    continue;
                }
                if (rule.matches)(child, parent) {
                    edges.push(Edge {
                        child: child_idx,
                        parent: parent_idx,
                        effect: rule.effect,
                    });
                }
            }
        }
    }
    edges
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn node(kind: ResourceKind, id: &str, extra: serde_json::Value) -> PlannedResource {
        PlannedResource {
            kind,
            id: id.into(),
            name: None,
            raw: extra,
            selected: false,
            reason: None,
        }
    }

    const NETWORK: ResourceKind = ResourceKind::new("network", "network");
    const PORT: ResourceKind = ResourceKind::new("network", "port");

    fn port_blocks_network_rule() -> RelationRule {
        RelationRule {
            parent_kind: NETWORK,
            child_kind: PORT,
            matches: |child, parent| {
                child.raw.get("network_id").and_then(|v| v.as_str()) == parent.raw.get("id").and_then(|v| v.as_str())
            },
            effect: RelationEffect::Blocks,
        }
    }

    #[test]
    fn blocks_edge_created_only_for_matching_pair() {
        let nodes = vec![
            node(NETWORK, "net-1", json!({"id": "net-1"})),
            node(NETWORK, "net-2", json!({"id": "net-2"})),
            node(PORT, "port-1", json!({"id": "port-1", "network_id": "net-1"})),
        ];
        let edges = evaluate_edges(&nodes, &[port_blocks_network_rule()]);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].child, 2); // port-1
        assert_eq!(edges[0].parent, 0); // net-1
        assert_eq!(edges[0].effect, RelationEffect::Blocks);
    }

    #[test]
    fn no_edges_when_nothing_matches() {
        let nodes = vec![
            node(NETWORK, "net-1", json!({"id": "net-1"})),
            node(PORT, "port-1", json!({"id": "port-1", "network_id": "net-2"})),
        ];
        let edges = evaluate_edges(&nodes, &[port_blocks_network_rule()]);
        assert!(edges.is_empty());
    }
}
```

- [ ] **Step 2: Run to verify tests fail (module not yet registered)**

Run: `cargo test -p openstack_sdk --lib cleanup::relations 2>&1 | tail -20`
Expected: FAIL — `cleanup::relations` module not found (not yet added to `mod.rs`).

- [ ] **Step 3: Register the module**

In `openstack_sdk/src/cleanup/mod.rs`, replace the file contents with:

```rust
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

//! Project cleanup: resource-level dependency graph, discover/apply engine,
//! and the [`provider::CleanupProvider`] extension point.

pub mod relations;
pub mod types;

pub use relations::{Edge, RelationEffect, RelationRule, evaluate_edges};
pub use types::{PlannedResource, ResourceKind};
```

- [ ] **Step 4: Run to verify tests pass**

Run: `cargo test -p openstack_sdk --lib cleanup:: 2>&1 | tail -20`
Expected: PASS — 4 tests total (2 from Task 1, 2 from this task).

- [ ] **Step 5: Commit**

```bash
git add openstack_sdk/src/cleanup/mod.rs openstack_sdk/src/cleanup/relations.rs
git commit -m "feat(sdk): add RelationRule and pure edge evaluation for cleanup"
```

---

### Task 3: `CleanupProvider` trait, `CleanupContext`, `CleanupError`, and service-level ordering

**Files:**
- Create: `openstack_sdk/src/cleanup/provider.rs`
- Modify: `openstack_sdk/src/cleanup/mod.rs`

**Interfaces:**
- Consumes: `PlannedResource`, `RelationRule` from Tasks 1-2.
- Produces:
  - `pub struct CleanupDependency { pub before: Vec<&'static str>, pub after: Vec<&'static str> }` (`Default` derived, both fields default to empty).
  - `pub struct CleanupContext<'a> { pub client: &'a crate::AsyncOpenStack, pub filters: std::collections::HashMap<String, String>, pub evaluation_fn: Option<std::sync::Arc<dyn Fn(&PlannedResource) -> bool + Send + Sync>> }`.
  - `#[derive(Debug, thiserror::Error)] pub enum CleanupError { #[error("cleanup error for {kind:?} {id}: {source}")] Provider { kind: ResourceKind, id: String, #[source] source: crate::OpenStackError }, #[error("cleanup engine error: {0}")] Engine(String) }` with `pub fn is_not_found(&self) -> bool`.
  - `#[async_trait::async_trait] pub trait CleanupProvider: Send + Sync { fn service_type(&self) -> &'static str; fn dependencies(&self) -> CleanupDependency { CleanupDependency::default() } fn relations(&self) -> Vec<RelationRule> { Vec::new() } async fn discover(&self, ctx: &CleanupContext<'_>) -> Result<Vec<PlannedResource>, CleanupError>; async fn delete(&self, ctx: &CleanupContext<'_>, resource: &PlannedResource) -> Result<(), CleanupError>; }`.
  - `pub fn service_order(providers: &[&dyn CleanupProvider]) -> Result<Vec<usize>, CleanupError>` — returns indices into `providers` in an order satisfying every `CleanupDependency` (topological sort over `before`/`after`), erroring with `CleanupError::Engine` on a cycle.

- [ ] **Step 1: Write the failing test**

Create `openstack_sdk/src/cleanup/provider.rs`:

```rust
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

//! The [`CleanupProvider`] extension point: the same trait built-in
//! service providers and caller-injected providers both implement, so the
//! engine treats them identically.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;

use crate::AsyncOpenStack;
use crate::OpenStackError;
use crate::cleanup::relations::RelationRule;
use crate::cleanup::types::{PlannedResource, ResourceKind};

/// Coarse, service-level ordering hint. Used only for orderings that are
/// not derivable from a resource-level [`RelationRule`] — e.g. a service
/// that must run after everything else because it drops the project
/// itself.
#[derive(Debug, Clone, Default)]
pub struct CleanupDependency {
    pub before: Vec<&'static str>,
    pub after: Vec<&'static str>,
}

/// Per-run context handed to every provider call.
pub struct CleanupContext<'a> {
    pub client: &'a AsyncOpenStack,
    pub filters: HashMap<String, String>,
    pub evaluation_fn: Option<Arc<dyn Fn(&PlannedResource) -> bool + Send + Sync>>,
}

#[derive(Debug, thiserror::Error)]
pub enum CleanupError {
    #[error("cleanup error for {kind:?} {id}: {source}")]
    Provider {
        kind: ResourceKind,
        id: String,
        #[source]
        source: OpenStackError,
    },
    #[error("cleanup engine error: {0}")]
    Engine(String),
}

impl CleanupError {
    pub fn is_not_found(&self) -> bool {
        match self {
            CleanupError::Provider { source, .. } => match source {
                OpenStackError::Api { source } => source.is_not_found(),
                _ => false,
            },
            CleanupError::Engine(_) => false,
        }
    }
}

#[async_trait]
pub trait CleanupProvider: Send + Sync {
    fn service_type(&self) -> &'static str;

    fn dependencies(&self) -> CleanupDependency {
        CleanupDependency::default()
    }

    fn relations(&self) -> Vec<RelationRule> {
        Vec::new()
    }

    async fn discover(&self, ctx: &CleanupContext<'_>) -> Result<Vec<PlannedResource>, CleanupError>;

    async fn delete(
        &self,
        ctx: &CleanupContext<'_>,
        resource: &PlannedResource,
    ) -> Result<(), CleanupError>;
}

/// Order providers so that every `before`/`after` hint is satisfied.
/// Returns indices into `providers`.
pub fn service_order(providers: &[&dyn CleanupProvider]) -> Result<Vec<usize>, CleanupError> {
    let mut graph = DiGraph::<usize, ()>::new();
    let node_ids: Vec<_> = (0..providers.len()).map(|i| graph.add_node(i)).collect();
    let index_of = |service_type: &str| providers.iter().position(|p| p.service_type() == service_type);

    for (idx, provider) in providers.iter().enumerate() {
        let deps = provider.dependencies();
        for before in &deps.before {
            if let Some(other) = index_of(before) {
                // `idx` must run before `other`: edge idx -> other
                graph.add_edge(node_ids[idx], node_ids[other], ());
            }
        }
        for after in &deps.after {
            if let Some(other) = index_of(after) {
                // `idx` must run after `other`: edge other -> idx
                graph.add_edge(node_ids[other], node_ids[idx], ());
            }
        }
    }

    toposort(&graph, None)
        .map(|order| order.into_iter().map(|n| graph[n]).collect())
        .map_err(|_| CleanupError::Engine("cycle in service-level cleanup dependencies".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeProvider {
        service_type: &'static str,
        deps: CleanupDependency,
    }

    #[async_trait]
    impl CleanupProvider for FakeProvider {
        fn service_type(&self) -> &'static str {
            self.service_type
        }
        fn dependencies(&self) -> CleanupDependency {
            self.deps.clone()
        }
        async fn discover(&self, _ctx: &CleanupContext<'_>) -> Result<Vec<PlannedResource>, CleanupError> {
            Ok(Vec::new())
        }
        async fn delete(&self, _ctx: &CleanupContext<'_>, _r: &PlannedResource) -> Result<(), CleanupError> {
            Ok(())
        }
    }

    #[test]
    fn network_before_identity_is_respected() {
        let network = FakeProvider {
            service_type: "network",
            deps: CleanupDependency {
                before: vec!["identity"],
                after: vec![],
            },
        };
        let identity = FakeProvider {
            service_type: "identity",
            deps: CleanupDependency::default(),
        };
        // Registered in the "wrong" order on purpose.
        let providers: Vec<&dyn CleanupProvider> = vec![&identity, &network];
        let order = service_order(&providers).unwrap();
        let network_pos = order.iter().position(|&i| i == 1).unwrap();
        let identity_pos = order.iter().position(|&i| i == 0).unwrap();
        assert!(network_pos < identity_pos, "network must be ordered before identity");
    }

    #[test]
    fn cycle_is_reported_as_engine_error() {
        let a = FakeProvider {
            service_type: "a",
            deps: CleanupDependency {
                before: vec!["b"],
                after: vec![],
            },
        };
        let b = FakeProvider {
            service_type: "b",
            deps: CleanupDependency {
                before: vec!["a"],
                after: vec![],
            },
        };
        let providers: Vec<&dyn CleanupProvider> = vec![&a, &b];
        let err = service_order(&providers).unwrap_err();
        assert!(matches!(err, CleanupError::Engine(_)));
    }
}
```

- [ ] **Step 2: Run to verify tests fail (module not registered)**

Run: `cargo test -p openstack_sdk --lib cleanup::provider 2>&1 | tail -20`
Expected: FAIL — module not found.

- [ ] **Step 3: Register the module and add `thiserror` (already a workspace dependency, confirm it's listed under `[dependencies]` in `openstack_sdk/Cargo.toml` — it is, per the existing `thiserror.workspace = true` line, so no Cargo.toml change is needed here)**

Update `openstack_sdk/src/cleanup/mod.rs`:

```rust
pub mod provider;
pub mod relations;
pub mod types;

pub use provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider, service_order};
pub use relations::{Edge, RelationEffect, RelationRule, evaluate_edges};
pub use types::{PlannedResource, ResourceKind};
```

(keep the file's existing module doc comment and license header at the top.)

- [ ] **Step 4: Run to verify tests pass**

Run: `cargo test -p openstack_sdk --lib cleanup:: 2>&1 | tail -30`
Expected: PASS — 6 tests total.

- [ ] **Step 5: Commit**

```bash
git add openstack_sdk/src/cleanup/mod.rs openstack_sdk/src/cleanup/provider.rs
git commit -m "feat(sdk): add CleanupProvider trait and service-level ordering"
```

---

### Task 4: Discover engine — `CleanupPlan` and cascade-group selection

**Files:**
- Create: `openstack_sdk/src/cleanup/engine.rs`
- Modify: `openstack_sdk/src/cleanup/mod.rs`

**Interfaces:**
- Consumes: `CleanupProvider`, `CleanupContext`, `CleanupDependency`, `CleanupError`, `service_order` from Task 3; `PlannedResource`, `ResourceKind` from Task 1; `RelationRule`, `RelationEffect`, `Edge`, `evaluate_edges` from Task 2.
- Produces:
  - `#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] pub struct CleanupPlan { pub nodes: Vec<PlannedResource>, pub edges: Vec<PlanEdge> }` where `pub struct PlanEdge { pub child: usize, pub parent: usize, pub effect: RelationEffect }` (mirrors `relations::Edge`, but `RelationEffect` must derive `Serialize`/`Deserialize` too — add those derives to `RelationEffect` in Task 2's file as part of this task, see Step 1 note).
  - `pub struct ProjectCleanupBuilder<'a> { /* private */ }` with `pub fn new(client: &'a crate::AsyncOpenStack) -> Self` and `pub fn with_provider(self, provider: impl CleanupProvider + 'static) -> Self`.
  - `pub struct ProjectCleanup<'a> { /* private */ }` with `pub fn discover(&self, filters: std::collections::HashMap<String, String>, evaluation_fn: Option<std::sync::Arc<dyn Fn(&PlannedResource) -> bool + Send + Sync>>) -> impl std::future::Future<Output = Result<CleanupPlan, CleanupError>> + '_` (an `async fn` on the struct).
  - `ProjectCleanupBuilder::build(self) -> ProjectCleanup<'a>`.

- [ ] **Step 1: Add `Serialize`/`Deserialize` to `RelationEffect`**

In `openstack_sdk/src/cleanup/relations.rs`, change:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationEffect {
```

to:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RelationEffect {
```

- [ ] **Step 2: Write the failing test**

Create `openstack_sdk/src/cleanup/engine.rs`:

```rust
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

//! Discover/apply engine: turns registered [`CleanupProvider`]s into a
//! materialized, inspectable [`CleanupPlan`] (discover), then executes a
//! (possibly caller-edited) plan (apply, added in a later task).

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::AsyncOpenStack;
use crate::cleanup::provider::{CleanupContext, CleanupError, CleanupProvider, service_order};
use crate::cleanup::relations::{Edge, RelationEffect, evaluate_edges};
use crate::cleanup::types::PlannedResource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlanEdge {
    pub child: usize,
    pub parent: usize,
    pub effect: RelationEffect,
}

impl From<Edge> for PlanEdge {
    fn from(e: Edge) -> Self {
        PlanEdge {
            child: e.child,
            parent: e.parent,
            effect: e.effect,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupPlan {
    pub nodes: Vec<PlannedResource>,
    pub edges: Vec<PlanEdge>,
}

/// Propagate `CascadeGroup` selection: if any node in a group formed by
/// `CascadeGroup` edges is selected, every node in that group becomes
/// selected. Pure function over plan data, used by both `discover()` and
/// tested directly here.
pub(crate) fn propagate_cascade_groups(nodes: &mut [PlannedResource], edges: &[Edge]) {
    // Union-find over cascade-group edges only.
    let mut parent: Vec<usize> = (0..nodes.len()).collect();
    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    fn union(parent: &mut [usize], a: usize, b: usize) {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra != rb {
            parent[ra] = rb;
        }
    }
    for edge in edges.iter().filter(|e| e.effect == RelationEffect::CascadeGroup) {
        union(&mut parent, edge.child, edge.parent);
    }

    let mut group_selected: HashMap<usize, usize> = HashMap::new(); // root -> index of a selected member
    for i in 0..nodes.len() {
        if nodes[i].selected {
            let root = find(&mut parent, i);
            group_selected.entry(root).or_insert(i);
        }
    }
    for i in 0..nodes.len() {
        let root = find(&mut parent, i);
        if let Some(&selected_idx) = group_selected.get(&root) {
            if !nodes[i].selected {
                let cause_id = nodes[selected_idx].id.clone();
                nodes[i].selected = true;
                nodes[i].reason = Some(format!("cascade: {cause_id}"));
            }
        }
    }
}

pub struct ProjectCleanupBuilder<'a> {
    client: &'a AsyncOpenStack,
    providers: Vec<Box<dyn CleanupProvider>>,
}

impl<'a> ProjectCleanupBuilder<'a> {
    pub fn new(client: &'a AsyncOpenStack) -> Self {
        Self {
            client,
            providers: Vec::new(),
        }
    }

    pub fn with_provider(mut self, provider: impl CleanupProvider + 'static) -> Self {
        self.providers.push(Box::new(provider));
        self
    }

    pub fn build(self) -> ProjectCleanup<'a> {
        ProjectCleanup {
            client: self.client,
            providers: self.providers,
        }
    }
}

pub struct ProjectCleanup<'a> {
    client: &'a AsyncOpenStack,
    providers: Vec<Box<dyn CleanupProvider>>,
}

impl<'a> ProjectCleanup<'a> {
    pub async fn discover(
        &self,
        filters: HashMap<String, String>,
        evaluation_fn: Option<Arc<dyn Fn(&PlannedResource) -> bool + Send + Sync>>,
    ) -> Result<CleanupPlan, CleanupError> {
        let provider_refs: Vec<&dyn CleanupProvider> = self.providers.iter().map(|p| p.as_ref()).collect();
        let order = service_order(&provider_refs)?;

        let ctx = CleanupContext {
            client: self.client,
            filters,
            evaluation_fn,
        };

        // Service-level ordering only gates *listing*, not selection: run
        // providers in dependency order, but nothing stops a later
        // provider's rules from referencing an earlier provider's nodes.
        let mut nodes: Vec<PlannedResource> = Vec::new();
        for idx in order {
            let provider = &self.providers[idx];
            let mut discovered = provider.discover(&ctx).await?;
            if let Some(eval) = &ctx.evaluation_fn {
                for r in &mut discovered {
                    r.selected = eval(r);
                    if r.selected && r.reason.is_none() {
                        r.reason = Some("matched evaluation_fn".into());
                    }
                }
            }
            nodes.append(&mut discovered);
        }

        let all_rules: Vec<_> = self.providers.iter().flat_map(|p| p.relations()).collect();
        let edges = evaluate_edges(&nodes, &all_rules);

        propagate_cascade_groups(&mut nodes, &edges);

        Ok(CleanupPlan {
            nodes,
            edges: edges.into_iter().map(PlanEdge::from).collect(),
        })
    }
}
```

- [ ] **Step 3: Register the module**

Update `openstack_sdk/src/cleanup/mod.rs`:

```rust
pub mod engine;
pub mod provider;
pub mod relations;
pub mod types;

pub use engine::{CleanupPlan, PlanEdge, ProjectCleanup, ProjectCleanupBuilder};
pub use provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider, service_order};
pub use relations::{Edge, RelationEffect, RelationRule, evaluate_edges};
pub use types::{PlannedResource, ResourceKind};
```

- [ ] **Step 4: Add the cascade-propagation test and run it (add to `engine.rs`, at the bottom)**

Append to `openstack_sdk/src/cleanup/engine.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cleanup::relations::RelationEffect;
    use crate::cleanup::types::ResourceKind;
    use serde_json::json;

    fn node(kind: ResourceKind, id: &str, selected: bool) -> PlannedResource {
        PlannedResource {
            kind,
            id: id.into(),
            name: None,
            raw: json!({"id": id}),
            selected,
            reason: None,
        }
    }

    const NETWORK: ResourceKind = ResourceKind::new("network", "network");
    const ROUTER: ResourceKind = ResourceKind::new("network", "router");
    const SUBNET: ResourceKind = ResourceKind::new("network", "subnet");

    #[test]
    fn cascade_selection_spreads_across_group() {
        // net-1 (selected) -- cascade -- router-1
        // router-1 -- cascade -- subnet-1
        // subnet-1 is not directly linked to net-1, only transitively.
        let mut nodes = vec![
            node(NETWORK, "net-1", true),
            node(ROUTER, "router-1", false),
            node(SUBNET, "subnet-1", false),
            node(NETWORK, "net-2", false), // unrelated, must stay false
        ];
        let edges = vec![
            Edge { child: 1, parent: 0, effect: RelationEffect::CascadeGroup },
            Edge { child: 2, parent: 1, effect: RelationEffect::CascadeGroup },
        ];
        propagate_cascade_groups(&mut nodes, &edges);
        assert!(nodes[0].selected);
        assert!(nodes[1].selected, "router-1 must be pulled in transitively");
        assert!(nodes[2].selected, "subnet-1 must be pulled in transitively");
        assert!(!nodes[3].selected, "unrelated network must not be selected");
        assert_eq!(nodes[1].reason.as_deref(), Some("cascade: net-1"));
    }

    #[tokio::test]
    async fn discover_merges_providers_and_applies_evaluation_fn() {
        use crate::cleanup::provider::{CleanupContext, CleanupDependency};
        use async_trait::async_trait;

        struct OnlyEvenIdsProvider;
        #[async_trait]
        impl CleanupProvider for OnlyEvenIdsProvider {
            fn service_type(&self) -> &'static str {
                "fake"
            }
            fn dependencies(&self) -> CleanupDependency {
                CleanupDependency::default()
            }
            async fn discover(&self, _ctx: &CleanupContext<'_>) -> Result<Vec<PlannedResource>, CleanupError> {
                Ok(vec![
                    node(ResourceKind::new("fake", "thing"), "1", false),
                    node(ResourceKind::new("fake", "thing"), "2", false),
                ])
            }
            async fn delete(&self, _ctx: &CleanupContext<'_>, _r: &PlannedResource) -> Result<(), CleanupError> {
                Ok(())
            }
        }

        // discover() needs a real AsyncOpenStack only to populate
        // CleanupContext::client; no HTTP call happens because the fake
        // provider never touches it. Build a minimally-configured client
        // via the existing test helpers pattern used across the crate
        // (see openstack_sdk/src/test.rs) — a mock server is started but
        // no request is expected against it in this test.
        // NOTE: this requires the httpmock dev-dependency already present
        // in openstack_sdk/Cargo.toml.
        let server = httpmock::MockServer::start_async().await;
        let config = openstack_sdk_core::config::CloudConfig {
            auth: Some(openstack_sdk_core::config::Auth {
                auth_url: Some(format!("{}/v3", server.base_url())),
                username: Some("test-user".into()),
                user_domain_name: Some("Default".into()),
                password: Some("test-password".into()),
                project_id: Some("test-project".into()),
                ..Default::default()
            }),
            region_name: Some("RegionOne".into()),
            interface: Some("public".into()),
            auth_cache: Some(false),
            ..Default::default()
        };
        let base_url = server.base_url();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/");
            then.status(200).json_body(serde_json::json!({
                "versions": [{"id": "v3", "status": "SUPPORTED",
                    "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]
            }));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v3/");
            then.status(200).json_body(serde_json::json!({
                "versions": [{"id": "v3", "status": "SUPPORTED",
                    "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]
            }));
        });
        let expires = (chrono::Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        server.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/v3/auth/tokens");
            then.status(201)
                .header("x-subject-token", "test-token")
                .json_body(serde_json::json!({"token": {
                    "id": "token-id", "expires_at": expires,
                    "project": {"id": "test-project", "name": "TestProject"},
                    "user": {"id": "test-user", "name": "test-user"},
                    "methods": ["password"], "audit_ids": ["audit-1"],
                    "catalog": []
                }}));
        });

        let client = AsyncOpenStack::new_with_authentication_helper(
            &config,
            crate::auth::auth_helper::Noop::default(),
            false,
        )
        .await
        .expect("client creation failed");

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(OnlyEvenIdsProvider)
            .build();

        let eval: Arc<dyn Fn(&PlannedResource) -> bool + Send + Sync> =
            Arc::new(|r: &PlannedResource| r.id == "2");
        let plan = cleanup
            .discover(HashMap::new(), Some(eval))
            .await
            .expect("discover failed");

        assert_eq!(plan.nodes.len(), 2);
        let selected: Vec<_> = plan.nodes.iter().filter(|n| n.selected).map(|n| n.id.clone()).collect();
        assert_eq!(selected, vec!["2".to_string()]);
    }
}
```

- [ ] **Step 5: Run to verify tests pass**

Run: `cargo test -p openstack_sdk --lib cleanup:: 2>&1 | tail -40`
Expected: PASS — 8 tests total. If `auth::auth_helper::Noop` is not `pub` from the crate root in this way, check its actual path with `grep -n "pub use.*Noop\|pub mod auth_helper" openstack_sdk/src/auth/mod.rs openstack_sdk/src/openstack_async.rs` and adjust the `use` path in the test to match — the test in `openstack_sdk/src/test.rs` (`crate::auth::auth_helper::Noop::default()`) is the reference for the correct path.

- [ ] **Step 6: Commit**

```bash
git add openstack_sdk/src/cleanup/mod.rs openstack_sdk/src/cleanup/engine.rs openstack_sdk/src/cleanup/relations.rs
git commit -m "feat(sdk): add discover engine with cascade-group selection propagation"
```

---

### Task 5: Apply engine — DAG delete walk

**Files:**
- Modify: `openstack_sdk/src/cleanup/engine.rs`
- Modify: `openstack_sdk/src/cleanup/mod.rs`

**Interfaces:**
- Consumes: `CleanupPlan`, `PlanEdge`, `ProjectCleanup` from Task 4; `RelationEffect`, `PlannedResource` from earlier tasks.
- Produces:
  - `#[derive(Debug, Clone, Default, serde::Serialize)] pub struct CleanupResult { pub deleted: Vec<ResourceKind>, pub deleted_ids: Vec<String>, pub skipped: Vec<(String, String)>, pub errors: Vec<(String, String)> }` — `skipped`/`errors` are `(resource_id, reason)` pairs.
  - `impl<'a> ProjectCleanup<'a> { pub async fn apply(&self, plan: CleanupPlan) -> Result<CleanupResult, CleanupError> }`.

- [ ] **Step 1: Write the failing test**

Append to `openstack_sdk/src/cleanup/engine.rs`, inside the existing `mod tests` block (add before the closing brace, alongside the other tests):

```rust
    #[tokio::test]
    async fn apply_deletes_children_before_parents_and_skips_blocked_parent() {
        use crate::cleanup::provider::CleanupDependency;
        use async_trait::async_trait;
        use std::sync::Mutex;

        struct RecordingProvider {
            log: Arc<Mutex<Vec<String>>>,
        }

        #[async_trait]
        impl CleanupProvider for RecordingProvider {
            fn service_type(&self) -> &'static str {
                "fake"
            }
            fn dependencies(&self) -> CleanupDependency {
                CleanupDependency::default()
            }
            async fn discover(&self, _ctx: &CleanupContext<'_>) -> Result<Vec<PlannedResource>, CleanupError> {
                Ok(Vec::new())
            }
            async fn delete(
                &self,
                _ctx: &CleanupContext<'_>,
                resource: &PlannedResource,
            ) -> Result<(), CleanupError> {
                self.log.lock().unwrap().push(resource.id.clone());
                Ok(())
            }
        }

        let log = Arc::new(Mutex::new(Vec::new()));

        // net-1 has one selected child (port-selected) and one kept child
        // (port-kept). net-2 has only a selected child. Expect: net-1 is
        // NOT deleted (blocked by port-kept), net-2 IS deleted, and
        // port-selected/port-kept... wait, port-kept is not selected so it
        // is never passed to delete() at all; only selected nodes are
        // touched by apply().
        let mut net1 = node(ResourceKind::new("network", "network"), "net-1", true);
        net1.reason = Some("matched filter".into());
        let net2 = node(ResourceKind::new("network", "network"), "net-2", true);
        let port_selected = node(ResourceKind::new("network", "port"), "port-on-net2", true);
        let mut port_kept = node(ResourceKind::new("network", "port"), "port-on-net1", false);
        port_kept.selected = false;

        let nodes = vec![net1, net2, port_selected, port_kept];
        // index: 0 net-1, 1 net-2, 2 port-on-net2, 3 port-on-net1
        let edges = vec![
            PlanEdge { child: 2, parent: 1, effect: RelationEffect::Blocks }, // port-on-net2 blocks net-2, but port-on-net2 IS selected -> not blocking
            PlanEdge { child: 3, parent: 0, effect: RelationEffect::Blocks }, // port-on-net1 blocks net-1, and port-on-net1 is NOT selected -> blocking
        ];
        let plan = CleanupPlan { nodes, edges };

        // Build a client the same way as the discover test (no HTTP calls
        // are made because RecordingProvider never touches ctx.client).
        let server = httpmock::MockServer::start_async().await;
        let base_url = server.base_url();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/");
            then.status(200).json_body(serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v3/");
            then.status(200).json_body(serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}));
        });
        let expires = (chrono::Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        server.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/v3/auth/tokens");
            then.status(201).header("x-subject-token", "test-token").json_body(serde_json::json!({"token": {
                "id": "token-id", "expires_at": expires,
                "project": {"id": "test-project", "name": "TestProject"},
                "user": {"id": "test-user", "name": "test-user"},
                "methods": ["password"], "audit_ids": ["audit-1"], "catalog": []
            }}));
        });
        let config = openstack_sdk_core::config::CloudConfig {
            auth: Some(openstack_sdk_core::config::Auth {
                auth_url: Some(format!("{}/v3", server.base_url())),
                username: Some("test-user".into()),
                user_domain_name: Some("Default".into()),
                password: Some("test-password".into()),
                project_id: Some("test-project".into()),
                ..Default::default()
            }),
            region_name: Some("RegionOne".into()),
            interface: Some("public".into()),
            auth_cache: Some(false),
            ..Default::default()
        };
        let client = AsyncOpenStack::new_with_authentication_helper(
            &config,
            crate::auth::auth_helper::Noop::default(),
            false,
        )
        .await
        .expect("client creation failed");

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(RecordingProvider { log: log.clone() })
            .build();

        let result = cleanup.apply(plan).await.expect("apply failed");

        let deleted = log.lock().unwrap().clone();
        assert!(deleted.contains(&"port-on-net2".to_string()));
        assert!(deleted.contains(&"net-2".to_string()));
        assert!(
            !deleted.contains(&"net-1".to_string()),
            "net-1 must not be deleted while port-on-net1 is kept"
        );
        assert!(!deleted.contains(&"port-on-net1".to_string()), "unselected node must never be passed to delete()");
        let port2_pos = deleted.iter().position(|id| id == "port-on-net2").unwrap();
        let net2_pos = deleted.iter().position(|id| id == "net-2").unwrap();
        assert!(port2_pos < net2_pos, "child must delete before parent");

        assert!(result.skipped.iter().any(|(id, _)| id == "net-1"));
    }
```

- [ ] **Step 2: Run to verify it fails to compile (no `apply` method yet)**

Run: `cargo test -p openstack_sdk --lib cleanup::engine 2>&1 | tail -30`
Expected: FAIL — `no method named 'apply' found`.

- [ ] **Step 3: Implement `apply`**

Add to `openstack_sdk/src/cleanup/engine.rs`, right after the `discover` method (still inside `impl<'a> ProjectCleanup<'a>`):

```rust
    pub async fn apply(&self, plan: CleanupPlan) -> Result<CleanupResult, CleanupError> {
        let ctx = CleanupContext {
            client: self.client,
            filters: HashMap::new(),
            evaluation_fn: None,
        };

        // A parent is blocked while any `Blocks` child of it is not
        // selected (i.e. is being kept). Compute this once, up front,
        // against the plan as handed in (apply does not re-run discovery).
        let mut blocked: Vec<bool> = vec![false; plan.nodes.len()];
        for edge in plan.edges.iter().filter(|e| e.effect == RelationEffect::Blocks) {
            if !plan.nodes[edge.child].selected {
                blocked[edge.parent] = true;
            }
        }

        // Deletion order: children (via Blocks edges) before parents.
        // Build a DAG over selected, unblocked nodes only and topo-sort it.
        let mut graph = petgraph::graph::DiGraph::<usize, ()>::new();
        let node_ids: Vec<_> = (0..plan.nodes.len()).map(|i| graph.add_node(i)).collect();
        for edge in plan.edges.iter().filter(|e| e.effect == RelationEffect::Blocks) {
            // child must run before parent: edge child -> parent
            graph.add_edge(node_ids[edge.child], node_ids[edge.parent], ());
        }
        let order = petgraph::algo::toposort(&graph, None)
            .map_err(|_| CleanupError::Engine("cycle in resource-level cleanup dependencies".into()))?;

        let mut result = CleanupResult::default();
        for node_idx in order {
            let idx = graph[node_idx];
            if !plan.nodes[idx].selected {
                continue;
            }
            if blocked[idx] {
                let id = plan.nodes[idx].id.clone();
                result.skipped.push((id, "blocked by a kept child resource".into()));
                continue;
            }
            let resource = &plan.nodes[idx];
            let provider = self
                .provider_for_kind(resource.kind)
                .ok_or_else(|| CleanupError::Engine(format!("no provider registered for {:?}", resource.kind)))?;
            match provider.delete(&ctx, resource).await {
                Ok(()) => {
                    result.deleted.push(resource.kind);
                    result.deleted_ids.push(resource.id.clone());
                }
                Err(e) if e.is_not_found() => {
                    result.deleted.push(resource.kind);
                    result.deleted_ids.push(resource.id.clone());
                }
                Err(e) => {
                    result.errors.push((resource.id.clone(), e.to_string()));
                }
            }
        }

        Ok(result)
    }

    fn provider_for_kind(&self, kind: crate::cleanup::types::ResourceKind) -> Option<&dyn CleanupProvider> {
        self.providers
            .iter()
            .find(|p| p.service_type() == kind.service_type)
            .map(|p| p.as_ref())
    }
```

`apply` takes `plan` by value and never reads it back — its (possibly caller-edited) `selected`/`reason` fields exist for `CleanupPlan` to serve as an inspectable artifact between discover and apply, not as apply's output; `CleanupResult` is the output.

Add `CleanupResult` above `ProjectCleanupBuilder` in the same file:

```rust
#[derive(Debug, Clone, Default, Serialize)]
pub struct CleanupResult {
    pub deleted: Vec<crate::cleanup::types::ResourceKind>,
    pub deleted_ids: Vec<String>,
    pub skipped: Vec<(String, String)>,
    pub errors: Vec<(String, String)>,
}
```

(`ResourceKind` already derives `Serialize` from Task 1.)

- [ ] **Step 4: Update the module's public exports**

In `openstack_sdk/src/cleanup/mod.rs`, change the `engine` re-export line to:

```rust
pub use engine::{CleanupPlan, CleanupResult, PlanEdge, ProjectCleanup, ProjectCleanupBuilder};
```

- [ ] **Step 5: Run to verify tests pass**

Run: `cargo test -p openstack_sdk --lib cleanup:: 2>&1 | tail -40`
Expected: PASS — 9 tests total.

- [ ] **Step 6: Commit**

```bash
git add openstack_sdk/src/cleanup/mod.rs openstack_sdk/src/cleanup/engine.rs
git commit -m "feat(sdk): add apply() DAG delete walk honoring Blocks edges"
```

---

### Task 6: `NetworkCleanupProvider` — real provider proving `Blocks` + `CascadeGroup`

**Files:**
- Create: `openstack_sdk/src/cleanup/providers/mod.rs`
- Create: `openstack_sdk/src/cleanup/providers/network.rs`
- Modify: `openstack_sdk/src/cleanup/mod.rs`

**Interfaces:**
- Consumes: `CleanupProvider`, `CleanupContext`, `CleanupError`, `CleanupDependency` from Task 3; `RelationRule`, `RelationEffect` from Task 2; `PlannedResource`, `ResourceKind` from Task 1.
- Produces: `pub struct NetworkCleanupProvider;` (unit struct, `Default` derived) implementing `CleanupProvider`, gated `#[cfg(feature = "network")]`.

This provider covers: `network`, `subnet`, `router`, and `router_interface` (a router's attached interface ports, modeled as their own kind since deleting one means detaching, not calling the port-delete endpoint). It intentionally does not yet cover floating IPs, security groups, or VPN resources — those are separate, additive follow-up tasks once this one is merged, per the design doc's v1 scope note.

Resource kinds used:
- `ResourceKind::new("network", "network")`
- `ResourceKind::new("network", "subnet")`
- `ResourceKind::new("network", "router")`
- `ResourceKind::new("network", "router_interface")`

Relations:
- `subnet` `Blocks` `network` (subnet.network_id == network.id) — a network cannot be deleted while a subnet still exists.
- `router_interface` `Blocks` `network` (router_interface.network_id == network.id) — a network cannot be deleted while still attached to a router.
- `router_interface` `Blocks` `router` (router_interface.device_id == router.id) — a router cannot be deleted while an interface is still attached (mirrors real Neutron behavior: `router_interface`'s `delete` detaches first).
- `subnet` `CascadeGroup` `network`, `router_interface` `CascadeGroup` `network` — selecting a network pulls in its subnets and router interfaces (and, transitively via the interface's `device_id`, its router) so the whole "networks are crazy" group is deleted together, matching python's behavior but generically.

- [ ] **Step 1: Write the failing test**

Create `openstack_sdk/src/cleanup/providers/network.rs`:

```rust
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

//! Network (Neutron) cleanup provider.
//!
//! Proves the resource-level `Blocks`/`CascadeGroup` primitives against
//! the case the python SDK hand-codes imperatively: a network cannot be
//! deleted while it still has subnets or router interfaces, and deleting
//! a network should pull its subnets/router-interfaces/router along with
//! it as one group.

use async_trait::async_trait;
use serde_json::Value;

use crate::api::{Pagination, QueryAsync, paged, raw};
use crate::api::network::v2::network;
use crate::api::network::v2::router;
use crate::api::network::v2::router::remove_router_interface;
use crate::api::network::v2::subnet;

use crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider};
use crate::cleanup::relations::{RelationEffect, RelationRule};
use crate::cleanup::types::{PlannedResource, ResourceKind};

pub const NETWORK: ResourceKind = ResourceKind::new("network", "network");
pub const SUBNET: ResourceKind = ResourceKind::new("network", "subnet");
pub const ROUTER: ResourceKind = ResourceKind::new("network", "router");
pub const ROUTER_INTERFACE: ResourceKind = ResourceKind::new("network", "router_interface");

#[derive(Debug, Default)]
pub struct NetworkCleanupProvider;

fn value_str<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(|x| x.as_str())
}

fn to_planned(kind: ResourceKind, v: Value) -> PlannedResource {
    let id = value_str(&v, "id").unwrap_or_default().to_string();
    let name = value_str(&v, "name").map(str::to_string);
    PlannedResource {
        kind,
        id,
        name,
        raw: v,
        selected: false,
        reason: None,
    }
}

#[async_trait]
impl CleanupProvider for NetworkCleanupProvider {
    fn service_type(&self) -> &'static str {
        "network"
    }

    fn dependencies(&self) -> CleanupDependency {
        CleanupDependency {
            before: vec!["identity"],
            after: vec![],
        }
    }

    fn relations(&self) -> Vec<RelationRule> {
        vec![
            RelationRule {
                parent_kind: NETWORK,
                child_kind: SUBNET,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: NETWORK,
                child_kind: SUBNET,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::CascadeGroup,
            },
            RelationRule {
                parent_kind: NETWORK,
                child_kind: ROUTER_INTERFACE,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: NETWORK,
                child_kind: ROUTER_INTERFACE,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::CascadeGroup,
            },
            RelationRule {
                parent_kind: ROUTER,
                child_kind: ROUTER_INTERFACE,
                matches: |child, parent| {
                    value_str(&child.raw, "device_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
        ]
    }

    async fn discover(&self, ctx: &CleanupContext<'_>) -> Result<Vec<PlannedResource>, CleanupError> {
        // Listing endpoints have no single resource id to attach to a
        // failure, so errors here are reported against an empty id; only
        // `delete()` (below) attaches a real resource id.
        let list_err = |kind: ResourceKind| {
            move |e: crate::OpenStackError| CleanupError::Provider {
                kind,
                id: String::new(),
                source: e,
            }
        };

        let mut nodes = Vec::new();

        let networks: Vec<Value> = paged(network::list::Request::builder().build().unwrap(), Pagination::All)
            .query_async(ctx.client)
            .await
            .map_err(|e| list_err(NETWORK)(e.into()))?;
        for v in networks {
            nodes.push(to_planned(NETWORK, v));
        }

        let subnets: Vec<Value> = paged(subnet::list::Request::builder().build().unwrap(), Pagination::All)
            .query_async(ctx.client)
            .await
            .map_err(|e| list_err(SUBNET)(e.into()))?;
        for v in subnets {
            nodes.push(to_planned(SUBNET, v));
        }

        let routers: Vec<Value> = paged(router::list::Request::builder().build().unwrap(), Pagination::All)
            .query_async(ctx.client)
            .await
            .map_err(|e| list_err(ROUTER)(e.into()))?;
        for router_v in &routers {
            let router_id = value_str(router_v, "id").unwrap_or_default().to_string();
            if let Some(interfaces) = router_v.get("interfaces_info").and_then(|v| v.as_array()) {
                for iface in interfaces {
                    let mut iface = iface.clone();
                    if let Value::Object(map) = &mut iface {
                        map.insert("device_id".into(), Value::String(router_id.clone()));
                        if !map.contains_key("id") {
                            if let Some(port_id) = map.get("port_id").cloned() {
                                map.insert("id".into(), port_id);
                            }
                        }
                    }
                    nodes.push(to_planned(ROUTER_INTERFACE, iface));
                }
            }
            nodes.push(to_planned(ROUTER, router_v.clone()));
        }

        Ok(nodes)
    }

    async fn delete(
        &self,
        ctx: &CleanupContext<'_>,
        resource: &PlannedResource,
    ) -> Result<(), CleanupError> {
        let err = |e: crate::OpenStackError| CleanupError::Provider {
            kind: resource.kind,
            id: resource.id.clone(),
            source: e,
        };

        if resource.kind == NETWORK {
            let req = network::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .unwrap();
            raw(req).query_async(ctx.client).await.map_err(|e| err(e.into()))?;
        } else if resource.kind == SUBNET {
            let req = subnet::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .unwrap();
            raw(req).query_async(ctx.client).await.map_err(|e| err(e.into()))?;
        } else if resource.kind == ROUTER {
            let req = router::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .unwrap();
            raw(req).query_async(ctx.client).await.map_err(|e| err(e.into()))?;
        } else if resource.kind == ROUTER_INTERFACE {
            let router_id = value_str(&resource.raw, "device_id").unwrap_or_default().to_string();
            let mut builder = remove_router_interface::Request::builder();
            builder.id(router_id);
            if let Some(subnet_id) = value_str(&resource.raw, "subnet_id") {
                builder.subnet_id(subnet_id.to_string());
            }
            if let Some(port_id) = value_str(&resource.raw, "port_id") {
                builder.port_id(port_id.to_string());
            }
            let req = builder.build().unwrap();
            raw(req).query_async(ctx.client).await.map_err(|e| err(e.into()))?;
        } else {
            return Err(CleanupError::Engine(format!(
                "NetworkCleanupProvider cannot delete resource kind {:?}",
                resource.kind
            )));
        }
        Ok(())
    }
}
```

`add_router_interface` is never called by cleanup (it only ever removes interfaces), so its `use` is intentionally omitted above.

Create `openstack_sdk/src/cleanup/providers/mod.rs`:

```rust
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

//! Built-in [`crate::cleanup::CleanupProvider`] implementations, one per
//! supported service.

#[cfg(feature = "network")]
pub mod network;
```

- [ ] **Step 2: Verify field/module names against the generated code before wiring in**

The exact shape of `interfaces_info` on a router, and the precise builder method names on `remove_router_interface::Request`, come from generated code this plan didn't fully transcribe (the generator output is large). Before proceeding, run:

```bash
grep -n "pub struct Request" -A 20 sdk/network/src/v2/router/remove_router_interface.rs
grep -n "interfaces_info" -r sdk/network/src
```

Adjust the builder call chain in `delete()`'s `ROUTER_INTERFACE` branch to match whatever fields `remove_router_interface::Request::builder()` actually exposes (likely `id` for the router id, and one of `subnet_id`/`port_id` for the interface identifier — confirm against the grep output rather than assuming). If `interfaces_info` is not present on the router list/get response in this codebase's generated types, discover router interfaces instead via `port::list::Request` filtered to `device_owner` values starting with `network:router_interface`, matching the python proxy's approach at `network/v2/_proxy.py:9972-9977` — in that case add `use crate::api::network::v2::port;` and replace the `interfaces_info` block with a `port::list` call filtered client-side on `device_owner`.

- [ ] **Step 3: Register the module and the `openstack_sdk` crate is imported correctly**

Update `openstack_sdk/src/cleanup/mod.rs`:

```rust
pub mod engine;
pub mod provider;
pub mod providers;
pub mod relations;
pub mod types;

pub use engine::{CleanupPlan, CleanupResult, PlanEdge, ProjectCleanup, ProjectCleanupBuilder};
pub use provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider, service_order};
pub use relations::{Edge, RelationEffect, RelationRule, evaluate_edges};
pub use types::{PlannedResource, ResourceKind};
```

- [ ] **Step 4: Run to verify it compiles**

Run: `cargo check -p openstack_sdk --features network 2>&1 | tail -60`
Expected: compiles cleanly. Fix any field/method-name mismatches surfaced here against the real generated request builders (this is expected — Step 2 flagged the likely spots).

- [ ] **Step 5: Write the integration test (httpmock)**

Append to `openstack_sdk/src/cleanup/providers/network.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cleanup::engine::ProjectCleanupBuilder;
    use httpmock::MockServer;
    use std::collections::HashMap;

    async fn mock_client(server: &MockServer) -> crate::AsyncOpenStack {
        let base_url = server.base_url();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/");
            then.status(200).json_body(serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v3/");
            then.status(200).json_body(serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}));
        });
        let expires = (chrono::Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        server.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/v3/auth/tokens");
            then.status(201).header("x-subject-token", "test-token").json_body(serde_json::json!({"token": {
                "id": "token-id", "expires_at": expires,
                "project": {"id": "test-project", "name": "TestProject"},
                "user": {"id": "test-user", "name": "test-user"},
                "methods": ["password"], "audit_ids": ["audit-1"],
                "catalog": [
                    {"type": "identity", "name": "keystone", "endpoints": [{"id": "identity-1",
                        "url": format!("{base_url}/v3"), "region": "RegionOne", "interface": "public"}]},
                    {"type": "network", "name": "neutron", "endpoints": [{"id": "network-1",
                        "url": format!("{base_url}/v2.0"), "region": "RegionOne", "interface": "public"}]}
                ]
            }}));
        });
        let config = openstack_sdk_core::config::CloudConfig {
            auth: Some(openstack_sdk_core::config::Auth {
                auth_url: Some(format!("{base_url}/v3")),
                username: Some("test-user".into()),
                user_domain_name: Some("Default".into()),
                password: Some("test-password".into()),
                project_id: Some("test-project".into()),
                ..Default::default()
            }),
            region_name: Some("RegionOne".into()),
            interface: Some("public".into()),
            auth_cache: Some(false),
            ..Default::default()
        };
        crate::AsyncOpenStack::new_with_authentication_helper(
            &config,
            crate::auth::auth_helper::Noop::default(),
            false,
        )
        .await
        .expect("client creation failed")
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn discover_marks_network_with_subnet_as_blocked_until_subnet_selected() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/networks");
            then.status(200).json_body(serde_json::json!({"networks": [
                {"id": "net-1", "name": "private"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/subnets");
            then.status(200).json_body(serde_json::json!({"subnets": [
                {"id": "subnet-1", "name": "private-subnet", "network_id": "net-1"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/routers");
            then.status(200).json_body(serde_json::json!({"routers": []}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(NetworkCleanupProvider)
            .build();

        // Evaluation function selects only the network by name; the
        // subnet must still be pulled in via CascadeGroup, and net-1 must
        // remain deletable (its only blocking child, subnet-1, is also
        // selected via cascade).
        let eval: std::sync::Arc<dyn Fn(&PlannedResource) -> bool + Send + Sync> =
            std::sync::Arc::new(|r: &PlannedResource| r.kind == NETWORK && r.name.as_deref() == Some("private"));

        let plan = cleanup
            .discover(HashMap::new(), Some(eval))
            .await
            .expect("discover failed");

        let net = plan.nodes.iter().find(|n| n.id == "net-1").unwrap();
        let subnet = plan.nodes.iter().find(|n| n.id == "subnet-1").unwrap();
        assert!(net.selected);
        assert!(subnet.selected, "subnet must be pulled in by the cascade group");

        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE).path("/v2.0/subnets/subnet-1");
            then.status(204);
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE).path("/v2.0/networks/net-1");
            then.status(204);
        });

        let result = cleanup.apply(plan).await.expect("apply failed");
        assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
        assert!(result.deleted_ids.contains(&"subnet-1".to_string()));
        assert!(result.deleted_ids.contains(&"net-1".to_string()));
        let subnet_pos = result.deleted_ids.iter().position(|id| id == "subnet-1").unwrap();
        let net_pos = result.deleted_ids.iter().position(|id| id == "net-1").unwrap();
        assert!(subnet_pos < net_pos, "subnet must delete before its network");
    }
}
```

- [ ] **Step 6: Run to verify tests pass**

Run: `cargo test -p openstack_sdk --features network --lib cleanup:: 2>&1 | tail -60`
Expected: PASS — all cleanup module tests, including this new integration test.

- [ ] **Step 7: Commit**

```bash
git add openstack_sdk/src/cleanup/mod.rs openstack_sdk/src/cleanup/providers/mod.rs openstack_sdk/src/cleanup/providers/network.rs
git commit -m "feat(sdk): add NetworkCleanupProvider proving Blocks/CascadeGroup rules"
```

---

## Follow-up work (not in this plan)

- Additional built-in providers (compute, block-storage, image, identity-scoped resources), each a small follow-up plan that only adds a new `providers/<service>.rs` file plus relation rules — no engine changes, per the extensibility goal in the design doc.
- `openstack_cli`/`openstack_tui` command surface for `osc project cleanup` presenting `CleanupPlan` as an editable table and calling `apply()` — separate plan, out of scope here since it's a CLI/TUI concern, not an SDK one.
- Floating IPs, security groups, and VPN resources on the network provider (python's `network/v2/_proxy.py` also handles these) — additive to Task 6's provider once merged.
