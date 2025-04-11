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
    #[structable(optional, serialize, wide)]
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

#[derive(Clone, Deserialize, Serialize)]
enum Status {
    Dummy,
}

#[derive(Deserialize, Serialize, StructTable)]
struct SerializeStatusStruct {
    #[structable(title = "ID")]
    id: u64,
    #[structable(serialize, status)]
    status: Status,
}

#[derive(Deserialize, Serialize, StructTable)]
struct SerializeOptionStatusStruct {
    #[structable(title = "ID")]
    id: u64,
    #[structable(optional, serialize, status)]
    status: Option<Status>,
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
        first_name: "Scooby".into(),
        last_name: "Doo".into(),
        extra: "XYZ".into(),
        complex_data: Some(json!({"a": "b", "c": "d"})),
        dummy: None,
    };
    let data = user.build(&config);
    assert_eq!(
        data,
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
        fields: BTreeSet::new(),
        wide: true,
        pretty: false,
    };
    let user = User {
        id: 1,
        first_name: "Scooby".into(),
        last_name: "Doo".into(),
        extra: "XYZ".into(),
        complex_data: Some(json!({"a": "b", "c": "d"})),
        dummy: None,
    };
    let data = user.build(&config);
    assert_eq!(
        data,
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
        wide: false,
        pretty: false,
    };
    let user = User {
        id: 1,
        first_name: "Scooby".into(),
        last_name: "Doo".into(),
        extra: "XYZ".into(),
        complex_data: Some(json!({"a": "b", "c": "d"})),
        dummy: None,
    };
    let data = user.build(&config);
    assert_eq!(
        data,
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
        pretty: false,
    };
    let user = User {
        id: 1,
        first_name: "Scooby".into(),
        last_name: "Doo".into(),
        extra: "XYZ".into(),
        complex_data: Some(json!({"a": "b", "c": "d"})),
        dummy: None,
    };
    let data = user.build(&config);
    assert_eq!(
        data,
        (
            vec!["Attribute".into(), "Value".into()],
            vec![vec!["Long".into(), "XYZ".into()],]
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
        first_name: "Scooby".into(),
        last_name: "Doo".into(),
        extra: "XYZ".into(),
        complex_data: Some(json!({"a": "b", "c": "d"})),
        dummy: None,
    };
    let data = user.build(&config);
    assert_eq!(
        data,
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
fn test_list() {
    let config = OutputConfig {
        fields: BTreeSet::new(),
        wide: false,
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
            dummy: None,
        },
    ];

    let data = users.build(&config);
    assert_eq!(
        data,
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
        wide: false,
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

    let data = users.build(&config);
    assert_eq!(
        data,
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

    let data = users.build(&config);
    assert_eq!(
        data,
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

    let data = users.build(&config);
    assert_eq!(
        data,
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
            vec!["Attribute".into(), "Value".into()],
            vec![vec!["ID".into(), "1".into()],]
        )
    );
}

#[test]
fn test_list_status() {
    let raw = vec![
        StatusStruct {
            id: 1,
            status: "foo".into(),
        },
        StatusStruct {
            id: 2,
            status: "bar".into(),
        },
    ];

    let data = raw.status();
    assert_eq!(data, vec![Some("foo".into()), Some("bar".into())]);
}

#[test]
fn test_list_status_no_status() {
    let raw = vec![User::default(), User::default()];

    let data = raw.status();
    assert_eq!(data, vec![None, None]);
}

#[test]
fn test_single_status() {
    assert_eq!(
        StatusStruct {
            id: 1,
            status: "foo".into(),
        }
        .status(),
        vec![Some("foo".into()),]
    );
}

#[test]
fn test_single_no_status() {
    assert_eq!(User::default().status(), vec![None]);
}

#[test]
fn test_single_option_status() {
    assert_eq!(
        OptionStatusStruct {
            id: 1,
            status: Some("foo".into()),
        }
        .status(),
        vec![Some("foo".into()),]
    );
}

#[test]
fn test_complex_status() {
    assert_eq!(
        SerializeStatusStruct {
            id: 1,
            status: Status::Dummy,
        }
        .status(),
        vec![Some("Dummy".into()),]
    );

    assert_eq!(
        SerializeOptionStatusStruct {
            id: 1,
            status: Some(Status::Dummy),
        }
        .status(),
        vec![Some("Dummy".into()),]
    );
}
