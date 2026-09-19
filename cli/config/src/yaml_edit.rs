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

//! Comment- and anchor-preserving edits to a `clouds.yaml`/`secure.yaml`
//! style document.
//!
//! `clouds.yaml` is frequently hand-maintained (comments, `&anchor`/`<<:
//! *anchor` merge keys) and consumed by other tools (Terraform, Ansible,
//! python-openstackclient). Round-tripping it through a generic YAML
//! (de)serializer, as an early revision of `osc config clouds add` did,
//! silently expands anchors and drops comments. This module instead
//! splices individual mapping entries into the existing document text via
//! [`yamlpatch`], leaving everything else byte-for-byte untouched.
//!
//! This module only offers the mechanical parse/locate/splice/render
//! primitives; policy (e.g. "error on a name collision unless
//! `--overwrite` was passed") is left to callers, so the same primitives
//! serve `clouds add` (insert-or-error) and future `clouds edit`/`clouds
//! remove`/`contexts *` commands (where an existing key is the expected
//! case, not an error).

use indexmap::IndexMap;
use serde::Serialize;
use yamlpatch::{Op, Patch, apply_yaml_patches};
use yamlpath::{Component, Document, Route, route};

/// Errors from parsing or splice-editing a YAML document.
#[derive(Debug, thiserror::Error)]
pub enum YamlEditError {
    /// The document text is not valid YAML.
    #[error("the file is not valid YAML: {0}")]
    Parse(#[from] yamlpath::QueryError),
    /// The value being inserted could not be serialized to YAML.
    #[error("could not serialize the value to YAML: {0}")]
    Serialize(#[from] yaml_serde::Error),
    /// The requested edit could not be applied (e.g. a name collision, or
    /// an unsupported document shape such as a multi-line flow mapping).
    #[error("the edit could not be applied: {0}")]
    Edit(#[from] yamlpatch::Error),
    /// A merge was asked for with a value that is not a string-keyed
    /// mapping, so it has no fields to merge.
    #[error("only a mapping can be merged into an existing entry")]
    NotAMapping,
}

/// A parsed YAML document that supports byte-preserving splice edits.
///
/// Untouched regions of the document (comments, anchors, formatting,
/// unrelated entries) are always preserved exactly; only the specific
/// mapping entry an edit targets is rewritten.
pub struct YamlDocument(Document);

impl YamlDocument {
    /// Parse an existing document.
    pub fn parse(text: &str) -> Result<Self, YamlEditError> {
        Ok(Self(Document::new(text)?))
    }

    /// An empty mapping document, for the "target file does not exist
    /// yet" case.
    ///
    /// Note: an entry inserted directly into this starting point renders
    /// in flow style (`{...}`); there is nothing to preserve in a
    /// brand-new file, so callers that want block-style output for a
    /// fresh file may prefer to build it directly instead of starting
    /// from this empty document.
    pub fn empty_mapping() -> Self {
        // "{}\n" is a fixed, always-valid YAML literal (an empty flow
        // mapping), so `Document::new` cannot fail on it.
        #[allow(clippy::unwrap_used)]
        let doc = Document::new("{}\n").unwrap();
        Self(doc)
    }

    /// Whether `parent.key` exists in the document, e.g.
    /// `contains_key("clouds", "mycloud")`.
    pub fn contains_key(&self, parent: &str, key: &str) -> bool {
        self.0.query_exists(&route![parent, key])
    }

    /// Insert or replace the mapping entry at `parent.key` with `value`.
    ///
    /// The `parent` mapping is created if it does not already exist (e.g.
    /// a missing top-level `clouds:`), as long as the document already has
    /// *some* top-level mapping content to attach it to (as
    /// [`empty_mapping`](Self::empty_mapping) provides) — a document that
    /// is empty or contains only comments has no root node to create the
    /// key under. An existing `parent.key` is replaced; callers that need
    /// to reject a collision should check
    /// [`contains_key`](Self::contains_key) first.
    pub fn upsert_mapping_entry<T: Serialize>(
        &mut self,
        parent: &str,
        key: &str,
        value: &T,
    ) -> Result<(), YamlEditError> {
        let value = yaml_serde::to_value(value)?;

        // A `parent:` key that is present but empty (`clouds:` with
        // nothing under it) has no mapping for `MergeInto` to merge into
        // and makes it fail outright, so build the mapping wholesale
        // instead. `Replace` at the parent route preserves the comments
        // around it, and there are no existing entries to lose.
        let patch = if self.parent_is_empty(parent) {
            let mut mapping = yaml_serde::Mapping::new();
            mapping.insert(key.into(), value);
            Patch {
                route: route![parent],
                operation: Op::Replace(yaml_serde::Value::Mapping(mapping)),
            }
        } else {
            let mut updates = IndexMap::new();
            updates.insert(key.to_string(), value);
            Patch {
                route: Route::default(),
                operation: Op::MergeInto {
                    key: parent.to_string(),
                    updates,
                },
            }
        };
        self.0 = apply_yaml_patches(&self.0, std::slice::from_ref(&patch))?;
        Ok(())
    }

    /// Merge `value`'s fields into the existing entry at `parent.key`,
    /// leaving the entry's position and every surrounding comment intact.
    ///
    /// This differs from [`upsert_mapping_entry`](Self::upsert_mapping_entry)
    /// in both directions, and the choice between them is a real trade-off:
    ///
    /// - `upsert_mapping_entry` replaces the entry *wholesale*, so no key of
    ///   the old entry survives — but it rewrites a region of the document
    ///   wide enough that the **following** entry's leading comment is lost.
    /// - This method rewrites only individual scalar leaves, so all comments
    ///   survive — but keys present in the old entry and absent from `value`
    ///   are **left in place** at the top level of the entry.
    ///
    /// Nested mappings in `value` (e.g. an `auth` block) are still replaced
    /// wholesale: a key inside one that `value` does not set is removed, so
    /// a credential never half-merges into the previous one.
    ///
    /// Prefer this when the document is hand-maintained and the caller's
    /// semantics are "update this entry", and `upsert_mapping_entry` when
    /// the entry must end up exactly equal to `value` and no comment can
    /// follow it.
    ///
    /// `value` must serialize to a string-keyed mapping; anything else is
    /// rejected as [`YamlEditError::NotAMapping`]. The entry must already
    /// exist — check with [`contains_key`](Self::contains_key) first.
    ///
    /// Note: a field whose existing value is a mapping and whose new value
    /// is a scalar (or vice versa) is replaced in place, which reintroduces
    /// the comment loss described above. `clouds.yaml` entries have fixed
    /// field shapes, so this does not arise in practice.
    pub fn merge_mapping_entry<T: Serialize>(
        &mut self,
        parent: &str,
        key: &str,
        value: &T,
    ) -> Result<(), YamlEditError> {
        let yaml_serde::Value::Mapping(fields) = yaml_serde::to_value(value)? else {
            return Err(YamlEditError::NotAMapping);
        };
        self.merge_fields(&[parent.to_string(), key.to_string()], fields)
    }

    /// Merge `fields` into the mapping at `base`, one scalar leaf at a time.
    ///
    /// Rewriting a multi-line block node as a whole is what loses the
    /// following entry's comment, so this only ever hands `yamlpatch` a
    /// scalar replacement, a scalar insertion, or a leaf removal — each of
    /// which leaves neighbouring comments alone.
    fn merge_fields(
        &mut self,
        base: &[String],
        fields: yaml_serde::Mapping,
    ) -> Result<(), YamlEditError> {
        for (field, new_value) in fields {
            let yaml_serde::Value::String(field) = field else {
                return Err(YamlEditError::NotAMapping);
            };
            let mut path = base.to_vec();
            path.push(field.clone());

            match (new_value, self.value_at(&path)) {
                // A nested mapping replacing a nested mapping: drop the keys
                // the new value does not carry, then recurse so only leaves
                // are ever rewritten.
                (yaml_serde::Value::Mapping(new_fields), Some(yaml_serde::Value::Mapping(old))) => {
                    for stale in old.keys().filter_map(|k| match k {
                        yaml_serde::Value::String(k) if !new_fields.contains_key(k.as_str()) => {
                            Some(k.clone())
                        }
                        _ => None,
                    }) {
                        let mut stale_path = path.clone();
                        stale_path.push(stale);
                        self.patch(Patch {
                            route: route_of(&stale_path),
                            operation: Op::Remove,
                        })?;
                    }
                    self.merge_fields(&path, new_fields)?;
                }
                // An existing leaf: replace it in place.
                (new_value, Some(_)) => self.patch(Patch {
                    route: route_of(&path),
                    operation: Op::Replace(new_value),
                })?,
                // Absent: insert it into the mapping at `base`. Inserting a
                // key is comment-safe even when the value is a block.
                (new_value, None) => {
                    let (owner, owner_key) = base
                        .split_last()
                        .map(|(key, rest)| (rest, key.clone()))
                        .ok_or(YamlEditError::NotAMapping)?;
                    let mut updates = IndexMap::new();
                    updates.insert(field, new_value);
                    self.patch(Patch {
                        route: route_of(owner),
                        operation: Op::MergeInto {
                            key: owner_key,
                            updates,
                        },
                    })?;
                }
            }
        }
        Ok(())
    }

    /// Apply one patch to the document.
    fn patch(&mut self, patch: Patch<'_>) -> Result<(), YamlEditError> {
        self.0 = apply_yaml_patches(&self.0, std::slice::from_ref(&patch))?;
        Ok(())
    }

    /// The current value at `path`, or `None` when `path` does not resolve.
    fn value_at(&self, path: &[String]) -> Option<yaml_serde::Value> {
        let mut value: yaml_serde::Value = yaml_serde::from_str(self.0.source()).ok()?;
        for key in path {
            value = value.get(key.as_str())?.clone();
        }
        Some(value)
    }

    /// Whether `parent` is present in the document but holds no mapping
    /// (`clouds:` with nothing under it).
    fn parent_is_empty(&self, parent: &str) -> bool {
        yaml_serde::from_str::<yaml_serde::Value>(self.0.source())
            .ok()
            .and_then(|root| root.get(parent).cloned())
            .is_some_and(|value| value.is_null())
    }

    /// Remove the mapping entry at `parent.key`.
    pub fn remove_mapping_entry(&mut self, parent: &str, key: &str) -> Result<(), YamlEditError> {
        let patch = Patch {
            route: route![parent, key],
            operation: Op::Remove,
        };
        self.0 = apply_yaml_patches(&self.0, std::slice::from_ref(&patch))?;
        Ok(())
    }

    /// The document's current YAML text.
    pub fn source(&self) -> &str {
        self.0.source()
    }
}

/// Build a [`Route`] from an owned key path.
fn route_of(path: &[String]) -> Route<'_> {
    Route::from(
        path.iter()
            .map(|key| Component::Key(key.as_str().into()))
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Entry {
        auth_url: String,
        region_name: String,
    }

    fn entry(auth_url: &str, region_name: &str) -> Entry {
        Entry {
            auth_url: auth_url.to_string(),
            region_name: region_name.to_string(),
        }
    }

    #[test]
    fn parse_rejects_malformed_yaml() {
        assert!(YamlDocument::parse(": not yaml : [").is_err());
    }

    #[test]
    fn insert_into_missing_parent_creates_it() -> Result<(), YamlEditError> {
        let mut doc = YamlDocument::parse("# top comment\ncache:\n  auth: true\n")?;
        doc.upsert_mapping_entry(
            "clouds",
            "mycloud",
            &entry("https://keystone:5000", "RegionOne"),
        )?;

        assert!(doc.source().starts_with("# top comment\n"));
        let parsed: yaml_serde::Value =
            yaml_serde::from_str(doc.source()).map_err(YamlEditError::from)?;
        assert_eq!(
            parsed["clouds"]["mycloud"]["auth_url"].as_str(),
            Some("https://keystone:5000")
        );
        Ok(())
    }

    #[test]
    fn insert_preserves_comments_and_other_entries() -> Result<(), YamlEditError> {
        let existing = "# my clouds\nclouds:\n  # existing entry\n  other:\n    auth_url: https://other:5000\n    region_name: RegionOther\n";
        let mut doc = YamlDocument::parse(existing)?;
        doc.upsert_mapping_entry("clouds", "mycloud", &entry("https://new:5000", "RegionOne"))?;

        let out = doc.source();
        assert!(out.contains("# my clouds"));
        assert!(out.contains("# existing entry"));
        assert!(out.contains("other:"));
        assert!(out.contains("https://other:5000"));
        assert!(out.contains("mycloud:"));
        assert!(out.contains("https://new:5000"));
        Ok(())
    }

    #[test]
    fn insert_preserves_anchors_and_merge_keys() -> Result<(), YamlEditError> {
        // This is the exact regression the switch away from a
        // round-tripping serializer (serde_yaml) was made for: `&anchor`
        // and `<<: *anchor` must survive byte-for-byte.
        let existing = "clouds:\n  base: &base\n    region_name: RegionOne\n  devstack:\n    <<: *base\n    auth_url: https://devstack:5000\n";
        let mut doc = YamlDocument::parse(existing)?;
        doc.upsert_mapping_entry("clouds", "mycloud", &entry("https://new:5000", "RegionOne"))?;

        let out = doc.source();
        assert!(
            out.contains("&base"),
            "anchor definition must survive: {out}"
        );
        assert!(out.contains("<<: *base"), "merge key must survive: {out}");
        Ok(())
    }

    #[test]
    fn upsert_replaces_existing_entry() -> Result<(), YamlEditError> {
        let existing =
            "clouds:\n  mycloud:\n    auth_url: https://old:5000\n    region_name: RegionOld\n";
        let mut doc = YamlDocument::parse(existing)?;
        assert!(doc.contains_key("clouds", "mycloud"));

        doc.upsert_mapping_entry("clouds", "mycloud", &entry("https://new:5000", "RegionNew"))?;

        let out = doc.source();
        assert!(!out.contains("https://old:5000"));
        assert!(out.contains("https://new:5000"));
        Ok(())
    }

    #[test]
    fn remove_deletes_only_targeted_entry() -> Result<(), YamlEditError> {
        let existing = "# header\nclouds:\n  keep:\n    auth_url: https://keep:5000\n  drop:\n    auth_url: https://drop:5000\n";
        let mut doc = YamlDocument::parse(existing)?;
        doc.remove_mapping_entry("clouds", "drop")?;

        let out = doc.source();
        assert!(out.contains("# header"));
        assert!(out.contains("keep:"));
        assert!(out.contains("https://keep:5000"));
        assert!(!out.contains("drop:"));
        assert!(!out.contains("https://drop:5000"));
        Ok(())
    }

    #[test]
    fn contains_key_reflects_document_state() -> Result<(), YamlEditError> {
        let mut doc = YamlDocument::parse("clouds:\n  existing:\n    auth_url: https://x:5000\n")?;
        assert!(doc.contains_key("clouds", "existing"));
        assert!(!doc.contains_key("clouds", "missing"));

        doc.upsert_mapping_entry("clouds", "missing", &entry("https://x:5000", "RegionOne"))?;
        assert!(doc.contains_key("clouds", "missing"));
        Ok(())
    }

    #[test]
    fn empty_mapping_round_trip() -> Result<(), YamlEditError> {
        let mut doc = YamlDocument::empty_mapping();
        doc.upsert_mapping_entry("clouds", "mycloud", &entry("https://x:5000", "RegionOne"))?;

        let parsed: yaml_serde::Value =
            yaml_serde::from_str(doc.source()).map_err(YamlEditError::from)?;
        assert_eq!(
            parsed["clouds"]["mycloud"]["auth_url"].as_str(),
            Some("https://x:5000")
        );
        Ok(())
    }

    #[test]
    fn empty_mapping_renders_flow_style() -> Result<(), YamlEditError> {
        // Pins the behavior called out in `empty_mapping`'s doc comment:
        // a fresh file renders flow-style (`{...}`), not block-style.
        // Callers that want block-style output for a brand-new file must
        // build it directly instead of starting from `empty_mapping`.
        let mut doc = YamlDocument::empty_mapping();
        doc.upsert_mapping_entry("clouds", "mycloud", &entry("https://x:5000", "RegionOne"))?;

        assert!(
            doc.source().trim_start().starts_with('{'),
            "expected flow-style output: {}",
            doc.source()
        );
        Ok(())
    }
    #[test]
    fn insert_into_empty_parent_key() -> Result<(), YamlEditError> {
        // `clouds:` with nothing under it has no mapping to merge into;
        // the wholesale-build fallback must still keep the comments.
        let mut doc = YamlDocument::parse("# my clouds\nclouds:\n")?;
        doc.upsert_mapping_entry(
            "clouds",
            "mycloud",
            &entry("https://keystone:5000", "RegionOne"),
        )?;

        let out = doc.source();
        assert!(out.contains("# my clouds"), "header must survive: {out}");
        let parsed: yaml_serde::Value = yaml_serde::from_str(out).map_err(YamlEditError::from)?;
        assert_eq!(
            parsed["clouds"]["mycloud"]["auth_url"].as_str(),
            Some("https://keystone:5000")
        );
        Ok(())
    }

    #[test]
    fn insert_into_empty_parent_key_keeps_siblings() -> Result<(), YamlEditError> {
        // The fallback replaces the `clouds` node only; keys after it and
        // their comments must be untouched.
        let mut doc =
            YamlDocument::parse("clouds:\n# cache settings\ncache:\n  expiration_time: 600\n")?;
        doc.upsert_mapping_entry("clouds", "mycloud", &entry("https://x:5000", "RegionOne"))?;

        let out = doc.source();
        assert!(out.contains("# cache settings"), "{out}");
        assert!(out.contains("expiration_time: 600"), "{out}");
        Ok(())
    }

    #[test]
    fn merge_keeps_the_next_entry_comment() -> Result<(), YamlEditError> {
        // The reason `merge_mapping_entry` exists: replacing an entry with
        // `upsert_mapping_entry` rewrites a wide enough region to take the
        // *next* entry's leading comment with it (pinned as known
        // behavior in `upsert_loses_the_next_entry_comment`), which
        // is silent data loss in a hand-maintained clouds.yaml.
        let existing = "# header\nclouds:\n  # attached to mycloud\n  mycloud:\n    auth_url: https://old:5000\n    region_name: RegionOld\n  # attached to other\n  other:\n    auth_url: https://other:5000\n# trailing\n";
        let mut doc = YamlDocument::parse(existing)?;
        doc.merge_mapping_entry("clouds", "mycloud", &entry("https://new:5000", "RegionNew"))?;

        let out = doc.source();
        for comment in [
            "# header",
            "# attached to mycloud",
            "# attached to other",
            "# trailing",
        ] {
            assert!(out.contains(comment), "{comment} must survive: {out}");
        }
        assert!(out.contains("https://new:5000"), "{out}");
        assert!(!out.contains("https://old:5000"), "{out}");
        Ok(())
    }

    #[test]
    fn merge_keeps_the_entry_position() -> Result<(), YamlEditError> {
        let existing = "clouds:\n  mycloud:\n    auth_url: https://old:5000\n    region_name: RegionOld\n  zzz:\n    auth_url: https://zzz:5000\n";
        let mut doc = YamlDocument::parse(existing)?;
        doc.merge_mapping_entry("clouds", "mycloud", &entry("https://new:5000", "RegionNew"))?;

        let out = doc.source();
        assert!(
            out.find("mycloud:") < out.find("zzz:"),
            "entry must not move to the end: {out}"
        );
        Ok(())
    }

    #[test]
    fn merge_leaves_keys_absent_from_the_new_value() -> Result<(), YamlEditError> {
        // The documented trade-off of `merge_mapping_entry`: a key the old
        // entry had and the new value does not is kept. For `clouds add
        // --overwrite` that is the point - hand-set connection settings
        // survive a credential rotation.
        let existing = "clouds:\n  mycloud:\n    auth_url: https://old:5000\n    region_name: RegionOld\n    interface: internal\n";
        let mut doc = YamlDocument::parse(existing)?;
        doc.merge_mapping_entry("clouds", "mycloud", &entry("https://new:5000", "RegionNew"))?;

        let out = doc.source();
        assert!(out.contains("interface: internal"), "{out}");
        assert!(out.contains("https://new:5000"), "{out}");
        Ok(())
    }

    #[test]
    fn merge_replaces_nested_mappings_wholesale() -> Result<(), YamlEditError> {
        // A credential block must never half-merge: the old username has
        // to be gone, not merged with the new application credential.
        #[derive(Serialize)]
        struct Nested {
            auth: IndexMap<String, String>,
        }
        let mut auth = IndexMap::new();
        auth.insert("auth_url".to_string(), "https://new:5000".to_string());
        auth.insert(
            "application_credential_id".to_string(),
            "abc123".to_string(),
        );

        let existing = "clouds:\n  mycloud:\n    auth:\n      auth_url: https://old:5000\n      username: admin\n      password: secret\n";
        let mut doc = YamlDocument::parse(existing)?;
        doc.merge_mapping_entry("clouds", "mycloud", &Nested { auth })?;

        let out = doc.source();
        assert!(!out.contains("username"), "stale username: {out}");
        assert!(!out.contains("password"), "stale password: {out}");
        assert!(out.contains("application_credential_id"), "{out}");
        Ok(())
    }

    #[test]
    fn merge_rejects_a_non_mapping_value() {
        let mut doc =
            YamlDocument::parse("clouds:\n  mycloud:\n    auth_url: https://x:5000\n").unwrap();
        assert!(matches!(
            doc.merge_mapping_entry("clouds", "mycloud", &"just a string"),
            Err(YamlEditError::NotAMapping)
        ));
    }

    #[test]
    fn upsert_loses_the_next_entry_comment() -> Result<(), YamlEditError> {
        // Known `yamlpatch` behavior, pinned so a future upstream fix is
        // noticed here rather than silently. Callers that must not lose
        // comments use `merge_mapping_entry` instead.
        let existing = "clouds:\n  mycloud:\n    auth_url: https://old:5000\n    region_name: RegionOld\n  # attached to other\n  other:\n    auth_url: https://other:5000\n";
        let mut doc = YamlDocument::parse(existing)?;
        doc.upsert_mapping_entry("clouds", "mycloud", &entry("https://new:5000", "RegionNew"))?;

        assert!(
            !doc.source().contains("# attached to other"),
            "upstream may have fixed this - see merge_mapping_entry's doc \
             comment and reconsider which operation --overwrite uses: {}",
            doc.source()
        );
        Ok(())
    }

    #[test]
    fn remove_loses_and_orphans_neighbouring_comments() -> Result<(), YamlEditError> {
        // Known `yamlpatch` behavior, pinned for the same reason. Removing
        // a middle entry drops the *next* entry's comment and leaves the
        // removed entry's own comment behind, now misattributed to its
        // neighbour. A future `clouds remove` must account for this.
        let existing = "clouds:\n  # drop me\n  drop:\n    auth_url: https://drop:5000\n  # keep me\n  keep:\n    auth_url: https://keep:5000\n";
        let mut doc = YamlDocument::parse(existing)?;
        doc.remove_mapping_entry("clouds", "drop")?;

        let out = doc.source();
        assert!(!out.contains("# keep me"), "next comment eaten: {out}");
        assert!(out.contains("# drop me"), "own comment orphaned: {out}");
        Ok(())
    }
}
