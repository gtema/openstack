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

//! Shared logic for converting JSON Schema to markdown documentation
//! and generating YAML configuration examples.
//!
//! Supports schemars 1.x which uses `$defs` instead of `definitions`.

use serde_json::Map;

/// Sensitive field name patterns used to flag fields in generated docs.
const SENSITIVE_PATTERNS: &[&str] = &[
    "password",
    "token",
    "secret",
    "passcode",
    "access_token",
    "jwt",
];

/// Check if a field name matches any sensitive pattern.
fn is_sensitive(name: &str) -> bool {
    SENSITIVE_PATTERNS
        .iter()
        .any(|p| name.to_lowercase().contains(p))
}

/// Convert a JSON Schema (as JSON Value) to markdown documentation.
pub fn schema_to_markdown(schema: &serde_json::Value) -> String {
    let mut out = String::new();

    // Parse definitions — schemars 1.x uses `$defs`
    let mut definitions: Map<_, _> = schema
        .get("$defs")
        .or_else(|| schema.get("definitions"))
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();

    // Add root schema as "ConfigFile" definition so it gets rendered
    definitions.insert("ConfigFile".to_string(), schema.clone());

    // Render ConfigFile first as the entry point
    if let Some(cf) = definitions.get("ConfigFile") {
        write_struct_section(&mut out, "ConfigFile", cf, &definitions);
    }

    // Render helper structs (skip ConfigFile which we already rendered)
    for (name, def) in &definitions {
        if name == "ConfigFile" {
            continue;
        }
        write_struct_section(&mut out, name, def, &definitions);
    }

    // YAML example
    out.push_str("\n\n---\n\n### YAML Example\n\n");
    out.push_str("```yaml\n");
    write_yaml_example(&mut out, schema, &definitions, 0);
    out.push_str("```\n");

    out
}

/// Write a section for a struct: description + name + table.
fn write_struct_section(
    out: &mut String,
    name: &str,
    def: &serde_json::Value,
    definitions: &Map<std::string::String, serde_json::Value>,
) {
    if let Some(desc) = def.get("description").and_then(|v| v.as_str()) {
        out.push_str(&format!("\n> {}\n", desc));
    }
    out.push_str(&format!("\n\n## `{}`\n\n", name));
    write_struct_table(out, name, def, definitions);
}

/// Write a markdown table for a struct definition.
fn write_struct_table(
    out: &mut String,
    name: &str,
    def: &serde_json::Value,
    definitions: &Map<std::string::String, serde_json::Value>,
) {
    out.push_str("| Field | Type | Default | Description |\n");
    out.push_str("|-------|------|---------|-------------|\n");

    if let Some(props) = def.get("properties").and_then(|v| v.as_object()) {
        for (field_name, field_schema) in props {
            let type_str = schema_type_to_string(field_schema, definitions);
            let default_str = get_default(field_schema);
            let desc = get_description(field_schema);
            let sensitive_str = if is_sensitive(field_name) {
                " *(sensitive)*"
            } else {
                ""
            };

            out.push_str(&format!(
                "| `{}` | {} | {} | {}{} |\n",
                field_name, type_str, default_str, desc, sensitive_str
            ));
        }
    }

    // Catch-all for CloudConfig
    if name == "CloudConfig" {
        out.push_str("| *catch-all* | any | — | Any additional configuration fields not explicitly typed |\n");
    }
}

/// Extract definition name from a JSON pointer ref path.
fn extract_def_name(ref_path: &str) -> Option<String> {
    ref_path
        .rsplit('/')
        .next()
        .map(|s| s.replace("$defs/", "").replace("definitions/", ""))
}

