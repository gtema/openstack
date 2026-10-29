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
      region_name: RegionOne
users:
  - name: alice
    user:
      username: alice
      user_domain_name: Default
      auth_helper: /usr/local/bin/osc-pass
scopes:
  - name: admin
    scope:
      project_name: admin
      project_domain_name: Default
contexts:
  - name: prod-admin
    context:
      server: prod
      user: alice
      scope: admin
```

The file is searched as `contexts.{yaml,yml,json}` in the same locations as
`clouds.yaml`. It is **not** merged with `clouds.yaml` or `secure.yaml`: a
connection is selected either by a cloud name or by a context name.

## Secrets

The file must not contain secrets. Loading fails when a user sets `password`,
`client_secret`, `application_credential_secret` or any other secret declared by
the authentication plugin. Short-lived credentials (`token`, `access_token`,
`passcode`, `jwt`) are not supported at all.

Secrets are requested at the authentication time from the external command
configured in the `auth_helper` of the user (the same mechanism as the
`--auth-helper-cmd` of the `osc`). The command is invoked with the name of the
requested attribute and the context name and prints the value on the standard
output, which allows using `pass`, OS keyrings, `vault` or similar tools. Since
the file has no secrets it is safe to keep it in a VCS or share it.

## Validation

- names are unique within a kind
- a context may only refer to existing servers, users and scopes
- `current-context` must refer to an existing context
- a scope is either project, domain or system
- user attributes must not duplicate what server or scope define
