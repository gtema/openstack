# WASM `sso` flavor: IdP-origin HTTP capability (`idp_http_request`)

## Problem

[[2026-08-12-wasm-sso-identity-io-design]] wired `identity_http_request` into
both `sso` exports, letting a plugin exchange a callback's contents for a
token via the identity endpoint (Keystone). That's not enough for a full
authorization-code-with-PKCE OIDC flow: the callback carries a `code`, and
turning that into an `id_token` requires a POST to the external IdP's own
token endpoint (Keycloak, in the motivating `s11auth` case) — a different
origin than Keystone, which `identity_http_request` structurally cannot
reach (by design; that restriction is unchanged and out of scope here).

## Change

Add a second host function, `idp_http_request`, available only during
`sso_parse_callback`, scoped to the origin `sso_build_request` already
returned in its `url` field — the same origin the host is about to (or just
did) open the user's real browser at. Same request/response JSON shape as
`identity_http_request` (`{"method","path","headers","body"}` in,
`{"status","headers","body"}` out); the guest still only ever supplies a
request-relative `path`, never a full URL.

### Why this needed a security pass, not just plumbing

The naive framing — "the host already trusts this origin enough to open a
browser there, so a background request to the same origin adds no new
boundary" — undersells two real gaps, found by review before this spec was
written:

1. **`url`'s host is validated for `https` scheme only, nothing else.**
   Unlike `identity_origin` (host-resolved from the user's own cloud config,
   never guest-influenced), `idp_origin` is entirely guest-chosen. Without
   further restriction, `idp_http_request` is a general-purpose HTTPS relay
   running inside the `osc` process itself, reachable by any installed
   plugin, to any origin the plugin's own code picks — classic SSRF shape.
   Wherever `osc` runs (a CI runner, a bastion host, anything with network
   reach a user's own interactive browser wouldn't have — cloud metadata
   services included) becomes reachable through this primitive if left
   unfiltered.
2. **Silent, unbounded exfiltration channel.** `sso_build_request` sees
   `values`/`scope`/`hints` (may carry secrets), and Extism's
   `var_get`/`var_set` lets a plugin carry that state over to
   `sso_parse_callback` in the same instance. Today the only leak path is
   embedding data in the `url` query string — size-bounded, and shown to the
   user in the "opening browser at `{url}`" confirmation before anything
   happens. An unrestricted `idp_http_request` removes both constraints:
   arbitrary method/headers/body, no size cap, no confirmation, firing after
   the user already agreed to something that looked like plain navigation.

Both gaps get closed as hard requirements below, not left as follow-ups.

### Origin resolution and caching

- After `validate_sso_build_response` accepts `sso_url` (existing `https`
  scheme + `redirect_host`-match checks, unchanged), the host additionally
  resolves `sso_url`'s host to its concrete IP address(es) and validates
  them against an SSRF denylist (see below) — resolution happens
  host-side, not guest-side, so the check runs against the address that
  will actually be connected to, not a hostname string a DNS answer could
  later change (closes the DNS-rebinding gap: checking the hostname pattern
  alone isn't enough, since the name can resolve to a safe address at
  validation time and an unsafe one at request time — pin the resolved
  address itself, or re-resolve and re-check immediately before each
  `idp_http_request` call within the same short-lived guest call).
- On success, `idp_origin = scheme+host+port` from `sso_url` is cached in
  `HostContextState`, alongside `identity_origin`.
