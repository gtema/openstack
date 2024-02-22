# OpenStack API bindings (SDK)

Every platform API requires SDK bindings to various programming languages.
OpenStack API bindings for Rust are not an exception. The OpenStack comes with
`openstack_sdk` crate providing an SDK with both synchronous and asynchronous
interfaces.

API bindings are generated from the OpenAPI specs of corresponding services.
That means that those are only wrapping the API and usually not providing
additional convenience features.

## Features

- Sync and Async interface
- `Query`, `Find` and `Pagination` interfaces implementing basic functionality
- `RawQuery` interface providing more control over the API
  invokation with upload and download capabilities.
- Every combination of URL + http method + body schema is represented by a
dedicated module

## Structure

Every single API call is represented by a dedicated module with a structure
implementing `REST Endpoint` interface. That means that a `GET` operation is a
dedicated implementation compared to a `POST` operation. Like described in the
[Structure](structure.md) document every RPC-like action and every microversion
is implemented with a single module.

# Using

The simplest example demonstrating how to list compute flavors:

```rust
use openstack_sdk::api::{paged, Pagination, QueryAsync};
use openstack_sdk::{AsyncOpenStack, config::ConfigFile, OpenStackError};
use openstack_sdk::types::ServiceType;
use openstack_sdk::api::compute::v2::flavor::list;

async fn list_flavors() -> Result<(), OpenStackError> {
    // Get the builder for the listing Flavors Endpoint
    let mut ep_builder = list::Request::builder();
    // Set the `min_disk` query param
    ep_builder.min_disk("15");
    let ep = ep_builder.build().unwrap();

    let cfg = ConfigFile::new().unwrap();
    // Get connection config from clouds.yaml/secure.yaml
    let profile = cfg.get_cloud_config("devstack".to_string()).unwrap().unwrap();
    // Establish connection
    let mut session = AsyncOpenStack::new(&profile).await?;

    // Invoke service discovery when desired.
    session.discover_service_endpoint(&ServiceType::Compute).await?;

    // Execute the call with pagination limiting maximum amount of entries to 1000
    let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(1000))
        .query_async(&session)
        .await.unwrap();

    println!("Data = {:?}", data);
    Ok(())
}
```

## Documentation

Crate documentation [is published here](https://docs.rs/openstack_sdk)

Project documentation [is available here](https://gtema.github.io/openstack)
