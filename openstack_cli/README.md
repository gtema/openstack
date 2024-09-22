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
(codegenerator)[https://opendev.org/openstack/codegenerator] what means there
is no maintenance required for that code.

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
