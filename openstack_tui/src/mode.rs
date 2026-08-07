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

use serde::{Deserialize, Serialize};

/// Stable string identifier for a resource view, reused from
/// `ResourceBehaviour::view_key()` (e.g. `"network.security_group_rule"`).
pub type ViewKey = &'static str;

// BEGIN GENERATED rust-tui-view mode-const network.security_group
pub const NETWORK_SECURITY_GROUP: ViewKey = "network.security_group";
// END GENERATED rust-tui-view mode-const network.security_group
// BEGIN GENERATED rust-tui-view mode-const network.security_group_rule
pub const NETWORK_SECURITY_GROUP_RULE: ViewKey = "network.security_group_rule";
// END GENERATED rust-tui-view mode-const network.security_group_rule
// GENERATED-ANCHOR: view-key consts
// BEGIN GENERATED rust-tui-view mode-const network.network
pub const NETWORK_NETWORK: ViewKey = "network.network";
// END GENERATED rust-tui-view mode-const network.network
// BEGIN GENERATED rust-tui-view mode-const network.router
pub const NETWORK_ROUTER: ViewKey = "network.router";
// END GENERATED rust-tui-view mode-const network.router
// BEGIN GENERATED rust-tui-view mode-const network.subnet
pub const NETWORK_SUBNET: ViewKey = "network.subnet";
// END GENERATED rust-tui-view mode-const network.subnet
// BEGIN GENERATED rust-tui-view mode-const block_storage.backup
pub const BLOCK_STORAGE_BACKUP: ViewKey = "block_storage.backup";
// END GENERATED rust-tui-view mode-const block_storage.backup
// BEGIN GENERATED rust-tui-view mode-const block_storage.snapshot
pub const BLOCK_STORAGE_SNAPSHOT: ViewKey = "block_storage.snapshot";
// END GENERATED rust-tui-view mode-const block_storage.snapshot
// BEGIN GENERATED rust-tui-view mode-const block_storage.volume
pub const BLOCK_STORAGE_VOLUME: ViewKey = "block_storage.volume";
// END GENERATED rust-tui-view mode-const block_storage.volume
// BEGIN GENERATED rust-tui-view mode-const dns.zone
pub const DNS_ZONE: ViewKey = "dns.zone";
// END GENERATED rust-tui-view mode-const dns.zone
// BEGIN GENERATED rust-tui-view mode-const dns.recordset
pub const DNS_RECORDSET: ViewKey = "dns.recordset";
// END GENERATED rust-tui-view mode-const dns.recordset
// BEGIN GENERATED rust-tui-view mode-const image.image
pub const IMAGE_IMAGE: ViewKey = "image.image";
// END GENERATED rust-tui-view mode-const image.image
// BEGIN GENERATED rust-tui-view mode-const compute.aggregate
pub const COMPUTE_AGGREGATE: ViewKey = "compute.aggregate";
// END GENERATED rust-tui-view mode-const compute.aggregate
// BEGIN GENERATED rust-tui-view mode-const compute.flavor
pub const COMPUTE_FLAVOR: ViewKey = "compute.flavor";
// END GENERATED rust-tui-view mode-const compute.flavor
// BEGIN GENERATED rust-tui-view mode-const compute.hypervisor
pub const COMPUTE_HYPERVISOR: ViewKey = "compute.hypervisor";
// END GENERATED rust-tui-view mode-const compute.hypervisor
// BEGIN GENERATED rust-tui-view mode-const compute.server
pub const COMPUTE_SERVER: ViewKey = "compute.server";
// END GENERATED rust-tui-view mode-const compute.server
// BEGIN GENERATED rust-tui-view mode-const compute.server/instance_action
pub const COMPUTE_SERVER_INSTANCE_ACTION: ViewKey = "compute.server/instance_action";
// END GENERATED rust-tui-view mode-const compute.server/instance_action
// BEGIN GENERATED rust-tui-view mode-const compute.server/instance_action/event
pub const COMPUTE_SERVER_INSTANCE_ACTION_EVENT: ViewKey = "compute.server/instance_action/event";
// END GENERATED rust-tui-view mode-const compute.server/instance_action/event
// BEGIN GENERATED rust-tui-view mode-const identity.user/application_credential
pub const IDENTITY_APPLICATION_CREDENTIAL: ViewKey = "identity.user/application_credential";
// END GENERATED rust-tui-view mode-const identity.user/application_credential
// BEGIN GENERATED rust-tui-view mode-const identity.group
pub const IDENTITY_GROUP: ViewKey = "identity.group";
// END GENERATED rust-tui-view mode-const identity.group
// BEGIN GENERATED rust-tui-view mode-const identity.group_user
pub const IDENTITY_GROUP_USER: ViewKey = "identity.group_user";
// END GENERATED rust-tui-view mode-const identity.group_user
// BEGIN GENERATED rust-tui-view mode-const identity.project
pub const IDENTITY_PROJECT: ViewKey = "identity.project";
// END GENERATED rust-tui-view mode-const identity.project
// BEGIN GENERATED rust-tui-view mode-const identity.user
pub const IDENTITY_USER: ViewKey = "identity.user";
// END GENERATED rust-tui-view mode-const identity.user
// BEGIN GENERATED rust-tui-view mode-const load-balancer.loadbalancer
pub const LB_LOADBALANCER: ViewKey = "load-balancer.loadbalancer";
// END GENERATED rust-tui-view mode-const load-balancer.loadbalancer
// BEGIN GENERATED rust-tui-view mode-const load-balancer.listener
pub const LB_LISTENER: ViewKey = "load-balancer.listener";
// END GENERATED rust-tui-view mode-const load-balancer.listener
// BEGIN GENERATED rust-tui-view mode-const load-balancer.pool
pub const LB_POOL: ViewKey = "load-balancer.pool";
// END GENERATED rust-tui-view mode-const load-balancer.pool
// BEGIN GENERATED rust-tui-view mode-const load-balancer.pool/member
pub const LB_POOL_MEMBER: ViewKey = "load-balancer.pool/member";
// END GENERATED rust-tui-view mode-const load-balancer.pool/member
// BEGIN GENERATED rust-tui-view mode-const load-balancer.healthmonitor
pub const LB_HEALTHMONITOR: ViewKey = "load-balancer.healthmonitor";
// END GENERATED rust-tui-view mode-const load-balancer.healthmonitor

