# WASM `sso` flavor: host-generated PKCE pair

## Problem

[[2026-08-13-wasm-idp-http-request-design]] added `idp_http_request`, closing
the code→id_token exchange gap for the motivating `s11auth` case. That plugin
also protects its authorize→token exchange with PKCE (RFC 7636): a
`code_verifier` generated client-side, and its SHA256-derived
`code_challenge` sent in the authorize URL. The wasm guest sandbox has no
secure RNG — `extism-pdk` 1.4 exposes none, and `wasm32-unknown-unknown`
has no OS entropy source a guest could reach even if it tried — so a guest
cannot generate a real `code_verifier` itself. Skipping PKCE, or faking it
with guest-side pseudo-randomness, was considered and rejected: PKCE's
protection depends on the verifier being genuinely unpredictable.

## Change

The host generates the PKCE pair and threads it through the existing
`sso_build_request` / `sso_parse_callback` request JSON, the same way it
already threads `callback_url`'s embedded `state` token. No new host
function — this is pure JSON-field plumbing, smaller in scope than
`idp_http_request`.

### Generation

In `sdk/websso-host/src/lib.rs`, `CallbackServer::bind` generates a PKCE
pair alongside the `state` token it already generates there, using the same
crypto primitives already in that crate's dependency tree:

- `code_verifier`: 32 random bytes via `ring::rand::SystemRandom` (the same
  RNG `generate_state` already uses), base64url-no-pad encoded (via the
  `base64` crate — already a workspace dependency, used elsewhere in this
  crate family, e.g. `sdk/plugin-wasm/Cargo.toml`) → a 43-character string.
  This satisfies RFC 7636's `code_verifier` requirements directly: 43-128
  characters, drawn from `[A-Za-z0-9-._~]`, which is exactly what unpadded
  base64url produces.
- `code_challenge`: `base64url-no-pad(SHA256(ascii_bytes(code_verifier)))`,
  computed via `ring::digest::{digest, SHA256}` (`ring` is already a
  dependency; no new crate needed for hashing).
- `code_challenge_method`: always the literal `"S256"` — RFC 7636's plain
  method is not offered; there's no reason to support the weaker variant.

`CallbackServer` stores both alongside `state` and exposes:

- `code_challenge(&self) -> &str` — safe to read multiple times, needed
  before `wait_for_callback` consumes `self`.
- `code_verifier(&self) -> &str` — same visibility; `plugin.rs` reads it
  into an owned `String` before calling `wait_for_callback` (which takes
  `self` by value), so the value survives past that point.

### Threading through `plugin.rs`

- `SsoBuildRequestMsg` (already sent to `sso_build_request`) gains two
  fields:

  ```json
  {"code_challenge": "...", "code_challenge_method": "S256"}
  ```

  populated from `server.code_challenge()` before the existing
  `sso_build_request` call.

- `SsoCallbackMsg` (already sent to `sso_parse_callback`) gains one field:

  ```json
  {"code_verifier": "..."}
  ```

  populated from the `code_verifier` string captured before
  `wait_for_callback` was called. The guest only ever sees `code_verifier`
  at this one point — never earlier, and never has to persist it itself
  across calls via Extism's `var_get`/`var_set`. This mirrors how
  `idp_origin` is bound late, for the same reason: nothing forces the guest
  to carry a secret forward through cross-call state that a review would
  then have to reason about as a potential exfiltration channel.

Both additions are backward compatible: existing guest structs that
deserialize `sso_build_request`'s or `sso_parse_callback`'s request JSON and
don't declare these fields simply ignore them (Rust's `serde` — and every
other language's typical JSON deserializer — silently drops unrecognized
object keys unless the struct opts into strict/deny-unknown-fields
behavior, which none of the ABI's existing guest-side structs do). A plugin
that doesn't want PKCE reads nothing new and pays only the negligible cost
of the host generating two strings it never asked for — no opt-in flag,
no ABI-level branching needed to keep this non-intrusive for plugins that
don't use it.

### ABI version

Stays `"1"`. Additive.

### Security note (for the spec record, not just this section)

This protects the authorization-code exchange from interception *between
the IdP's redirect and this plugin* — a network-path or logging-leak
concern. It does **not** protect against the plugin itself: a `sso`-flavor
plugin legitimately receives both the authorization code (via
`sso_parse_callback`'s `params`) and, with this change, the `code_verifier`
— a plugin that is itself malicious already has everything it needs
regardless of PKCE. This should not be read as a defense against a
compromised plugin; that boundary is enforced elsewhere (origin pinning,
SSRF denylist, the confirmation prompt), not by PKCE.

### Docs

- `doc/src/plugins/author-guide.md`: document the new `code_challenge` /
  `code_challenge_method` fields on `sso_build_request`'s request JSON, and
  `code_verifier` on `sso_parse_callback`'s request JSON, in the same
  bullet-list style as the existing fields there.
- Matching update to `plugin.rs`'s module-level `## Guest ABI (version 1)`
  doc comment.

### Testing

- `sdk/websso-host/src/lib.rs`: unit tests on the new PKCE generation
  function —
  - `code_verifier` is 43 characters, drawn only from
    `[A-Za-z0-9-._~]`.
  - `code_challenge` equals the independently-computed
    base64url-no-pad SHA256 of the returned `code_verifier`.
  - Two calls produce different `code_verifier` values (uniqueness; RNG is
    actually being exercised, not a fixed value).
- `sdk/plugin-wasm/fixtures/example-sso-plugin/src/lib.rs`: extend with a
  mode where `sso_build_request` embeds `code_challenge` +
  `code_challenge_method` into the returned authorize URL's query string,
  and `sso_parse_callback` includes the received `code_verifier` in the
  `idp_http_request` POST body it sends for token exchange (reuse the
  existing IdP-token-exchange mock-server mode from the `idp_http_request`
  work, extended to also assert the `code_verifier` form field is present
  and matches what `sso_build_request` was told to expect via `code_challenge`
  — the mock IdP can independently recompute SHA256(code_verifier) and
  compare against the `code_challenge` it saw earlier in the authorize
  request, proving the two round-tripped correctly end to end).
- `sdk/plugin-wasm/tests/wasm_sso_plugin.rs`: one new end-to-end test
  driving the extended fixture mode above through the full
  `sso_build_request` → browser-open-skipped-in-test → `sso_parse_callback`
  path, asserting the mock IdP saw a `code_verifier` whose SHA256 matches
  the `code_challenge` from the earlier `sso_build_request` call.

## Out of scope

- The out-of-tree `s11auth` wasm plugin itself (sub-project B, unblocked
  once this lands).
- Any change to `identity_http_request` or `idp_http_request`.
- `nonce` / id_token-replay validation. The Python `s11auth` plugin
  generates a `nonce` value but never validates it against the returned
  `id_token`'s `nonce` claim. Whether the wasm port should add real nonce
  validation is a decision for sub-project B (the plugin itself), not this
  ABI increment — nonce validation, if added, is guest-side logic against
  data the guest already receives (the id_token in the token-exchange
  response), and needs no new host capability.
- An opt-in flag to suppress PKCE-pair generation. Considered and rejected
  above: the generation cost is negligible and unused fields are already
  silently ignored by conforming plugins, so there is no cost an opt-in
  would meaningfully avoid.
