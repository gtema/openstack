# Introduction

**openstack-rs**'s intension is to start adoption (or support) of Rust in
OpenStack itself. The very beginning of the journey is implementing Rust SDK for
OpenStack APIs and a CLI.

In difference to existing OpenStackSDK and the CLI this project aims to have a
code generated from the OpenAPI specification for the OpenStack service APIs.
That does not mean that 100% of the SDK and CLI code is being generated, but
rather that the API bindings themselves are generated while leaving place for
certain framework required to combine things and make them working.
