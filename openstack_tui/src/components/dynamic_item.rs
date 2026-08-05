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

//! Support for resources whose response schema genuinely changes shape (not just adds fields)
//! across microversions (e.g. compute.flavor's `swap` i64->i32, compute.hypervisor's `id`
//! i32->String). For those, no single Rust struct can correctly represent every microversion, so
//! `ResourceBehaviour::Item` is a thin newtype around the raw `serde_json::Value` instead of a
//! generated `openstack_types` struct -- table columns are read out of the JSON by pointer at
//! render time, tolerating whatever shape actually came back, with no per-version deserialization
//! logic at all. See `impl_dynamic_item!` for the per-resource wiring.

use serde_json::Value;
use structable::StructTableOptions;

/// One table column: display title, JSON pointer (RFC 6901) into the raw response entry, whether
/// it's a `wide`-only column, and whether it backs `StructTable::status()` (row-coloring hook).
pub struct ColumnSpec {
    pub title: &'static str,
    pub pointer: &'static str,
    pub wide: bool,
    pub status: bool,
}

/// Stringify whatever's at `pointer` in `value`, the same way `StructTable`'s derive macro would
/// for a typed field: `None` for missing/null, plain text for a JSON string, `Display`-style (via
/// `Value`'s own formatting) for a scalar, and pretty-vs-compact JSON (per
/// `options.pretty_mode()`) for an object/array -- matching how `structable_derive` renders
/// `serialize`/`pretty`-tagged (i.e. non-primitive) fields.
fn stringify(value: &Value, pointer: &str, pretty: bool) -> Option<String> {
    match value.pointer(pointer) {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => Some(s.clone()),
        Some(complex @ (Value::Object(_) | Value::Array(_))) => Some(if pretty {
            serde_json::to_string_pretty(complex).unwrap_or_else(|_| complex.to_string())
        } else {
            complex.to_string()
        }),
        Some(other) => Some(other.to_string()),
    }
}

/// `StructTable::class_headers` body for a dynamic item: header list is just the configured
/// column titles, filtered by `should_return_field` like any generated struct's headers are.
pub fn dynamic_headers<O: StructTableOptions>(
    columns: &[ColumnSpec],
    options: &O,
) -> Option<Vec<String>> {
    Some(
        columns
            .iter()
            .filter(|c| options.should_return_field(c.title, c.wide))
            .map(|c| c.title.to_string())
            .collect(),
    )
}

/// `StructTable::data` body for a dynamic item. Honors `StructTableOptions::field_data_json_pointer`
/// so user config can remap a column to a different pointer into the raw response entry, same as
/// the `structable_derive` macro does for `serialize`/`pretty` fields.
pub fn dynamic_data<O: StructTableOptions>(
    value: &Value,
    columns: &[ColumnSpec],
    options: &O,
) -> Vec<Option<String>> {
    let pretty = options.pretty_mode();
    columns
        .iter()
        .filter(|c| options.should_return_field(c.title, c.wide))
        .map(|c| {
            let pointer = options.field_data_json_pointer(c.title);
            stringify(value, pointer.as_deref().unwrap_or(c.pointer), pretty)
        })
        .collect()
}

/// `StructTable::status` body for a dynamic item: the value at the column marked `status: true`
/// (if any). Unlike `dynamic_data`, `StructTable::status` has no `options` parameter to consult
/// for a pointer override or pretty-mode, so this always reads the column's baked pointer as
/// plain text.
pub fn dynamic_status(value: &Value, columns: &[ColumnSpec]) -> Option<String> {
    let column = columns.iter().find(|c| c.status)?;
    stringify(value, column.pointer, false)
}