- `call_guest` gains an `idp_origin: Option<url::Url>` parameter: `None` for
  the `sso_build_request` call (no trusted IdP origin exists yet — the
  whole point is that origin comes *from* that call's own response), and
  `Some(idp_origin)` for the `sso_parse_callback` call. Every `call_guest`
  invocation explicitly sets `state.idp_origin` (including explicitly
  clearing it to `None` on the build-request call) so no stale origin from
  a prior auth attempt or prior plugin instance can leak forward.

### SSRF denylist

Reject the resolved address if it falls in any of:

- IPv4: loopback (`127.0.0.0/8`), link-local (`169.254.0.0/16`, which
  covers the `169.254.169.254` cloud metadata address), private
  (`10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`), multicast/reserved
  (`224.0.0.0/4`, `240.0.0.0/4`), unspecified (`0.0.0.0/8`).
- IPv6: loopback (`::1`), unique-local (`fc00::/7`), link-local
  (`fe80::/10`), multicast (`ff00::/8`), unspecified (`::`), and IPv4-mapped
  addresses (`::ffff:0:0/96`) evaluated against the IPv4 list above after
  unwrapping.
- If a hostname resolves to *any* address in the denylist, reject the whole
  request — don't pick-and-choose among multiple resolved addresses.

Rejection surfaces as the same `WasmPluginError::InvalidRedirect` variant
`validate_sso_build_response` already uses for scheme/redirect-host
failures — this is conceptually the same guest-response validation step,
just one more check on the same `url`.

### Confirmation prompt disclosure

The existing `Confirm::new().with_prompt(...)` before opening the browser
(`auth_via_sso`) gets an added line disclosing the background-request
capability explicitly, once `idp_origin` has passed the SSRF check:

```
A default browser is going to be opened at `{sso_url}`.
This plugin may also make background network requests to `{idp_origin}`.
Do you want to continue?
```

Both lines show the same origin in the common case (`idp_origin` is derived
from `sso_url`), so this mostly reads as making an implicit capability
explicit rather than surfacing a surprising second destination — but it's
still a materially different consent than "will navigate my browser," and
should be named as such rather than left implicit.

### `host.rs`

- Extract `resolve_request` and the request-building/response-shaping body
  of `identity_http_request` into a shared, origin-parameterized helper so
  `idp_http_request` doesn't duplicate the proxy logic. `HttpRequestMsg`/
  `HttpResponseMsg` are reused as-is (no shape changes).
- `HostContextState` gains `idp_origin: Option<url::Url>` alongside
  `identity_origin`/`client`.
- New `idp_http_request` host_fn macro entry, same shape as the existing
  one, reading `idp_origin` instead of `identity_origin`; errors cleanly
  (`"idp_http_request: no IdP endpoint bound to this call"`) when unset —
  which is always the case during `sso_build_request` and during any
  `auth`-flavor call, so an `auth`-flavor plugin or a `sso` plugin that
  never imports this function is completely unaffected.

### `plugin.rs::load()`

Register the second `Function::new("idp_http_request", ...)` alongside the
existing `identity_http_request` registration, same `host_ctx`.

### ABI version

Stays `"1"`. Additive: invisible to any plugin that doesn't import
`idp_http_request`.

### Docs

- `doc/src/plugins/author-guide.md`: new paragraph under the `sso` flavor
  section describing `idp_http_request` — availability restricted to
  `sso_parse_callback`, origin pinned to `sso_build_request`'s returned
  `url` (after the same `https` check plus the new SSRF-address check),
  and that the confirmation prompt discloses this to the user.
- Matching update to `plugin.rs`'s module-level `## Guest ABI (version 1)`
  doc comment.

### Testing

- SSRF denylist: unit tests over the resolved-address-checking function
  directly (not requiring real DNS) — at minimum one address from each
  denylisted range above, plus one public address that must pass, plus an
  IPv4-mapped IPv6 address that must be caught via the unwrapped check.
- `example-sso-plugin` fixture: extend with a mode where `sso_parse_callback`
  calls `idp_http_request` against a mock IdP server (POST `code`, read back
  a `token` field), alongside existing modes.
- `wasm_sso_plugin.rs`: new cases —
  - happy path: `sso_parse_callback` reaches the mock IdP via
    `idp_http_request` and the response drives the resulting
    `Auth::AuthToken`.
  - `idp_http_request` called during `sso_build_request` (via
    `call_guest_for_test` with `idp_origin: None`) errors instead of
    silently reaching anything.
  - `sso_build_request` returning a `url` whose host resolves to a
    denylisted address is rejected before any confirmation prompt or
    browser-open — regression-shaped the same way the existing
    `bad_scheme_is_rejected_before_any_prompt` /
    `undeclared_redirect_host_is_rejected_before_any_prompt` tests are.
- No changes needed to the existing `identity_http_request` fuzz target;
  a new fuzz target for the SSRF address-check function is worth adding
  given it's parsing/deciding on attacker-influenced-shaped input (a
  hostname/IP a malicious plugin fully controls), but is not required to
  land this change — flagged as a nice-to-have, not blocking.

## Out of scope

- The out-of-tree `s11auth` wasm plugin itself (sub-project B, after this
  lands).
- Any change to `identity_http_request`'s destination restriction or its
  host-resolved (not guest-chosen) trust model.
- Response body size caps and explicit `reqwest` request timeouts for
  either host function — pre-existing gaps shared by `identity_http_request`
  today, not introduced by this change; worth a follow-up but not bundled
  here to keep this spec's diff reviewable as one thing.
- Logging/auditing of host-mediated plugin HTTP calls — the confirmation
  prompt disclosure above is the only new user-visibility mechanism this
  spec adds; a fuller audit trail is a separate, optional enhancement.
