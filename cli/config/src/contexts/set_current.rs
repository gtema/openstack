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

//! Set the active (current) context.

use std::path::PathBuf;

use clap::Args;
use eyre::{WrapErr, eyre};
use zeroize::Zeroizing;

use openstack_cli_core::cli::CliArgs;
use openstack_cli_core::error::OpenStackCliError;
use openstack_sdk_core::config::contexts::ContextsFile;

use crate::clouds::add::{StagedFile, read_existing};
use crate::yaml_edit::YamlDocument;

/// Set the active context (`current-context` of `contexts.yaml`).
///
/// The context must exist. Comments and formatting of the file are kept.
#[derive(Debug, Args)]
pub struct UseCommand {
    /// Name of the context to activate.
    #[arg(value_name = "CONTEXT")]
    name: String,

    /// Path to the `contexts.yaml` (default: `--os-context-file` or the
    /// standard search locations).
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,
}

impl UseCommand {
    /// Perform command action
    pub fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        let path = super::resolve_contexts_path(
            self.file.as_deref(),
            parsed_args
                .global_opts()
                .connection
                .os_context_file
                .as_deref(),
        );
        let text = read_existing(&path)?
            .ok_or_else(|| eyre!("contexts file {} does not exist", path.display()))?;
        let file = ContextsFile::from_yaml(&text).map_err(|err| eyre!("{err}"))?;
        if !file.contexts.iter().any(|c| c.name == self.name) {
            return Err(eyre!(
                "context `{}` not found, available: {}",
                self.name,
                file.get_available_contexts().join(", ")
            )
            .into());
        }

        let mut doc = YamlDocument::parse(&text).map_err(|err| eyre!("{err}"))?;
        doc.set_top_level("current-context", &self.name)
            .map_err(|err| eyre!("{err}"))?;
        let content = Zeroizing::new(doc.source().to_string());
        // The result must still be a valid contexts file.
        ContextsFile::from_yaml(&content).map_err(|err| eyre!("{err}"))?;

        let secrets = file.contains_secrets();
        StagedFile::new(&path, &content, secrets)?
            .commit()
            .wrap_err_with(|| format!("cannot update {}", path.display()))?;
        println!("Switched to context `{}` in {}", self.name, path.display());
        Ok(())
    }
}
