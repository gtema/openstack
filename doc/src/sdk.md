# OpenStack API bindings (SDK)

Every platform API requires SDK bindings to various programming languages.
OpenStack API bindings for Rust are not an expection. The OpenStack comes with
`openstack_sdk` crate providing an SDK with both synchronous and asynchronous
interfaces.

API bindings are generated from the OpenAPI specs of corresponding services.
That means that those are only wrapping the API and usually not providing
additional convinience features.
