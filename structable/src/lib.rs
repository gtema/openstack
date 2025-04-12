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

//! Representing data to the user (i.e. in CLI or TUI) usually requires converting data into vector
//! of vector of strings with the data. Further this data is being passed to tools like
//! `comfy_table`, `cli-table`or similar. Preparing such data is a tedious job. This is where
//! StructTable is coming to help.
//!
//! For a structure like:
//!
//! ```rust
//! use serde::Serialize;
//! use serde_json::Value;
//! use structable::{StructTable, StructTableOptions};
//!
//! #[derive(Serialize, StructTable)]
//! struct User {
//!     #[structable(title = "ID")]
//!     id: u64,
//!     first_name: String,
//!     last_name: String,
//!     #[structable(title = "Long", wide)]
//!     extra: String,
//!     #[structable(optional, serialize, wide)]
//!     complex_data: Option<Value>,
//!     #[structable(optional)]
//!     dummy: Option<String>,
//! }
//! ```
//!
//! What you get is:
//!
//! ```rust
//! # use serde::Serialize;
//! # use serde_json::Value;
//! # use structable::{StructTable, StructTableOptions};
//! # #[derive(Serialize)]
//! # struct User {
//! #     id: u64,
//! #     first_name: String,
//! #     last_name: String,
//! #     extra: String,
//! #     complex_data: Option<Value>,
//! #     dummy: Option<String>,
//! # }
//! impl StructTable for User {
//!     fn headers<O: StructTableOptions>(
//!         options: &O,
//!     ) -> ::std::vec::Vec<::std::string::String> {
//!         let mut headers: Vec<String> = Vec::new();
//!         if options.should_return_field("ID", false) {
//!             headers.push("ID".to_string());
//!         }
//!         if options.should_return_field("first_name", false) {
//!             headers.push("first_name".to_string());
//!         }
//!         if options.should_return_field("last_name", false) {
//!             headers.push("last_name".to_string());
//!         }
//!         if options.should_return_field("Long", true) {
//!             headers.push("Long".to_string());
//!         }
//!         if options.should_return_field("complex_data", true) {
//!             headers.push("complex_data".to_string());
//!         }
//!         if options.should_return_field("dummy", false) {
//!             headers.push("dummy".to_string());
//!         }
//!         headers
//!     }
//!
//!     fn data<O: StructTableOptions>(
//!         &self,
//!         options: &O,
//!     ) -> ::std::vec::Vec<Option<::std::string::String>> {
//!         let mut row: Vec<Option<String>> = Vec::new();
//!         if options.should_return_field("ID", false) {
//!             row.push(Some(self.id.to_string()));
//!         }
//!         if options.should_return_field("first_name", false) {
//!             row.push(Some(self.first_name.to_string()));
//!         }
//!         if options.should_return_field("last_name", false) {
//!             row.push(Some(self.last_name.to_string()));
//!         }
//!         if options.should_return_field("Long", true) {
//!             row.push(Some(self.extra.to_string()));
//!         }
//!         if options.should_return_field("complex_data", true) {
//!             row.push(
//!                 self
//!                     .complex_data
//!                     .clone()
//!                     .map(|v| {
//!                         if options.pretty_mode() {
//!                             serde_json::to_string_pretty(&v)
//!                         } else {
//!                             serde_json::to_string(&v)
//!                         }
//!                             .unwrap_or_else(|_| String::from(
//!                                 "<ERROR SERIALIZING DATA>",
//!                             ))
//!                     }),
//!             );
//!         }
//!         if options.should_return_field("dummy", false) {
//!             row.push(self.dummy.clone().map(|x| x.to_string()));
//!         }
//!         row
//!     }
//!     fn status(&self) -> Option<String> {
//!         None
//!     }
//! }
//! ```
//!
//! Example
//!
//! ```rust
//! # use std::collections::BTreeSet;
//! # use serde_json::{json, Value};
//! # use serde::Serialize;
//! use structable::{build_table, build_list_table};
//! use structable::{OutputConfig, StructTable, StructTableOptions};
//!
//! #[derive(Serialize, StructTable)]
//! struct User {
//!     #[structable(title = "ID")]
//!     id: u64,
//!     first_name: &'static str,
//!     last_name: &'static str,
//!     #[structable(title = "Long(only in wide mode)", wide)]
//!     extra: &'static str,
//!     #[structable(optional, pretty)]
//!     complex_data: Option<Value>
//! }
//!
//! let users = vec![
//!     User {
//!         id: 1,
//!         first_name: "Scooby",
//!         last_name: "Doo",
//!         extra: "Foo",
//!         complex_data: Some(json!({"a": "b", "c": "d"}))
//!     },
//!     User {
//!         id: 2,
//!         first_name: "John",
//!         last_name: "Cena",
//!         extra: "Bar",
//!         complex_data: None
//!     },
//! ];
//! let user = User {
//!     id: 1,
//!     first_name: "Scooby",
//!     last_name: "Doo",
//!     extra: "XYZ",
//!     complex_data: Some(json!({"a": "b", "c": "d"}))
//! };
//!
//! let config = OutputConfig {
//!     fields: BTreeSet::from(["Last Name".to_string()]),
//!     wide: false,
//!     pretty: false
//! };
//!
//! let data = build_table(&user, &config);
//! println!("Single user {:?} => {:?}", data.0, data.1);
//! let data2 = build_list_table(users.iter(), &config);
//! println!("multiple users {:?} => {:?}", data2.0, data2.1);
//!
//! ```
//!
//! ```text
//! Single user ["Attribute", "Value"] => [["id", "1"], ["first_name", "Scooby"], ["last_name", "Doo"], ["long_only", "XYZ"]]
//! multiple user ["id", "first_name", "last_name", "long_only"] => [["1", "Scooby", "Doo", "Foo"], ["2", "John", "Cena", "Bar"]]
//! ```
//!
//!
//!

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub use structable_derive::StructTable;

