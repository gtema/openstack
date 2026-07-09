# SDK / CLI / TUI Design Review (July 2026)

Scope: `openstack_sdk`, `sdk/core`, `sdk/auth-core`, with follow-on observations for
`openstack_cli` and `openstack_tui`. Generated bindings (per-service crates under
`sdk/*`, `openstack_types`) are produced from OpenAPI specs; suggestions affecting
them are phrased as template changes.

---

## 1. Overall architecture

The split into `sdk/auth-core` (auth plugin contract + token model), `sdk/core`
(transport, catalog, config, state) and per-service generated crates re-exported
through the `openstack_sdk` facade is sound and matches the direction of the
ongoing decomposition. Feature-gating per service and per sync/async in the facade
is good. The findings below are ordered roughly by impact.

### 1.1 Correctness: 401 retry loses request headers (bug)

`AsyncOpenStack::rest_async` (`openstack_sdk/src/openstack_async.rs:192`) rebuilds
the request on a 401 retry with only method + URI:

```rust
request = http::Request::builder()
    .method(orig_method.clone())
    .uri(orig_uri.clone());
```

Headers set by `prepare_request` — `Content-Type`, `Accept`, microversion headers
(`OpenStack-API-Version`, `X-OpenStack-Nova-API-Version`, …) and any endpoint
`request_headers()` — are dropped. After a successful re-auth, a retried POST/PUT
is sent without `Content-Type` and without the negotiated microversion, which will
typically fail with 400 or silently execute with wrong microversion semantics.

Fix: capture the original `HeaderMap` (e.g. via `request.headers_ref().cloned()`)
before the loop and re-apply it on rebuild, or restructure the loop so the builder
is cloned per attempt (build the `http::Request<Vec<u8>>` once, then clone parts).

### 1.2 Concurrency: re-authentication is not single-flight

`AsyncOpenStack` is `Clone` and shares `Arc<RwLock<SessionContext>>`, so several
in-flight requests can hit 401 simultaneously (typical when a token expires under
a TUI or a parallel pager). Each of them independently runs `handle_401_retry` →
`clear_all_auth` + full re-auth. Consequences: N parallel keystone logins,
interleaved cache writes, and with an interactive `AuthHelper` potentially N
credential prompts.

Fix: guard re-auth with a `tokio::sync::Mutex<()>` (or a single-flight cell that
stores the epoch/token generation). A waiter that acquires the lock after another
task finished re-auth should first re-check the token state and skip its own
re-auth. The same guard should be shared with the proactive renewal task (§3).

### 1.3 Sync client duplication and per-call runtimes

`openstack.rs` (~1.4k lines) re-implements most of `openstack_async.rs` (~2k
lines): authorize flow, discovery loop, 401 handling. Divergence is already
visible (the async side gained the catalog-refresh-on-miss path and the header
bug fix surface, the sync side has its own copies). In addition the sync client
creates `tokio::runtime::Runtime::new()` inside individual operations
(`openstack.rs:274`, `:343`) — expensive (thread pool spawn per call) and prone to
"cannot start a runtime from within a runtime" panics when embedded.

Recommendation: make the sync client a thin façade over `AsyncOpenStack` with one
lazily-initialized, shared current-thread runtime (the `reqwest::blocking`
pattern), so the auth/catalog/retry logic exists exactly once. This removes ~1k
lines and makes fixes like §1.1/§1.2 apply to both interfaces automatically.

### 1.4 Finish the auth module migration

`openstack_sdk/src/auth/` still holds `auth_helper.rs`, `authtoken.rs`,
`v3_token_info.rs`, partially duplicating `sdk/core/src/auth/*` and
`sdk/auth-core` (`authtoken_scope` exists in *three* crates). This is transitional,
but the duplicated scope/helper types are a place where behavior can silently
diverge (e.g. wildcard `matches()` semantics used by the cache). Suggest driving
the remaining pieces into `auth-core` (token/scope) and `core` (helpers), leaving
only re-exports in the facade, and marking the old paths `#[deprecated]` for one
release.

### 1.5 Plugin registry via `inventory`

Link-time plugin collection required the `anchor_plugins()` workaround, i.e. the
mechanism is already fighting the linker. It also makes the active plugin set
invisible in the type system, and any binary that forgets the anchor gets
"AuthType not supported" at runtime. Options, in increasing order of change:

1. Keep `inventory`, but call `anchor_plugins()` from `AsyncOpenStack::new_impl`
   so no consumer can forget it, and log the discovered plugin list at debug level.
2. Provide an explicit registry: `AuthPluginSet::default()` built from the
   feature-enabled plugin crates (a generated `match auth_type { ... }` — this can
   come from a template too), with `inventory` kept only for out-of-tree plugins.

### 1.6 Locking style and poisoning

`std::sync::RwLock` with short critical sections is fine in async code, but the
poison handling is inconsistent: trait paths return `RestError::SessionPoisoned`,
while getters use `unwrap_or_else(|e| e.into_inner())` and the `Debug` impl
panics. Since every write path already treats poisoning as recoverable, the
simplest consistent choice is `parking_lot::RwLock` (no poisoning, smaller code)
or uniformly `into_inner()`. Also consider snapshotting: most read paths only
need `auth` + an endpoint; an `arc_swap::ArcSwap<AuthSnapshot>` for the hot path
would remove the lock from every request entirely.

