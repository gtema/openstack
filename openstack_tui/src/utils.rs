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

use eyre::Result;
use lazy_static::lazy_static;
use ratatui::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, de, de::Visitor};
use serde_json::Value;
use std::fmt;
use std::path::PathBuf;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    self, Layer, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

const VERSION_MESSAGE: &str = concat!(env!("CARGO_PKG_VERSION"),);

pub trait ResourceKey {
    fn get_key() -> &'static str {
        ""
    }
}

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        std::env::var(format!("{}_DATA", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref CONFIG_FOLDER: Option<PathBuf> =
        std::env::var(format!("{}_CONFIG", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());
    pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
}

pub fn initialize_panic_handler() -> Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
        .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
        .issue_filter(|kind| match kind {
            color_eyre::ErrorKind::NonRecoverable(_) => true,
            color_eyre::ErrorKind::Recoverable(error) => !error.is::<std::fmt::Error>(),
        })
        .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
        .capture_span_trace_by_default(true)
        .display_location_section(true)
        .display_env_section(true)
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

pub fn get_data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("Cannot determine users XDG_DATA_HOME")
        .join(env!("CARGO_PKG_NAME"))
}

pub fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("Cannot determine users XDG_CONFIG_HOME")
        .join(env!("CARGO_PKG_NAME"))
}

pub fn initialize_logging() -> Result<()> {
    let directory = get_data_dir();
    std::fs::create_dir_all(directory.clone())?;
    let log_path = directory.join(LOG_FILE.clone());
    let log_file = std::fs::File::create(log_path)?;
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe {
        std::env::set_var(
            "RUST_LOG",
            std::env::var("RUST_LOG")
                .or_else(|_| std::env::var(LOG_ENV.clone()))
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
        )
    };
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());
    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// By default, the verbosity level for the generated events is `DEBUG`, but
/// this can be customized.
#[macro_export]
macro_rules! trace_dbg {
    (target: $target:expr_2021, level: $level:expr_2021, $ex:expr_2021) => {{
        match $ex {
            value => {
                tracing::event!(target: $target, $level, ?value, stringify!($ex));
                value
            }
        }
    }};
    (level: $level:expr_2021, $ex:expr_2021) => {
        trace_dbg!(target: module_path!(), level: $level, $ex)
    };
    (target: $target:expr_2021, $ex:expr_2021) => {
        trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr_2021) => {
        trace_dbg!(level: tracing::Level::DEBUG, $ex)
    };
}

pub fn version() -> String {
    let author = clap::crate_authors!();

    let config_dir_path = get_config_dir().display().to_string();
    let data_dir_path = get_data_dir().display().to_string();

    format!(
        "\
{VERSION_MESSAGE}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
    )
}

/// IntString (Integer or Integer as string)
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct IntString(u64);
impl fmt::Display for IntString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<'de> Deserialize<'de> for IntString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl Visitor<'_> for MyVisitor {
            type Value = IntString;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("integer or string")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(IntString(val))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<u64>() {
                    Ok(val) => self.visit_u64(val),
                    Err(_) => Ok(IntString(0)),
                }
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

pub(crate) fn as_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s,
        Value::Number(num) => num.to_string(),
        Value::Null => String::from(""),
        _ => return Err(de::Error::custom("Wrong type, expected boolean")),
    })
}

//pub trait StructTable {
//    /// Build a vector of headers and rows from the data
//    fn build(&self, options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>);
//    /// Get a status of entry
//    fn status(&self) -> Vec<Option<String>>;
//}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect_percent(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect_fixed(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(height),
        Constraint::Fill(1),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .split(popup_layout[1])[1]
}
