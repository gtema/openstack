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

//! Edit `contexts.yaml` in the default editor.

use std::path::PathBuf;
use std::process::Command;

use clap::Args;
use eyre::{WrapErr, eyre};
use zeroize::Zeroizing;

use openstack_cli_core::cli::CliArgs;
use openstack_cli_core::error::OpenStackCliError;
use openstack_sdk_core::config::contexts::ContextsFile;

use crate::clouds::add::{StagedFile, read_existing};

/// Content used when the file does not exist yet.
const TEMPLATE: &str = "\
# contexts.yaml: combine servers, users and scopes into contexts.
# current-context: my-context
servers: []
users: []
scopes: []
contexts: []
";

/// Open `contexts.yaml` in the default editor.
///
/// The editor is taken from `$VISUAL`, then `$EDITOR`, falling back to `vi`.
/// The file is edited as a copy and validated afterwards: when it is not a
/// valid contexts file the original is left untouched and the edited copy
/// is kept so the changes are not lost.
#[derive(Debug, Args)]
pub struct EditCommand {
    /// Path to the `contexts.yaml` (default: `--os-context-file` or the
    /// standard search locations).
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,
}

impl EditCommand {
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
        let original = read_existing(&path)?;
        let initial = Zeroizing::new(original.clone().unwrap_or_else(|| TEMPLATE.to_string()));

        let mut temp = tempfile::Builder::new()
            .prefix("osc-contexts-")
            .suffix(".yaml")
            .tempfile()
            .wrap_err("cannot create a temporary file")?;
        std::io::Write::write_all(&mut temp, initial.as_bytes())
            .wrap_err("cannot write the temporary file")?;

        run_editor(temp.path())?;

        let edited = Zeroizing::new(
            std::fs::read_to_string(temp.path()).wrap_err("cannot read the edited file")?,
        );
        if *edited == *initial {
            println!("No changes made");
            return Ok(());
        }
        match validate_and_commit(&path, original.as_deref(), &edited) {
            Ok(()) => {
                println!("Updated {}", path.display());
                Ok(())
            }
            Err(err) => {
                let kept = temp
                    .into_temp_path()
                    .keep()
                    .wrap_err("cannot keep the edited copy")?;
                Err(eyre!(
                    "{err}. {} was not changed, the edited copy is kept in {}",
                    path.display(),
                    kept.display()
                )
                .into())
            }
        }
    }
}

/// Validate the edited text and write it over `path`.
///
/// The checks of loading the file apply to the result as they do on every
/// other read: an `auth_helper` is refused while group or others can modify
/// the file, secrets in a file they can read are warned about. The file must
/// also be as it was before the editor was started.
fn validate_and_commit(
    path: &std::path::Path,
    original: Option<&str>,
    edited: &str,
) -> Result<(), eyre::Report> {
    let file = ContextsFile::from_yaml(edited)
        .map_err(|err| eyre!("the edited file is not a valid contexts file: {err}"))?;
    let secrets = file.contains_secrets();
    // A new file is created owner-only when it holds secrets; the mode of an
    // existing one is kept, so that is the one to check.
    #[cfg(unix)]
    if let Ok(meta) = std::fs::metadata(path) {
        use std::os::unix::fs::PermissionsExt;
        let mode = meta.permissions().mode();
        if file.users.iter().any(|u| u.user.auth_helper.is_some()) && mode & 0o022 != 0 {
            return Err(eyre!(
                "the edited file sets an `auth_helper` but {} can be modified by group or \
                 others, restrict its permissions (i.e. `chmod 600`)",
                path.display()
            ));
        }
        if secrets && mode & 0o077 != 0 {
            eprintln!(
                "warning: contexts file {} contains secrets and is accessible by group or \
                 others, restrict its permissions (i.e. `chmod 600`)",
                path.display()
            );
        }
    }
    super::ensure_unchanged(path, original)?;
    StagedFile::new(path, edited, secrets)?
        .commit()
        .wrap_err_with(|| format!("cannot update {}", path.display()))
}

/// The command running the editor of the environment on `path`.
///
/// The value of `$VISUAL`/`$EDITOR` may carry arguments and quoting (as with
/// git, `EDITOR='"/opt/My Editor/ed" --wait'`), so on unix the shell
/// interprets it.
fn editor_command(editor: &str, path: &std::path::Path) -> Result<Command, eyre::Report> {
    #[cfg(unix)]
    let mut command = {
        let mut command = Command::new("sh");
        command.arg("-c").arg(format!("{editor} \"$@\"")).arg("sh");
        command
    };
    #[cfg(not(unix))]
    let mut command = {
        let mut parts = editor.split_whitespace();
        let program = parts.next().ok_or_else(|| eyre!("no editor configured"))?;
        let mut command = Command::new(program);
        command.args(parts);
        command
    };
    command.arg(path);
    Ok(command)
}

/// Run the user's editor on `path`.
fn run_editor(path: &std::path::Path) -> Result<(), eyre::Report> {
    let editor = ["VISUAL", "EDITOR"]
        .iter()
        .find_map(|var| std::env::var(var).ok().filter(|v| !v.trim().is_empty()))
        .unwrap_or_else(|| "vi".to_string());
    let status = editor_command(&editor, path)?
        .status()
        .wrap_err_with(|| format!("cannot run the editor `{editor}`"))?;
    if !status.success() {
        return Err(eyre!("the editor `{editor}` exited with {status}"));
    }
    Ok(())
}
