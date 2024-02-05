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
- Every combination of URL + http method + body schema is
- represented by a dedicated module

## Structure

Every single API call is represented by a dedicated module with a structure
implementing `REST Endpoint` interface. That means that a `GET` operation is a
dedicated implementation compared to a `POST` operation. Like described in the
[Structure](structure.md) document every RPC-like action and every microversion
is implemented with a single module.
