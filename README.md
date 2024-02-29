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
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gtema/openstack/releases/download/v0.1.1/openstack_cli-installer.sh | sh
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
