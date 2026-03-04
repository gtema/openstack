# OpenStack API bindings (SDK) - core

`openstack_sdk_core` implements basic functionality used by the `openstack_sdk`
crate as the OpenStack SDK (interfaces, connection client, catalog, version
discovery, etc). This crate does not implement service bindings, and is
therefore not very useful alone.

## Features

- Sync and Async interface
- `Query`, `Find` and `Pagination` interfaces implementing basic functionality
- `RawQuery` interface providing more control over the API invocation with
  upload and download capabilities.

# Using

The simplest example demonstrating how to list compute flavors:

```rust
use openstack_sdk_core::api::{paged, Pagination, QueryAsync, Pageable,
    RestEndpoint};
use openstack_sdk_core::{AsyncOpenStack, config::ConfigFile, OpenStackError};
use openstack_sdk_core::types::ServiceType;
use std::borrow::Cow;

#[derive(derive_builder::Builder)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    id: Cow<'a, str>,
    #[builder(default, setter(into))]
    min_disk: Option<Cow<'a, str>>,
}

impl RestEndpoint for Request<'_> {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "flavors".to_string().into()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("flavor".into())
    }
}
impl Pageable for Request<'_> {}

async fn list_flavors() -> Result<(), OpenStackError> {
    // Get the builder for the listing Flavors Endpoint
    let mut ep_builder = RequestBuilder::default();
    // Set the `min_disk` query param
    ep_builder.min_disk("15");
    let ep = ep_builder.build().unwrap();

    let cfg = ConfigFile::new().unwrap();
    // Get connection config from clouds.yaml/secure.yaml
    let profile = cfg.get_cloud_config("devstack").unwrap().unwrap();
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

Current crate documentation is known to be not sufficient. It will be addressed in
future, but for now the best way to figure out how it works is to look at
openstack_cli and openstack_tui using it.

Crate documentation [is published here](https://docs.rs/openstack_sdk_core)

Project documentation [is available here](https://gtema.github.io/openstack)
