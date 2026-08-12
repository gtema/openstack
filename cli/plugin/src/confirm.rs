// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Shared install/update confirmation UX, used by both `osc plugin install`
//! and `osc plugin update`.

use std::io::IsTerminal;

use dialoguer::Confirm;

use openstack_cli_core::error::OpenStackCliError;
use openstack_sdk_plugin_wasm::registry::{PendingInstall, ProvenanceOutcome};

/// The single permission a WASM auth plugin's guest ABI can use today: an
/// HTTP request scoped to the identity provider origin passed at auth time.
/// See `sdk/plugin-wasm/src/plugin.rs` module docs — the guest ABI exposes
/// exactly one host function, so this is not currently derived per-plugin.
const PERMISSIONS_SUMMARY: &str =
    "identity_http (HTTP requests to the identity provider origin passed at auth time)";

/// Print what's about to be installed/updated — publisher, source repo, the
/// permission this plugin ABI can use, checksum, and the provenance result
/// — then, unless `yes` is set, ask for interactive confirmation.
///
/// Returns `Ok(true)` to proceed, `Ok(false)` if the user declined. Errors
/// out (rather than prompting) in a non-interactive context without `--yes`,
/// so this never silently hangs on stdin.
pub fn confirm_pending(pending: &PendingInstall, yes: bool) -> Result<bool, OpenStackCliError> {
    println!("Plugin:      {}@{}", pending.name, pending.version);
    println!("Source repo: {}", pending.source_repo);
    println!("Permissions: {PERMISSIONS_SUMMARY}");
    println!("SHA-256:     {}", pending.sha256);
    match &pending.provenance {
        ProvenanceOutcome::Verified(record) => {
            println!(
                "Provenance:  verified — published by CI in {} (OIDC issuer: {})",
                record.source_repo,
                record.oidc_issuer.as_deref().unwrap_or("unknown")
            );
            if let Some(workflow_ref) = &record.workflow_ref {
                println!("Workflow:    {workflow_ref}");
            }
        }
        ProvenanceOutcome::Unverified { reason } => {
            println!("Provenance:  UNVERIFIED — {reason}");
        }
    }

    if yes {
        return Ok(true);
    }
    if !std::io::stdin().is_terminal() {
        return Err(eyre::eyre!(
            "refusing to install {}@{} without confirmation in a non-interactive context; pass --yes to proceed",
            pending.name,
            pending.version
        )
        .into());
    }
    Ok(Confirm::new()
        .with_prompt(format!("Install {}@{}?", pending.name, pending.version))
        .interact()?)
}

/// Unconditionally warn (stderr, not gated by log level, plus a structured
/// `tracing::warn!`) that a plugin is being trusted without provenance
/// verification. Call whenever `--allow-unsigned` is what made an
/// install/update proceed.
pub fn warn_allow_unsigned(name: &str, version: &str) {
    eprintln!(
        "WARNING: installing {name}@{version} without provenance verification (--allow-unsigned). This plugin's origin has not been cryptographically verified."
    );
    tracing::warn!(
        name,
        version,
        "installing plugin without provenance verification (allow_unsigned)"
    );
}
