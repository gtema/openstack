# `s11auth` out-of-tree wasm plugin (sub-project B)

## Problem

Port the Python `s11auth` keystoneauth1 plugin (browser-based OIDC-via-Keycloak
auth against Keystone, source at
`gitlab.syseleven.de/ncs/iam/openstack-s11-auth/s11auth/plugin.py`) into an
out-of-tree wasm auth plugin conforming to this repo's plugin ABI v1. The
in-tree ABI surface it depends on is now complete: `identity_http_request`,
`idp_http_request`, the host-generated PKCE pair, and the host-generated
`nonce` ([[2026-08-13-wasm-sso-pkce-design]],
[[2026-08-13-wasm-sso-nonce-design]]).

This design covers the plugin's own logic only. It lives in a separate
repository (bootstrap deferred — out of scope here, see below) and this
change makes **zero modifications to `gtema/openstack`**.

## Architecture

A single `sso`-flavor wasm plugin, one crate, structured like this repo's
`example-sso-plugin` fixture: the four common exports
(`plugin_abi_version`/`auth_supported_methods`/`auth_api_version`/
`auth_requirements`) plus `sso_build_request`/`sso_parse_callback`, and the
two `#[host_fn]` imports (`identity_http_request`, `idp_http_request`). No
`auth`-flavor export — the Python original is exclusively browser-based, so
there's nothing non-interactive to port.

`auth_type` name: `s11auth`, matching the Python plugin's existing name (drop-in
replacement, not a rename).

## Config (`auth_requirements`)

JSON Schema, all fields optional:

- `oidc_endpoint` (string) — default
  `https://idp.apis.syseleven.de/realms/application/protocol/openid-connect`,
  same default as the Python original's `DEFAULT_OIDC_ENDPOINT`.
- `client_id` (string) — default `s11-user`, same as `DEFAULT_CLIENT_ID`.
- `redirect_port` (string/integer) — default `8080`. Fixed port, not
  ephemeral: the Keycloak client registration for `s11-user` is assumed
  pinned to this specific redirect URI (matches the Python original's
  `--os-redirect-port` default), so an OS-assigned ephemeral port would
  likely be rejected by the IdP's redirect_uri allowlist.
- `project_id` — handled through osc's normal `scope` mechanism, not a
  bespoke field; no plugin-side format validation (see below).

These replace the Python original's `OIDC_ENDPOINT`/`CLIENT_ID` environment
variables with osc's normal config surface (CLI flags / `clouds.yaml`).

## `sso_build_request`

Reads `oidc_endpoint`/`client_id` out of `values`. Builds the authorize URL:

```
{oidc_endpoint}/auth?client_id={client_id}&redirect_uri={callback_url}
  &response_type=code&response_mode=form_post&scope=openid
  &code_challenge={code_challenge}&code_challenge_method=S256&nonce={nonce}
```

`code_challenge`/`nonce` come straight from the request JSON (host-generated,
per the two prior specs). No plugin-generated `state` parameter: the ABI's
`callback_url` already carries the host's own anti-CSRF token embedded in its
query string, and the IdP echoes that back verbatim as part of the exact
redirect_uri it POSTs to — this makes the Python original's separate
`state = secrets.token_urlsafe()` redundant. Dropped, not ported.

Before returning, the plugin stashes `oidc_endpoint`, `client_id`, and
`callback_url` via Extism's `var_set` — alongside `nonce`, which the same
mechanism already carries across the build→callback gap per the nonce spec.
This is necessary because `sso_parse_callback`'s input carries only
`{params, code_verifier}`, no config passthrough: the plugin has to remember
its own configuration to build the token-exchange request later. None of
these three values are secret, so this carries no new exposure beyond what
the nonce design already established.

Returns `{"url": authorize_url, "redirect_host": <authority of callback_url>}`.

## `sso_parse_callback`

1. `params["code"]` is POSTed via `idp_http_request` to `{oidc_endpoint}/token`,
   form-encoded: `grant_type=authorization_code`, `client_id`, `code`,
   `redirect_uri=<stashed callback_url>`, `code_verifier` (from the callback
   message). A non-2xx response, or a response missing `id_token`, becomes
   `{"error": "..."}`.
2. The `id_token` JWT's payload is base64url-decoded (no signature
   verification — same stance as the PKCE/nonce specs: the channel is
   already origin-pinned and SSRF-checked by `idp_http_request`). Its
   `nonce` claim is compared against the stashed `nonce`; mismatch returns
   `{"error": "id_token nonce did not match"}` (or equivalent message) —
   guest-side, recoverable, not a host-level rejection.
3. On match, the `id_token` is POSTed via `identity_http_request` to
   `/auth/tokens`:

   ```json
   {"auth": {"identity": {"methods": ["s11auth"], "s11auth": {"token": "<id_token>"}}}}
   ```

   with `scope.project.id` added when a project scope was configured. This
   assumes the target deployment's Keystone still runs the custom `s11auth`
   identity method accepting a raw OIDC id_token under `methods: ["s11auth"]`
   — same payload shape as the Python original, unchanged.
4. A non-2xx response, or a response missing a token, becomes
   `{"error": "..."}`. Success returns
   `{"ok": {"token": "<keystone token>", "auth_info": null}}`.

## Dropped from the Python original

- **`state` generation** — superseded by the ABI's own host-managed
  anti-CSRF token (see above).
- **`project_id` regex validation** (`^[a-z0-9-]{32}$` — a Python-side bug:
  the regex permits dashes despite the accompanying error message claiming
  it doesn't). No client-side check at all; an invalid `project_id` fails at
  Keystone's `/auth/tokens` response instead, which is authoritative anyway.
- **Plugin-side JWT disk cache** (`~/.config/openstack-s11auth/auth`) — the
  Python original's dead-simple file cache is superseded by osc SDK's
  existing token/session cache (`sdk/core/src/session.rs`,
  `sdk/core/src/state.rs`); no plugin-side caching needed.
- **`nonce` field generated but never validated** — the Python original's
  latent bug. This port validates it for real (step 2 above).

## Error handling

Every failure path returns `{"error": "..."}` — guest-recoverable, never a
host-level panic. No case in this design needs to fail the call itself
(distinct from a returned error result); the plugin never has a reason to
return a Rust-level `Err` from either export under normal operation.

## Out of scope

- Repository bootstrap, CI/attestation setup (`actions/attest`, matching
  this repo's `release.yml` pattern), and registry publication
  (`plugins/registry/index.json` entry) — deferred to when implementation
  actually starts, per the earlier "design now, repo later" decision.
- Any change to `gtema/openstack` itself. This sub-project ships zero diffs
  here.
- `id_token` JWS signature verification — out of scope for the same reason
  it's out of scope in the PKCE/nonce specs.
- Testing infrastructure specifics (mock Keycloak, etc.) — belongs in the
  new repo's own plan once bootstrapped; this spec only notes that the
  validation approach should mirror `wasm_sso_plugin.rs`'s
  `sso_build_request`→`sso_parse_callback` round-trip pattern against a
  mock IdP, the same shape `example-sso-plugin`'s PKCE/nonce fixture modes
  already exercise in this repo.
