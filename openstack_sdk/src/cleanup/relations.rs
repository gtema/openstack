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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
                if parent.kind != rule.parent_kind || parent_idx == child_idx {
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
                child.raw.get("network_id").and_then(|v| v.as_str())
                    == parent.raw.get("id").and_then(|v| v.as_str())
            },
            effect: RelationEffect::Blocks,
        }
    }

    #[test]
    fn blocks_edge_created_only_for_matching_pair() {
        let nodes = vec![
            node(NETWORK, "net-1", json!({"id": "net-1"})),
            node(NETWORK, "net-2", json!({"id": "net-2"})),
            node(
                PORT,
                "port-1",
                json!({"id": "port-1", "network_id": "net-1"}),
            ),
        ];
        let edges = evaluate_edges(&nodes, &[port_blocks_network_rule()]);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].child, 2); // port-1
        assert_eq!(edges[0].parent, 0); // net-1
        assert_eq!(edges[0].effect, RelationEffect::Blocks);
    }

    #[test]
    fn self_pair_never_produces_an_edge() {
        // A rule whose parent_kind == child_kind (e.g. a same-service
        // "group" relation) must never match a node against itself, even
        // if `matches` would otherwise return true for identical raw data.
        let nodes = vec![node(NETWORK, "net-1", json!({"id": "net-1"}))];
        let self_matching_rule = RelationRule {
            parent_kind: NETWORK,
            child_kind: NETWORK,
            matches: |_child, _parent| true,
            effect: RelationEffect::Blocks,
        };
        let edges = evaluate_edges(&nodes, &[self_matching_rule]);
        assert!(edges.is_empty());
    }

    #[test]
    fn no_edges_when_nothing_matches() {
        let nodes = vec![
            node(NETWORK, "net-1", json!({"id": "net-1"})),
            node(
                PORT,
                "port-1",
                json!({"id": "port-1", "network_id": "net-2"}),
            ),
        ];
        let edges = evaluate_edges(&nodes, &[port_blocks_network_rule()]);
        assert!(edges.is_empty());
    }
}
