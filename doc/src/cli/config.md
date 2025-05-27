# CLI configuration

It is possible to configure different aspects of the OpenStackClient (not the
clouds connection credentials) using the configuration file
([$XDG_CONFIG_DIR](https://docs.rs/dirs/latest/dirs/fn.config_dir.html)`/osc/config.yaml`).
This enables user to configurate which columns should be returned when no
corresponding run time arguments on a resource base.

```yaml
views:
  compute.server:
    # Listing compute servers will only return ID, NAME and IMAGE columns unless `-o wide` or
    `-f XXX` parameters are being passed
    default_fields: [id, name, image]
    fields:
      - name: id
        width: 38 # Set column width at fixed 38 chars
        # min_width: 1 - Set minimal column width
        # max_width: 1 - Set maximal column width
  dns.zone/recordset:
    # DNS zone recordsets are listed in the wide mode by default.
    wide: true
    fields:
      - name: status
        max_width: 15  # status column can be maximum 15 chars wide
```

The key of the `views` map is a resource key shared among all
`openstack_rust`tools and is built in the following form:
`<SERVICE-TYPE>.<RESOURCE_NAME>[/<SUBRESOURCE_NAME>]` where
`<RESOURCE_NAME>[/<SUBRESOURCE_NAME>` is a url based naming (for designate
`/zone/<ID>/recordset/<RS_ID>` would be names as zone.recordset and
`/volumes/{volume_id}/metadata`would become volume.metadata. Please consult
[Codegenerator
metadata](https://opendev.org/openstack/codegenerator/src/branch/master/metadata)
for known resource keys.

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
