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

//! OpenStack CLI
//!
//! The binary of the CLI
#![deny(missing_docs)]

use color_eyre::eyre::{Report, Result};
use color_eyre::owo_colors::OwoColorize;
use color_eyre::section::PanicMessage;
use std::env;
use std::{fmt, panic::Location};

#[tokio::main]
async fn main() -> Result<(), Report> {
    initialize_panic_handler()?;
    openstack_cli::entry_point().await?;
    Ok(())
}

struct MyPanicMessage;

impl PanicMessage for MyPanicMessage {
    fn display(&self, pi: &std::panic::PanicInfo<'_>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "The application panicked (crashed).".red())?;
        writeln!(
            f,
            "{}",
            "This is very sad, but you can help with the issue diagnose by reporting the issue."
                .red()
        )?;

        // Print panic message.
        writeln!(f, "\n{}", "What is known:".red())?;
        writeln!(f, "\n{}", "Crash information:".yellow())?;
        let payload = pi
            .payload()
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| pi.payload().downcast_ref::<&str>().cloned())
            .unwrap_or("<non string panic payload>");

        // Print command
        write!(f, "{}", "Command:  ".yellow())?;
        writeln!(f, "{}", env::args().collect::<Vec<_>>().join(" ").purple())?;

        // Error message
        write!(f, "{}", "Message:  ".yellow())?;
        writeln!(f, "{}", payload.cyan())?;

        // If known, print panic location.
        write!(f, "{}", "Location: ".yellow())?;
        if let Some(loc) = pi.location() {
            write!(f, "{}", loc.file().purple())?;
            write!(f, ":")?;
            writeln!(f, "{}", loc.line().purple())?;
        } else {
            writeln!(f, "<unknown>")?;
        }
        // Error message
        write!(f, "{}", "Version:  ".yellow())?;
        writeln!(f, "{}", env!("CARGO_PKG_VERSION").purple())?;

        Ok(())
    }
}

/// Initialize panic handling
fn initialize_panic_handler() -> Result<()> {
    let command = env::args().collect::<Vec<_>>().join("+");
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
        .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
        .issue_filter(|kind| match kind {
            color_eyre::ErrorKind::NonRecoverable(_) => true,
            color_eyre::ErrorKind::Recoverable(error) => !error.is::<std::fmt::Error>(),
        })
        .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
        .add_issue_metadata("command", command)
        .panic_message(MyPanicMessage)
        .capture_span_trace_by_default(true)
        .display_location_section(true)
        .display_env_section(false)
        .into_hooks();
    eyre_hook.install()?;
    std::panic::set_hook(Box::new(move |panic_info| {
        #[cfg(not(debug_assertions))]
        {
            eprintln!("\n{}", panic_hook.panic_report(panic_info)); // prints color-eyre stack trace to stderr
        }
        let msg = format!("{}", panic_hook.panic_report(panic_info));
        tracing::error!("Error: {}", strip_ansi_escapes::strip_str(msg));

        #[cfg(debug_assertions)]
        {
            eprintln!("\n{}", panic_hook.panic_report(panic_info)); // prints color-eyre stack trace to stderr
        }

        std::process::exit(1);
    }));
    Ok(())
}
