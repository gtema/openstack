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

  This is the only export in this flavor. Like `sso` below, it may perform
  outbound HTTP only through the host-provided `identity_http_request`
  import, never directly (there is no direct-socket capability to use even
  if you wanted to).

### `identity_http_request`

The host function available to both `auth`- and `sso`-flavor plugins. It
proxies a request to the identity endpoint `osc` already resolved for the
configured cloud — never to a URL the plugin picks itself:

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

For interactive, browser-based (WebSSO-style) auth methods. The host owns
the local callback listener, the anti-CSRF `state` check, and the actual
browser-launch step — the guest never gets a socket, DNS resolver, or
browser-opening capability of its own. Both exports may, like `auth`,
perform outbound HTTP through the host-provided `identity_http_request`
import described above, restricted to the same identity endpoint `osc`
resolved for the configured cloud.

- **`sso_build_request(request: string) -> string`** — `request` is:

  ```json
  {"identity_url": "...", "callback_url": "...", "values": {...}, "scope": {...}, "hints": {...}, "code_challenge": "...", "code_challenge_method": "S256", "nonce": "..."}
  ```

  `callback_url` is the host-bound local callback URL, with the anti-CSRF
  `state` token already embedded — the plugin doesn't generate or see the
  raw CSRF secret, it just has to make sure the identity provider redirects
  back to this exact URL. `code_challenge`/`code_challenge_method` are a
  host-generated RFC 7636 PKCE pair (`code_challenge_method` is always the
  literal `"S256"`): the wasm guest sandbox has no secure RNG, so the host
  generates these and hands them in — a plugin protecting its
  authorize→token exchange with PKCE should embed both as query parameters
  in the authorize URL it returns. A plugin that doesn't use PKCE can
  ignore these two fields entirely; they're always present but never
  required. `nonce` is a host-generated OIDC nonce, for the same reason
  (no guest-side secure RNG) — a plugin should embed it as a query
  parameter in the authorize URL too, then, after exchanging the
  authorization code, compare it against the `nonce` claim of any
  `id_token` it receives (defense against a stolen/replayed `id_token`
  from a previous flow being injected into this one). Unlike
  `code_verifier`, `nonce` isn't secret — it's only ever handed over once,
  in this request — so a plugin that wants to validate it must carry the
  value forward itself (e.g. via Extism's `var_get`/`var_set`) across the
  gap until `sso_parse_callback`. Validation is entirely optional and
  guest-side; the host has no visibility into the `id_token`'s contents to
  check it itself. Returns:

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
  {"params": {"code": "..."}, "code_verifier": "..."}
  ```

  `params` are the form fields from the callback POST, handed to the guest
  only *after* the host has already validated and stripped the `state`
  token itself — `state` is read from the callback URL's own query string,
  not the POST body, so it never appears in `params`.
  `code_verifier` is the same value whose SHA256 the host told the guest to
  commit to as `code_challenge` in `sso_build_request` — if the plugin used
  PKCE there, it sends `code_verifier` back to the identity provider's
  token endpoint here to complete the exchange. The guest only ever sees
  `code_verifier` at this one point; it's never available earlier and never
  needs to be persisted across calls. Returns the same
  `{"ok": ...} | {"error": ...}` shape as `auth`.

### `idp_http_request`

A second host function, available **only** during `sso_parse_callback` —
not during `sso_build_request`, since the trusted origin doesn't exist yet
at that point. Same request/response JSON shape as `identity_http_request`:

```json
// guest -> host
{"method": "POST", "path": "/token", "headers": {...}, "body": "..."}
```

```json
// host -> guest
{"status": 200, "headers": {...}, "body": "..."}
```

The origin it's restricted to is whatever `https://` URL your own
`sso_build_request` returned in its `url` field, after the host has
verified that origin resolves to a public (non-SSRF-denylisted — no
loopback, link-local, private, multicast/reserved, or unspecified address)
address. This is what lets a plugin exchange an authorization code from the
callback directly against the external identity provider's own token
endpoint (Keycloak, Okta, etc.) — a different origin than Keystone, which
`identity_http_request` structurally cannot reach.

The user is told about this capability up front: the confirmation prompt
shown before the browser opens discloses both the browser-navigation
destination and the background-request origin.

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
