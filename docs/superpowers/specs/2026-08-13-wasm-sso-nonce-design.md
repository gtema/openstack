# WASM `sso` flavor: host-generated nonce

## Problem

Designing the out-of-tree `s11auth` plugin (sub-project B) surfaced a second
gap alongside [[2026-08-13-wasm-sso-pkce-design]]: the Python plugin being
ported generates an OIDC `nonce` and sends it in the authorize request, to
be echoed back in the returned `id_token`'s `nonce` claim and checked on
receipt — a defense against a stolen/replayed `id_token` from a previous
flow being injected into this one. (The Python plugin actually generates
this value but never validates it on return; that's a latent bug there, not
a design choice worth preserving.) Same root problem as PKCE's
`code_verifier`: the wasm guest sandbox has no secure RNG, so a guest
cannot generate an unpredictable `nonce` itself.

## Change

The host generates the `nonce` in `CallbackServer::bind`, alongside the
`state` token and PKCE pair it already generates there, and threads it
through the existing `sso_build_request` request JSON — one more additive
field, same mechanism as PKCE.

### Generation

`sdk/websso-host/src/lib.rs`: a new `generate_nonce` function, mirroring
the existing `generate_state` exactly (same `ring::rand::SystemRandom`,
same hex-encoding approach) — 16 random bytes, hex-encoded to a 32-character
string. `CallbackServer` stores the result alongside `state`/`code_verifier`/
`code_challenge`, and exposes:

- `nonce(&self) -> &str` — safe to read multiple times, same visibility
  pattern as `code_challenge`.

### Threading through `plugin.rs` — simpler than PKCE

Unlike `code_verifier`, `nonce` is not secret: the guest embeds it directly
in the browser-visible authorize URL itself, so there is nothing to protect
by withholding it until later. The host only needs to hand it over once:

- `SsoBuildRequestMsg` gains one field:

  ```json
  {"nonce": "..."}
  ```

  populated from `server.nonce()` before the existing `sso_build_request`
  call — same call site as the PKCE `code_challenge`/`code_challenge_method`
  fields added in the prior spec.

- `SsoCallbackMsg` is **unchanged**. A guest that wants to validate the
  nonce already has the value from its own `sso_build_request` call and can
  carry it forward itself (via Extism's `var_get`/`var_set`) if it chooses
  to — this is fine here specifically because the value isn't confidential,
  unlike `code_verifier`, which the host deliberately withholds until
  `sso_parse_callback` for that reason.

As with the PKCE fields, this is backward compatible: a guest struct that
doesn't declare a `nonce` field simply ignores it via serde's default
unknown-field behavior.

### Guest-side validation (not a host concern)

Purely guest-side logic, needing no new host capability — noted here for
completeness since it's the whole point of generating the value, but it's
the guest's decision whether to do it:

1. Embed `nonce` as a query parameter in the authorize URL returned from
   `sso_build_request`.
2. After exchanging the authorization code (via `idp_http_request`),
   base64url-decode the middle segment of the returned `id_token` JWT (no
   signature verification needed — the token arrived over a channel
   `idp_http_request` already SSRF-checked and origin-pinned to the IdP
   `sso_build_request` itself declared).
3. Compare the decoded payload's `nonce` claim against the value the guest
   embedded in step 1; reject the flow on mismatch.

The `example-sso-plugin` fixture demonstrates this (see Testing below) so
the mechanism is exercised end-to-end, but nothing in the ABI *requires* a
guest to validate — same non-intrusive stance as PKCE.

### ABI version

Stays `"1"`. Additive.

### Docs

- `doc/src/plugins/author-guide.md`: document the new `nonce` field on
  `sso_build_request`'s request JSON, same bullet-list style as the
  existing fields there, noting it's for guest-side replay protection and
  validation is optional.
- Matching update to `plugin.rs`'s module-level `## Guest ABI (version 1)`
  doc comment.

### Testing

- `sdk/websso-host/src/lib.rs`: unit tests on `generate_nonce` —
  - Returned string is 32 hex characters.
  - Two calls produce different values (uniqueness; RNG is actually
    exercised).
- `sdk/plugin-wasm/fixtures/example-sso-plugin/src/lib.rs`: extend the
  existing PKCE-round-trip fixture mode so `sso_build_request` also embeds
  `nonce` into the returned authorize URL's query string, and the
  token-exchange response's mock `id_token` (already synthesized by the
  mock IdP mode from the PKCE work) carries a `nonce` claim matching what
  was sent — `sso_parse_callback` decodes it and asserts the match,
  returning an error result if they don't.
- `sdk/plugin-wasm/tests/wasm_sso_plugin.rs`: extend the existing PKCE
  end-to-end test (or add one alongside it) asserting the full
  `sso_build_request` → `sso_parse_callback` path succeeds when the nonce
  round-trips correctly, and a second case asserting `sso_parse_callback`
  returns an `{"error": ...}` result (not a host-level failure) when the
  fixture is driven with a mismatched nonce claim — proving validation is
  guest-side, recoverable logic, not a host-enforced rejection.

## Out of scope

- The out-of-tree `s11auth` wasm plugin itself (sub-project B, designed
  next once this lands).
- Any change to `identity_http_request` or `idp_http_request`.
- Signature verification of the `id_token` JWT. Out of scope for the same
  reason it's out of scope for the PKCE work's token exchange generally:
  the channel is already origin-pinned and SSRF-checked; full JWS
  signature verification would need a JWK-fetching capability and crypto
  library this ABI increment has no motivating need for yet.
- Host-side nonce validation. The host has no visibility into the
  `id_token`'s contents (it's opaque bytes passed through
  `idp_http_request`'s response body); validation is necessarily guest-side.