/// Every `ViewKey` migrated to `Mode::Resource`, paired with its const name. Single source
/// of truth used both by the `resolve_view_key` round-trip test and by coverage tests that
/// check `app.rs` and `.config/config.yaml` stay in sync with this list — add a new
/// resource's `(name, const)` pair here too.
#[cfg(test)]
pub(crate) const ALL_VIEW_KEYS: &[(&str, ViewKey)] = &[
    // BEGIN GENERATED rust-tui-view mode-all_view_keys network.security_group
    ("NETWORK_SECURITY_GROUP", NETWORK_SECURITY_GROUP),
    // END GENERATED rust-tui-view mode-all_view_keys network.security_group
    // BEGIN GENERATED rust-tui-view mode-all_view_keys network.security_group_rule
    ("NETWORK_SECURITY_GROUP_RULE", NETWORK_SECURITY_GROUP_RULE),
    // END GENERATED rust-tui-view mode-all_view_keys network.security_group_rule
    // GENERATED-ANCHOR: ALL_VIEW_KEYS entries
    // BEGIN GENERATED rust-tui-view mode-all_view_keys network.network
    ("NETWORK_NETWORK", NETWORK_NETWORK),
    // END GENERATED rust-tui-view mode-all_view_keys network.network
    // BEGIN GENERATED rust-tui-view mode-all_view_keys network.router
    ("NETWORK_ROUTER", NETWORK_ROUTER),
    // END GENERATED rust-tui-view mode-all_view_keys network.router
    // BEGIN GENERATED rust-tui-view mode-all_view_keys network.subnet
    ("NETWORK_SUBNET", NETWORK_SUBNET),
    // END GENERATED rust-tui-view mode-all_view_keys network.subnet
    // BEGIN GENERATED rust-tui-view mode-all_view_keys block_storage.backup
    ("BLOCK_STORAGE_BACKUP", BLOCK_STORAGE_BACKUP),
    // END GENERATED rust-tui-view mode-all_view_keys block_storage.backup
    // BEGIN GENERATED rust-tui-view mode-all_view_keys block_storage.snapshot
    ("BLOCK_STORAGE_SNAPSHOT", BLOCK_STORAGE_SNAPSHOT),
    // END GENERATED rust-tui-view mode-all_view_keys block_storage.snapshot
    // BEGIN GENERATED rust-tui-view mode-all_view_keys block_storage.volume
    ("BLOCK_STORAGE_VOLUME", BLOCK_STORAGE_VOLUME),
    // END GENERATED rust-tui-view mode-all_view_keys block_storage.volume
    // BEGIN GENERATED rust-tui-view mode-all_view_keys dns.zone
    ("DNS_ZONE", DNS_ZONE),
    // END GENERATED rust-tui-view mode-all_view_keys dns.zone
    // BEGIN GENERATED rust-tui-view mode-all_view_keys dns.recordset
    ("DNS_RECORDSET", DNS_RECORDSET),
    // END GENERATED rust-tui-view mode-all_view_keys dns.recordset
    // BEGIN GENERATED rust-tui-view mode-all_view_keys image.image
    ("IMAGE_IMAGE", IMAGE_IMAGE),
    // END GENERATED rust-tui-view mode-all_view_keys image.image
    // BEGIN GENERATED rust-tui-view mode-all_view_keys compute.aggregate
    ("COMPUTE_AGGREGATE", COMPUTE_AGGREGATE),
    // END GENERATED rust-tui-view mode-all_view_keys compute.aggregate
    // BEGIN GENERATED rust-tui-view mode-all_view_keys compute.flavor
    ("COMPUTE_FLAVOR", COMPUTE_FLAVOR),
    // END GENERATED rust-tui-view mode-all_view_keys compute.flavor
    // BEGIN GENERATED rust-tui-view mode-all_view_keys compute.hypervisor
    ("COMPUTE_HYPERVISOR", COMPUTE_HYPERVISOR),
    // END GENERATED rust-tui-view mode-all_view_keys compute.hypervisor
    // BEGIN GENERATED rust-tui-view mode-all_view_keys compute.server
    ("COMPUTE_SERVER", COMPUTE_SERVER),
    // END GENERATED rust-tui-view mode-all_view_keys compute.server
    // BEGIN GENERATED rust-tui-view mode-all_view_keys compute.server/instance_action
    (
        "COMPUTE_SERVER_INSTANCE_ACTION",
        COMPUTE_SERVER_INSTANCE_ACTION,
    ),
    // END GENERATED rust-tui-view mode-all_view_keys compute.server/instance_action
    // BEGIN GENERATED rust-tui-view mode-all_view_keys compute.server/instance_action/event
    (
        "COMPUTE_SERVER_INSTANCE_ACTION_EVENT",
        COMPUTE_SERVER_INSTANCE_ACTION_EVENT,
    ),
    // END GENERATED rust-tui-view mode-all_view_keys compute.server/instance_action/event
    // BEGIN GENERATED rust-tui-view mode-all_view_keys identity.user/application_credential
    (
        "IDENTITY_APPLICATION_CREDENTIAL",
        IDENTITY_APPLICATION_CREDENTIAL,
    ),
    // END GENERATED rust-tui-view mode-all_view_keys identity.user/application_credential
    // BEGIN GENERATED rust-tui-view mode-all_view_keys identity.group
    ("IDENTITY_GROUP", IDENTITY_GROUP),
    // END GENERATED rust-tui-view mode-all_view_keys identity.group
    // BEGIN GENERATED rust-tui-view mode-all_view_keys identity.group_user
    ("IDENTITY_GROUP_USER", IDENTITY_GROUP_USER),
    // END GENERATED rust-tui-view mode-all_view_keys identity.group_user
    // BEGIN GENERATED rust-tui-view mode-all_view_keys identity.project
    ("IDENTITY_PROJECT", IDENTITY_PROJECT),
    // END GENERATED rust-tui-view mode-all_view_keys identity.project
    // BEGIN GENERATED rust-tui-view mode-all_view_keys identity.user
    ("IDENTITY_USER", IDENTITY_USER),
    // END GENERATED rust-tui-view mode-all_view_keys identity.user
    // BEGIN GENERATED rust-tui-view mode-all_view_keys load-balancer.loadbalancer
    ("LB_LOADBALANCER", LB_LOADBALANCER),
    // END GENERATED rust-tui-view mode-all_view_keys load-balancer.loadbalancer
    // BEGIN GENERATED rust-tui-view mode-all_view_keys load-balancer.listener
    ("LB_LISTENER", LB_LISTENER),
    // END GENERATED rust-tui-view mode-all_view_keys load-balancer.listener
    // BEGIN GENERATED rust-tui-view mode-all_view_keys load-balancer.pool
    ("LB_POOL", LB_POOL),
    // END GENERATED rust-tui-view mode-all_view_keys load-balancer.pool
    // BEGIN GENERATED rust-tui-view mode-all_view_keys load-balancer.pool/member
    ("LB_POOL_MEMBER", LB_POOL_MEMBER),
    // END GENERATED rust-tui-view mode-all_view_keys load-balancer.pool/member
    // BEGIN GENERATED rust-tui-view mode-all_view_keys load-balancer.healthmonitor
    ("LB_HEALTHMONITOR", LB_HEALTHMONITOR),
    // END GENERATED rust-tui-view mode-all_view_keys load-balancer.healthmonitor
];

