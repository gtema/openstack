use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use structable::build_table;
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

    let user = User {
        id: 1,
        first_name: "Scooby".into(),
        last_name: "Doo".into(),
        extra: "XYZ".into(),
        complex_data: Some(json!({"a": "b", "c": "d"})),
        dummy: None,
    };

    println!("User data:");
    let (headers, rows) = build_table(&user, &config);
    println!("{}", headers.join(","));
    for row in rows {
        println!("{}", row.join(","));
    }
}
