//! Example for using paged combinator to list all available flavors (paginating through the
//! results)
//!
//! `OS_CLOUD=devstack FLAVOR_MIN_RAM=32768 cargo run -p openstack_sdk --example paged`

#![allow(dead_code)]

use serde::Deserialize;
use serde_json::Value;
use std::env;

use openstack_sdk::api::paged;
use openstack_sdk::api::Pagination;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::OpenStackError;
use openstack_sdk::{config::ConfigFile, AsyncOpenStack};

use openstack_sdk::api::compute::v2::flavor::list_detailed;

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

    // use the list_detailed endpoint
    let mut ep_builder = list_detailed::Request::builder();
    if let Ok(min_ram) = env::var("FLAVOR_MIN_RAM") {
        // set query parameter
        ep_builder.min_ram(min_ram);
    }

    // Build the endpoint with parameters
    let ep = ep_builder.build().unwrap();

    // Execute the query enabling pagination
    let data: Vec<Flavor> = paged(ep, Pagination::Limit(100))
        .query_async(&client)
        .await?;

    println!("Flavors = {:?}", data);
    Ok(())
}
