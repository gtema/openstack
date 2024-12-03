use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use structable_derive::StructTable;

#[derive(Default, Deserialize, Serialize, StructTable)]
struct User {
    #[structable(title = "ID")]
    id: u64,
    first_name: String,
    last_name: String,
    #[structable(title = "Long", wide)]
    extra: String,
    #[structable(optional, pretty, wide)]
    complex_data: Option<Value>,
    #[structable(optional)]
    dummy: Option<String>,
}

#[derive(Deserialize, Serialize, StructTable)]
struct StatusStruct {
    #[structable(title = "ID")]
    id: u64,
    #[structable(status)]
    status: String,
}
#[derive(Deserialize, Serialize, StructTable)]
struct OptionStatusStruct {
    #[structable(title = "ID")]
    id: u64,
    #[structable(status, optional)]
    status: Option<String>,
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
    fn status(&self) -> Vec<Option<String>>;
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
        first_name: "Scooby".to_string(),
        last_name: "Doo".to_string(),
        extra: "XYZ".to_string(),
        complex_data: Some(json!({"a": "b", "c": "d"})),
        dummy: None,
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
        first_name: "Scooby".to_string(),
        last_name: "Doo".to_string(),
        extra: "XYZ".to_string(),
        complex_data: Some(json!({"a": "b", "c": "d"})),
        dummy: None,
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
        first_name: "Scooby".to_string(),
        last_name: "Doo".to_string(),
        extra: "XYZ".to_string(),
        complex_data: Some(json!({"a": "b", "c": "d"})),
        dummy: None,
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
            first_name: "Scooby".to_string(),
            last_name: "Doo".to_string(),
            extra: "Foo".to_string(),
            complex_data: Some(json!({"a": "b", "c": "d"})),
            dummy: None,
        },
        User {
            id: 2,
            first_name: "John".to_string(),
            last_name: "Cena".to_string(),
            extra: "Bar".to_string(),
            complex_data: None,
            dummy: None,
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
                "dummy".to_string()
            ],
            vec![
                vec![
                    "1".to_string(),
                    "Scooby".to_string(),
                    "Doo".to_string(),
                    " ".to_string()
                ],
                vec![
                    "2".to_string(),
                    "John".to_string(),
                    "Cena".to_string(),
                    " ".to_string()
                ],
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
            first_name: "Scooby".to_string(),
            last_name: "Doo".to_string(),
            extra: "Foo".to_string(),
            complex_data: Some(json!({"a": "b", "c": "d"})),
            dummy: None,
        },
        User {
            id: 2,
            first_name: "John".to_string(),
            last_name: "Cena".to_string(),
            extra: "Bar".to_string(),
            complex_data: None,
            dummy: Some("foo".to_string()),
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
                "complex_data".to_string(),
                "dummy".to_string()
            ],
            vec![
                vec![
                    "1".to_string(),
                    "Scooby".to_string(),
                    "Doo".to_string(),
                    "Foo".to_string(),
                    "{\n  \"a\": \"b\",\n  \"c\": \"d\"\n}".to_string(),
                    " ".to_string()
                ],
                vec![
                    "2".to_string(),
                    "John".to_string(),
                    "Cena".to_string(),
                    "Bar".to_string(),
                    " ".to_string(),
                    "foo".to_string()
                ],
            ]
        )
    );
}

#[test]
fn test_deser() {
    let config = OutputConfig {
        fields: BTreeSet::new(),
        wide: false,
        pretty: false,
    };

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
    let data = foo.build(&config);
    assert_eq!(
        data,
        (
            vec!["Attribute".to_string(), "Value".to_string()],
            vec![vec!["ID".to_string(), "1".to_string()],]
        )
    );
}

#[test]
fn test_list_status() {
    let raw = vec![
        StatusStruct {
            id: 1,
            status: String::from("foo"),
        },
        StatusStruct {
            id: 2,
            status: String::from("bar"),
        },
    ];

    let data = raw.status();
    assert_eq!(
        data,
        vec![Some(String::from("foo")), Some(String::from("bar"))]
    );
}

#[test]
fn test_list_status_no_status() {
    let raw = vec![User::default(), User::default()];

    let data = raw.status();
    assert_eq!(data, vec![None, None]);
}

#[test]
fn test_single_status() {
    let raw = StatusStruct {
        id: 1,
        status: String::from("foo"),
    };

    let data = raw.status();
    assert_eq!(data, vec![Some(String::from("foo")),]);
}

#[test]
fn test_single_no_status() {
    let raw = User::default();

    let data = raw.status();
    assert_eq!(data, vec![None]);
}

#[test]
fn test_single_option_status() {
    let raw = OptionStatusStruct {
        id: 1,
        status: Some(String::from("foo")),
    };

    let data = raw.status();
    assert_eq!(data, vec![Some(String::from("foo")),]);
}