/// Resolve a config-supplied view-key string to the canonical `'static` `ViewKey`.
/// Add an arm here when migrating another resource to `Mode::Resource`.
pub(crate) fn resolve_view_key(s: &str) -> Option<ViewKey> {
    match s {
        // BEGIN GENERATED rust-tui-view mode-resolve network.security_group
        "network.security_group" => Some(NETWORK_SECURITY_GROUP),
        // END GENERATED rust-tui-view mode-resolve network.security_group
        // BEGIN GENERATED rust-tui-view mode-resolve network.security_group_rule
        "network.security_group_rule" => Some(NETWORK_SECURITY_GROUP_RULE),
        // END GENERATED rust-tui-view mode-resolve network.security_group_rule
        // GENERATED-ANCHOR: resolve_view_key arms
        // BEGIN GENERATED rust-tui-view mode-resolve network.network
        "network.network" => Some(NETWORK_NETWORK),
        // END GENERATED rust-tui-view mode-resolve network.network
        // BEGIN GENERATED rust-tui-view mode-resolve network.router
        "network.router" => Some(NETWORK_ROUTER),
        // END GENERATED rust-tui-view mode-resolve network.router
        // BEGIN GENERATED rust-tui-view mode-resolve network.subnet
        "network.subnet" => Some(NETWORK_SUBNET),
        // END GENERATED rust-tui-view mode-resolve network.subnet
        // BEGIN GENERATED rust-tui-view mode-resolve block_storage.backup
        "block_storage.backup" => Some(BLOCK_STORAGE_BACKUP),
        // END GENERATED rust-tui-view mode-resolve block_storage.backup
        // BEGIN GENERATED rust-tui-view mode-resolve block_storage.snapshot
        "block_storage.snapshot" => Some(BLOCK_STORAGE_SNAPSHOT),
        // END GENERATED rust-tui-view mode-resolve block_storage.snapshot
        // BEGIN GENERATED rust-tui-view mode-resolve block_storage.volume
        "block_storage.volume" => Some(BLOCK_STORAGE_VOLUME),
        // END GENERATED rust-tui-view mode-resolve block_storage.volume
        // BEGIN GENERATED rust-tui-view mode-resolve dns.zone
        "dns.zone" => Some(DNS_ZONE),
        // END GENERATED rust-tui-view mode-resolve dns.zone
        // BEGIN GENERATED rust-tui-view mode-resolve dns.recordset
        "dns.recordset" => Some(DNS_RECORDSET),
        // END GENERATED rust-tui-view mode-resolve dns.recordset
        // BEGIN GENERATED rust-tui-view mode-resolve image.image
        "image.image" => Some(IMAGE_IMAGE),
        // END GENERATED rust-tui-view mode-resolve image.image
        // BEGIN GENERATED rust-tui-view mode-resolve compute.aggregate
        "compute.aggregate" => Some(COMPUTE_AGGREGATE),
        // END GENERATED rust-tui-view mode-resolve compute.aggregate
        // BEGIN GENERATED rust-tui-view mode-resolve compute.flavor
        "compute.flavor" => Some(COMPUTE_FLAVOR),
        // END GENERATED rust-tui-view mode-resolve compute.flavor
        // BEGIN GENERATED rust-tui-view mode-resolve compute.hypervisor
        "compute.hypervisor" => Some(COMPUTE_HYPERVISOR),
        // END GENERATED rust-tui-view mode-resolve compute.hypervisor
        // BEGIN GENERATED rust-tui-view mode-resolve compute.server
        "compute.server" => Some(COMPUTE_SERVER),
        // END GENERATED rust-tui-view mode-resolve compute.server
        // BEGIN GENERATED rust-tui-view mode-resolve compute.server/instance_action
        "compute.server/instance_action" => Some(COMPUTE_SERVER_INSTANCE_ACTION),
        // END GENERATED rust-tui-view mode-resolve compute.server/instance_action
        // BEGIN GENERATED rust-tui-view mode-resolve compute.server/instance_action/event
        "compute.server/instance_action/event" => Some(COMPUTE_SERVER_INSTANCE_ACTION_EVENT),
        // END GENERATED rust-tui-view mode-resolve compute.server/instance_action/event
        // BEGIN GENERATED rust-tui-view mode-resolve identity.user/application_credential
        "identity.user/application_credential" => Some(IDENTITY_APPLICATION_CREDENTIAL),
        // END GENERATED rust-tui-view mode-resolve identity.user/application_credential
        // BEGIN GENERATED rust-tui-view mode-resolve identity.group
        "identity.group" => Some(IDENTITY_GROUP),
        // END GENERATED rust-tui-view mode-resolve identity.group
        // BEGIN GENERATED rust-tui-view mode-resolve identity.group_user
        "identity.group_user" => Some(IDENTITY_GROUP_USER),
        // END GENERATED rust-tui-view mode-resolve identity.group_user
        // BEGIN GENERATED rust-tui-view mode-resolve identity.project
        "identity.project" => Some(IDENTITY_PROJECT),
        // END GENERATED rust-tui-view mode-resolve identity.project
        // BEGIN GENERATED rust-tui-view mode-resolve identity.user
        "identity.user" => Some(IDENTITY_USER),
        // END GENERATED rust-tui-view mode-resolve identity.user
        // BEGIN GENERATED rust-tui-view mode-resolve load-balancer.loadbalancer
        "load-balancer.loadbalancer" => Some(LB_LOADBALANCER),
        // END GENERATED rust-tui-view mode-resolve load-balancer.loadbalancer
        // BEGIN GENERATED rust-tui-view mode-resolve load-balancer.listener
        "load-balancer.listener" => Some(LB_LISTENER),
        // END GENERATED rust-tui-view mode-resolve load-balancer.listener
        // BEGIN GENERATED rust-tui-view mode-resolve load-balancer.pool
        "load-balancer.pool" => Some(LB_POOL),
        // END GENERATED rust-tui-view mode-resolve load-balancer.pool
        // BEGIN GENERATED rust-tui-view mode-resolve load-balancer.pool/member
        "load-balancer.pool/member" => Some(LB_POOL_MEMBER),
        // END GENERATED rust-tui-view mode-resolve load-balancer.pool/member
        // BEGIN GENERATED rust-tui-view mode-resolve load-balancer.healthmonitor
        "load-balancer.healthmonitor" => Some(LB_HEALTHMONITOR),
        // END GENERATED rust-tui-view mode-resolve load-balancer.healthmonitor
        _ => None,
    }
}

