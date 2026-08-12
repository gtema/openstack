# Plugin author guide

This page describes the guest ABI a WASM auth plugin must implement, and how
to build, test, and publish one. For the operator-facing view (installing,
trust model), see [WASM Auth Plugins](../plugins.md).

A plugin is a single `.wasm` module targeting `wasm32-unknown-unknown`,
built against [Extism's PDK](https://extism.org/docs/quickstart/plugin-quickstart)
(`extism-pdk` for Rust, or any other language the PDK supports — the ABI is
just exported functions taking and returning strings, so it isn't Rust-only).
It runs with no filesystem, no environment variables, no WASI, 16 MiB of
linear memory, and a 20-second per-call timeout. It cannot open a socket or
resolve DNS itself; the only way it ever touches the network is the
host-mediated capability described below.

## Common exports (every plugin)

Every conforming module exports these four functions regardless of ABI
flavor:

| Export | Signature | Purpose |
| --- | --- | --- |
| `plugin_abi_version` | `(_: string) -> string` | Must return the literal `"1"`. This is the only ABI version `osc` currently understands; a mismatch is rejected at load time before anything else runs. |
| `auth_supported_methods` | `(_: string) -> string` | JSON array of the `auth_type` name(s) this plugin implements, e.g. `["my_corp_sso"]`. Must be non-empty. |
| `auth_api_version` | `(_: string) -> string` | JSON `[major, minor]` pair, informational. |
| `auth_requirements` | `(hints: string) -> string` | `hints` is a JSON value or the literal `null`. Returns a JSON Schema object describing the fields this auth method needs from the user, in the same shape `OpenStackAuthType::requirements` produces for compiled-in auth types. |

At load time `osc` calls these four in order, validates the answers, and
then inspects which of the two ABI flavors below the module additionally
exports (via `extism::Plugin::function_exists`) — a module must implement
**exactly one**; exporting both, or neither, is rejected.

## The `auth` ABI flavor

For non-interactive auth methods (token exchange, credential-based flows,
anything that doesn't need a browser).

- **`auth(request: string) -> string`** — `request` is a JSON object:

  ```json
  {"identity_url": "...", "values": {...}, "scope": {...}, "hints": {...}}
  ```

  Returns either:

  ```json
  {"ok": {"token": "...", "auth_info": null}}
  ```

  or:

  ```json
  {"error": "human readable message"}
  ```

  This is the only export in this flavor, and it's also the only place a
  plugin may perform outbound HTTP — and only through the host-provided
  `identity_http_request` import, never directly (there is no direct-socket
  capability to use even if you wanted to).

### `identity_http_request`

The one host function available to `auth`-flavor plugins. It proxies a
request to the identity endpoint `osc` already resolved for the configured
cloud — never to a URL the plugin picks itself:

```json
// guest -> host
{"method": "POST", "path": "/v3/auth/tokens", "headers": {...}, "body": "..."}
```

```json
// host -> guest
{"status": 201, "headers": {...}, "body": "..."}
```

`path` must be relative (start with `/`) and is resolved against the bound
identity origin (scheme + host + port only) — there is no field for
supplying a different host, so a plugin structurally cannot be redirected
into calling anything other than the identity endpoint it was invoked for.

## The `sso` ABI flavor

For interactive, browser-based (WebSSO-style) auth methods. Both exports
are pure functions: no I/O capability is available to this flavor at all.
The host owns the local callback listener, the anti-CSRF `state` check, and
the actual browser-launch step — the guest only ever computes strings from
strings.

- **`sso_build_request(request: string) -> string`** — `request` is:

  ```json
  {"identity_url": "...", "callback_url": "...", "values": {...}, "scope": {...}, "hints": {...}}
  ```

  `callback_url` is the host-bound local callback URL, with the anti-CSRF
  `state` token already embedded — the plugin doesn't generate or see the
  raw CSRF secret, it just has to make sure the identity provider redirects
  back to this exact URL. Returns:

  ```json
  {"url": "https://idp.example.com/authorize?...", "redirect_host": "127.0.0.1:PORT"}
  ```

  `url` is the page the host will open in the user's browser after showing
  it to the user for confirmation. `redirect_host` is the `host:port`
  authority the plugin configured as the identity provider's redirect
  target. Before opening anything, the host independently verifies that
  `url`'s scheme is `https` and that `redirect_host` exactly matches the
  authority of the callback listener it itself bound — a plugin that
  returns a URL pointing at a different, undeclared redirect host is
  rejected outright, with **no override**, since this is the one place a
  compromised plugin could otherwise redirect a real user's browser
  somewhere attacker-controlled.

- **`sso_parse_callback(callback: string) -> string`** — `callback` is:

  ```json
  {"params": {"code": "...", "state": "..."}}
  ```

  the form fields from the callback POST, handed to the guest only *after*
  the host has already validated the `state` token itself. Returns the same
  `{"ok": ...} | {"error": ...}` shape as `auth`.

## Building

```console
$ rustup target add wasm32-unknown-unknown
$ cargo build --release --target wasm32-unknown-unknown
```

Using `extism-pdk`, the common exports and one ABI flavor's exports are
ordinary `#[plugin_fn]`-annotated functions returning `FnResult<String>` (or
a PDK JSON-typed wrapper) — see the
[Extism Rust PDK docs](https://github.com/extism/rust-pdk) for the exact
macro shape. There's nothing `osc`-specific about the build step beyond the
export names and JSON shapes above.

## Testing locally

Before publishing, install straight from the built artifact with
`--allow-unsigned` (a local file has no attestation to check, so this is
always required — see [Trust model](../plugins.md#trust-model)):

```console
$ osc plugin install --file ./target/wasm32-unknown-unknown/release/my_plugin.wasm --allow-unsigned
$ osc plugin list
$ osc --os-auth-type my_corp_sso ... # exercise it
```

`osc plugin verify <name>` re-checks the installed file's SHA-256 against
the lockfile at any point, useful for confirming nothing got corrupted
during iteration.

## Publishing

1. In your own repository's CI, build the release artifact and attest it
   with [`actions/attest`](https://github.com/actions/attest) — the same
   action this repository's own release workflow uses
   (`.github/workflows/release.yml`). This is what lets `osc` later prove
   the `.wasm` file it downloaded actually came from a build in your
   repository, not from an attacker who compromised the registry index.
2. Publish the artifact (a GitHub release asset is the natural choice) and
   compute its SHA-256.
3. Open a pull request against this repository adding your plugin's
   `name`/`versions[]` entry to `plugins/registry/index.json`. See
   [Registry governance](./registry-governance.md) for what reviewers check
   before merging.

Once merged, `osc plugin search`/`install <name>` picks it up from the
default registry immediately — there's no separate publish step on the
`osc` side.