/// Output configuration
///
/// This structure is controlling how the table table is being built for a structure.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputConfig {
    /// Limit fields (their titles) to be returned
    pub fields: BTreeSet<String>,
    /// Wide mode (additional fields requested)
    pub wide: bool,
    /// Pretty-print
    pub pretty: bool,
}

/// StructTable output configuration trait
///
/// When OutputConfig can not be used you can implement this trait on you structure.
pub trait StructTableOptions {
    /// Whether to return fields marked as `wide`-only
    fn wide_mode(&self) -> bool;

    /// Whether to serialize values using `to_pretty_string`
    fn pretty_mode(&self) -> bool;

    /// Whether the attribute should be returned
    fn should_return_field<S: AsRef<str>>(&self, field: S, is_wide_field: bool) -> bool;
}

impl StructTableOptions for OutputConfig {
    fn wide_mode(&self) -> bool {
        self.wide
    }

    fn pretty_mode(&self) -> bool {
        self.pretty
    }

    fn should_return_field<S: AsRef<str>>(&self, field: S, is_wide_field: bool) -> bool {
        if !is_wide_field {
            self.fields.is_empty() || self.fields.contains(field.as_ref())
        } else {
            (self.fields.is_empty() && self.wide_mode()) || self.fields.contains(field.as_ref())
        }
    }
}

/// Trait for building tables out of structures
pub trait StructTable {
    /// Return Vector of table headers (attribute titles to be returned)
    fn headers<O: StructTableOptions>(config: &O) -> Vec<String>;

    /// Return vector of selected fields as `Option<String>`
    fn data<O: StructTableOptions>(&self, config: &O) -> Vec<Option<String>>;

    /// Return structure status property
    fn status(&self) -> Option<String>;
}