### 1.7 Hard-coded knobs

- `max_auth_retries` is fixed at 1 (`new_impl`); make it configurable via
  `CloudConfig`/builder.
- `pool_max_idle_per_host`, timeouts, keepalive are fixed except `api_timeout`.
  A `ClientBuilder`-style constructor (`AsyncOpenStack::builder()`) would avoid
  growing more `new_*` variants (there are already three).
- `set_latest_microversion` hardcodes the service→header mapping in `sdk/core`;
  this table is spec-derivable and could be generated (see §5).

---

## 2. Auth cache (`sdk/core/src/state.rs`)

The lock-file + atomic-replace design is good and the test coverage is solid.
Remaining issues:

### 2.1 Cache key uses an unstable, ambiguous hash

`get_config_identity_hash` (`sdk/core/src/config.rs:430`) feeds `Option`-al fields
into `DefaultHasher`:

- **Unstable**: `DefaultHasher`'s algorithm is explicitly not guaranteed stable
  across Rust releases; a toolchain bump can orphan every cache file (silent, but
  contributes to §2.4 accumulation and breaks "shared cache between osc versions").
- **Ambiguous**: `None` fields are skipped, so `(username=Some("x"), user_id=None)`
  and `(username=None, user_id=Some("x"))` hash identically, as do any other
  shifted combinations. Two different identities can collide on the same cache
  file; the wildcard scope matching then happily returns a token belonging to a
  different user. Unlikely, but it is a credential-mixing bug class.