/// Convert a schema node to a readable type string.
fn schema_type_to_string(
    schema: &serde_json::Value,
    definitions: &Map<std::string::String, serde_json::Value>,
) -> String {
    // Handle anyOf — unwrap optional to find actual type
    if let Some(any_of) = schema.get("anyOf").and_then(|v| v.as_array()) {
        let is_optional = any_of.iter().any(|v| {
            matches!(v, serde_json::Value::Null)
                || (v.get("type").and_then(|t| t.as_str()) == Some("null"))
        });

        // Look for $ref inside anyOf for nested structs
        for item in any_of {
            if let Some(ref_path) = item.get("$ref").and_then(|v| v.as_str()) {
                if let Some(name) = extract_def_name(ref_path) {
                    if definitions.contains_key(&name) {
                        let opt_prefix = if is_optional { "`*optional*` " } else { "" };
                        return format!("{}[`{}`](#{})", opt_prefix, name, name.to_lowercase());
                    }
                }
            }
        }

        // Recurse into non-null items
        for item in any_of {
            if !matches!(item, serde_json::Value::Null) {
                return schema_type_to_string(item, definitions);
            }
        }
    }

    // Handle type array pattern ["string", "null"], ["object", "null"], etc.
    if let Some(type_arr) = schema.get("type").and_then(|v| v.as_array()) {
        let has_null = type_arr
            .iter()
            .any(|v| matches!(v, serde_json::Value::String(s) if s == "null"));
        let has_object = type_arr
            .iter()
            .any(|v| matches!(v, serde_json::Value::String(s) if s == "object"));

        // Get the non-null type
        let non_null_types: Vec<_> = type_arr
            .iter()
            .filter_map(|v| v.as_str().filter(|s| *s != "null"))
            .collect();

        // If it's an optional object with additionalProperties, it's a map-to-struct
        if has_null && has_object {
            if let Some(add_props) = schema.get("additionalProperties") {
                if let Some(ref_path) = add_props.get("$ref").and_then(|v| v.as_str()) {
                    if let Some(name) = extract_def_name(ref_path) {
                        if definitions.contains_key(&name) {
                            return format!("`*optional*` [`{}`](#{})", name, name.to_lowercase());
                        }
                    }
                }
            }
        }

        // For simple types like ["string", "null"], return the actual type
        if let Some(ty) = non_null_types.first() {
            let opt_prefix = if has_null { "`*optional*` " } else { "" };
            return format!("{}{}", opt_prefix, ty);
        }
    }

    // Check if it's a reference to a definition
    if let Some(ref_path) = schema.get("$ref").and_then(|v| v.as_str()) {
        if let Some(name) = extract_def_name(ref_path) {
            if definitions.contains_key(&name) {
                return format!("[`{}`](#{})", name, name.to_lowercase());
            }
        }
        return "object".to_string();
    }

    // Direct type
    if let Some(ty) = schema.get("type").and_then(|v| v.as_str()) {
        match ty {
            "string" => "string".to_string(),
            "number" => "number".to_string(),
            "integer" => "integer".to_string(),
            "boolean" => "boolean".to_string(),
            "object" => {
                if let Some(add_props) = schema.get("additionalProperties") {
                    if let Some(ref_path) = add_props.get("$ref").and_then(|v| v.as_str()) {
                        if let Some(name) = extract_def_name(ref_path) {
                            if definitions.contains_key(&name) {
                                return format!("[`{}`](#{})", name, name.to_lowercase());
                            }
                        }
                    }
                }
                "object".to_string()
            }
            "array" => {
                if let Some(items) = schema.get("items") {
                    if let Some(ref_path) = items.get("$ref").and_then(|v| v.as_str()) {
                        if let Some(name) = extract_def_name(ref_path) {
                            if definitions.contains_key(&name) {
                                return format!("array<[`{}`](#{})>", name, name.to_lowercase());
                            }
                        }
                    }
                    let item_type = schema_type_to_string(items, definitions);
                    format!("array<{}>", item_type)
                } else {
                    "array".to_string()
                }
            }
            "null" => "null".to_string(),
            _ => "any".to_string(),
        }
    } else {
        "any".to_string()
    }
}

/// Extract default value from a schema node.
fn get_default(schema: &serde_json::Value) -> String {
    // Check for explicit default in the schema
    if let Some(default) = schema.get("default") {
        let text = if let Some(v) = default.as_str() {
            format!("`{}`", v)
        } else {
            format!("`{}`", default)
        };
        return text;
    }

    // Check for examples
    if let Some(examples) = schema.get("examples").and_then(|v| v.as_array()) {
        if !examples.is_empty() {
            return format!("`{}`", examples[0]);
        }
    }

    // Check anyOf for nested default
    if let Some(any_of) = schema.get("anyOf").and_then(|v| v.as_array()) {
        for item in any_of {
            if !matches!(
                item,
                serde_json::Value::Null | serde_json::Value::Bool(false)
            ) {
                if let Some(d) = item.get("default") {
                    return format!("`{}`", d);
                }
            }
        }
    }

    "-".to_string()
}