/// Build a table for a single structure
///
/// Returns a vector with first row being column headers ["Attribute", "Value"]. All other rows
/// represent transposed table with first value in the vector being an attribute name and second
/// value being the value itself. The optional attribute, which is `None` is not being returned.
pub fn build_table<T, O>(data: &T, options: &O) -> (Vec<String>, Vec<Vec<String>>)
where
    T: StructTable,
    O: StructTableOptions,
{
    let headers = Vec::from(["Attribute".into(), "Value".into()]);
    let mut rows: Vec<Vec<String>> = Vec::new();
    for (a, v) in T::headers(options).iter().zip(data.data(options).iter()) {
        if let Some(data) = v {
            rows.push(Vec::from([a.to_string(), data.to_string()]));
        }
    }
    (headers, rows)
}

/// Build a table for list of entries
///
/// Returns vector of vector of strings with first row being table headers and all other rows are
/// the values themselves.
pub fn build_list_table<I, T, O>(data: I, options: &O) -> (Vec<String>, Vec<Vec<String>>)
where
    I: Iterator<Item = T>,
    T: StructTable,
    O: StructTableOptions,
{
    let headers = T::headers(options);
    let rows: Vec<Vec<String>> = Vec::from_iter(data.map(|item| {
        item.data(options)
            .into_iter()
            .map(|el| el.unwrap_or_else(|| String::from(" ")))
            .collect::<Vec<String>>()
    }));
    (headers, rows)
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;

    #[derive(Default, Deserialize, Serialize, StructTable)]
    struct User {
        #[structable(title = "ID")]
        id: u64,
        first_name: String,
        last_name: String,
        #[structable(title = "Long", wide)]
        extra: String,
        #[structable(optional, serialize, wide)]
        complex_data: Option<Value>,
        #[structable(optional)]
        dummy: Option<String>,
    }

    #[derive(Deserialize, Serialize, StructTable)]
    struct StatusStruct {
        #[structable(status)]
        status: String,
    }

    #[derive(Clone, Deserialize, Serialize)]
    enum Status {
        Dummy,
    }

    #[derive(Deserialize, Serialize, StructTable)]
    struct SerializeStatusStruct {
        #[structable(serialize, status)]
        status: Status,
    }

    #[derive(Deserialize, Serialize, StructTable)]
    struct SerializeOptionStatusStruct {
        #[structable(optional, serialize, status)]
        status: Option<Status>,
    }

    #[derive(Deserialize, Serialize, StructTable)]
    struct OptionStatusStruct {
        #[structable(status, optional)]
        status: Option<String>,
    }

    #[test]
    fn test_single() {
        let config = OutputConfig::default();
        let user = User {
            id: 1,
            first_name: "Scooby".into(),
            last_name: "Doo".into(),
            extra: "XYZ".into(),
            complex_data: Some(json!({"a": "b", "c": "d"})),
            dummy: None,
        };
        assert_eq!(
            build_table(&user, &config),
            (
                vec!["Attribute".into(), "Value".into()],
                vec![
                    vec!["ID".into(), "1".into()],
                    vec!["first_name".into(), "Scooby".into()],
                    vec!["last_name".into(), "Doo".into()],
                ]
            )
        );
    }

    #[test]
    fn test_single_wide() {
        let config = OutputConfig {
            wide: true,
            ..Default::default()
        };
        let user = User {
            id: 1,
            first_name: "Scooby".into(),
            last_name: "Doo".into(),
            extra: "XYZ".into(),
            complex_data: Some(json!({"a": "b", "c": "d"})),
            dummy: None,
        };
        assert_eq!(
            build_table(&user, &config),
            (
                vec!["Attribute".into(), "Value".into()],
                vec![
                    vec!["ID".into(), "1".into()],
                    vec!["first_name".into(), "Scooby".into()],
                    vec!["last_name".into(), "Doo".into()],
                    vec!["Long".into(), "XYZ".into()],
                    vec![
                        "complex_data".into(),
                        "{\"a\":\"b\",\"c\":\"d\"}".to_string()
                    ],
                ]
            )
        );
    }

    #[test]
    fn test_single_wide_column() {
        let config = OutputConfig {
            fields: BTreeSet::from(["Long".into()]),
            ..Default::default()
        };
        let user = User {
            id: 1,
            first_name: "Scooby".into(),
            last_name: "Doo".into(),
            extra: "XYZ".into(),
            complex_data: Some(json!({"a": "b", "c": "d"})),
            dummy: None,
        };
        assert_eq!(
            build_table(&user, &config),
            (
                vec!["Attribute".into(), "Value".into()],
                vec![vec!["Long".into(), "XYZ".into()],]
            )
        );
    }

    #[test]
    fn test_single_wide_column_wide_mode() {
        let config = OutputConfig {
            fields: BTreeSet::from(["Long".into()]),
            wide: true,
            ..Default::default()
        };
        let user = User {
            id: 1,
            first_name: "Scooby".into(),
            last_name: "Doo".into(),
            extra: "XYZ".into(),
            complex_data: Some(json!({"a": "b", "c": "d"})),
            dummy: None,
        };
        assert_eq!(
            build_table(&user, &config),
            (
                vec!["Attribute".into(), "Value".into()],
                vec![vec!["Long".into(), "XYZ".into()],]
            )
        );
    }

    #[test]
    fn test_single_wide_pretty() {
        let config = OutputConfig {
            wide: true,
            pretty: true,
            ..Default::default()
        };
        let user = User {
            id: 1,
            first_name: "Scooby".into(),
            last_name: "Doo".into(),
            extra: "XYZ".into(),
            complex_data: Some(json!({"a": "b", "c": "d"})),
            dummy: None,
        };
        assert_eq!(
            build_table(&user, &config),
            (
                vec!["Attribute".into(), "Value".into()],
                vec![
                    vec!["ID".into(), "1".into()],
                    vec!["first_name".into(), "Scooby".into()],
                    vec!["last_name".into(), "Doo".into()],
                    vec!["Long".into(), "XYZ".into()],
                    vec![
                        "complex_data".into(),
                        "{\n  \"a\": \"b\",\n  \"c\": \"d\"\n}".to_string()
                    ],
                ]
            )
        );
    }

    #[test]
    fn test_single_status() {
        assert_eq!(
            StatusStruct {
                status: "foo".into(),
            }
            .status(),
            Some("foo".into())
        );
    }
    #[test]
    fn test_single_no_status() {
        assert_eq!(User::default().status(), None);
    }
    #[test]
    fn test_single_option_status() {
        assert_eq!(
            OptionStatusStruct {
                status: Some("foo".into()),
            }
            .status(),
            Some("foo".into())
        );
    }

    #[test]
    fn test_complex_status() {
        assert_eq!(
            SerializeStatusStruct {
                status: Status::Dummy,
            }
            .status(),
            Some("Dummy".into())
        );

        assert_eq!(
            SerializeOptionStatusStruct {
                status: Some(Status::Dummy),
            }
            .status(),
            Some("Dummy".into())
        );
    }
    #[test]
    fn test_status() {
        #[derive(Deserialize, Serialize, StructTable)]
        struct StatusStruct {
            #[structable(title = "ID")]
            id: u64,
            #[structable(status)]
            status: String,
        }
    }

    #[test]
    fn test_list() {
        let config = OutputConfig::default();
        let users = vec![
            User {
                id: 1,
                first_name: "Scooby".into(),
                last_name: "Doo".into(),
                extra: "Foo".into(),
                complex_data: Some(json!({"a": "b", "c": "d"})),
                dummy: None,
            },
            User {
                id: 2,
                first_name: "John".into(),
                last_name: "Cena".into(),
                extra: "Bar".into(),
                complex_data: None,
                dummy: None,
            },
        ];

        assert_eq!(
            build_list_table(users.iter(), &config),
            (
                vec![
                    "ID".into(),
                    "first_name".into(),
                    "last_name".into(),
                    "dummy".into()
                ],
                vec![
                    vec!["1".into(), "Scooby".into(), "Doo".into(), " ".into()],
                    vec!["2".into(), "John".into(), "Cena".into(), " ".into()],
                ]
            )
        );
    }

    #[test]
    fn test_list_wide_column() {
        let config = OutputConfig {
            fields: BTreeSet::from(["Long".into()]),
            ..Default::default()
        };
        let users = vec![
            User {
                id: 1,
                first_name: "Scooby".into(),
                last_name: "Doo".into(),
                extra: "Foo".into(),
                complex_data: Some(json!({"a": "b", "c": "d"})),
                dummy: None,
            },
            User {
                id: 2,
                first_name: "John".into(),
                last_name: "Cena".into(),
                extra: "Bar".into(),
                complex_data: None,
                dummy: Some("foo".into()),
            },
        ];

        assert_eq!(
            build_list_table(users.iter(), &config),
            (
                vec!["Long".into(),],
                vec![vec!["Foo".into(),], vec!["Bar".into(),],]
            )
        );
    }

    #[test]
    fn test_list_wide_column_wide_mode() {
        let config = OutputConfig {
            fields: BTreeSet::from(["Long".into()]),
            wide: true,
            pretty: false,
        };
        let users = vec![
            User {
                id: 1,
                first_name: "Scooby".into(),
                last_name: "Doo".into(),
                extra: "Foo".into(),
                complex_data: Some(json!({"a": "b", "c": "d"})),
                dummy: None,
            },
            User {
                id: 2,
                first_name: "John".into(),
                last_name: "Cena".into(),
                extra: "Bar".into(),
                complex_data: None,
                dummy: Some("foo".into()),
            },
        ];

        assert_eq!(
            build_list_table(users.iter(), &config),
            (
                vec!["Long".into(),],
                vec![vec!["Foo".into(),], vec!["Bar".into(),],]
            )
        );
    }

    #[test]
    fn test_list_wide() {
        let config = OutputConfig {
            fields: BTreeSet::new(),
            wide: true,
            pretty: false,
        };
        let users = vec![
            User {
                id: 1,
                first_name: "Scooby".into(),
                last_name: "Doo".into(),
                extra: "Foo".into(),
                complex_data: Some(json!({"a": "b", "c": "d"})),
                dummy: None,
            },
            User {
                id: 2,
                first_name: "John".into(),
                last_name: "Cena".into(),
                extra: "Bar".into(),
                complex_data: None,
                dummy: Some("foo".into()),
            },
        ];

        assert_eq!(
            build_list_table(users.iter(), &config),
            (
                vec![
                    "ID".into(),
                    "first_name".into(),
                    "last_name".into(),
                    "Long".into(),
                    "complex_data".into(),
                    "dummy".into()
                ],
                vec![
                    vec![
                        "1".into(),
                        "Scooby".into(),
                        "Doo".into(),
                        "Foo".into(),
                        "{\"a\":\"b\",\"c\":\"d\"}".to_string(),
                        " ".to_string()
                    ],
                    vec![
                        "2".into(),
                        "John".into(),
                        "Cena".into(),
                        "Bar".into(),
                        " ".to_string(),
                        "foo".into()
                    ],
                ]
            )
        );
    }

    #[test]
    fn test_deser() {
        #[derive(Deserialize, Serialize, StructTable)]
        struct Foo {
            #[structable(title = "ID")]
            id: u64,
            #[structable(optional)]
            foo: Option<String>,
            #[structable(optional)]
            bar: Option<bool>,
        }

        let foo: Foo = serde_json::from_value(json!({"id": 1})).expect("Foo object");

        assert_eq!(
            build_table(&foo, &OutputConfig::default()),
            (
                vec!["Attribute".into(), "Value".into()],
                vec![vec!["ID".into(), "1".into()],]
            )
        );
    }
}
