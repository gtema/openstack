# CLI configuration

It is possible to configure different aspects of the OpenStackClient (not the
clouds connection credentials) using the configuration file
(`$XDG_CONFIG_DIR/osc/config.yaml`). This enables user to configurate which
columns should be returned when no corresponding run time arguments on a
resource base.

```yaml
views:
  compute.server:
    # Listing compute servers will only return ID, NAME and IMAGE columns unless `-o wide` or
    `-f XXX` parameters are being passed
    fields: [id, name, image]
  dns.zone/recordset:
    # DNS zone recordsets are listed in the wide mode by default.
    wide: true
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
