//! Example for using query and find combinator
//!
//! `OS_CLOUD=devstack FLAVOR_NAME=m1.tiny cargo run -p openstack_sdk --example query_find`
//!
#![allow(dead_code)]
use serde::Deserialize;
use serde_json::Value;
use std::env;

use openstack_sdk::OpenStackError;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find;
use openstack_sdk::{AsyncOpenStack, config::ConfigFile};

use openstack_sdk::api::compute::v2::flavor::find;

#[derive(Deserialize, Debug, Clone)]
struct Flavor {
    disk: Option<i32>,
    extra_specs: Option<Value>,
    id: Option<String>,
    name: Option<String>,
    #[serde(rename = "os-flavor-access:is_public")]
    os_flavor_access_is_public: Option<bool>,
    ram: Option<i32>,
    vcpus: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), OpenStackError> {
    let cfg = ConfigFile::new().unwrap();
    // Get connection config from clouds.yaml/secure.yaml
    let profile = cfg
        .get_cloud_config(env::var("OS_CLOUD").expect("OS_CLOUD variable must be set"))
        .unwrap()
        .unwrap();

    // Establish connection
    let client = AsyncOpenStack::new(&profile).await?;

    // Prepare the find combinator
    let mut find_builder = find::Request::builder();

    // Set the `id` which is NAME or ID to find
    find_builder.id(env::var("FLAVOR_NAME").expect("FLAVOR_NAME variable must be set"));

    // Build the endpoint
    let find_ep = find_builder.build().unwrap();

    // Execute query casting data to the Flavor type
    let find_data: Flavor = find(find_ep.clone()).query_async(&client).await?;

    println!("Flavor Data = {find_data:?}");

    // Execute query keeping data as raw json_value
    let find_data_raw: Value = find(find_ep).query_async(&client).await?;

    println!(
        "Flavor raw json {:?}",
        serde_json::to_string(&find_data_raw)?
    );
    Ok(())
}
