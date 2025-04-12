use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use structable::build_list_table;
use structable::{OutputConfig, StructTable, StructTableOptions};

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

fn main() {
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

    println!("Users data:");
    let (headers, rows) = build_list_table(users.iter(), &config);
    println!("{}", headers.join(","));
    for row in rows {
        println!("{}", row.join(","));
    }
}
