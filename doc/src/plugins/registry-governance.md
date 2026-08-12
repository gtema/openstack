# Registry governance

`osc plugin search`/`install`/`update` resolve against a plugin index —
by default `plugins/registry/index.json` in this repository, served over
plain HTTPS via `raw.githubusercontent.com` (see
[the registry directory's own README](https://github.com/gtema/openstack/blob/main/plugins/registry/README.md)
for the exact JSON schema). This page describes how an entry gets into that
index, what stops a bad entry from being trusted even if it does, and how a
published plugin gets removed.

## Why the index itself doesn't need to be trusted blindly

The index is a plain JSON file in a normal git repository, merged through
normal pull-request review like any other change here. It is **not** a
security boundary on its own — anyone who can get a PR merged (including,
in principle, a reviewer who makes a mistake) can add or edit an entry.
What actually stops a malicious or mistaken index entry from being trusted
is downstream of the index, at install time:

- **Checksum**: the downloaded bytes must match the `sha256` the index
  declares, checked before anything touches disk.
- **Provenance**: the artifact's GitHub attestation must verify against a
  pinned Fulcio trust root, and the attestation's embedded identity
  (`owner/repo`) must match the index's `source_repo` field for that
  version.

So an index entry that points `download_url`/`sha256` at a legitimate
release, but lies about `source_repo`, fails closed at install time — the
downloaded bytes' real attestation won't match the claimed repo. An index
entry pointing at an attacker-controlled URL with a self-consistent
`sha256` still fails unless that URL's artifact also carries a valid
attestation naming the claimed `source_repo`, which requires control of
that repository's CI, not just of the index text. Full detail on exactly
what the provenance check does and does not prove is in
[Trust model](../plugins.md#trust-model).

This is a deliberate design point: **reviewing an index PR is a
plausibility check, not the trust boundary itself.** It means review can
stay lightweight without turning into the sole thing standing between users
and a malicious plugin.

## What a reviewer checks before merging an index PR

1. **The PR only touches `plugins/registry/index.json`** (and, for a new
   plugin, its `name`/`description`), not unrelated files.
2. **`source_repo` is a real, publicly reachable repository** that plausibly
   belongs to whoever opened the PR — a repository under someone else's
   account/org, or a private repository, is not an appropriate provenance
   target for a public registry entry.
3. **`download_url` actually resolves to a release asset in that
   `source_repo`** (typically a GitHub Releases download URL), and the
   declared `sha256` matches what's actually at that URL — a reviewer can
   check this by downloading the asset and hashing it locally.
4. **The repository's release workflow attests the artifact** with
   `actions/attest` (or an equivalent Sigstore-based GitHub attestation) —
   visible in the repo's own Actions workflow file. An entry whose CI
   doesn't attest will simply fail closed for every installer later
   (`--allow-unsigned` required), which is safe but a poor experience, so
   reviewers should ask for this before merging rather than after users
   start filing confused issues.
5. **`abi_version` matches what the artifact actually reports** — a
   mismatch here is caught automatically at install time, but catching it
   in review saves a round trip.
6. For an update to an *existing* plugin (a new `versions[]` entry, not a
   new plugin), the new entry's `source_repo` should match prior versions'
   unless the PR explains why ownership moved (e.g. a transferred
   repository) — a silent `source_repo` change on an established plugin
   name is exactly the shape a supply-chain compromise would take, so it
   gets a closer look, not a rubber stamp.

None of this review substitutes for the checksum/provenance checks `osc`
itself performs — it exists to keep obviously broken or obviously
inappropriate entries from being merged at all, and to give users installing
a plugin for the first time (who see the source repo and provenance result
before confirming, per [Installing a plugin](../plugins.md#installing-a-plugin))
something meaningful to look at.

## Removal and revocation

There is currently no automated revocation mechanism — removing or editing
an entry in `index.json` is, like adding one, a normal pull request. If a
published plugin turns out to be malicious, broken, or abandoned:

- Removing its entry (or a specific bad `versions[]` entry) from the index
  stops new `osc plugin install`/`update` calls from resolving it. This
  does **not** reach back into anyone's already-installed copy — `osc
  plugin remove <name>` on the affected machine is still up to the user (or
  their own tooling) to run.
- For an actively malicious release, opening an issue/PR promptly and
  flagging it for expedited review is preferable to waiting for a routine
  review cycle — the index has no separate "yank" mechanism, only "the
  entry is gone from `main`" the next time someone fetches it.

## Scope of this bootstrap registry

This index living in `gtema/openstack` rather than a dedicated registry
repository is a bootstrap choice, not a schema constraint — nothing about
the index format or the fetch/verification code is tied to this
repository. `--registry-url` already lets anyone point at a different
index today (their own private registry, an alternate community one, etc.),
subject to the same checksum/provenance checks on the client side
regardless of which index served the entry. If plugin release cadence ever
needs to move independently of `osc`'s own releases, the default index can
move to a dedicated repository with no code changes beyond the pinned
default URL.
