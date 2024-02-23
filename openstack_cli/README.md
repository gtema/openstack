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

## Features

- `osc api` as an API wrapper allowing user to perform any direct API call
  specifying service type, url, method and payload. This can be used for
  example when certain resource is not currently implemented natively.
- `osc auth` with subcommands for deailng explicitly with authentication
  (showing current auth info, renewing auth, MFA/SSO support)
- Every resource is having a service type in the command solving confusions
  like user groups vs volume groups
- Every multi-word resource name is "-" separated (i.e. floating-ip,
  access-rule)

### Output

- `osc ... -o json` as an explicit machine readable format output. It allows
  seeing raw resource json representation as send by the API without any
  processing on the client side. **Note:** the result is not the raw json
  response, but the raw json resource information found underneath expected
  resource key. This mode can be used i.e. to see fields that are not expected
  by the `osc` and allows further easy machine processing with tools like `jq`
- `osc ... -o wide` for list operations to return all known fields. By default
  list operation will only return a subset of known generic resource fields to
  prevent multiline tables. This mode (together with not specifying `-o`
  parameter at all) is considered as an output for humans. Field names are not
  generally renamed and are names as the API returns them.

### Microversions

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
