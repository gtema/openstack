# Contexts.yaml configuration

`contexts.yaml` is an alternative to `clouds.yaml` that follows the structure of
the kubernetes `kubeconfig`. Named `servers`, `users` and `scopes` are defined
independently and a `context` combines them by name:

```yaml
current-context: prod-admin
servers:
  - name: prod
    server:
      auth_url: https://keystone.example.com/v3
users:
  - name: alice
    user:
      user_name: alice
      domain:
        name: Default
      auth_helper: /usr/local/bin/osc-pass
scopes:
  - name: admin
    scope:
      project:
        name: admin
        domain:
          name: Default
contexts:
  - name: prod-admin
    context:
      server: prod
      user: alice
      scope: admin
      region_name: RegionOne  # optional
```

The file is searched as `contexts.{yaml,yml}` in
`$XDG_CONFIG_HOME/openstack/`, `~/.config/openstack/` and `/etc/openstack/`
(in this order). Unlike `clouds.yaml` the current working directory is **not**
searched, because `auth_helper` names a program that gets executed and must not
be picked up from an untrusted directory. The file is **not** merged with
`clouds.yaml` or `secure.yaml`: a connection is selected either by a cloud name
or by a context name. The only exception is the `profile` of a server, which is
looked up in the `clouds-public.yaml` and supplies the defaults of the vendor.

## Users

A user is identified by `user_name` (`username` is accepted as well) or `user_id`
and the optional `domain` (`id` and/or `name`, same as the domain of a scope).
Every other key (i.e. `application_credential_id`, `auth_type`) is passed to the
authentication plugin. Setting an attribute twice (i.e. `domain` together with
`user_domain_name`) is an error.

## Scopes

A scope holds exactly one of `project`, `domain` or `system`, so an ambiguous
scope is rejected when the file is parsed:

```yaml
scopes:
  - name: project-scope
    scope:
      project:          # id, name and optionally the domain of the project
        id: 0123abcd
        domain:
          name: Default
  - name: domain-scope
    scope:
      domain:           # id and/or name
        name: Default
  - name: system-scope
    scope:
      system: all
```

## Secrets

Same as the kubernetes `kubeconfig` (which embeds client certificates and
keys) the file may contain long-lived secrets: `password`, `client_secret`,
`application_credential_secret` or any other secret declared by the
authentication plugin. This keeps the format usable by other tools (i.e.
`openstacksdk`, `gophercloud`) without any extra machinery.

A file with secrets must be protected like `clouds.yaml`/`secure.yaml`: keep it
readable by the owner only (`chmod 600`). When the file contains secrets and is
accessible by group or others, a warning is logged and printed to the standard
error on load. A file which names an `auth_helper` and is writable by group or
others is **refused** (an error, or, when the context is used implicitly, a
warning on the standard error and the fallback to `clouds.yaml`), since anybody
able to edit it could make the tool run an arbitrary command. Make it writable
by the owner only (`chmod 600`).

Short-lived credentials (`token`, `access_token`, `passcode`, `jwt`) are not
supported at all and loading fails when a user sets one.

As an alternative the secrets can be left out and requested at the
authentication time from the external command configured in the `auth_helper`
of the user (the same mechanism as the `--auth-helper-cmd` of the `osc`). The
command is invoked with the name of the requested attribute and the context
name and prints the value on the standard output, which allows using `pass`, OS
keyrings, `vault` or similar tools. A non-zero exit status or an empty output is
a failure: on a terminal `osc` and `ostui` then ask the user for the value (this
differs from `--auth-helper-cmd`, whose output is used as is). A file without
secrets is safe to keep in a VCS or to share.

## Usage

Same as `kubectl`, the `current-context` of `osc` is used when the cloud is not
specified: if no option selects or overrides the cloud connection (see below)
and the file exists and sets `current-context`, that context is used. Otherwise
`clouds.yaml` is used as before. A `contexts.yaml` that cannot be
loaded is ignored in this case, with a warning printed to the standard error.
`ostui` does not use the `current-context` implicitly and needs `--os-context`.

```console
osc compute server list   # uses `current-context` of the contexts.yaml, if any
osc --os-cloud devstack compute server list   # uses clouds.yaml
```

The `--os-context-file` option (or the `OS_CONTEXT_FILE` environment variable) points to
the file to use instead of searching the default locations, same as `--kubeconfig`
of `kubectl`. The file must exist (it is an error otherwise) and its `current-context`
is used the same way. The option does not select the connection by itself.

```console
osc --os-context-file ./my-contexts.yaml --os-context prod-admin compute server list
```

Another context is selected with the `--os-context` option (or the `OS_CONTEXT`
environment variable):

```console
osc --os-context prod-admin compute server list
ostui --os-context prod-admin
```

The `--os-context` option conflicts with the options which select or override
the cloud connection: `--os-cloud`, `--cloud-config-from-env`,
`--os-cloud-name`, `--os-project-id`, `--os-project-name` and
`--os-region-name` (`ostui` only has the first three). Two of them given on the
command line are an error. A value coming from an environment variable always
loses against the command line: with `OS_CLOUD` exported (i.e. by an openrc
file) `--os-context` still selects the context and the environment variable is
ignored, and with `OS_CONTEXT` exported `--os-cloud` selects the cloud. The
same options, as well as `--os-client-config-file` and
`--os-client-secure-file`, turn off the implicit use of the `current-context`.
The region is defined by the `region_name` of the context. When the context comes
from an environment variable (`OS_CONTEXT`) and anything else selects the cloud,
the context is ignored and a warning is printed to the standard error.

When no secret is stored in the file, the `auth_helper` of the user is used to
get it, unless `--auth-helper-cmd` is given (`osc`). `ostui` is limited to the
selected context, same as with `--cloud-config-from-env`.

## Validation

- names are unique within a kind
- a context may only refer to existing servers, users and scopes
- `current-context` must refer to an existing context
- a scope is exactly one of project, domain or system; a user must not add scope attributes of another kind
- user attributes must not duplicate what server or scope define, nor set server or context level attributes (`region_name`, `interface`, `verify`, `cacert`, `profile`, `auth_cache`, `endpoint`)
- a scope must identify a project, domain or system (`id` or `name`)
- `auth_helper` must be an absolute path of an executable (no arguments)
- explicit `null` values are treated as not set
