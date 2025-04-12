# StructTable

Representing data to the user (i.e. in CLI or TUI) usually requires converting
data into vector of vector of strings with the data. Further this data is being
passed to tools like `comfy_table`, `cli-table`or similar. Preparing such data
is a tedious job. This is where StructTable is coming to help.

For a structure like:

```rust
#[derive(Serialize, StructTable)]
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
```

What you get is:

```rust
impl StructTable for User {
    fn headers<O: StructTableOptions>(
        options: &O,
    ) -> ::std::vec::Vec<::std::string::String> {
        let mut headers: Vec<String> = Vec::new();
        if options.should_return_field("ID", false) {
            headers.push("ID".to_string());
        }
        if options.should_return_field("first_name", false) {
            headers.push("first_name".to_string());
        }
        if options.should_return_field("last_name", false) {
            headers.push("last_name".to_string());
        }
        if options.should_return_field("Long", true) {
            headers.push("Long".to_string());
        }
        if options.should_return_field("complex_data", true) {
            headers.push("complex_data".to_string());
        }
        if options.should_return_field("dummy", false) {
            headers.push("dummy".to_string());
        }
        headers
    }

    fn data<O: StructTableOptions>(
        &self,
        options: &O,
    ) -> ::std::vec::Vec<Option<::std::string::String>> {
        let mut row: Vec<Option<String>> = Vec::new();
        if options.should_return_field("ID", false) {
            row.push(Some(self.id.to_string()));
        }
        if options.should_return_field("first_name", false) {
            row.push(Some(self.first_name.to_string()));
        }
        if options.should_return_field("last_name", false) {
            row.push(Some(self.last_name.to_string()));
        }
        if options.should_return_field("Long", true) {
            row.push(Some(self.extra.to_string()));
        }
        if options.should_return_field("complex_data", true) {
            row.push(
                self
                    .complex_data
                    .clone()
                    .map(|v| {
                        if options.pretty_mode() {
                            serde_json::to_string_pretty(&v)
                        } else {
                            serde_json::to_string(&v)
                        }
                            .unwrap_or_else(|_| String::from(
                                "<ERROR SERIALIZING DATA>",
                            ))
                    }),
            );
        }
        if options.should_return_field("dummy", false) {
            row.push(self.dummy.clone().map(|x| x.to_string()));
        }
        row
    }
    fn status(&self) -> Option<String> {
        None
    }
}
```

## Usage

```rust
use structable::{build_table, build_list_table};
use structable::{OutputConfig, StructTable, StructTableOptions};

#[derive(Serialize, StructTable)]
struct User {
    #[structable(title = "ID")]
    id: u64,
    first_name: &'static str,
    last_name: &'static str,
    #[structable(title = "Long(only in wide mode)", wide)]
    extra: &'static str,
    #[structable(optional, pretty)]
    complex_data: Option<Value>
}

let users = vec![
    User {
        id: 1,
        first_name: "Scooby",
        last_name: "Doo",
        extra: "Foo",
        complex_data: Some(json!({"a": "b", "c": "d"}))
    },
    User {
        id: 2,
        first_name: "John",
        last_name: "Cena",
        extra: "Bar",
        complex_data: None
    },
];
let user = User {
    id: 1,
    first_name: "Scooby",
    last_name: "Doo",
    extra: "XYZ",
    complex_data: Some(json!({"a": "b", "c": "d"}))
};

let config = OutputConfig {
    fields: BTreeSet::from(["Last Name".to_string()]),
    wide: false,
    pretty: false
};

let data = build_table(&user, &config);
println!("Single user {:?} => {:?}", data.0, data.1);
let data2 = build_list_table(users.iter(), &config);
println!("multiple users {:?} => {:?}", data2.0, data2.1);

```

```text
Single user ["Attribute", "Value"] => [["id", "1"], ["first_name", "Scooby"], ["last_name", "Doo"], ["long_only", "XYZ"]]
multiple user ["id", "first_name", "last_name", "long_only"] => [["1", "Scooby", "Doo", "Foo"], ["2", "John", "Cena", "Bar"]]
```
