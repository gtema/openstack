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

//! Post-YAML-parse validation for the create/edit-via-external-editor flow.
//!
//! Plain `serde_yaml`/`serde_json` parsing (already looped-on-failure in
//! `app.rs`) only catches syntax errors. This module adds the next stage:
//! checking the parsed buffer against a resource's `BODY_SCHEMA` (when it
//! has one, via `ResourceBehaviour::editor_schema`) -- required fields,
//! enums, ranges and the like that a `serde` struct alone can't express.

use serde::de::DeserializeOwned;
use serde_json::Value;

/// Drop object keys whose value is `null`, recursively.
///
/// The editor template leaves optional fields blank (`field:`), which YAML
/// parses as an explicit `null` rather than omitting the key. Most
/// `BODY_SCHEMA` property entries are typed as a plain `"string"`/`"integer"`
/// (nullability isn't declared -- the OpenAPI convention for "optional" is
/// to omit the key, not send `null`), so validating the buffer as-is flags
/// *every* blank optional field as a type mismatch, making it look like
/// they're all mandatory. Stripping nulls before validation (and before the
/// value is used to build the request) restores "blank means omitted" while
/// still letting a truly required field left blank fail validation, since
/// its key is removed too.
pub fn strip_null_fields(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k, strip_null_fields(v)))
                .collect(),
        ),
        Value::Array(arr) => Value::Array(arr.into_iter().map(strip_null_fields).collect()),
        other => other,
    }
}

/// Validate `instance` against `schema` (a `BODY_SCHEMA`-style JSON Schema
/// string), collecting *every* violation instead of stopping at the first,
/// each formatted as `<json pointer>: <message>`.
///
/// A schema that fails to compile is treated as "nothing to check against"
/// (logged, not surfaced as a user-facing error) rather than permanently
/// blocking the create/edit flow on a codegen-side bug.
pub fn validate_body(schema: &str, instance: &Value) -> Vec<String> {
    let schema_value: Value = match serde_json::from_str(schema) {
        Ok(v) => v,
        Err(err) => {
            tracing::warn!("BODY_SCHEMA is not valid JSON, skipping validation: {err}");
            return Vec::new();
        }
    };
    let validator = match jsonschema::validator_for(&schema_value) {
        Ok(v) => v,
        Err(err) => {
            tracing::warn!("BODY_SCHEMA is not a valid JSON Schema, skipping validation: {err}");
            return Vec::new();
        }
    };
    validator
        .iter_errors(instance)
        .map(|err| format!("{}: {}", err.instance_path(), err))
        .collect()
}

/// Deserialize `data` into `T`, reporting the field path on failure instead
/// of serde's default "invalid type" message with no location.
pub fn deserialize_with_path<T: DeserializeOwned>(data: &Value) -> Result<T, String> {
    serde_path_to_error::deserialize(data.clone()).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn strip_null_fields_drops_top_level_nulls() {
        let value = json!({"direction": "ingress", "protocol": null, "port_range_min": null});
        assert_eq!(strip_null_fields(value), json!({"direction": "ingress"}));
    }

    #[test]
    fn strip_null_fields_recurses_into_nested_objects() {
        let value = json!({"security_group_rule": {"direction": "ingress", "protocol": null}});
        assert_eq!(
            strip_null_fields(value),
            json!({"security_group_rule": {"direction": "ingress"}})
        );
    }

    #[test]
    fn strip_null_fields_leaves_non_null_values_untouched() {
        let value = json!({"port_range_min": 80, "tags": ["a", "b"]});
        assert_eq!(strip_null_fields(value.clone()), value);
    }

    #[test]
    fn validate_body_flags_blank_optional_field_unless_stripped() {
        let schema = json!({
            "type": "object",
            "required": ["direction"],
            "properties": {
                "direction": {"type": "string"},
                "protocol": {"type": "string"}
            }
        })
        .to_string();
        let instance = json!({"direction": "ingress", "protocol": null});
        assert!(!validate_body(&schema, &instance).is_empty());
        assert!(validate_body(&schema, &strip_null_fields(instance)).is_empty());
    }

    #[test]
    fn validate_body_returns_no_errors_for_valid_instance() {
        let schema = json!({
            "type": "object",
            "required": ["direction"],
            "properties": {
                "direction": {"type": "string", "enum": ["ingress", "egress"]}
            }
        })
        .to_string();
        let instance = json!({"direction": "ingress"});
        assert!(validate_body(&schema, &instance).is_empty());
    }

    #[test]
    fn validate_body_reports_missing_required_field() {
        let schema = json!({
            "type": "object",
            "required": ["direction"]
        })
        .to_string();
        let errors = validate_body(&schema, &json!({}));
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("direction"));
    }

    #[test]
    fn validate_body_reports_all_violations_not_just_the_first() {
        let schema = json!({
            "type": "object",
            "required": ["direction", "ethertype"],
            "properties": {
                "port_range_min": {"type": "integer", "maximum": 65535}
            }
        })
        .to_string();
        let errors = validate_body(&schema, &json!({"port_range_min": 70000}));
        assert_eq!(errors.len(), 3, "{errors:?}");
    }

    #[test]
    fn validate_body_skips_malformed_schema_without_panicking() {
        let errors = validate_body("not json", &json!({"a": 1}));
        assert!(errors.is_empty());
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Sample {
        name: String,
    }

    #[test]
    fn deserialize_with_path_succeeds_for_matching_shape() {
        let data = json!({"name": "foo"});
        let sample: Sample = deserialize_with_path(&data).unwrap();
        assert_eq!(sample, Sample { name: "foo".into() });
    }

    #[test]
    fn deserialize_with_path_reports_field_path_on_mismatch() {
        let data = json!({"name": 42});
        let err = deserialize_with_path::<Sample>(&data).unwrap_err();
        assert!(err.contains("name"), "{err}");
    }
}
