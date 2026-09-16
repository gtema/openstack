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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
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

/// Interns `s`, returning a `'static` reference shared by every prior and
/// future call with an equal string value. This bounds total leaked memory
/// by the number of *distinct* strings ever deserialized, rather than by the
/// number of deserialize calls.
fn intern(s: String) -> &'static str {
    static INTERN: std::sync::OnceLock<std::sync::Mutex<std::collections::HashSet<&'static str>>> =
        std::sync::OnceLock::new();
    let table = INTERN.get_or_init(|| std::sync::Mutex::new(std::collections::HashSet::new()));
    let mut table = table
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = table.get(s.as_str()) {
        return existing;
    }
    let leaked: &'static str = Box::leak(s.into_boxed_str());
    table.insert(leaked);
    leaked
}

impl<'de> Deserialize<'de> for ResourceKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            ServiceType,
            ResourceType,
        }

        struct ResourceKindVisitor;

        impl<'de> Visitor<'de> for ResourceKindVisitor {
            type Value = ResourceKind;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ResourceKind")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ResourceKind, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut service_type: Option<String> = None;
                let mut resource_type: Option<String> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ServiceType => {
                            if service_type.is_some() {
                                return Err(de::Error::duplicate_field("service_type"));
                            }
                            service_type = Some(map.next_value()?);
                        }
                        Field::ResourceType => {
                            if resource_type.is_some() {
                                return Err(de::Error::duplicate_field("resource_type"));
                            }
                            resource_type = Some(map.next_value()?);
                        }
                    }
                }

                let service_type =
                    service_type.ok_or_else(|| de::Error::missing_field("service_type"))?;
                let resource_type =
                    resource_type.ok_or_else(|| de::Error::missing_field("resource_type"))?;

                Ok(ResourceKind {
                    service_type: intern(service_type),
                    resource_type: intern(resource_type),
                })
            }
        }

        const FIELDS: &[&str] = &["service_type", "resource_type"];
        deserializer.deserialize_struct("ResourceKind", FIELDS, ResourceKindVisitor)
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

    #[test]
    fn resource_kind_deserialize_interns_strings() {
        let json_a = r#"{"service_type":"network","resource_type":"network"}"#;
        let json_b = r#"{"service_type":"network","resource_type":"network"}"#;

        let a: ResourceKind = serde_json::from_str(json_a).unwrap();
        let b: ResourceKind = serde_json::from_str(json_b).unwrap();

        assert_eq!(a, b);
        assert!(
            std::ptr::eq(a.service_type, b.service_type),
            "service_type should be interned to the same allocation"
        );
        assert!(
            std::ptr::eq(a.resource_type, b.resource_type),
            "resource_type should be interned to the same allocation"
        );
    }
}
