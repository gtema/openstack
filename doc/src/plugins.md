# WASM Auth Plugins

`osc` supports authentication methods beyond the ones compiled in, loaded at
runtime from [Extism](https://extism.org/) (WebAssembly) modules. This lets a
third party ship a new `auth_type` — for example a corporate SSO flow, or an
identity provider-specific token exchange — without a fork of `osc` or a
recompile, and without ever giving that third party's code a socket, the
filesystem, or environment variables: every WASM plugin runs in a sandbox
that structurally cannot perform I/O other than through the narrow,
host-mediated capabilities described below.

This page covers using plugins as an `osc` operator. If you want to write
one, see the [plugin author guide](./plugins/author-guide.md). For how
plugins get into the registry `osc plugin install` resolves against, see
[registry governance](./plugins/registry-governance.md).

## The sandbox, briefly

A loaded plugin gets:

- A bounded amount of linear memory (16 MiB) and a per-call timeout (20s).
- No filesystem, no environment variables, no WASI.
- For the `auth` ABI flavor: a single host function,
  `identity_http_request`, that proxies HTTP requests to the identity
  endpoint `osc` resolved for the configured cloud — never to a URL the
  plugin picks itself.
- For the `sso` ABI flavor: no host function at all. The plugin never holds
  a socket or a browser-opening capability; it only ever computes the
  identity-provider URL to open and, later, parses an already
  CSRF-validated callback. The local callback listener, the anti-CSRF
  `state` check, and the actual browser launch are all owned by `osc`
  itself.

Either way, the plugin cannot reach any host other than the one `osc`
explicitly hands it, and it cannot exfiltrate data through any channel other
than the token it ultimately returns.

## Installing a plugin

```console
$ osc plugin search sso
$ osc plugin install example_auth
$ osc plugin install example_auth@1.2.0   # pin a specific version
$ osc plugin install --file ./my-plugin.wasm --allow-unsigned
```

`osc plugin install <name>[@version]` resolves the name against a registry
index (`--registry-url` to point at a different one than the built-in
default), downloads the `.wasm` artifact, and before ever trusting it:

1. Checks the downloaded bytes' SHA-256 against what the registry index
   declares, before anything touches disk.
2. Verifies the artifact's GitHub attestation (produced by `actions/attest`
   in the plugin's own CI) proves it was actually built and published from
   the repository the index claims, not just that the bytes are
   internally consistent. See
   [Trust model](#trust-model) below for exactly what this does and does
   not prove.
3. Shows you the source repository, the checksum, and the provenance
   result, and asks for confirmation (skip with `--yes`).

Installing from a local file (`--file`) skips steps 1–2 entirely — there is
no registry entry and no attestation to check for a file on your own disk —
so it always requires `--allow-unsigned` and is always loudly logged as
such.

Other commands:

| Command | Purpose |
| --- | --- |
| `osc plugin search [query]` | List registry entries matching `query` (name/description substring), or every entry if omitted. |
| `osc plugin list` | List installed `name@version` pairs, which one is active, and whether it was trusted via `--allow-unsigned`. |
| `osc plugin info <name>` | Show every installed version of a plugin. |
| `osc plugin update [<name> \| --all]` | Re-resolve non-pinned installs against the registry and upgrade. Provenance is re-checked fresh every time, never reused from the original install. |
| `osc plugin verify <name>` | Re-check installed file(s) against the SHA-256 recorded in the lockfile at install time — catches on-disk tampering or corruption after install. |
| `osc plugin remove <name> [--version]` | Remove one or every installed version of a plugin. |

## Trust model

Two independent checks stand between a downloaded `.wasm` file and it being
loaded and used for authentication:

- **Checksum**: the bytes must match the SHA-256 the registry index
  declares for that version. This only proves the download wasn't
  corrupted or tampered with in transit relative to what the index says —
  it says nothing about whether the index itself can be trusted.
- **Provenance**: the artifact's GitHub attestation must verify against a
  vendored, pinned Fulcio root/intermediate CA, and the attestation's
  embedded identity (OIDC issuer + `owner/repo`) must match the source
  repository the registry index claims for that plugin. This is what
  actually proves the file was built by CI in the claimed repository,
  rather than uploaded by hand or by an attacker who compromised the
  index.

  This verifier deliberately does **not** check Rekor transparency-log
  inclusion (the Merkle audit-path proof that the attestation was actually
  published to the public log, not just handed to `osc` directly) — see
  `sdk/plugin-wasm/src/provenance.rs` for why. In practice this means: a
  tampered or entirely unattested plugin still fails closed, but a
  withheld-from-the-log attestation would not be caught.

If either check fails, `osc plugin install`/`update` refuse outright unless
you pass `--allow-unsigned`, which is always logged (both to the terminal
and in structured logs) and always recorded in the lockfile, so
`osc plugin list` continues to show you which installed plugins are running
without full verification.

## Using an installed plugin

Once installed, a plugin's declared auth method(s) become usable exactly
like a built-in one — set `auth_type` in `clouds.yaml` (or pass it however
you normally configure authentication) to the method name the plugin
reports (visible via `osc plugin info <name>` or `osc plugin install`'s
output).

For an `sso`-flavor plugin, `osc` will additionally ask for confirmation
before opening your browser, showing you the exact URL it's about to open.
Declining, or the plugin returning anything other than an `https://` URL on
the callback address `osc` itself bound, is refused outright with no
override — see the [author guide](./plugins/author-guide.md#the-sso-abi-flavor)
for the mechanics.
