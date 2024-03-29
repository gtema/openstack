# OpenStack Rust

This is a package combining SDK and CLI

- `openstack_sdk` - SDK
- `openstack_cli` - The new and shiny CLI for OpenStack
- `structable_derive` - Helper crate for having Output in some way similar to
  old OpenStackClient
- `xtask` - workflow helper
- `doc` - Project documentation

## Trying out

It is possible to install compiled version from the GitHub releases. It comes
with a dedicated installer and can be retrieved with the following command:

```console
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gtema/openstack/releases/latest/download/openstack_cli-installer.sh | sh
```

Alternatively it is possible to compile project from sources. Since the project
is a pure `Rust` it requires having a Rust compile suite.

```console
cargo b
```

Once the binary is available just start playing with it:

```console
osc --help
osc --os-cloud devstack compute flavor list
```

## Documentation

Project [Documentation](https://gtema.github.io/openstack) is part of this
repository as well. It follows "code as a documentation" approach to keep
document matching the real code.

## Functional testing

SDK and CLI are coming with a set of basic functional tests that are not
executed by default since that requires access to the real cloud. In addition to that some tests require extended privileges so those are additionally ignored.

To trigger functional tests invoke:

```console
OS_CLOUD=devstack cargo t --test functional
```