Fix: hash a canonical serialization with field tags (e.g. SHA-256 over
`"auth_url=<v>\0username=<v>\0..."`, truncated for the filename). This is stable
across releases, unambiguous, and additionally not reversible (today the u64 gives
no useful protection anyway, but with names in the clear it's moot — see §2.2).

### 2.2 On-disk token confidentiality

Tokens are plaintext postcard with `0o700` dir / `0o600` file — an acceptable
baseline (same as `python-openstackclient` keyring-off mode). Two improvements:

- On **Windows**, `set_readonly(true)` does not restrict other users at all; it
  only sets the read-only attribute — and a read-only data file makes the next
  `NamedTempFile::persist`/`File::create` fail, so cache *updates* likely break on
  Windows after the first write. Use proper ACLs (or accept default ACLs of the
  profile dir and drop the readonly bit).
- Offer an opt-in `keyring` backend (`cache.auth_backend: keyring`) for storing
  the postcard blob, keeping the file backend as fallback. The `State` API is
  already narrow enough (`load_auth_state`/`save_scope_auth_to_file`) to hide a
  storage trait behind it: `trait AuthStateStore { load(hash) -> Option<ScopeAuths>; store(hash, &ScopeAuths); clear(hash); }`.

### 2.3 Validity margin on cache reads

`filter_invalid_auths` calls `get_state(None)` — a token that expires in 2 seconds
is "valid", gets returned from cache, and the request then 401s (or worse, a
non-idempotent request lands exactly at expiry). Use a small offset when
filtering for *use* (e.g. 60 s, configurable), while keeping `None` for filtering
what is worth *persisting*. This also makes the TUI's ad-hoc 10-second check
(§4.2) unnecessary.

### 2.4 Housekeeping

- Files under `~/.osc/` are never garbage-collected: stale `<hash>` and
  `<hash>.lock` files accumulate forever (every credential/toolchain change makes
  a new one). Cheap fix: on `State::new()`, best-effort delete cache files with
  mtime older than the max token lifetime (e.g. 7 days).
- `load_auth_state` only takes the shared lock if the `.lock` file already exists
  (`create(false)`); the very first reader of a cache written by an older layout
  races unlocked. Use `create(true)` for the lock file on the read path too.
- The postcard payload has no version envelope; any struct evolution turns old
  caches into "corrupted → removed". That is self-healing but silent; a one-byte
  format version would let you distinguish corruption from schema change and log
  accordingly.
- `ScopeAuths` iteration order (HashMap) makes `find_first_valid_auth`
  non-deterministic; when both a domain- and a project-scoped token are cached,
  which one seeds re-scoping varies run to run. Prefer an ordering (unscoped >
  project > domain > system) — the unscoped preference already exists, extend it.

---

## 3. Proactive re-authentication (requested feature)

Recommendation: an **in-process background task**, not a separate OS process.
A spawned process would need its own access to `clouds.yaml`/`secure.yaml` (or an
IPC channel for secrets), a lifecycle/daemon story, and per-platform packaging —
while everything it does (refresh a token before expiry, update the shared file
cache) can be done by a `tokio` task inside the client that owns the credentials
already. A separate `osc auth agent` daemon only pays off if you later want
cross-process *push* renewal beyond the shared file cache; the file cache plus
in-process renewal already gives cross-process reuse.

Sketch:

```rust
pub struct RenewHandle(tokio::task::JoinHandle<()>);

impl AsyncOpenStack {
    /// Spawn a background token renewal task. Renews `margin` before expiry.
    pub fn enable_auto_renew(&self, margin: chrono::TimeDelta) -> RenewHandle {
        let client = self.clone(); // shares Arc<RwLock<SessionContext>>
        RenewHandle(tokio::spawn(async move {
            loop {
                let Some(expires_at) = client.token_expiry() else { return };
                let wake = expires_at - margin - jitter();
                tokio::time::sleep_until(wake).await;
                // single-flight guard shared with the 401 path (§1.2)
                let _g = client.reauth_lock.lock().await;
                if client.get_auth_state(Some(margin)) == Some(AuthState::Valid) {
                    continue; // someone else already renewed
                }
                if let Err(e) = client.renew_token().await {
                    warn!("background token renewal failed: {e}");
                    // backoff and retry until hard expiry, then stop
                }
            }
        }))
    }
}
```

Design points:

- **Renew by token exchange, not by credentials.** `reauth()` (token plugin with
  the *current, still-valid* token) already exists and needs no secrets and no
  interaction — it works even for password/MFA/SSO-authenticated sessions as long
  as renewal happens *before* expiry. That is the whole point of renewing early.
  Application credentials can always renew non-interactively as a fallback after
  expiry. Note Keystone token exchange preserves but never extends
  app-credential/trust restrictions; also beware clouds with short
  `max_token_lifetime` chains — renewal-by-token produces a token whose lifetime
  is counted from *now*, so chaining is fine.
- **Cooperate across processes.** Before renewing, take the cache's exclusive
  file lock and re-read the cache: another osc/ostui process may have renewed
  already. Add jitter to the wake time so multiple processes don't stampede
  Keystone.
- **Lifecycle.** Return a handle whose `Drop` aborts the task (or tie it to
  `AsyncOpenStack` being the last `Arc` — a `Weak` upgrade per iteration ends the
  task when the client is gone). Never keep the session `RwLock` held across
  `.await`.
- **Consumers**: the TUI should enable this unconditionally (removing its
  pre-request check, §4.2); the CLI is short-lived and doesn't need it — the CLI
  benefits instead from the read margin (§2.3). Long-running SDK consumers opt in.
- Emit a tracing event (`token_renewed`, `expires_at=...`) so the TUI can surface
  it in the status bar.

---

## 4. `openstack_cli` and `openstack_tui`

### 4.1 CLI

- **Startup latency**: every invocation performs identity version discovery plus
  (per command) service discovery before the actual request. The auth token and
  its catalog are cached, but discovered *version documents* are not. Persisting
  discovery results (per endpoint URL, with a TTL) next to the auth cache would
  remove one round-trip per service per run — noticeable for `osc`'s
  one-shot-usage pattern.
- The project-override rescope block in `lib.rs:173-204` re-implements scope
  resolution that conceptually belongs in `CloudConfig`/SDK (merge CLI overrides
  into the config before connecting, and let `authorize` do the rest). That would
  also fix the "config chosen with args, result may be unexpected" warning path.
- `renew_auth` detection peeks into the parsed command tree in `entry_point`
  (`if let TopLevelCommands::Auth(...)`); as more commands need pre-connect flags
  this pattern won't scale. Consider a small `Command::connection_requirements()`
  hook implemented by the generated command structs (template change) returning
  `{ scope, renew, needs_auth: bool }` — this would also let `osc api-version`
  or purely local commands skip authentication entirely.
- Suggest an `osc auth status` subcommand (token scope, expiry, cache file in
  use) — cheap to build on `get_auth_state`/`get_auth_info` and invaluable for
  debugging cache issues like §2.

### 4.2 TUI

- **Worker loop resilience**: in `cloud_worker.rs:240`,
  `conn.authorize(None, false, true).await?` propagates auth failure out of
  `run()`, killing the worker loop (and with it all further actions) instead of
  emitting `Action::Error` like every other failure in that match. Same for the
  `discover_service_endpoint(...).await?` calls inside `SwitchToRegion`. These
  should be converted to error actions so an expired session degrades gracefully
  to a re-login prompt.
- The pre-request expiry check (10 s margin) is racy and per-request; replace
  with `enable_auto_renew` (§3) once available.
- The six sequential `discover_service_endpoint` calls at connect (and the
  four-call copies in `switch_auth_scope` / `SwitchToRegion`) should be one
  helper driven by a single service list, executed concurrently
  (`futures::future::try_join_all`) — connect time drops to the slowest service
  instead of the sum. Better still: rely on lazy discovery in
  `get_service_endpoint` and drop the upfront sweep entirely; the first request
  per service pays the (cached) discovery cost.
- Consider deriving the TUI's service list from the enabled cargo features so a
  newly generated service crate shows up without touching the worker.

---

## 5. Template-level suggestions for generated bindings

The `RestEndpoint` trait is a clean, minimal contract. Since bindings are
regenerated, several improvements are cheap to roll out:

- **Static response header maps**: `response_headers()` allocates a `HashMap` per
  call; the data is known at codegen time — return `&'static [(&'static str,
  &'static str)]`.
- **Microversion metadata**: the OpenAPI specs know which parameters/fields are
  gated by microversions. Emit per-request `min_version`/`max_version` (and
  per-field gating where the spec has it) so `set_latest_microversion` can
  negotiate per request instead of always sending the endpoint's latest — this
  also allows a clear client-side error ("field X requires compute 2.79, endpoint
  has 2.60") instead of a server 400. The hardcoded service→header table in
  `set_latest_microversion` should come from the same generated metadata.
- **Structured service errors**: `check_response_error` surfaces raw bodies;
  most services have a stable error schema in their specs. Generating per-service
  error types (behind the existing service features) would let CLI/TUI show
  human-readable faults and let SDK users match on error codes.
- **Connection requirements hook for CLI commands** (§4.1): generated command
  templates can emit whether a command needs auth/scope, enabling no-auth
  fast paths.
- **`Cow<'static, str>` endpoint building** already avoids most allocation;
  consider also emitting `const SERVICE_TYPE`/`const METHOD` associated items to
  allow match-free dispatch in tooling.

---

## 6. Summary of prioritized actions

| # | Item | Area | Effort |
|---|------|------|--------|
| 1 | Preserve headers on 401 retry (bug) | sdk | S |
| 2 | Single-flight re-auth guard | sdk | S |
| 3 | Stable, tagged auth-cache hash | core | S |
| 4 | Expiry margin on cache reads | core | S |
| 5 | TUI worker: auth failure → error action, not loop exit | tui | S |
| 6 | Background auto-renew task (`enable_auto_renew`) | sdk | M |
| 7 | Windows cache file permissions (readonly breaks rewrite) | core | S |
| 8 | Sync client as wrapper over async, shared runtime | sdk | M |
| 9 | Cache GC + lock-file creation on read path + format version | core | S |
| 10 | Concurrent/lazy service discovery in TUI; discovery cache for CLI | cli/tui | M |
| 11 | Finish auth module migration; dedupe scope/helper types | sdk | M |
| 12 | Microversion + error-schema metadata in templates | codegen | M/L |
| 13 | Optional keyring backend for auth cache | core | M |
| 14 | Explicit/anchored plugin registry | auth-core | S |

---

# Part 2: Deeper SDK implementation review + `Waitable` design

## 7. Structure and dependencies

### 7.1 `sdk/core` dependency weight

`openstack_sdk_core` is the crate every consumer must compile, and it currently
pulls in concerns that are not "core":

- **`dialoguer`** — an interactive terminal prompting library — is an
  unconditional dependency because the `Dialoguer` `AuthHelper` lives in
  `sdk/core/src/auth/auth_helper.rs`. Server-side/embedded SDK users pay for a
  TTY prompt stack they can never use. Move interactive helpers to a small
  `openstack-sdk-auth-helpers` crate (or gate behind an `interactive` feature,
  off by default); `AuthHelper`/`Noop` stay in core.
- **`tokio` / `tokio-util`** are unconditional even though the crate has a
  `sync`-only configuration; gate them under the `async` feature.
- **`config` (+yaml), `dirs`** are only needed by `ConfigFile`/`CloudConfig`
  parsing; a `config-file` feature would let programmatic-config users
  (constructing `CloudConfig` in code) drop them.
- `inventory`, `postcard`, `tempfile` are justified but tie into §1.5/§2 — if
  the state store becomes a trait (§2.2), `postcard`/`tempfile` move behind the
  file-backend feature.

Rule of thumb worth adopting: *core = types + traits + transport*; anything that
does terminal I/O, file formats, or OS integration should be a feature or a
leaf crate. This also improves compile times for the generated service crates,
all of which depend on core.

### 7.2 Facade and crate layout

- Two crates per service (`sdk/<service>` requests + `types/<service>`
  responses) is a defensible split (CLI/TUI consume response types without the
  request machinery), but it doubles the release/version surface. If the split
  isn't load-bearing anywhere, folding response types into the service crate
  under a `types` module (still a separate codegen template) would halve the
  crate count. If it is load-bearing, document the dependency rule ("types
  crates must not depend on sdk crates") in CONTRIBUTING to keep it that way.
- `openstack_sdk/src/api/<service>.rs` files are pure `pub use` shims — fine.
  Consider generating them too, so adding a service is a codegen-only change.
- Duplicated request execution/telemetry: `execute_auth_request` in
  `auth-core/src/lib.rs` and `AsyncOpenStack::execute_request` are near-copies
  (same `http_request` event). Move the instrumented executor into core and use
  it from both, so tracing fields stay consistent (the auth one lacks body
  censoring, the client one lacks nothing — one implementation, one truth).

## 8. Interface patterns

### 8.1 Untyped request→response coupling (biggest ergonomic gap)

`query_async::<T>` accepts any `T: DeserializeOwned`; nothing connects
`compute::v2::server::get::Request` to
`openstack_types::compute::v2::server::response::get::ServerResponse`. Users
must find the right response struct by convention, and a mismatch compiles fine
and fails at deserialization time. Since both sides are generated from the same
spec, the linkage is known at codegen time. Template change:

```rust
impl RestEndpoint for Request<'_> {
    // existing items...
}
// generated alongside:
impl TypedEndpoint for Request<'_> {
    type Response = openstack_types::compute::v2::server::response::get::ServerResponse;
}
```

plus one blanket combinator in core:

```rust
#[async_trait]
pub trait SendAsync<C: AsyncClient>: TypedEndpoint {
    async fn send(&self, client: &C) -> Result<Self::Response, ApiError<C::Error>>;
}
```

`query_async` stays for raw/`serde_json::Value` use. This one change removes
the most common user error, makes examples self-documenting, and is a
prerequisite for a *typed* waiter (§10).

### 8.2 Pagination should offer a `Stream`

`paged(ep, Pagination::Limit(n))` materializes the whole `Vec<T>` — O(n) memory
and no early exit. Add `paged(...).into_stream(client)` returning
`impl Stream<Item = Result<T, ApiError<_>>>` (page-by-page fetch under the
hood; `futures::stream::try_unfold` fits the existing code shape almost 1:1),
with the collecting `QueryAsync` impl kept for compatibility. The CLI's
`--limit`-less list commands and the TUI's incremental table fill are the
immediate beneficiaries; sync gets the analogous `Iterator`.

### 8.3 Microversioned endpoint variants

Generated modules like `create_21`, `create_257`, `rebuild_254` push the
microversion choice to the user with no guidance at the call site. Two
template-level mitigations:

- generate a `latest` re-export per operation (`pub use create_257 as create_latest;`)
  so casual users have an obvious default; and
- emit `pub const API_VERSION: ApiVersion` on each request so runtime
  negotiation ("pick the newest variant ≤ endpoint max") becomes possible for
  tooling — today that information is only in the module name.

### 8.4 `Findable` details

- `locate_resource_in_list` clones `data[0]`; take `mut data` and
  `data.swap_remove(0)` (values can be large server documents).
- Treating `400` as "not an ID, fall back to list" is pragmatic but also
  swallows genuine bad requests (e.g. a malformed query the user should see).
  Consider limiting the fallback to the name-lookup path, or attaching the
  original GET error to the final `ResourceNotFound` as context.

## 9. Testability

The layering is already test-friendly at the bottom: generated endpoints are
client-generic, `FakeOpenStackClient` + `httpmock` covers endpoint behavior,
and `sdk/core` has real concurrency tests for the state file. Gaps are at the
top:

- **`AsyncOpenStack` is a concrete type**, so the CLI's `take_action` and the
  TUI's `cloud_worker` cannot be tested without a live Keystone. Extract the
  session surface they actually use into a trait (`OpenStackSession`:
  `AsyncClient + RestClient` + `authorize`/`discover_service_endpoint`/
  `get_auth_*`), implemented by both `AsyncOpenStack` and a fake. The TUI
  worker in particular is pure message-plumbing that would become trivially
  testable.
- **`authorize_with_auth_helper` is a ~190-line method** mixing cache lookup,
  plugin selection, receipt handling, token introspection, rescoping and
  catalog processing — each with its own failure modes, none independently
  testable. Decompose into named steps (`try_cached_auth`, `login_with_plugin`,
  `handle_receipt`, `rescope`, `apply_catalog`) on `SessionContext`; the
  orchestration shrinks to ~20 lines and each step can be unit-tested against
  the fake client. This is also the precondition for reusing the flow from the
  sync client (§1.3).
- **Scope matching is load-bearing for cache correctness** (`AuthTokenScope::matches`
  decides which cached token a request gets) but only example-tested. Property
  tests (proptest: reflexivity, `matches` ⊇ equality, None-field wildcard
  behavior) are cheap insurance against credential-mixing regressions.
- **Cache format regression**: add a golden-file test (committed postcard blob
  from the current schema) so an innocent struct change that silently
  invalidates every user's cache shows up in review.
- `ApiError` carries no `x-openstack-request-id`; capturing it (header is
  already logged) into `ApiError::OpenStack`/`OpenStackService` makes bug
  reports actionable and is easy to assert in tests.

## 10. `Waitable`: waiting for a resource to reach a status

Prior art worth matching semantically: python-openstacksdk
`wait_for_status(res, status, failures, interval, wait)` and gophercloud
`WaitFor`. The requested semantics — *periodically refetch until the resource
reaches a status, or disappears* — decompose into three layers, of which only
the third needs codegen.

### 10.1 Layer 1: a generic `Wait` combinator in `sdk/core` (no codegen)

Follow the existing combinator pattern (`Find`, `Paged`): a wrapper that is
generic over a GET `RestEndpoint` and implements `QueryAsync`. Generated
request structs already derive `Clone`, so re-issuing the GET is free.

```rust
/// What a single poll observed.
pub enum Observation<T> {
    /// Resource exists; deserialized body.
    Present(T),
    /// GET returned 404 — resource is gone.
    Gone,
}

/// Caller's verdict after each poll.
pub enum WaitDecision {
    /// Keep polling.
    Continue,
    /// Terminal: condition met.
    Done,
    /// Terminal: resource entered a failure state (e.g. ERROR).
    Fail(String),
}

pub struct Wait<E, F> {
    endpoint: E,
    check: F,
    interval: Backoff,          // fixed or expo-capped, with jitter
    timeout: Option<Duration>,
}

pub fn wait<E, F, T>(endpoint: E, check: F) -> Wait<E, F>
where
    F: FnMut(Observation<&T>) -> WaitDecision;

/// Convenience: wait until the resource disappears (deletion).
pub fn wait_deleted<E>(endpoint: E) -> Wait<E, impl FnMut(Observation<&Value>) -> WaitDecision>;
```

Execution loop (async; sync mirrors with `thread::sleep`):

```rust
loop {
    match self.endpoint.query_async(client).await {
        Ok(body) => match (self.check)(Observation::Present(&body)) {
            WaitDecision::Done => return Ok(body),
            WaitDecision::Fail(reason) =>
                return Err(ApiError::WaitFailed { reason, last: Some(body) }),
            WaitDecision::Continue => {}
        },
        Err(e) if e.is_not_found() => match (self.check)(Observation::Gone) {
            WaitDecision::Done => return Ok(/* Gone marker */),
            _ => return Err(ApiError::WaitResourceVanished),
        },
        Err(e) if e.is_transient() => { /* tolerate N transient errors, then bail */ }
        Err(e) => return Err(e),
    }
    if deadline_exceeded() {
        return Err(ApiError::WaitTimeout { last_status });
    }
    tokio::time::sleep(self.interval.next()).await;
}
```

Design decisions embedded there:

- **"Disappears" is an outcome, not an error, and its meaning depends on
  intent**: for `wait_deleted` a 404 is success; for wait-for-ACTIVE it is a
  hard failure (`WaitResourceVanished`). Routing 404 through the same `check`
  closure keeps one loop for both.
- **Fail-fast on error states.** Waiting for `ACTIVE` must terminate
  immediately when the resource hits `ERROR` — that is what `Fail` is for.
  Timeouts alone (gophercloud's early mistake) make failed creates take the
  full timeout to report.
- **Transient tolerance**: a single 503 or connection reset mid-wait should
  not abort a 10-minute server build; budget a small number of consecutive
  transient errors. The existing 401-retry in `rest_async` already covers token
  expiry mid-wait — and the auto-renew task (§3) removes even that.
- **Backoff with jitter, capped** (e.g. start 1s, ×1.5, cap 15s, ±20% jitter):
  polling APIs at fixed 1s across a fleet of CLI invocations is how clouds get
  hammered. No `Retry-After` exists on OpenStack GETs, so client-side policy is
  it.
- **Cancellation is free**: dropping the future stops the wait (important for
  the TUI, which can drop it when the user navigates away). No token needed.
- **Progress**: emit a `tracing` event per poll
  (`wait_progress{resource, status, elapsed}`); the CLI renders a spinner from
  it via its existing tracing collector, the TUI its status bar — no callback
  plumbing in the API.

This layer alone already satisfies the requirement, with the caller supplying
the predicate over `serde_json::Value` (status extraction by pointer, e.g.
`body.pointer("/status")`).

### 10.2 Layer 2: typed status trait (small core trait, generated impls)

The generated response types already have typed `Status` enums
(`types/compute/src/v2/server/response/get_2100_a.rs:397`). A one-method trait
in core:

```rust
pub trait HasStatus {
    type Status: PartialEq + Debug + Clone;
    fn status(&self) -> Option<Self::Status>;
}
```

with template-generated impls for every response struct that has a
spec-declared `status` field, enables the ergonomic form on top of layer 1 +
`TypedEndpoint` (§8.1):

```rust
pub fn wait_for_status<E>(
    ep: E,
    target: <E::Response as HasStatus>::Status,
    failures: &[<E::Response as HasStatus>::Status],
) -> Wait<E, impl FnMut(...)>
where E: TypedEndpoint, E::Response: HasStatus;

// usage:
let server = wait_for_status(
        server::get::Request::builder().id(id).build()?,
        server::Status::Active,
        &[server::Status::Error],
    )
    .timeout(Duration::from_secs(600))
    .query_async(&session)
    .await?;
```

Notes:

- Response types are per-microversion; the impl is generated per struct, so
  this costs nothing extra at codegen time.
- Status enums should be generated `#[non_exhaustive]` with an
  `Unknown(String)` catch-all variant (clouds add states; a wait must not
  fail to deserialize on `SHELVED_OFFLOADED` from a newer cloud than the spec).
  Worth doing regardless of the waiter — today an unknown enum value is a
  deserialization error for any consumer.
- Some resources signal failure in a companion field (`fault.message` on
  servers) — an optional `fn failure_detail(&self) -> Option<String>` on
  `HasStatus` (generated where the spec has such a field) turns
  `Fail("status=ERROR")` into `Fail("No valid host was found")`.

### 10.3 Layer 3: spec/metadata-driven defaults (optional, later)

OpenAPI specs do not encode state machines, but the codegenerator's metadata
overlays could: per resource, `wait: { ready: [ACTIVE], failed: [ERROR],
deleted_ok: true }`. That would let templates emit
`server.wait_ready(&session)` one-liners and let the CLI templates grow a
`--wait`/`--wait-timeout` flag on create/delete commands uniformly — the same
metadata serving SDK, CLI and TUI. Recommended sequencing: ship layers 1–2
first (they are pure Rust + one template touch), collect the metadata
incrementally for the handful of resources people actually wait on (server,
volume, image, load balancer, stack, cluster).

### 10.4 Placement and naming

- `sdk/core/src/api/wait.rs` next to `find.rs`/`paged.rs`; exported as
  `api::{wait, wait_deleted, wait_for_status}` — consistent with the existing
  free-function combinator style (`find(...)`, `paged(...)`).
- Add `ApiError::{WaitTimeout, WaitFailed, WaitResourceVanished}` variants
  (the enum is `#[non_exhaustive]`, so this is non-breaking).
- Both `sync` and `async` impls behind the existing features, like `Find`.

## 11. Additional prioritized actions (part 2)

| # | Item | Area | Effort |
|---|------|------|--------|
| 15 | `TypedEndpoint` (request→response linkage) in templates + `send()` | codegen/core | M |
| 16 | `wait`/`wait_deleted` combinator + error variants | core | M |
| 17 | `HasStatus` trait + generated impls, `wait_for_status` | codegen/core | S (after 15/16) |
| 18 | `Unknown(String)` catch-all on generated status enums | codegen | S |
| 19 | Move `dialoguer` helper out of core; feature-gate tokio/config | core | S |
| 20 | Stream-based pagination | core | M |
| 21 | Session trait for CLI/TUI testability | sdk/cli/tui | M |
| 22 | Decompose `authorize_with_auth_helper` into testable steps | sdk | M |
| 23 | Request-id in `ApiError`; property tests for scope matching; cache golden file | core | S |
| 24 | `--wait` CLI flags from wait metadata | codegen/cli | M (after 17) |

---

# Part 3: TUI resource coverage scaling and schema-driven create/edit

## 12. Making TUI resource coverage cheaper to extend

### 12.1 Anatomy of adding a resource today

The heavy lifting is already done well: `cloud_worker/<service>/<version>/<resource>/*`
request types are **generated** (with `ExecuteApiRequest` impls), and
`GenericResourceView<B: ResourceBehaviour>` means a view is ~120 lines of
mostly-declarative behaviour impl. What remains manual per resource:

| Touch point | Kind |
|---|---|
| `components/<service>/<resource>.rs` (`ResourceBehaviour` impl + tests) | ~120 + ~160 lines, largely mechanical |
| `mode.rs` — `Mode` enum variant | enum edit |
| `action.rs` — `Show*`, `Create*`, `Delete*`, `Set*ListFilters` variants | enum edit |
| `app.rs` — `Mode → component` registration | match/map edit |
| `.config/config.yaml` — mode keybindings | config edit |

Roughly five files must be touched in sync; forgetting one produces silent
gaps (a mode with no keybinding, an action nobody translates). Two levels of
fix, in order of ambition:

### 12.2 Generate the behaviour layer (near-term)

The codegenerator already targets the TUI (cloud_worker types carry the
"automatically generated" header). The `ResourceBehaviour` impls are ~90%
derivable from the same metadata:

- `view_key`/`title`/`mode` — naming convention.
- `request_from_filter`, `matches_request`, `handle_set_filter_action`,
  `confirm_request` (delete), `handle_mutation_response` — pure boilerplate
  over the generated request enums; the existing
  `security_group_rules.rs` shows the pattern verbatim.
- Leave hooks for the genuinely resource-specific parts: `normalise_filter`
  (default sort), `filter_carry_action` (drill-down), overridden via a
  companion hand-written file or codegen metadata (`tui: {sort: [...], drill: ...}`).

The tests in these files are equally mechanical and can be generated alongside
(or dropped in favor of one generic test parameterized over the behaviour
trait — testing generated code with generated tests has limited value).

### 12.3 De-enum `Mode` and `Action` (the structural fix)

Per-resource enum variants (`ShowNetworkSecurityGroupRules`,
`DeleteNetworkSecurityGroupRule`, `SetNetworkSecurityGroupRuleListFilters(...)`)
are the reason five files need coordinated edits. Replace them with
parameterized variants keyed by the `view_key` string that already exists:

```rust
pub enum Action {
    ShowResource(ResourceKey),                       // "network.security_group_rule"
    ResourceOp { key: ResourceKey, op: ResourceOp }, // Create | Delete | Describe...
    SetListFilters { key: ResourceKey, filter: serde_json::Value },
    ...
}
```

with a registry mapping `ResourceKey → Box<dyn ResourceViewFactory>`, populated
by one generated `register_all(&mut registry)` function (an explicit generated
registry is preferable to `inventory` here — see §1.5 for the linker
experience). `Mode` collapses to `Mode::Resource(ResourceKey)` plus the few
non-resource modes (Home, Describe). Keybindings in `config.yaml` already
address actions by name, so `action: ShowResource("network.security_group_rule")`
(or a serde alias preserving today's names) keeps configs working. After this,
**adding a resource = one generated module + one registry line**, and the
generated default keybindings section can come from the same metadata.

### 12.4 Related cleanups noticed on the way

- `app.rs` `Action::Confirm` uses an `unsafe` raw-pointer downcast of
  `Box<dyn Component>` to `ConfirmPopup`. `dyn Any`-based `downcast_mut`
  (add `fn as_any_mut(&mut self) -> &mut dyn Any` to `Component`) removes the
  UB risk for zero cost.
- The hardcoded `discover_service_endpoint` sweep (§4.2) can be driven from the
  same resource registry: discover lazily per service on first resource view.

## 13. Create/edit via external editor: schema hinting and validation

### 13.1 Current flow and its failure modes

`ResourceBehaviour::editor_template` → `Action::Edit{template}` → TUI suspends,
`edit::edit()` → `serde_yaml::from_str` → `Action::EditResult` →
`deserialize_edit_result` → `Confirm` → worker executes. Problems:

1. **A YAML typo crashes the TUI and loses the edit**: in `app.rs:477` the
   parse result is propagated with `?` out of `handle_actions` → `run()` →
   `main`. This must become a re-open-editor loop, never an exit.
2. **The template is a hand-maintained string** — no field documentation, no
   knowledge of what's required, and the security-group-rule TODO
   (`security_group_rules.rs:102`) shows even parent-id prefill is manual.
3. **Validation is whatever serde says**, after the editor closes, one error
   at a time, with no line numbers.

All three are solved by making the OpenAPI schema, which the codegenerator
already holds, available at runtime.

### 13.2 Ship the schema with the generated request types

Two options:

- **(A) `schemars`**: derive `JsonSchema` on the generated TUI request structs
  (the `json-schema` feature already exists in core for config types). Schema is
  assembled at runtime from the Rust types.
- **(B) Embed the OpenAPI subschema** (recommended): the codegenerator emits the
  request-body schema it generated the struct *from*, as a static string next to
  the struct:

```rust
impl NetworkSecurityGroupRuleCreate {
    /// JSON Schema of the request body (from the OpenAPI spec).
    pub const BODY_SCHEMA: &'static str = r#"{ "type": "object", ... }"#;
}
```

(B) is preferable because it preserves everything the spec knows that the Rust
type system doesn't express — descriptions, `format`, ranges
(`port_range_min: 0..65535`), patterns, defaults, and cross-field notes —
with zero runtime dependencies and exact fidelity. It also matches existing
precedent: auth plugins already communicate their input requirements as JSON
Schema (`OpenStackAuthType::requirements()`).

### 13.3 Generate the template from the schema

`editor_template` becomes generated: render the schema as commented YAML, the
`kubectl`/git-commit convention — required fields uncommented with
placeholders, optional fields commented out, docs and enums inline:

```yaml
# Create Security Group Rule  (network.security_group_rule)
# Lines starting with '#' are ignored. Required fields are uncommented.
security_group_rule:
  # The security group to attach the rule to. (required)
  security_group_id: "a7734e61-..."     # prefilled from current view
  # Ingress or egress. One of: ingress, egress  (required)
  direction: ingress
  # IP protocol: tcp, udp, icmp, ... or number. Default: any
  # protocol: tcp
  # Port range start (0..65535).
  # port_range_min: 443
```

Context prefill (the TODO): `editor_template(action, filter)` already receives
the filter; the generated impl maps parent identifiers from the filter into the
template via codegen metadata (`prefill: {security_group_id: filter.security_group_id}`)
— the same overlay mechanism as §10.3/§12.2.

For editors that understand it, prepend the modeline and drop the schema next
to the temp file — completion and live validation for free in VS Code /
neovim+yamlls, harmless elsewhere:

```yaml
# yaml-language-server: $schema=/tmp/.../security_group_rule.create.schema.json
```

### 13.4 Post-edit validation pipeline

Validate in three stages before anything is sent, all inside a retry loop:

1. **YAML parse** → `serde_yaml::Value` (syntax errors carry line/column).
2. **JSON Schema validation** with the `jsonschema` crate against
   `BODY_SCHEMA`: reports *all* violations at once with JSON pointers
   (`/security_group_rule/port_range_min: 70000 is greater than maximum 65535`),
   covering enums, ranges, formats, required — things serde reports poorly or
   not at all.
3. **Typed deserialization** into the generated request struct via
   `serde_path_to_error` (precise path on the residual errors), then the
   existing builder → `ApiRequest`.

On any failure: re-open the editor with the user's buffer **preserved** and the
errors prepended as comments (git `COMMIT_EDITMSG` pattern), plus an "abandon"
escape (empty buffer = cancel). The `Action::Edit` handler needs a small state
machine (`EditSession { schema, buffer, attempt }`) instead of the current
one-shot; that also fixes crash-on-typo (§13.1.1) since errors are consumed,
not propagated.

### 13.5 Edit (update) of existing resources

Same machinery, two additions, both schema-driven:

- Template = the *current* resource serialized to YAML, filtered to the fields
  present in the **update** request schema (which encodes writability — the
  read-only fields simply aren't in it), with the rest shown as trailing
  comments for context.
- On save, diff edited vs. original and build the update body from changed
  fields only (OpenStack PUTs are mostly merge-patch-ish per-field; sending
  unchanged fields risks policy errors on fields the user may not set).
  Include the resource's current `updated_at`/revision in the session so a
  concurrent change can at least be warned about.

### 13.6 Later: in-TUI forms from the same schema

Once `BODY_SCHEMA` ships, a ratatui form (enum → select list, boolean →
toggle, string+format → input with inline validation) can be generated *at
runtime* from the schema for simple resources, keeping the external editor as
the fallback for complex bodies. This is strictly additive — same schema, same
validation pipeline, different front end — which is the main argument for
putting the schema, not the template, into the generated code.

## 14. Prioritized actions (part 3)

| # | Item | Area | Effort |
|---|------|------|--------|
| 25 | Editor loop: never crash on invalid YAML; retry with buffer + errors preserved | tui | S |
| 26 | Emit `BODY_SCHEMA` const on generated create/update request types | codegen | S |
| 27 | Generate editor templates (commented YAML) from schema, with filter prefill metadata | codegen/tui | M |
| 28 | Validation pipeline: `jsonschema` + `serde_path_to_error` before send | tui | S |
| 29 | `yaml-language-server` modeline + schema temp file | tui | S |
| 30 | Generate `ResourceBehaviour` impls from codegen metadata | codegen/tui | M |
| 31 | Parameterize `Action`/`Mode` by `ResourceKey`; generated registry | tui | M/L |
| 32 | Update-flow: writable-field template from update schema + minimal diff body | codegen/tui | M |
| 33 | Replace `unsafe` popup downcast with `Any` | tui | S |