/// TUI Modes (screens)
// NOTE: `Serialize` (derived) and `Deserialize` (hand-written below) are asymmetric for
// the `Resource` variant — derive emits `{"Resource": "<key>"}`, but `Deserialize` only
// accepts `"Resource(<key>)"`. Fine today since nothing serializes `Mode`; fix if that changes.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, strum::VariantNames)]
pub enum Mode {
    #[default]
    Home,
    Describe,

    /// A resource view identified by its `ViewKey` (e.g. `"network.security_group"`).
    /// Migrated resources use this instead of a dedicated unit variant.
    Resource(ViewKey),
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ModeVisitor;

        impl serde::de::Visitor<'_> for ModeVisitor {
            type Value = Mode;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    "a Mode name (e.g. \"Home\") or \"Resource(<view_key>)\" (e.g. \"Resource(network.security_group)\")",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Mode, E>
            where
                E: serde::de::Error,
            {
                if let Some(inner) = v
                    .strip_prefix("Resource(")
                    .and_then(|s| s.strip_suffix(')'))
                {
                    return resolve_view_key(inner)
                        .map(Mode::Resource)
                        .ok_or_else(|| E::custom(format!("unknown resource view key: {inner}")));
                }
                match v {
                    "Home" => Ok(Mode::Home),
                    "Describe" => Ok(Mode::Describe),
                    other => Err(E::custom(format!("unknown Mode: {other}"))),
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Mode, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_str(ModeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_unit_variant_from_bare_string() {
        let mode: Mode = serde_yaml::from_str("Home").unwrap();
        assert_eq!(mode, Mode::Home);
    }

    #[test]
    fn deserializes_resource_variant_from_parenthesised_string() {
        let mode: Mode = serde_yaml::from_str("Resource(network.security_group)").unwrap();
        assert_eq!(mode, Mode::Resource(NETWORK_SECURITY_GROUP));

        let mode: Mode = serde_yaml::from_str("Resource(network.security_group_rule)").unwrap();
        assert_eq!(mode, Mode::Resource(NETWORK_SECURITY_GROUP_RULE));
    }

    #[test]
    fn rejects_unknown_resource_view_key() {
        let result: Result<Mode, _> = serde_yaml::from_str("Resource(bogus.key)");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_unknown_mode_name() {
        let result: Result<Mode, _> = serde_yaml::from_str("TotallyNotAMode");
        assert!(result.is_err());
    }

    #[test]
    fn every_non_resource_variant_name_is_deserializable() {
        use strum::VariantNames;
        for name in Mode::VARIANTS {
            if *name == "Resource" {
                continue;
            }
            let result: Result<Mode, _> = serde_yaml::from_str(name);
            assert!(
                result.is_ok(),
                "variant name {name:?} failed to deserialize"
            );
        }
    }

    #[test]
    fn resource_mode_is_hashable_and_copy() {
        use std::collections::HashMap;
        let mut map: HashMap<Mode, &str> = HashMap::new();
        map.insert(Mode::Resource(NETWORK_SECURITY_GROUP_RULE), "rules view");
        let key = Mode::Resource(NETWORK_SECURITY_GROUP_RULE); // Copy, not moved from map insertion
        assert_eq!(map.get(&key), Some(&"rules view"));
    }

    #[test]
    fn every_view_key_roundtrips_through_resolve() {
        for &(_, key) in ALL_VIEW_KEYS {
            assert_eq!(
                resolve_view_key(key),
                Some(key),
                "{key:?} missing from resolve_view_key"
            );
        }
    }

    /// View keys reachable only by drilling down from a parent row (e.g. selecting an
    /// instance action to see its events), so they have no `mode_keybindings` block of
    /// their own — the parent's `ShowResource` action is the only way in.
    const SKIP_KEYBINDING_CHECK: &[ViewKey] = &[COMPUTE_SERVER_INSTANCE_ACTION_EVENT];

    /// Guards against the "silent gap" failure mode: a resource migrated to
    /// `Mode::Resource` (added to `ALL_VIEW_KEYS`) but forgotten in `.config/config.yaml`,
    /// or a keybinding block left over for a view key that no longer resolves.
    #[test]
    fn config_yaml_mode_keybindings_match_all_view_keys() {
        let config_yaml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/.config/config.yaml"));
        let doc: serde_yaml::Value = serde_yaml::from_str(config_yaml).unwrap();
        let keybindings = doc
            .get("mode_keybindings")
            .and_then(|v| v.as_mapping())
            .expect("mode_keybindings map present in config.yaml");

        let mut configured_keys = Vec::new();
        for key in keybindings.keys() {
            let key = key.as_str().expect("mode_keybindings key is a string");
            if let Some(inner) = key
                .strip_prefix("Resource(")
                .and_then(|s| s.strip_suffix(')'))
            {
                assert!(
                    resolve_view_key(inner).is_some(),
                    "config.yaml has a keybinding block for unknown view key {inner:?}"
                );
                configured_keys.push(inner);
            }
        }

        for &(_, view_key) in ALL_VIEW_KEYS {
            if SKIP_KEYBINDING_CHECK.contains(&view_key) {
                continue;
            }
            assert!(
                configured_keys.contains(&view_key),
                "view key {view_key:?} is in ALL_VIEW_KEYS but has no \
                 \"Resource({view_key})\" keybinding block in config.yaml"
            );
        }
    }

    /// Guards against forgetting to register a resource's component in `App::new()`
    /// (`openstack_tui/src/app.rs`) after adding its view key to `ALL_VIEW_KEYS`.
    #[test]
    fn app_rs_registers_every_view_key() {
        let app_rs = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/app.rs"));
        for &(const_name, view_key) in ALL_VIEW_KEYS {
            let needle = format!("crate::mode::{const_name}");
            assert!(
                app_rs.contains(&needle),
                "app.rs has no `{needle}` component registration for view key {view_key:?}"
            );
        }
    }
}
