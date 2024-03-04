#[cfg(test)]
mod tests {
    mod structable_derive {

        use serde::Serialize;
        use serde_json::{json, Value};
        use std::collections::BTreeSet;
        use structable_derive::StructTable;

        #[derive(Serialize, StructTable)]
        struct User {
            #[structable(title = "ID")]
            id: u64,
            first_name: &'static str,
            last_name: &'static str,
            #[structable(title = "Long", wide)]
            extra: &'static str,
            #[structable(optional, pretty, wide)]
            complex_data: Option<Value>,
        }

        struct OutputConfig {
            /// Limit fields (their titles) to be returned
            fields: BTreeSet<String>,
            /// Wide mode (additional fields requested)
            wide: bool,
            // Pretty-print
            pretty: bool,
        }

        trait StructTable {
            fn build(&self, options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>);
        }

        #[test]
        fn test_single() {
            let config = OutputConfig {
                fields: BTreeSet::new(),
                wide: false,
                pretty: false,
            };
            let user = User {
                id: 1,
                first_name: "Scooby",
                last_name: "Doo",
                extra: "XYZ",
                complex_data: Some(json!({"a": "b", "c": "d"})),
            };
            let data = user.build(&config);
            assert_eq!(
                data,
                (
                    vec!["Attribute".to_string(), "Value".to_string()],
                    vec![
                        vec!["ID".to_string(), "1".to_string()],
                        vec!["first_name".to_string(), "Scooby".to_string()],
                        vec!["last_name".to_string(), "Doo".to_string()],
                    ]
                )
            );
        }

        #[test]
        fn test_single_wide() {
            let config = OutputConfig {
                fields: BTreeSet::new(),
                wide: true,
                pretty: false,
            };
            let user = User {
                id: 1,
                first_name: "Scooby",
                last_name: "Doo",
                extra: "XYZ",
                complex_data: Some(json!({"a": "b", "c": "d"})),
            };
            let data = user.build(&config);
            assert_eq!(
                data,
                (
                    vec!["Attribute".to_string(), "Value".to_string()],
                    vec![
                        vec!["ID".to_string(), "1".to_string()],
                        vec!["first_name".to_string(), "Scooby".to_string()],
                        vec!["last_name".to_string(), "Doo".to_string()],
                        vec!["Long".to_string(), "XYZ".to_string()],
                        vec![
                            "complex_data".to_string(),
                            "{\"a\":\"b\",\"c\":\"d\"}".to_string()
                        ],
                    ]
                )
            );
        }

        #[test]
        fn test_single_wide_pretty() {
            let config = OutputConfig {
                fields: BTreeSet::new(),
                wide: true,
                pretty: true,
            };
            let user = User {
                id: 1,
                first_name: "Scooby",
                last_name: "Doo",
                extra: "XYZ",
                complex_data: Some(json!({"a": "b", "c": "d"})),
            };
            let data = user.build(&config);
            assert_eq!(
                data,
                (
                    vec!["Attribute".to_string(), "Value".to_string()],
                    vec![
                        vec!["ID".to_string(), "1".to_string()],
                        vec!["first_name".to_string(), "Scooby".to_string()],
                        vec!["last_name".to_string(), "Doo".to_string()],
                        vec!["Long".to_string(), "XYZ".to_string()],
                        vec![
                            "complex_data".to_string(),
                            "{\n  \"a\": \"b\",\n  \"c\": \"d\"\n}".to_string()
                        ],
                    ]
                )
            );
        }

        #[test]
        fn test_list() {
            let config = OutputConfig {
                fields: BTreeSet::new(),
                wide: false,
                pretty: false,
            };
            let users = vec![
                User {
                    id: 1,
                    first_name: "Scooby",
                    last_name: "Doo",
                    extra: "Foo",
                    complex_data: Some(json!({"a": "b", "c": "d"})),
                },
                User {
                    id: 2,
                    first_name: "John",
                    last_name: "Cena",
                    extra: "Bar",
                    complex_data: None,
                },
            ];

            let data = users.build(&config);
            assert_eq!(
                data,
                (
                    vec![
                        "ID".to_string(),
                        "first_name".to_string(),
                        "last_name".to_string()
                    ],
                    vec![
                        vec!["1".to_string(), "Scooby".to_string(), "Doo".to_string()],
                        vec!["2".to_string(), "John".to_string(), "Cena".to_string()],
                    ]
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
                    first_name: "Scooby",
                    last_name: "Doo",
                    extra: "Foo",
                    complex_data: Some(json!({"a": "b", "c": "d"})),
                },
                User {
                    id: 2,
                    first_name: "John",
                    last_name: "Cena",
                    extra: "Bar",
                    complex_data: None,
                },
            ];

            let data = users.build(&config);
            assert_eq!(
                data,
                (
                    vec![
                        "ID".to_string(),
                        "first_name".to_string(),
                        "last_name".to_string(),
                        "Long".to_string(),
                        "complex_data".to_string()
                    ],
                    vec![
                        vec![
                            "1".to_string(),
                            "Scooby".to_string(),
                            "Doo".to_string(),
                            "Foo".to_string(),
                            "{\n  \"a\": \"b\",\n  \"c\": \"d\"\n}".to_string()
                        ],
                        vec![
                            "2".to_string(),
                            "John".to_string(),
                            "Cena".to_string(),
                            "Bar".to_string(),
                            "".to_string()
                        ],
                    ]
                )
            );
        }
    }
}
