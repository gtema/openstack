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

/// Caller-supplied predicate used to override built-in filter evaluation
/// for a [`PlannedResource`] during discovery.
pub type EvaluationFn = Arc<dyn Fn(&PlannedResource) -> bool + Send + Sync>;

/// Per-run context handed to every provider call.
pub struct CleanupContext<'a> {
    pub client: &'a AsyncOpenStack,
    pub filters: HashMap<String, String>,
    pub evaluation_fn: Option<EvaluationFn>,
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

    async fn discover(
        &self,
        ctx: &CleanupContext<'_>,
    ) -> Result<Vec<PlannedResource>, CleanupError>;

    async fn delete(
        &self,
        ctx: &CleanupContext<'_>,
        resource: &PlannedResource,
    ) -> Result<(), CleanupError>;
}

fn build_service_graph(providers: &[&dyn CleanupProvider]) -> DiGraph<usize, ()> {
    let mut graph = DiGraph::<usize, ()>::new();
    let node_ids: Vec<_> = (0..providers.len()).map(|i| graph.add_node(i)).collect();
    let index_of = |service_type: &str| {
        providers
            .iter()
            .position(|p| p.service_type() == service_type)
    };

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

    graph
}

/// Order providers so that every `before`/`after` hint is satisfied.
/// Returns indices into `providers`. Kept for callers that only need a
/// flat order (e.g. anything that doesn't care about concurrency); prefer
/// `service_layers` when providers with no dependency relationship should
/// run concurrently.
pub fn service_order(providers: &[&dyn CleanupProvider]) -> Result<Vec<usize>, CleanupError> {
    let graph = build_service_graph(providers);
    toposort(&graph, None)
        .map(|order| order.into_iter().map(|n| graph[n]).collect())
        .map_err(|_| CleanupError::Engine("cycle in service-level cleanup dependencies".into()))
}

/// Group providers into ordered layers: every provider in layer N has all
/// of its `before`/`after` dependencies satisfied by providers in layers
/// `0..N`, and providers within the same layer have no dependency
/// relationship to each other. Layers must be processed in order; the
/// providers within one layer may be processed concurrently.
pub fn service_layers(providers: &[&dyn CleanupProvider]) -> Result<Vec<Vec<usize>>, CleanupError> {
    let graph = build_service_graph(providers);

    // Repeated Kahn peeling: each round, every node with no remaining
    // incoming edge (from nodes not yet placed in an earlier layer) forms
    // the next layer.
    let mut in_degree: Vec<usize> = graph
        .node_indices()
        .map(|n| {
            graph
                .neighbors_directed(n, petgraph::Direction::Incoming)
                .count()
        })
        .collect();
    let mut remaining: usize = graph.node_count();
    let mut placed = vec![false; graph.node_count()];
    let mut layers: Vec<Vec<usize>> = Vec::new();

    while remaining > 0 {
        let ready: Vec<_> = graph
            .node_indices()
            .filter(|n| !placed[n.index()] && in_degree[n.index()] == 0)
            .collect();
        if ready.is_empty() {
            return Err(CleanupError::Engine(
                "cycle in service-level cleanup dependencies".into(),
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
        async fn discover(
            &self,
            _ctx: &CleanupContext<'_>,
        ) -> Result<Vec<PlannedResource>, CleanupError> {
            Ok(Vec::new())
        }
        async fn delete(
            &self,
            _ctx: &CleanupContext<'_>,
            _r: &PlannedResource,
        ) -> Result<(), CleanupError> {
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
        assert!(
            network_pos < identity_pos,
            "network must be ordered before identity"
        );
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

    #[test]
    fn service_layers_groups_independent_providers_together() {
        // identity has no deps; network must run before identity;
        // compute has no deps either, so compute and network should
        // land in the same layer (both are "ready" immediately), while
        // identity must land in a strictly later layer than network.
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
        let compute = FakeProvider {
            service_type: "compute",
            deps: CleanupDependency::default(),
        };
        let providers: Vec<&dyn CleanupProvider> = vec![&identity, &network, &compute];
        let layers = service_layers(&providers).unwrap();

        let layer_of = |idx: usize| {
            layers
                .iter()
                .position(|layer| layer.contains(&idx))
                .unwrap()
        };
        let network_idx = 1;
        let identity_idx = 0;
        let compute_idx = 2;

        assert!(
            layer_of(network_idx) < layer_of(identity_idx),
            "network must be in a strictly earlier layer than identity"
        );
        // compute has no dependency on anything, so it must share
        // network's layer (both are immediately ready) rather than being
        // serialized after it for no reason.
        assert_eq!(
            layer_of(compute_idx),
            layer_of(network_idx),
            "compute and network have no dependency relationship and must share a layer"
        );

        // Every provider must appear in exactly one layer.
        let total: usize = layers.iter().map(|l| l.len()).sum();
        assert_eq!(total, 3);
    }

    #[test]
    fn service_layers_reports_cycle_as_engine_error() {
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
        let err = service_layers(&providers).unwrap_err();
        assert!(matches!(err, CleanupError::Engine(_)));
    }
}
