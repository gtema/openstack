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
use yamlpath::{Document, Route, route};

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
        let mut updates = IndexMap::new();
        updates.insert(key.to_string(), value);
        let patch = Patch {
            route: Route::default(),
            operation: Op::MergeInto {
                key: parent.to_string(),
                updates,
            },
        };
        self.0 = apply_yaml_patches(&self.0, std::slice::from_ref(&patch))?;
        Ok(())
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
}