/// Declare a newtype wrapper around `serde_json::Value` usable as `ResourceBehaviour::Item` for a
/// resource with a genuinely breaking microversion schema change. `$columns` is a
/// `&'static [ColumnSpec]`.
///
/// A per-resource newtype (rather than one shared `Item = Value`) is required because
/// `ResourceKey::get_key` is a static method with no way to know which resource a bare `Value` is
/// for.
macro_rules! impl_dynamic_item {
    ($name:ident, $key:expr, $columns:expr) => {
        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name(pub serde_json::Value);

        impl $name {
            pub fn get(&self, pointer: &str) -> Option<&serde_json::Value> {
                self.0.pointer(pointer)
            }

            pub fn get_str(&self, pointer: &str) -> Option<String> {
                self.get(pointer)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            }
        }

        impl crate::utils::ResourceKey for $name {
            fn get_key() -> &'static str {
                $key
            }
        }

        impl structable::StructTable for $name {
            fn class_headers<O: structable::StructTableOptions>(
                options: &O,
            ) -> Option<Vec<String>> {
                crate::components::dynamic_item::dynamic_headers($columns, options)
            }
            fn data<O: structable::StructTableOptions>(&self, options: &O) -> Vec<Option<String>> {
                crate::components::dynamic_item::dynamic_data(&self.0, $columns, options)
            }
            fn status(&self) -> Option<String> {
                crate::components::dynamic_item::dynamic_status(&self.0, $columns)
            }
        }

        impl structable::StructTable for &$name {
            fn class_headers<O: structable::StructTableOptions>(
                options: &O,
            ) -> Option<Vec<String>> {
                <$name as structable::StructTable>::class_headers(options)
            }
            fn data<O: structable::StructTableOptions>(&self, options: &O) -> Vec<Option<String>> {
                structable::StructTable::data(*self, options)
            }
            fn status(&self) -> Option<String> {
                structable::StructTable::status(*self)
            }
        }
    };
}

pub(crate) use impl_dynamic_item;

#[cfg(test)]
mod tests {
    use super::*;

    const COLUMNS: &[ColumnSpec] = &[
        ColumnSpec {
            title: "ID",
            pointer: "/id",
            wide: false,
            status: false,
        },
        ColumnSpec {
            title: "Status",
            pointer: "/status",
            wide: false,
            status: true,
        },
        ColumnSpec {
            title: "Metadata",
            pointer: "/metadata",
            wide: false,
            status: false,
        },
    ];

    struct Opts {
        pretty: bool,
    }

    impl StructTableOptions for Opts {
        fn wide_mode(&self) -> bool {
            false
        }
        fn pretty_mode(&self) -> bool {
            self.pretty
        }
        fn should_return_field<S: AsRef<str>>(&self, _field: S, _is_wide_field: bool) -> bool {
            true
        }
    }

    #[test]
    fn stringify_scalar_ignores_pretty() {
        let value = serde_json::json!({"id": "abc"});
        assert_eq!(stringify(&value, "/id", true), Some("abc".to_string()));
    }

    #[test]
    fn stringify_object_respects_pretty_mode() {
        let value = serde_json::json!({"metadata": {"a": 1}});
        assert_eq!(
            stringify(&value, "/metadata", false),
            Some(serde_json::json!({"a": 1}).to_string())
        );
        assert_eq!(
            stringify(&value, "/metadata", true),
            Some(serde_json::to_string_pretty(&serde_json::json!({"a": 1})).unwrap())
        );
    }

    #[test]
    fn stringify_missing_or_null_is_none() {
        let value = serde_json::json!({"id": null});
        assert_eq!(stringify(&value, "/id", false), None);
        assert_eq!(stringify(&value, "/missing", false), None);
    }

    #[test]
    fn dynamic_data_honors_pretty_mode_for_complex_fields() {
        let value = serde_json::json!({"id": "abc", "status": "ACTIVE", "metadata": {"a": 1}});
        let compact = dynamic_data(&value, COLUMNS, &Opts { pretty: false });
        assert_eq!(compact[2], Some(serde_json::json!({"a": 1}).to_string()));
        let pretty = dynamic_data(&value, COLUMNS, &Opts { pretty: true });
        assert_eq!(
            pretty[2],
            Some(serde_json::to_string_pretty(&serde_json::json!({"a": 1})).unwrap())
        );
    }

    #[test]
    fn dynamic_status_returns_marked_column_value() {
        let value = serde_json::json!({"id": "abc", "status": "ACTIVE"});
        assert_eq!(dynamic_status(&value, COLUMNS), Some("ACTIVE".to_string()));
    }

    #[test]
    fn dynamic_status_none_when_no_column_marked() {
        let value = serde_json::json!({"id": "abc"});
        let columns: &[ColumnSpec] = &[ColumnSpec {
            title: "ID",
            pointer: "/id",
            wide: false,
            status: false,
        }];
        assert_eq!(dynamic_status(&value, columns), None);
    }
}
