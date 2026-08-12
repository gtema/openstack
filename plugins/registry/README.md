# osc plugin registry index

This directory hosts the bootstrap index consulted by `osc plugin search`
and `osc plugin install <name>[@version]`. The default registry URL
(`openstack_sdk_plugin_wasm::index::DEFAULT_REGISTRY_URL`) points at
`index.json` in this directory on the default branch of this repository,
fetched over plain HTTPS via `raw.githubusercontent.com`.

This is a bootstrap: nothing about the index schema or the fetch/verify code
is tied to living in this repo. If plugin release cadence ever needs to move
independently of `osc` releases, the index (and the plugins it points at) can
move to a dedicated repository with no code changes beyond the pinned default
URL — `--registry-url` already lets any URL be used explicitly today.

## Schema

```json
{
  "schema_version": 1,
  "plugins": [
    {
      "name": "example_auth",
      "description": "Human readable one-liner.",
      "versions": [
        {
          "version": "1.0.0",
          "download_url": "https://github.com/<owner>/<repo>/releases/download/v1.0.0/example_auth.wasm",
          "sha256": "<lowercase hex sha256 of the .wasm file>",
          "source_repo": "<owner>/<repo>",
          "abi_version": "1",
          "min_cli_version": "0.13.0"
        }
      ]
    }
  ]
}
```

- `schema_version` — bumped on breaking changes to this shape; `osc` rejects
  an index whose `schema_version` it doesn't understand.
- `plugins[].versions[]` — one entry per published version; `osc plugin
  install <name>` without `@version` installs the highest by semver, `osc
  plugin update` re-resolves this same way.
- `source_repo` — the GitHub repository (`owner/repo`) whose CI is expected
  to have published this exact `.wasm` artifact and its attestation
  (`actions/attest`). This is what provenance verification checks the
  downloaded file's GitHub attestation against before `osc` trusts it.
- `sha256` — checked against the downloaded bytes before anything touches
  disk, independent of and prior to provenance verification.
- `abi_version` — informational; matched against the guest ABI's own
  self-reported `plugin_abi_version` after download.
- `min_cli_version` — optional; if set and the running `osc` is older,
  install is refused with a clear version-mismatch error rather than a
  confusing runtime failure.

## Publishing a plugin

A publisher's own CI (in the plugin's own repository) builds and releases
the `.wasm` artifact and attests it with `actions/attest` (the same GitHub
Action already used by this repo's own release workflow, see
`.github/workflows/release.yml`). Getting an entry added to `index.json`
here is a normal PR against this repository, adding the plugin's
`name`/`versions[]` entry with the release's real `download_url` and
`sha256`.
