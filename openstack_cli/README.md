# OpenStackClient

`osc` is a CLI for the OpenStack written in Rust. It is relying on the
corresponding `openstack_sdk` crate (library) and is generated using OpenAPI
specifications. That means that the maintenance effort for the tool is much
lower compared to the fully human written `python-openstackclient`. Due to the
fact of being auto-generated there are certain differences to the python cli
but also an enforced UX consistency.

**NOTE:** As a new tool it tries to solve some issues with the original
`python-openstackclient`. That means that it can not provide seamless migration
from one tool to another.

Commands implementation code is being produced by
[codegenerator](https://opendev.org/openstack/codegenerator) what means only
low maintenance is required for that code.

## Features

- Advanced authentication caching built-in and enabled by default

- Status based resource coloring (resource list table rows are colored by the
  resource state)

- Output configuration (using `$XDG_CONFIG_DIR/osc/config.yaml` it is possible
  to configure which fields should be returned when listing resources to enable
  customization).

- Strict microversion binding for resource modification requests (instead of
  `openstack server create ...` which will not work with all microversions you
  use `osc compute server create290` which will only work if server supports it.
  It is similar to `openstack --os-compute-api-version X.Y`). It behaves the same
  on every cloud independent of which microversion this cloud supports (as long
  as it supports required microversion).

- Can be wonderfully combined with jq for ultimate control of the necessary
  data (`osc server list -o json | jq -r ".[].flavor.original_name"`)

- Output everything what cloud sent (`osc compute server list -o json` to
  return fields that we never even knew about, but the cloud sent us).

- `osc` api as an API wrapper allowing user to perform any direct API call
  specifying service type, url, method and payload. This can be used for example
  when certain resource is not currently implemented natively.

- osc auth with subcommands for dealing explicitly with authentication (showing
  current auth info, renewing auth, MFA/SSO support)

## Microversions

Initially `python-openstackclient` was using lowest microversion unless
additional argument specifying microversion was passed. Later, during switching
commands towards using of the `OpenStackSDK` a highest possible microversion
started being used (again unless user explicitly requested microversion with
`--XXX-api-version Y.Z`). One common thing both approaches use is to give user
control over the version what is crucial to guarantee stability. The
disadvantage of both approaches is that they come with certain opinions that
does not necessarily match what user expects and make expectation on what will
happen hard. For the end user reading help page of the command is pretty
complex and error prone when certain parameters appear, disappear and re-appear
with different types between microversion. Implementing (and using) the command
is also both complex and error prone in this case.

`osc` is trying to get the best of 2 approaches and providing dedicated
commands for microversions (i.e. `create20`, `create294`). Latest microversion
command is always having a general alias (`create` in the above case) to let
user explicitly use latest microversion, what, however, does not guarantee it
can be invoked with requested parameters. This approach allows user to be very
explicit in the requirement and have a guarantee of the expected parameters.
When a newer microversion is required user should explicitly to do "migration"
step adapting the invocation to a newer set of parameters. Microversion (or
functionality) deprecation is also much simpler this way and is handled by
marking the whole command deprecated and/or drop it completely.

## Request timing

`osc` supports `--timing` argument that enables capturing of all HTTP requests
and outputs timings grouped by URL (ignoring the query parameters) and method.

## Connection configuration

`osc` tool supports connection configuration using the `clouds.yaml` files and
environment variables. In difference to the `python-openstackclient` no merging
of configuration file data with the environment variables is supported. Reason
for that is number of errors and unexpected behavior users are experiencing due
to that.

- `--os-cloud <CLOUD_NAME>` command argument points to the connection configured
  in the `clouds.yaml` file(s).

- `$OS_CLOUD` environment variable points to the configuration in the
  `clouds.yaml` file

- `--cloud-config-from-env` flag directs cli to ignore `clouds.yaml`
  configuration file completely and only rely on the environment variables
  (prefixed as usual with `OS_` prefix).

- `--os-cloud-name <CLOUD_NAME>` or `$OS_CLOUD_NAME` environment variable uses
  the specified value as the reference connection name i.e when the
  authentication helper resolves the missing authentication parameters (like
  password or similar).

- `--os-project-name` and `--os-project-id` cause the connection to be
  established with the regular credentials, but use the different project for
  the `scoped token.
