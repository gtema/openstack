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

//! Built-in resource filters, evaluated during [`crate::cleanup::engine::ProjectCleanup::discover`]
//! when no `evaluation_fn` is supplied.

use std::collections::HashMap;

use chrono::DateTime;

use crate::cleanup::types::PlannedResource;

const TIMESTAMP_FILTER_KEYS: [&str; 2] = ["created_at", "updated_at"];

/// Evaluate the built-in `created_at`/`updated_at` timestamp filters
/// against a discovered resource, mirroring python openstacksdk's
/// `_service_cleanup_resource_filters_evaluation`: a resource is selected
/// only if, for every filter key present, the resource's own timestamp
/// field parses as an RFC3339 timestamp that is less than or equal to the
/// filter's own RFC3339 value. A resource missing the field, or either
/// value failing to parse, does not match that filter. An unrecognized
/// filter key never matches (mirrors python silently treating an
/// unsupported key as a failing condition, not an error).
///
/// No filters at all is vacuously true (`all()` over an empty iterator),
/// so calling this with an empty `filters` map selects every resource --
/// this matches python's `project_cleanup()` default of cleaning the
/// whole project when no filters/evaluation_fn are given.
pub fn evaluate_filters(resource: &PlannedResource, filters: &HashMap<String, String>) -> bool {
    filters.iter().all(|(key, value)| {
        if !TIMESTAMP_FILTER_KEYS.contains(&key.as_str()) {
            return false;
        }
        let Some(res_val) = resource.raw.get(key).and_then(|v| v.as_str()) else {
            return false;
        };
        let Ok(res_date) = DateTime::parse_from_rfc3339(res_val) else {
            return false;
        };
        let Ok(cmp_date) = DateTime::parse_from_rfc3339(value) else {
            return false;
        };
        res_date <= cmp_date
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cleanup::types::ResourceKind;
    use serde_json::json;

    fn resource_with_raw(raw: serde_json::Value) -> PlannedResource {
        PlannedResource {
            kind: ResourceKind::new("fake", "thing"),
            id: "thing-1".into(),
            name: None,
            raw,
            selected: false,
            reason: None,
        }
    }

    #[test]
    fn empty_filters_select_everything() {
        let resource = resource_with_raw(json!({}));
        assert!(evaluate_filters(&resource, &HashMap::new()));
    }

    #[test]
    fn created_at_at_or_before_cutoff_matches() {
        let resource = resource_with_raw(json!({"created_at": "2024-01-01T00:00:00Z"}));
        let mut filters = HashMap::new();
        filters.insert("created_at".to_string(), "2024-06-01T00:00:00Z".to_string());
        assert!(evaluate_filters(&resource, &filters));
    }

    #[test]
    fn created_at_after_cutoff_does_not_match() {
        let resource = resource_with_raw(json!({"created_at": "2024-12-01T00:00:00Z"}));
        let mut filters = HashMap::new();
        filters.insert("created_at".to_string(), "2024-06-01T00:00:00Z".to_string());
        assert!(!evaluate_filters(&resource, &filters));
    }

    #[test]
    fn created_at_exactly_at_cutoff_matches() {
        let resource = resource_with_raw(json!({"created_at": "2024-06-01T00:00:00Z"}));
        let mut filters = HashMap::new();
        filters.insert("created_at".to_string(), "2024-06-01T00:00:00Z".to_string());
        assert!(evaluate_filters(&resource, &filters));
    }

    #[test]
    fn missing_field_does_not_match() {
        let resource = resource_with_raw(json!({}));
        let mut filters = HashMap::new();
        filters.insert("created_at".to_string(), "2024-06-01T00:00:00Z".to_string());
        assert!(!evaluate_filters(&resource, &filters));
    }

    #[test]
    fn unparsable_resource_timestamp_does_not_match() {
        let resource = resource_with_raw(json!({"created_at": "not-a-date"}));
        let mut filters = HashMap::new();
        filters.insert("created_at".to_string(), "2024-06-01T00:00:00Z".to_string());
        assert!(!evaluate_filters(&resource, &filters));
    }

    #[test]
    fn unrecognized_filter_key_never_matches() {
        let resource = resource_with_raw(json!({"name": "whatever"}));
        let mut filters = HashMap::new();
        filters.insert("name".to_string(), "whatever".to_string());
        assert!(
            !evaluate_filters(&resource, &filters),
            "only created_at/updated_at are recognized filter keys"
        );
    }

    #[test]
    fn both_filters_must_match() {
        let resource = resource_with_raw(json!({
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-12-01T00:00:00Z"
        }));
        let mut filters = HashMap::new();
        filters.insert("created_at".to_string(), "2024-06-01T00:00:00Z".to_string());
        filters.insert("updated_at".to_string(), "2024-06-01T00:00:00Z".to_string());
        assert!(
            !evaluate_filters(&resource, &filters),
            "created_at matches but updated_at does not, so overall must be false"
        );
    }
}
