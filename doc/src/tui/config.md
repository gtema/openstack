# Configuration

Certain aspects of the TUI can be configured using configuration file.

Config file can be in the yaml or json format and placed under
`XDG_CONFIG_HOME/openstack_tui` named as `config.yaml` or `views.yaml`.
It is possible to split configuration parts into dediated files (i.e.
`views.yaml` for configuring views). Files are merged in no particular order
(in difference to the `clouds.yaml/secure.yaml`) so it is not possible to
predict the behavior when configuration option is being set in different files
with different value.

## Default config

```yaml
{{#include ../../../openstack_tui/.config/config.yaml}}
```

## Views configuration

Every resource view can be configured in a separate section of the config.
Resource key in a form <SERVICE>.<RESOURCE>[/<SUBRESOURCE>] as used by the
codegenerator is a name of a view. `fields` is an array of field names to be
populated.

NOTE: This a work in progress to pre-generate resource structures and column
names.
