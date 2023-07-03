# StructTable derive macro

Most likely you do not want to use this crate directly. It is a helper for the OpenStack

This crate implements derive macros for converting structures
(or structure vectors) as tables (vector of vector of strings -
as rows and columns).

Depending on the OutputConfig it is possible to build a tuple (headers, rows) with selected fields

```rust
  #[derive(Serialize, StructTable)]
  struct User {
      #[structable(title = "ID")]
      id: u64,
      first_name: &'static str,
      last_name: &'static str,
      #[structable(title = "Long(only in wide mode)", wide)]
      extra: &'static str,
  }

  use std::collections::BTreeSet;

  struct OutputConfig {
      /// Limit fields (their titles) to be returned
      fields: BTreeSet<String>,
      /// Wide mode (additional fields requested)
      wide: bool,
  }

  trait TableOutput {
      fn build(&self, options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>);
  }
```

Example

```rust
    let users = vec![
        User {
            id: 1,
            first_name: "Scooby",
            last_name: "Doo",
            long_only: "Foo",
        },
        User {
            id: 2,
            first_name: "John",
            last_name: "Cena",
            long_only: "Bar",
        },
    ];
    let user = User {
        id: 1,
        first_name: "Scooby",
        last_name: "Doo",
        long_only: "XYZ",
    };

    let ln_fields: BTreeSet<String> = BTreeSet::from(["Last Name".to_string()]);

    let config = OutputConfig {
        fields: BTreeSet::new(), // ln_fields,
        wide: true,
    };
    let data = user.build(&config);
    println!("Single user {:?} => {:?}", data.0, data.1);
    let data2 = users.build(&config);
    println!("multiple users {:?} => {:?}", data2.0, data2.1);

```

```
Single user ["Attribute", "Value"] => [["id", "1"], ["first_name", "Scooby"], ["last_name", "Doo"], ["long_only", "XYZ"]]
multiple user ["id", "first_name", "last_name", "long_only"] => [["1", "Scooby", "Doo", "Foo"], ["2", "John", "Cena", "Bar"]]
```
