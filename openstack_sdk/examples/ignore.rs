//! Example for using ignore combinator to send API request not expecting any response. This is
//! typically used for DELETE operations
//!
//! `OS_CLOUD=devstack FLAVOR_ID=... cargo run -p openstack_sdk --example ignore`
//!
use std::env;

use openstack_sdk::OpenStackError;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::ignore;
use openstack_sdk::{AsyncOpenStack, config::ConfigFile};

use openstack_sdk::api::compute::v2::flavor::get;

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
    let mut ep_builder = get::Request::builder();

    // Set the `id` which is NAME or ID to find
    ep_builder.id(env::var("FLAVOR_ID").expect("FLAVOR_ID variable must be set"));

    // Build the endpoint
    let ep = ep_builder.build().unwrap();

    // Execute query keeping data as raw json_value
    ignore(ep).query_async(&client).await?;

    println!("Flavor was retrieved but we ignored the result");
    Ok(())
}
