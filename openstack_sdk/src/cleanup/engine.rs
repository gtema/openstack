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

use serde::{Deserialize, Serialize};

use crate::AsyncOpenStack;
use crate::cleanup::provider::{
    CleanupContext, CleanupError, CleanupProvider, EvaluationFn, service_layers,
};
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CleanupPlan {
    pub nodes: Vec<PlannedResource>,
    pub edges: Vec<PlanEdge>,
    /// Providers that failed during discovery, as `(service_type, message)`.
    /// Discovery continues past a failing provider so the rest of the plan
    /// still reflects every service that listed successfully; a caller
    /// that needs an all-or-nothing guarantee should check this is empty
    /// before calling `apply()`.
    #[serde(default)]
    pub errors: Vec<(String, String)>,
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
    for edge in edges
        .iter()
        .filter(|e| e.effect == RelationEffect::CascadeGroup)
    {
        union(&mut parent, edge.child, edge.parent);
    }

    let mut group_selected: HashMap<usize, usize> = HashMap::new(); // root -> index of a selected member
    for (i, node) in nodes.iter().enumerate() {
        if node.selected {
            let root = find(&mut parent, i);
            group_selected.entry(root).or_insert(i);
        }
    }
    for i in 0..nodes.len() {
        let root = find(&mut parent, i);
        if let Some(&selected_idx) = group_selected.get(&root)
            && !nodes[i].selected
        {
            let cause_id = nodes[selected_idx].id.clone();
            nodes[i].selected = true;
            nodes[i].reason = Some(format!("cascade: {cause_id}"));
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CleanupResult {
    pub deleted: Vec<crate::cleanup::types::ResourceKind>,
    pub deleted_ids: Vec<String>,
    pub skipped: Vec<(String, String)>,
    pub errors: Vec<(String, String)>,
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
        evaluation_fn: Option<EvaluationFn>,
    ) -> Result<CleanupPlan, CleanupError> {
        let provider_refs: Vec<&dyn CleanupProvider> =
            self.providers.iter().map(|p| p.as_ref()).collect();
        let layers = service_layers(&provider_refs)?;

        let ctx = CleanupContext {
            client: self.client,
            filters,
            evaluation_fn,
        };

        // Service-level ordering only gates *listing*, not selection: run
        // providers in dependency order, but nothing stops a later
        // provider's rules from referencing an earlier provider's nodes.
        let mut nodes: Vec<PlannedResource> = Vec::new();
        let mut errors: Vec<(String, String)> = Vec::new();
        for layer in layers {
            // Providers within one layer have no dependency relationship
            // to each other (see `service_layers`), so list them
            // concurrently; layers themselves still run in order.
            let futures = layer.iter().map(|&idx| self.providers[idx].discover(&ctx));
            let results = futures::future::join_all(futures).await;

            for (&idx, result) in layer.iter().zip(results) {
                let mut discovered = match result {
                    Ok(discovered) => discovered,
                    Err(e) => {
                        errors.push((
                            self.providers[idx].service_type().to_string(),
                            e.to_string(),
                        ));
                        continue;
                    }
                };
                for r in &mut discovered {
                    r.selected = if let Some(eval) = &ctx.evaluation_fn {
                        eval(r)
                    } else {
                        crate::cleanup::filters::evaluate_filters(r, &ctx.filters)
                    };
                    if r.selected && r.reason.is_none() {
                        r.reason = Some(if ctx.evaluation_fn.is_some() {
                            "matched evaluation_fn".into()
                        } else {
                            "matched filters".into()
                        });
                    }
                }
                nodes.append(&mut discovered);
            }
        }

        let all_rules: Vec<_> = self.providers.iter().flat_map(|p| p.relations()).collect();
        let edges = evaluate_edges(&nodes, &all_rules);

        propagate_cascade_groups(&mut nodes, &edges);

        Ok(CleanupPlan {
            nodes,
            edges: edges.into_iter().map(PlanEdge::from).collect(),
            errors,
        })
    }

    pub async fn apply(&self, plan: CleanupPlan) -> Result<CleanupResult, CleanupError> {
        let ctx = CleanupContext {
            client: self.client,
            filters: HashMap::new(),
            evaluation_fn: None,
        };

        // A parent is blocked while any `Blocks` child of it is not
        // selected (i.e. is being kept), or -- once that child's own
        // layer has been processed below -- while it turned out not to
        // be actually deleted (itself blocked, or its delete call
        // errored). `blocked` starts from the directly-unselected case
        // and is then extended as each layer resolves.
        let mut blocked: Vec<bool> = vec![false; plan.nodes.len()];
        let mut blocks_parents_of: Vec<Vec<usize>> = vec![Vec::new(); plan.nodes.len()];
        for edge in plan
            .edges
            .iter()
            .filter(|e| e.effect == RelationEffect::Blocks)
        {
            blocks_parents_of[edge.child].push(edge.parent);
            if !plan.nodes[edge.child].selected {
                blocked[edge.parent] = true;
            }
        }

        // Deletion order: children (via `Blocks` edges) before parents,
        // grouped into layers so that nodes with no `Blocks` relationship
        // to each other -- which, by construction, is every pair within
        // one layer -- can be deleted concurrently. A node's own
        // `Blocks`-children are always in a strictly earlier layer, so by
        // the time layer N starts, every flag that could affect it has
        // already been finalized by earlier layers; no locking is needed
        // for `blocked`/`result`, since all mutation of that shared state
        // happens in a purely sequential phase after each layer's
        // concurrent deletes have fully resolved.
        let mut graph = petgraph::graph::DiGraph::<usize, ()>::new();
        let node_ids: Vec<_> = (0..plan.nodes.len()).map(|i| graph.add_node(i)).collect();
        for edge in plan
            .edges
            .iter()
            .filter(|e| e.effect == RelationEffect::Blocks)
        {
            // child must run before parent: edge child -> parent
            graph.add_edge(node_ids[edge.child], node_ids[edge.parent], ());
        }
        let layers = Self::compute_layers(&graph)?;

        let mut result = CleanupResult::default();
        for layer in layers {
            let mut to_delete: Vec<usize> = Vec::new();
            for &idx in &layer {
                if !plan.nodes[idx].selected {
                    continue;
                }
                if blocked[idx] {
                    let id = plan.nodes[idx].id.clone();
                    result.skipped.push((
                        id,
                        "blocked by a kept, skipped, or failed child resource".into(),
                    ));
                    for &parent_idx in &blocks_parents_of[idx] {
                        blocked[parent_idx] = true;
                    }
                    continue;
                }
                to_delete.push(idx);
            }

            // Concurrent phase: no shared mutable state is touched here,
            // only immutable `&ctx`/`&resource` borrows and each
            // provider's own `delete()` call.
            let mut futures = Vec::with_capacity(to_delete.len());
            for &idx in &to_delete {
                let resource = &plan.nodes[idx];
                let provider = self.provider_for_kind(resource.kind).ok_or_else(|| {
                    CleanupError::Engine(format!("no provider registered for {:?}", resource.kind))
                })?;
                futures.push(provider.delete(&ctx, resource));
            }
            let delete_results = futures::future::join_all(futures).await;

            // Sequential phase: safe to mutate `result`/`blocked` here,
            // since every future above has already resolved.
            for (&idx, delete_result) in to_delete.iter().zip(delete_results) {
                let resource = &plan.nodes[idx];
                match delete_result {
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
                        for &parent_idx in &blocks_parents_of[idx] {
                            blocked[parent_idx] = true;
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Group a `Blocks`-edge DAG into ordered layers: every node in layer
    /// N has all of its incoming-edge predecessors in layers `0..N`, and
    /// nodes within the same layer have no edge between them at all.
    /// Layers must be processed in order; nodes within one layer have no
    /// dependency relationship and may be processed concurrently.
    fn compute_layers(
        graph: &petgraph::graph::DiGraph<usize, ()>,
    ) -> Result<Vec<Vec<usize>>, CleanupError> {
        let mut in_degree: Vec<usize> = graph
            .node_indices()
            .map(|n| {
                graph
                    .neighbors_directed(n, petgraph::Direction::Incoming)
                    .count()
            })
            .collect();
        let mut remaining = graph.node_count();
        let mut placed = vec![false; graph.node_count()];
        let mut layers: Vec<Vec<usize>> = Vec::new();

        while remaining > 0 {
            let ready: Vec<_> = graph
                .node_indices()
                .filter(|n| !placed[n.index()] && in_degree[n.index()] == 0)
                .collect();
            if ready.is_empty() {
                return Err(CleanupError::Engine(
                    "cycle in resource-level cleanup dependencies".into(),
                ));
            }
            for &n in &ready {
                placed[n.index()] = true;
                remaining -= 1;
                for succ in graph.neighbors_directed(n, petgraph::Direction::Outgoing) {
                    in_degree[succ.index()] -= 1;
                }
            }
            layers.push(ready.into_iter().map(|n| graph[n]).collect());
        }

        Ok(layers)
    }

    fn provider_for_kind(
        &self,
        kind: crate::cleanup::types::ResourceKind,
    ) -> Option<&dyn CleanupProvider> {
        self.providers
            .iter()
            .find(|p| p.service_type() == kind.service_type)
            .map(|p| p.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cleanup::relations::RelationEffect;
    use crate::cleanup::types::ResourceKind;
    use serde_json::json;
    use std::sync::Arc;

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
            Edge {
                child: 1,
                parent: 0,
                effect: RelationEffect::CascadeGroup,
            },
            Edge {
                child: 2,
                parent: 1,
                effect: RelationEffect::CascadeGroup,
            },
        ];
        propagate_cascade_groups(&mut nodes, &edges);
        assert!(nodes[0].selected);
        assert!(nodes[1].selected, "router-1 must be pulled in transitively");
        assert!(nodes[2].selected, "subnet-1 must be pulled in transitively");
        assert!(!nodes[3].selected, "unrelated network must not be selected");
        assert_eq!(nodes[1].reason.as_deref(), Some("cascade: net-1"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
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
            async fn discover(
                &self,
                _ctx: &CleanupContext<'_>,
            ) -> Result<Vec<PlannedResource>, CleanupError> {
                Ok(vec![
                    node(ResourceKind::new("fake", "thing"), "1", false),
                    node(ResourceKind::new("fake", "thing"), "2", false),
                ])
            }
            async fn delete(
                &self,
                _ctx: &CleanupContext<'_>,
                _r: &PlannedResource,
            ) -> Result<(), CleanupError> {
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
                    "catalog": [
                        {"type": "identity", "name": "keystone", "endpoints": [{
                            "id": "identity-1", "url": format!("{base_url}/v3"),
                            "region": "RegionOne", "interface": "public"
                        }]}
                    ]
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
        let selected: Vec<_> = plan
            .nodes
            .iter()
            .filter(|n| n.selected)
            .map(|n| n.id.clone())
            .collect();
        assert_eq!(selected, vec!["2".to_string()]);
    }

    #[tokio::test]
    async fn discover_applies_built_in_filters_when_no_evaluation_fn_given() {
        use crate::cleanup::provider::CleanupDependency;
        use async_trait::async_trait;

        struct TimestampedProvider;
        #[async_trait]
        impl CleanupProvider for TimestampedProvider {
            fn service_type(&self) -> &'static str {
                "fake"
            }
            fn dependencies(&self) -> CleanupDependency {
                CleanupDependency::default()
            }
            async fn discover(
                &self,
                _ctx: &CleanupContext<'_>,
            ) -> Result<Vec<PlannedResource>, CleanupError> {
                Ok(vec![
                    PlannedResource {
                        kind: ResourceKind::new("fake", "thing"),
                        id: "old-1".into(),
                        name: None,
                        raw: serde_json::json!({"created_at": "2024-01-01T00:00:00Z"}),
                        selected: false,
                        reason: None,
                    },
                    PlannedResource {
                        kind: ResourceKind::new("fake", "thing"),
                        id: "new-1".into(),
                        name: None,
                        raw: serde_json::json!({"created_at": "2024-12-01T00:00:00Z"}),
                        selected: false,
                        reason: None,
                    },
                ])
            }
            async fn delete(
                &self,
                _ctx: &CleanupContext<'_>,
                _resource: &PlannedResource,
            ) -> Result<(), CleanupError> {
                Ok(())
            }
        }

        // Build a client the same way as
        // discover_merges_providers_and_applies_evaluation_fn (no HTTP
        // calls are made because TimestampedProvider never touches
        // ctx.client).
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
                    "catalog": [
                        {"type": "identity", "name": "keystone", "endpoints": [{
                            "id": "identity-1", "url": format!("{base_url}/v3"),
                            "region": "RegionOne", "interface": "public"
                        }]}
                    ]
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
            .with_provider(TimestampedProvider)
            .build();

        let mut filters = HashMap::new();
        filters.insert("created_at".to_string(), "2024-06-01T00:00:00Z".to_string());
        let plan = cleanup
            .discover(filters, None)
            .await
            .expect("discover must succeed");

        let old = plan
            .nodes
            .iter()
            .find(|n| n.id == "old-1")
            .expect("old-1 node must exist");
        let new = plan
            .nodes
            .iter()
            .find(|n| n.id == "new-1")
            .expect("new-1 node must exist");
        assert!(
            old.selected,
            "old-1 was created before the cutoff, must be selected"
        );
        assert!(
            !new.selected,
            "new-1 was created after the cutoff, must not be selected"
        );

        // With NO filters and no evaluation_fn, everything must be
        // selected -- the real project_cleanup()-style default.
        let plan_no_filters = cleanup
            .discover(HashMap::new(), None)
            .await
            .expect("discover must succeed");
        assert!(plan_no_filters.nodes.iter().all(|n| n.selected));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn discover_continues_past_a_failing_provider() {
        use crate::cleanup::provider::CleanupDependency;
        use async_trait::async_trait;

        struct OkProvider;
        #[async_trait]
        impl CleanupProvider for OkProvider {
            fn service_type(&self) -> &'static str {
                "ok-service"
            }
            fn dependencies(&self) -> CleanupDependency {
                CleanupDependency::default()
            }
            async fn discover(
                &self,
                _ctx: &CleanupContext<'_>,
            ) -> Result<Vec<PlannedResource>, CleanupError> {
                Ok(vec![node(
                    ResourceKind::new("ok-service", "thing"),
                    "ok-1",
                    true,
                )])
            }
            async fn delete(
                &self,
                _ctx: &CleanupContext<'_>,
                _resource: &PlannedResource,
            ) -> Result<(), CleanupError> {
                Ok(())
            }
        }

        struct FailingProvider;
        #[async_trait]
        impl CleanupProvider for FailingProvider {
            fn service_type(&self) -> &'static str {
                "failing-service"
            }
            fn dependencies(&self) -> CleanupDependency {
                CleanupDependency::default()
            }
            async fn discover(
                &self,
                _ctx: &CleanupContext<'_>,
            ) -> Result<Vec<PlannedResource>, CleanupError> {
                Err(CleanupError::Engine("simulated listing failure".into()))
            }
            async fn delete(
                &self,
                _ctx: &CleanupContext<'_>,
                _resource: &PlannedResource,
            ) -> Result<(), CleanupError> {
                Ok(())
            }
        }

        // Build a client the same way as discover_merges_providers_and_applies_evaluation_fn
        // (no HTTP calls are made because neither test double touches ctx.client).
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
                    "catalog": [
                        {"type": "identity", "name": "keystone", "endpoints": [{
                            "id": "identity-1", "url": format!("{base_url}/v3"),
                            "region": "RegionOne", "interface": "public"
                        }]}
                    ]
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
            .with_provider(FailingProvider)
            .with_provider(OkProvider)
            .build();

        let plan = cleanup
            .discover(HashMap::new(), None)
            .await
            .expect("discover must not abort on a single provider failure");

        assert!(
            plan.nodes.iter().any(|n| n.id == "ok-1"),
            "ok-service's node must still be present even though failing-service errored"
        );
        assert_eq!(plan.errors.len(), 1);
        assert_eq!(plan.errors[0].0, "failing-service");
        assert!(plan.errors[0].1.contains("simulated listing failure"));
    }

    #[tokio::test]
    async fn discover_respects_before_after_ordering_across_layers() {
        use crate::cleanup::provider::CleanupDependency;
        use async_trait::async_trait;
        use std::sync::atomic::{AtomicU32, Ordering as AtomicOrdering};

        // `dependent` has a `before` dependency on `independent`, meaning
        // `independent` must be listed first. `other_independent` has no
        // dependency on anything and should share `independent`'s layer.
        // A shared atomic counter records the order calls actually
        // completed in, so this test catches a regression where layering
        // stops respecting before/after (not just a node-presence check).
        struct OrderRecordingProvider {
            name: &'static str,
            deps: CleanupDependency,
            counter: std::sync::Arc<AtomicU32>,
            recorded_at: std::sync::Arc<std::sync::Mutex<Option<u32>>>,
        }
        #[async_trait]
        impl CleanupProvider for OrderRecordingProvider {
            fn service_type(&self) -> &'static str {
                self.name
            }
            fn dependencies(&self) -> CleanupDependency {
                self.deps.clone()
            }
            async fn discover(
                &self,
                _ctx: &CleanupContext<'_>,
            ) -> Result<Vec<PlannedResource>, CleanupError> {
                let seq = self.counter.fetch_add(1, AtomicOrdering::SeqCst);
                *self.recorded_at.lock().unwrap_or_else(|p| p.into_inner()) = Some(seq);
                Ok(vec![node(
                    ResourceKind::new(self.name, "thing"),
                    self.name,
                    true,
                )])
            }
            async fn delete(
                &self,
                _ctx: &CleanupContext<'_>,
                _resource: &PlannedResource,
            ) -> Result<(), CleanupError> {
                Ok(())
            }
        }

        let counter = std::sync::Arc::new(AtomicU32::new(0));
        let independent_at = std::sync::Arc::new(std::sync::Mutex::new(None));
        let dependent_at = std::sync::Arc::new(std::sync::Mutex::new(None));

        let independent = OrderRecordingProvider {
            name: "independent-svc",
            deps: CleanupDependency::default(),
            counter: counter.clone(),
            recorded_at: independent_at.clone(),
        };
        let dependent = OrderRecordingProvider {
            name: "dependent-svc",
            deps: CleanupDependency {
                before: vec![],
                after: vec!["independent-svc"],
            },
            counter: counter.clone(),
            recorded_at: dependent_at.clone(),
        };

        // Build a client the same way as discover_continues_past_a_failing_provider
        // (no HTTP calls are made because neither test double touches ctx.client).
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
                    "catalog": [
                        {"type": "identity", "name": "keystone", "endpoints": [{
                            "id": "identity-1", "url": format!("{base_url}/v3"),
                            "region": "RegionOne", "interface": "public"
                        }]}
                    ]
                }}));
        });

        let client = AsyncOpenStack::new_with_authentication_helper(
            &config,
            crate::auth::auth_helper::Noop::default(),
            false,
        )
        .await
        .expect("client creation failed");

        // Register `dependent` before `independent` on purpose, to prove
        // ordering comes from the dependency graph, not registration order.
        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(dependent)
            .with_provider(independent)
            .build();

        let plan = cleanup
            .discover(HashMap::new(), None)
            .await
            .expect("discover must succeed");

        assert!(plan.nodes.iter().any(|n| n.id == "independent-svc"));
        assert!(plan.nodes.iter().any(|n| n.id == "dependent-svc"));

        let independent_seq = independent_at
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .expect("independent must have run");
        let dependent_seq = dependent_at
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .expect("dependent must have run");
        assert!(
            independent_seq < dependent_seq,
            "independent-svc must complete its discover() call before dependent-svc's starts, \
             despite dependent-svc being registered first"
        );
    }

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
            async fn discover(
                &self,
                _ctx: &CleanupContext<'_>,
            ) -> Result<Vec<PlannedResource>, CleanupError> {
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
        let mut net1 = node(ResourceKind::new("fake", "network"), "net-1", true);
        net1.reason = Some("matched filter".into());
        let net2 = node(ResourceKind::new("fake", "network"), "net-2", true);
        let port_selected = node(ResourceKind::new("fake", "port"), "port-on-net2", true);
        let mut port_kept = node(ResourceKind::new("fake", "port"), "port-on-net1", false);
        port_kept.selected = false;

        let nodes = vec![net1, net2, port_selected, port_kept];
        // index: 0 net-1, 1 net-2, 2 port-on-net2, 3 port-on-net1
        let edges = vec![
            PlanEdge {
                child: 2,
                parent: 1,
                effect: RelationEffect::Blocks,
            }, // port-on-net2 blocks net-2, but port-on-net2 IS selected -> not blocking
            PlanEdge {
                child: 3,
                parent: 0,
                effect: RelationEffect::Blocks,
            }, // port-on-net1 blocks net-1, and port-on-net1 is NOT selected -> blocking
        ];
        let plan = CleanupPlan {
            nodes,
            edges,
            errors: Vec::new(),
        };

        // Build a client the same way as the discover test (no HTTP calls
        // are made because RecordingProvider never touches ctx.client).
        let server = httpmock::MockServer::start_async().await;
        let base_url = server.base_url();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/");
            then.status(200).json_body(
                serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}),
            );
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v3/");
            then.status(200).json_body(
                serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}),
            );
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
        assert!(
            !deleted.contains(&"port-on-net1".to_string()),
            "unselected node must never be passed to delete()"
        );
        let port2_pos = deleted.iter().position(|id| id == "port-on-net2").unwrap();
        let net2_pos = deleted.iter().position(|id| id == "net-2").unwrap();
        assert!(port2_pos < net2_pos, "child must delete before parent");

        assert!(result.skipped.iter().any(|(id, _)| id == "net-1"));
    }

    #[tokio::test]
    async fn apply_propagates_block_transitively_when_child_delete_errors() {
        use crate::cleanup::provider::CleanupDependency;
        use async_trait::async_trait;

        // grandparent <-Blocks- parent <-Blocks- child
        // child's delete always errors (a real, non-"not found" failure).
        // parent must therefore be skipped (it still "has" child), and
        // grandparent must ALSO be skipped, transitively, even though its
        // own direct Blocks-child (parent) was selected and not itself
        // "kept" in the traditional sense -- it just never actually got
        // deleted.
        struct FailingChildProvider;
        #[async_trait]
        impl CleanupProvider for FailingChildProvider {
            fn service_type(&self) -> &'static str {
                "fake"
            }
            fn dependencies(&self) -> CleanupDependency {
                CleanupDependency::default()
            }
            async fn discover(
                &self,
                _ctx: &CleanupContext<'_>,
            ) -> Result<Vec<PlannedResource>, CleanupError> {
                Ok(Vec::new())
            }
            async fn delete(
                &self,
                _ctx: &CleanupContext<'_>,
                resource: &PlannedResource,
            ) -> Result<(), CleanupError> {
                if resource.id == "child-1" {
                    Err(CleanupError::Engine("simulated delete failure".into()))
                } else {
                    Ok(())
                }
            }
        }

        let grandparent = node(ResourceKind::new("fake", "thing"), "grandparent-1", true);
        let parent = node(ResourceKind::new("fake", "thing"), "parent-1", true);
        let child = node(ResourceKind::new("fake", "thing"), "child-1", true);
        let nodes = vec![grandparent, parent, child];
        // index: 0 grandparent, 1 parent, 2 child
        let edges = vec![
            PlanEdge {
                child: 2,
                parent: 1,
                effect: RelationEffect::Blocks,
            }, // child-1 blocks parent-1
            PlanEdge {
                child: 1,
                parent: 0,
                effect: RelationEffect::Blocks,
            }, // parent-1 blocks grandparent-1
        ];
        let plan = CleanupPlan {
            nodes,
            edges,
            errors: Vec::new(),
        };

        let server = httpmock::MockServer::start_async().await;
        let base_url = server.base_url();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/");
            then.status(200).json_body(
                serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}),
            );
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v3/");
            then.status(200).json_body(
                serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}),
            );
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
            .with_provider(FailingChildProvider)
            .build();

        let result = cleanup.apply(plan).await.expect("apply failed");

        assert!(
            result.errors.iter().any(|(id, _)| id == "child-1"),
            "child-1's real delete failure must be recorded"
        );
        assert!(
            result.skipped.iter().any(|(id, _)| id == "parent-1"),
            "parent-1 must be skipped because child-1 was never actually deleted"
        );
        assert!(
            result.skipped.iter().any(|(id, _)| id == "grandparent-1"),
            "grandparent-1 must be transitively skipped because parent-1 was never actually deleted, \
             even though grandparent-1's direct Blocks-child (parent-1) was itself selected"
        );
        assert!(
            !result.deleted_ids.contains(&"parent-1".to_string())
                && !result.deleted_ids.contains(&"grandparent-1".to_string()),
            "neither parent-1 nor grandparent-1 may be deleted"
        );
    }

    #[tokio::test]
    async fn apply_deletes_independent_subgraphs_concurrently() {
        use crate::cleanup::provider::CleanupDependency;
        use async_trait::async_trait;
        use tokio::sync::Barrier;

        // Two completely independent single-node "resources" (no edges
        // between them at all) plus a two-node Blocks chain (child must
        // finish before parent starts). If layering works, layer 0
        // contains {independent-a, independent-b, chain-child} and layer
        // 1 contains {chain-parent} -- so independent-a and
        // independent-b's delete() calls should be in flight
        // concurrently within layer 0. Use a `Barrier` sized to the
        // number of concurrent deletes expected in layer 0 to prove they
        // actually overlap: each of the three layer-0 deletes waits on
        // the barrier before completing, so the test would hang/timeout
        // if the engine ran them one at a time instead of concurrently.
        struct BarrierProvider {
            barrier: Arc<Barrier>,
            completed_order: Arc<std::sync::Mutex<Vec<String>>>,
        }
        #[async_trait]
        impl CleanupProvider for BarrierProvider {
            fn service_type(&self) -> &'static str {
                "fake"
            }
            fn dependencies(&self) -> CleanupDependency {
                CleanupDependency::default()
            }
            async fn discover(
                &self,
                _ctx: &CleanupContext<'_>,
            ) -> Result<Vec<PlannedResource>, CleanupError> {
                Ok(Vec::new())
            }
            async fn delete(
                &self,
                _ctx: &CleanupContext<'_>,
                resource: &PlannedResource,
            ) -> Result<(), CleanupError> {
                if resource.id == "chain-parent" {
                    // Not part of the concurrent layer-0 barrier: runs
                    // alone in layer 1, after layer 0 fully resolves.
                    self.completed_order
                        .lock()
                        .unwrap_or_else(|p| p.into_inner())
                        .push(resource.id.clone());
                    return Ok(());
                }
                // All three layer-0 members (independent-a,
                // independent-b, chain-child) must reach this barrier
                // concurrently for the test to proceed; if the engine
                // serialized them, this would deadlock and the test
                // would time out.
                self.barrier.wait().await;
                self.completed_order
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .push(resource.id.clone());
                Ok(())
            }
        }

        let independent_a = node(ResourceKind::new("fake", "thing"), "independent-a", true);
        let independent_b = node(ResourceKind::new("fake", "thing"), "independent-b", true);
        let chain_child = node(ResourceKind::new("fake", "thing"), "chain-child", true);
        let chain_parent = node(ResourceKind::new("fake", "thing"), "chain-parent", true);
        let nodes = vec![independent_a, independent_b, chain_child, chain_parent];
        // index: 0 independent-a, 1 independent-b, 2 chain-child, 3 chain-parent
        let edges = vec![PlanEdge {
            child: 2,
            parent: 3,
            effect: RelationEffect::Blocks,
        }];
        let plan = CleanupPlan {
            nodes,
            edges,
            errors: Vec::new(),
        };

        let server = httpmock::MockServer::start_async().await;
        let base_url = server.base_url();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/");
            then.status(200).json_body(
                serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}),
            );
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v3/");
            then.status(200).json_body(
                serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}),
            );
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

        let barrier = Arc::new(Barrier::new(3));
        let completed_order = Arc::new(std::sync::Mutex::new(Vec::new()));
        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(BarrierProvider {
                barrier: barrier.clone(),
                completed_order: completed_order.clone(),
            })
            .build();

        let result = tokio::time::timeout(std::time::Duration::from_secs(5), cleanup.apply(plan))
            .await
            .expect("apply() must not hang/deadlock waiting on the barrier -- if this times out, layer 0's three independent deletes are NOT running concurrently")
            .expect("apply failed");

        assert_eq!(result.deleted_ids.len(), 4);
        assert!(result.errors.is_empty());
        assert!(result.skipped.is_empty());

        let order = completed_order.lock().unwrap_or_else(|p| p.into_inner());
        let chain_parent_pos = order.iter().position(|id| id == "chain-parent").unwrap();
        let chain_child_pos = order.iter().position(|id| id == "chain-child").unwrap();
        assert!(
            chain_child_pos < chain_parent_pos,
            "chain-child must still complete before chain-parent starts, even with concurrency enabled for independent nodes"
        );
    }
}
