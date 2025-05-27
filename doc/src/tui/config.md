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
populated. All column names are forcibly converted to the UPPER CASE.

## Resource view options

- **default_fields** (*list[str]*)

  A list of fields to be displayed by default (in not wide mode). If not
  specified, only certain fields determined internally are displayed. Output
  columns are sorted in the order given in the list.

- **wide** (*bool*)

  If set to true, display all fields. If set to false, display only the
  default_fields.

- **fields** (*list[obj]*)

  A list of column configuration. Consists of:

  - **name** (*str*) - field name (resource attribute name)

  - **width** (*int*) - column width in characters

  - **min_width** (*int*) - minimum column width in characters

  - **max_width** (*int*) - maximum column width in characters

  - **json_pointer** (*str*) - JSON pointer to the extract from the resource
    field. This is only applied in the list and not `wide` mode.