/// Extract description from a schema node.
fn get_description(schema: &serde_json::Value) -> String {
    schema
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

/// Generate a YAML example from the schema.
fn write_yaml_example(
    out: &mut String,
    schema: &serde_json::Value,
    definitions: &Map<String, serde_json::Value>,
    indent: usize,
) {
    let prefix = "  ".repeat(indent);

    // Walk through root schema or definitions
    // schemars 1.x uses $defs
    let root_def = if let Some(defs) = schema.get("$defs").or_else(|| schema.get("definitions")) {
        // Use ConfigFile if it exists
        if let Some(cf) = defs.get("ConfigFile") {
            cf
        } else {
            schema
        }
    } else {
        schema
    };

    if let Some(props) = root_def.get("properties").and_then(|v| v.as_object()) {
        for (field_name, field_schema) in props {
            let field_prefix = if is_sensitive(field_name) {
                format!("{}# ", prefix)
            } else {
                prefix.clone()
            };

            // Check if it's a reference to a definition
            if let Some(ref_path) = field_schema.get("$ref").and_then(|v| v.as_str()) {
                if let Some(def_name) = extract_def_name(ref_path) {
                    if let Some(ref_def) = definitions.get(&def_name) {
                        out.push_str(&format!("{}{}:\n", field_prefix, field_name));
                        write_object_yaml(out, ref_def, definitions, indent + 1);
                        continue;
                    }
                }
            }

            // Check optional wrapping via anyOf
            if let Some(any_of) = field_schema.get("anyOf").and_then(|v| v.as_array()) {
                let non_null: Vec<_> = any_of
                    .iter()
                    .filter(|s| !matches!(s, serde_json::Value::Bool(false)))
                    .collect();
                if !non_null.is_empty() {
                    write_field_value(
                        out,
                        field_name,
                        &non_null[0],
                        definitions,
                        indent,
                        field_prefix,
                    );
                    continue;
                }
            }

            // Check type array ["object", "null"] with additionalProperties (HashMap)
            if let Some(type_arr) = field_schema.get("type").and_then(|v| v.as_array()) {
                let has_object = type_arr
                    .iter()
                    .any(|v| matches!(v, serde_json::Value::String(s) if s == "object"));
                if has_object {
                    if let Some(add_props) = field_schema.get("additionalProperties") {
                        if let Some(ref_path) = add_props.get("$ref").and_then(|v| v.as_str()) {
                            if let Some(def_name) = extract_def_name(ref_path) {
                                if let Some(ref_def) = definitions.get(&def_name) {
                                    out.push_str(&format!("{}{}:\n", field_prefix, field_name));
                                    out.push_str(&format!("{}  default:\n", field_prefix));
                                    write_object_yaml(out, ref_def, definitions, indent + 2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }

            // Direct type
            write_field_value(
                out,
                field_name,
                field_schema,
                definitions,
                indent,
                field_prefix,
            );
        }
    }
}

/// Write the YAML value for a single field.
fn write_field_value(
    out: &mut String,
    field_name: &str,
    schema: &serde_json::Value,
    definitions: &Map<String, serde_json::Value>,
    indent: usize,
    field_prefix: String,
) {
    // Handle anyOf — unwrap optional to find actual type
    if let Some(any_of) = schema.get("anyOf").and_then(|v| v.as_array()) {
        let non_null: Vec<_> = any_of
            .iter()
            .filter(|s| !matches!(s, serde_json::Value::Bool(false)))
            .collect();
        if !non_null.is_empty() {
            return write_field_value(
                out,
                field_name,
                &non_null[0],
                definitions,
                indent,
                field_prefix,
            );
        }
    }

    // Handle type array ["boolean", "null"] etc.
    if let Some(type_arr) = schema.get("type").and_then(|v| v.as_array()) {
        let has_object = type_arr
            .iter()
            .any(|v| matches!(v, serde_json::Value::String(s) if s == "object"));
        // If it's an object with additionalProperties, handle as map
        if has_object {
            if let Some(add_props) = schema.get("additionalProperties") {
                if let Some(ref_path) = add_props.get("$ref").and_then(|v| v.as_str()) {
                    if let Some(def_name) = extract_def_name(ref_path) {
                        if let Some(ref_def) = definitions.get(&def_name) {
                            out.push_str(&format!("{}{}:\n  default:\n", field_prefix, field_name));
                            write_object_yaml(out, ref_def, definitions, indent + 1);
                            return;
                        }
                    }
                }
            }
            out.push_str(&format!("{}{}:\n", field_prefix, field_name));
            write_object_yaml(out, schema, definitions, indent + 1);
            return;
        }

        let non_null: Vec<_> = type_arr
            .iter()
            .filter(|v| !matches!(v, serde_json::Value::String(s) if s == "null"))
            .collect();
        if !non_null.is_empty() {
            return write_scalar_yaml(
                out,
                field_name,
                non_null[0].as_str().unwrap_or("any"),
                &field_prefix,
            );
        }
    }

    if let Some(ty) = schema.get("type").and_then(|v| v.as_str()) {
        match ty {
            "string" => {
                let example_val = if is_sensitive(field_name) {
                    "VALUE"
                } else {
                    get_string_example(field_name)
                };
                out.push_str(&format!(
                    "{}{}: {}\n",
                    field_prefix, field_name, example_val
                ));
            }
            "boolean" => {
                out.push_str(&format!("{}{}: true\n", field_prefix, field_name));
            }
            "integer" | "number" => {
                out.push_str(&format!("{}{}: 0\n", field_prefix, field_name));
            }
            "object" => {
                out.push_str(&format!("{}{}:\n", field_prefix, field_name));
                write_object_yaml(out, schema, definitions, indent + 1);
            }
            "array" => {
                out.push_str(&format!("{}{}:\n", field_prefix, field_name));
                out.push_str(&format!("{}  - item\n", "  ".repeat(indent + 1)));
            }
            _ => {
                out.push_str(&format!("{}{}: value\n", field_prefix, field_name));
            }
        }
    } else if let Some(ref_path) = schema.get("$ref").and_then(|v| v.as_str()) {
        if let Some(def_name) = extract_def_name(ref_path) {
            if let Some(ref_def) = definitions.get(&def_name) {
                out.push_str(&format!("{}{}:\n", field_prefix, field_name));
                write_object_yaml(out, ref_def, definitions, indent + 1);
            }
        }
    } else {
        out.push_str(&format!("{}{}: value\n", field_prefix, field_name));
    }
}

/// Write a scalar YAML value.
fn write_scalar_yaml(out: &mut String, field_name: &str, ty: &str, prefix: &str) {
    let val = match (ty, is_sensitive(field_name)) {
        ("string", true) => "VALUE",
        ("string", false) => get_string_example(field_name),
        ("boolean", _) => "true",
        ("integer" | "number", _) => "0",
        _ => "value",
    };
    out.push_str(&format!("{}{}: {}\n", prefix, field_name, val));
}

/// Write YAML for an object schema's properties.
fn write_object_yaml(
    out: &mut String,
    schema: &serde_json::Value,
    definitions: &Map<String, serde_json::Value>,
    indent: usize,
) {
    if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
        for (field_name, field_schema) in props {
            let field_prefix = if is_sensitive(field_name) {
                format!("{}# ", "  ".repeat(indent))
            } else {
                "  ".repeat(indent)
            };

            // Handle references
            if let Some(ref_path) = field_schema.get("$ref").and_then(|v| v.as_str()) {
                if let Some(def_name) = extract_def_name(ref_path) {
                    if let Some(ref_def) = definitions.get(&def_name) {
                        out.push_str(&format!("{}{}:\n", field_prefix, field_name));
                        write_object_yaml(out, ref_def, definitions, indent + 1);
                        continue;
                    }
                }
            }

            // Handle anyOf (optional wrapper)
            if let Some(any_of) = field_schema.get("anyOf").and_then(|v| v.as_array()) {
                let non_null: Vec<_> = any_of
                    .iter()
                    .filter(|s| !matches!(s, serde_json::Value::Bool(false)))
                    .collect();
                if !non_null.is_empty() {
                    write_field_value(
                        out,
                        field_name,
                        &non_null[0],
                        definitions,
                        indent,
                        field_prefix,
                    );
                    continue;
                }
            }

            write_field_value(
                out,
                field_name,
                field_schema,
                definitions,
                indent,
                field_prefix.clone(),
            );
        }
    }
}

/// Return an example string value for a field.
fn get_string_example(field_name: &str) -> &str {
    match field_name {
        "auth_url" => "https://example.com:5000/v3",
        "username" | "user_name" => "admin",
        "user_domain_name" | "user_domain_id" => "Default",
        "domain_name" | "domain_id" => "Default",
        "project_name" | "project_id" => "myproject",
        "project_domain_name" | "project_domain_id" => "Default",
        "region_name" => "RegionOne",
        "interface" => "public",
        "profile" => "aws",
        "cacert" => "/path/to/ca.crt",
        "endpoint" => "public",
        "name" => "mycloud",
        "auth_type" => "v3password",
        "protocol" => "openid connect",
        "identity_provider" => "keycloak",
        "attribute_mapping_name" => "my-mapping",
        "access_token_type" => "Bearer",
        "application_credential_id" => "ac-uuid",
        "application_credential_name" => "my-credential",
        "system_scope" => "all",
        _ => "value",
    }
}
