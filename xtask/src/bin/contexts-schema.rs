fn main() {
    let schema = schemars::schema_for!(openstack_sdk_core::config::contexts::ContextsFile);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
