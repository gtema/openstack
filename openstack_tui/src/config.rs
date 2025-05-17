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

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use derive_deref::{Deref, DerefMut};
use eyre::Result;
use ratatui::style::{Color, palette::tailwind};
use serde::{Deserialize, de::Deserializer};
use std::fmt;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::{Path, PathBuf},
};
use structable::OutputConfig;
use thiserror::Error;
use tracing::error;

use crate::{action::Action, mode::Mode};

const CONFIG: &str = include_str!("../.config/config.yaml");

//#[derive(Clone, Debug, Deserialize, Default)]
//pub struct AppConfig {
//    #[serde(default)]
//    pub _data_dir: PathBuf,
//    #[serde(default)]
//    pub _config_dir: PathBuf,
//}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    //#[serde(default, flatten)]
    //pub config: AppConfig,
    #[serde(default)]
    pub mode_keybindings: HashMap<Mode, KeyBindings>,
    #[serde(default)]
    pub global_keybindings: KeyBindings,
    /// Aliases for the mode (for use in the mode selector)
    #[serde(default)]
    pub mode_aliases: BTreeMap<String, Mode>,
    #[serde(default)]
    pub styles: Styles,
    #[serde(default)]
    pub views: HashMap<String, OutputConfig>,
}

/// Errors which may occur when dealing with OpenStack connection
/// configuration data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    #[error("failed to deserialize config: {}", source)]
    Parse {
        /// The source of the error.
        #[from]
        source: config::ConfigError,
    },
}

impl ConfigError {
    pub fn parse(source: config::ConfigError) -> Self {
        ConfigError::Parse { source }
    }
}

/// Errors which may occur when adding sources to the [`ConfigFileBuilder`].
#[derive(Error)]
#[non_exhaustive]
pub enum ConfigFileBuilderError {
    #[error("Failed to parse file {path:?}: {source}")]
    FileParse {
        source: Box<config::ConfigError>,
        builder: ConfigFileBuilder,
        path: PathBuf,
    },
    #[error("Failed to deserialize config {path:?}: {source}")]
    ConfigDeserialize {
        source: Box<config::ConfigError>,
        builder: ConfigFileBuilder,
        path: PathBuf,
    },
}

/// A builder to create a [`ConfigFile`] by specifying which files to load.
pub struct ConfigFileBuilder {
    sources: Vec<config::Config>,
}

impl ConfigFileBuilder {
    /// Add a source to the builder. This will directly parse the config and check if it is valid.
    /// Values of sources added first will be overridden by later added sources, if the keys match.
    /// In other words, the sources will be merged, with the later taking precedence over the
    /// earlier ones.
    pub fn add_source(mut self, source: impl AsRef<Path>) -> Result<Self, ConfigFileBuilderError> {
        let config = match config::Config::builder()
            .add_source(config::File::from(source.as_ref()))
            .build()
        {
            Ok(config) => config,
            Err(error) => {
                return Err(ConfigFileBuilderError::FileParse {
                    source: Box::new(error),
                    builder: self,
                    path: source.as_ref().to_owned(),
                });
            }
        };

        if let Err(error) = config.clone().try_deserialize::<Config>() {
            return Err(ConfigFileBuilderError::ConfigDeserialize {
                source: Box::new(error),
                builder: self,
                path: source.as_ref().to_owned(),
            });
        }

        self.sources.push(config);
        Ok(self)
    }

    /// This will build a [`ConfigFile`] with the previously specified sources. Since
    /// the sources have already been checked on errors, this will not fail.
    pub fn build(self) -> Config {
        let mut config = config::Config::builder();

        for source in self.sources {
            config = config.add_source(source);
        }

        config.build().unwrap().try_deserialize().unwrap()
    }
}

impl fmt::Debug for ConfigFileBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigFileBuilderError::FileParse { source, path, .. } => f
                .debug_struct("FileParse")
                .field("source", source)
                .field("path", path)
                .finish_non_exhaustive(),
            ConfigFileBuilderError::ConfigDeserialize { source, path, .. } => f
                .debug_struct("ConfigDeserialize")
                .field("source", source)
                .field("path", path)
                .finish_non_exhaustive(),
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let default_config: config::Config = config::Config::builder()
            .add_source(config::File::from_str(CONFIG, config::FileFormat::Yaml))
            .build()?;

        let config_dir = crate::utils::get_config_dir();
        let mut builder = ConfigFileBuilder {
            sources: Vec::from([default_config]),
        };

        let config_files = [
            ("config.yaml", config::FileFormat::Yaml),
            ("views.yaml", config::FileFormat::Yaml),
        ];
        let mut found_config = false;
        for (file, _format) in &config_files {
            if config_dir.join(file).exists() {
                found_config = true;

                builder = match builder.add_source(config_dir.join(file)) {
                    Ok(builder) => builder,
                    Err(ConfigFileBuilderError::FileParse { source, .. }) => {
                        return Err(ConfigError::parse(*source));
                    }
                    Err(ConfigFileBuilderError::ConfigDeserialize {
                        source,
                        builder,
                        path,
                    }) => {
                        error!(
                            "The file {path:?} could not be deserialized and will be ignored: {source}"
                        );
                        builder
                    }
                }
            }
        }
        if !found_config {
            tracing::error!("No configuration file found. Application may not behave as expected");
        }

        Ok(builder.build())
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Ord, PartialOrd)]
pub enum CommandType {
    /// ApiRequest action
    #[default]
    ApiRequestAction,
    /// Filter command
    Filter,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Command {
    /// Action
    pub action: Action,
    /// Action description
    pub description: Option<String>,
    /// Type
    #[serde(default)]
    pub r#type: CommandType,
}

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct KeyBindings(pub HashMap<Vec<KeyEvent>, Command>);

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct ModeKeyBindings(pub HashMap<Mode, KeyBindings>);

impl<'de> Deserialize<'de> for KeyBindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parsed_map = HashMap::<String, Command>::deserialize(deserializer)?;

        let keybindings: HashMap<Vec<KeyEvent>, Command> = parsed_map
            .into_iter()
            .map(|(key_str, command)| (parse_key_sequence(&key_str).unwrap(), command))
            .collect();

        Ok(KeyBindings(keybindings))
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ViewConfig {
    /// List of fields to be shown
    pub fields: BTreeSet<String>,
    /// Wide mode (additional fields requested)
    #[serde(default)]
    pub wide: bool,
}

fn parse_key_event(raw: &str) -> Result<KeyEvent, String> {
    let raw_lower = raw.to_ascii_lowercase();
    let (remaining, modifiers) = extract_modifiers(&raw_lower);
    parse_key_code_with_modifiers(remaining, modifiers)
}

fn extract_modifiers(raw: &str) -> (&str, KeyModifiers) {
    let mut modifiers = KeyModifiers::empty();
    let mut current = raw;

    loop {
        match current {
            rest if rest.starts_with("ctrl-") => {
                modifiers.insert(KeyModifiers::CONTROL);
                current = &rest[5..];
            }
            rest if rest.starts_with("alt-") => {
                modifiers.insert(KeyModifiers::ALT);
                current = &rest[4..];
            }
            rest if rest.starts_with("shift-") => {
                modifiers.insert(KeyModifiers::SHIFT);
                current = &rest[6..];
            }
            _ => break, // break out of the loop if no known prefix is detected
        };
    }

    (current, modifiers)
}

fn parse_key_code_with_modifiers(
    raw: &str,
    mut modifiers: KeyModifiers,
) -> Result<KeyEvent, String> {
    let c = match raw {
        "esc" => KeyCode::Esc,
        "enter" => KeyCode::Enter,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pageup" => KeyCode::PageUp,
        "pagedown" => KeyCode::PageDown,
        "backtab" => {
            modifiers.insert(KeyModifiers::SHIFT);
            KeyCode::BackTab
        }
        "backspace" => KeyCode::Backspace,
        "delete" => KeyCode::Delete,
        "insert" => KeyCode::Insert,
        "f1" => KeyCode::F(1),
        "f2" => KeyCode::F(2),
        "f3" => KeyCode::F(3),
        "f4" => KeyCode::F(4),
        "f5" => KeyCode::F(5),
        "f6" => KeyCode::F(6),
        "f7" => KeyCode::F(7),
        "f8" => KeyCode::F(8),
        "f9" => KeyCode::F(9),
        "f10" => KeyCode::F(10),
        "f11" => KeyCode::F(11),
        "f12" => KeyCode::F(12),
        "space" => KeyCode::Char(' '),
        "hyphen" => KeyCode::Char('-'),
        "minus" => KeyCode::Char('-'),
        "tab" => KeyCode::Tab,
        c if c.len() == 1 => {
            let mut c = c.chars().next().unwrap();
            if modifiers.contains(KeyModifiers::SHIFT) {
                c = c.to_ascii_uppercase();
            }
            KeyCode::Char(c)
        }
        _ => return Err(format!("Unable to parse {raw}")),
    };
    Ok(KeyEvent::new(c, modifiers))
}

pub fn key_event_to_string_with_unicode(key_event: &KeyEvent) -> String {
    let char;
    let key_code = match key_event.code {
        KeyCode::Backspace => "⌫",
        KeyCode::Enter => "↵",
        KeyCode::Left => "←",
        KeyCode::Right => "→",
        KeyCode::Up => "↑",
        KeyCode::Down => "↓",
        KeyCode::Home => "home",
        KeyCode::End => "end",
        KeyCode::PageUp => "pageup",
        KeyCode::PageDown => "pagedown",
        KeyCode::Tab => "↹",
        KeyCode::BackTab => "backtab",
        KeyCode::Delete => "⌦",
        KeyCode::Insert => "insert",
        KeyCode::F(c) => {
            char = format!("f({c})");
            &char
        }
        KeyCode::Char(' ') => "⎵",
        KeyCode::Char(c) => {
            char = c.to_string();
            &char
        }
        KeyCode::Esc => "␛",
        KeyCode::Null => "",
        KeyCode::CapsLock => "",
        KeyCode::Menu => "",
        KeyCode::ScrollLock => "",
        KeyCode::Media(_) => "",
        KeyCode::NumLock => "",
        KeyCode::PrintScreen => "",
        KeyCode::Pause => "",
        KeyCode::KeypadBegin => "",
        KeyCode::Modifier(_) => "",
    };

    let mut modifiers = Vec::with_capacity(3);

    if key_event.modifiers.intersects(KeyModifiers::CONTROL) {
        modifiers.push("ctrl");
    }

    if key_event.modifiers.intersects(KeyModifiers::SHIFT) {
        modifiers.push("⇧");
    }

    if key_event.modifiers.intersects(KeyModifiers::ALT) {
        modifiers.push("alt");
    }

    let mut key = modifiers.join("-");

    if !key.is_empty() {
        key.push('-');
    }
    key.push_str(key_code);

    key
}

pub fn key_event_to_string(key_event: &KeyEvent) -> String {
    let char;
    let key_code = match key_event.code {
        KeyCode::Backspace => "backspace",
        KeyCode::Enter => "enter",
        KeyCode::Left => "left",
        KeyCode::Right => "right",
        KeyCode::Up => "up",
        KeyCode::Down => "down",
        KeyCode::Home => "home",
        KeyCode::End => "end",
        KeyCode::PageUp => "pageup",
        KeyCode::PageDown => "pagedown",
        KeyCode::Tab => "tab",
        KeyCode::BackTab => "backtab",
        KeyCode::Delete => "delete",
        KeyCode::Insert => "insert",
        KeyCode::F(c) => {
            char = format!("f({c})");
            &char
        }
        KeyCode::Char(' ') => "space",
        KeyCode::Char(c) => {
            char = c.to_string();
            &char
        }
        KeyCode::Esc => "esc",
        KeyCode::Null => "",
        KeyCode::CapsLock => "",
        KeyCode::Menu => "",
        KeyCode::ScrollLock => "",
        KeyCode::Media(_) => "",
        KeyCode::NumLock => "",
        KeyCode::PrintScreen => "",
        KeyCode::Pause => "",
        KeyCode::KeypadBegin => "",
        KeyCode::Modifier(_) => "",
    };

    let mut modifiers = Vec::with_capacity(3);

    if key_event.modifiers.intersects(KeyModifiers::CONTROL) {
        modifiers.push("ctrl");
    }

    if key_event.modifiers.intersects(KeyModifiers::SHIFT) {
        modifiers.push("shift");
    }

    if key_event.modifiers.intersects(KeyModifiers::ALT) {
        modifiers.push("alt");
    }

    let mut key = modifiers.join("-");

    if !key.is_empty() {
        key.push('-');
    }
    key.push_str(key_code);

    key
}

pub fn parse_key_sequence(raw: &str) -> Result<Vec<KeyEvent>, String> {
    if raw.chars().filter(|c| *c == '>').count() != raw.chars().filter(|c| *c == '<').count() {
        return Err(format!("Unable to parse `{}`", raw));
    }
    let raw = if !raw.contains("><") {
        let raw = raw.strip_prefix('<').unwrap_or(raw);

        raw.strip_prefix('>').unwrap_or(raw)
    } else {
        raw
    };
    let sequences = raw
        .split("><")
        .map(|seq| {
            if let Some(s) = seq.strip_prefix('<') {
                s
            } else if let Some(s) = seq.strip_suffix('>') {
                s
            } else {
                seq
            }
        })
        .collect::<Vec<_>>();

    sequences.into_iter().map(parse_key_event).collect()
}

#[derive(Clone, Debug, Deserialize)]
pub struct Styles {
    pub table: TableColors,
    pub buffer_bg: Color,
    pub fg: Color,
    pub border_fg: Color,
    pub popup_bg: Color,
    pub popup_title_fg: Color,
    pub popup_item_title_fg: Color,
    pub popup_border_fg: Color,
    pub popup_border_error_fg: Color,
    pub popup_border_confirm_fg: Color,
    pub item_fg: Color,
    pub item_selected_bg: Color,
    pub item_highlight_fg: Color,
    pub title_loading_fg: Color,
    pub title_details_fg: Color,
    pub title_filters_fg: Color,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            table: TableColors::default(),
            buffer_bg: tailwind::SLATE.c950,
            fg: tailwind::WHITE,
            border_fg: tailwind::BLUE.c900,
            popup_bg: tailwind::BLUE.c900,
            popup_title_fg: tailwind::BLUE.c400,
            popup_item_title_fg: tailwind::SLATE.c200,
            popup_border_fg: tailwind::WHITE,
            popup_border_error_fg: tailwind::RED.c600,
            popup_border_confirm_fg: tailwind::BLUE.c600,
            item_selected_bg: tailwind::BLUE.c400,
            item_fg: tailwind::SLATE.c200,
            item_highlight_fg: tailwind::RED.c950,
            title_loading_fg: tailwind::PINK.c400,
            title_details_fg: tailwind::BLUE.c400,
            title_filters_fg: tailwind::VIOLET.c400,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TableColors {
    pub buffer_bg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub row_fg_error: Color,
    pub row_fg_processing: Color,
    pub row_fg_inactive: Color,
    pub row_fg_selected: Color,
    pub row_bg_normal: Color,
    pub row_bg_alt: Color,
    pub footer_border: Color,
}

impl Default for TableColors {
    fn default() -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: tailwind::BLUE.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            row_fg_error: tailwind::RED.c600,
            row_fg_processing: tailwind::FUCHSIA.c500,
            row_fg_inactive: tailwind::CYAN.c300,
            row_fg_selected: tailwind::BLUE.c400,
            row_bg_normal: tailwind::SLATE.c950,
            row_bg_alt: tailwind::SLATE.c900,
            footer_border: tailwind::BLUE.c400,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_simple_keys() {
        assert_eq!(
            parse_key_event("a").unwrap(),
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty())
        );

        assert_eq!(
            parse_key_event("enter").unwrap(),
            KeyEvent::new(KeyCode::Enter, KeyModifiers::empty())
        );

        assert_eq!(
            parse_key_event("esc").unwrap(),
            KeyEvent::new(KeyCode::Esc, KeyModifiers::empty())
        );
    }

    #[test]
    fn test_with_modifiers() {
        assert_eq!(
            parse_key_event("ctrl-a").unwrap(),
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)
        );

        assert_eq!(
            parse_key_event("alt-enter").unwrap(),
            KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT)
        );

        assert_eq!(
            parse_key_event("shift-esc").unwrap(),
            KeyEvent::new(KeyCode::Esc, KeyModifiers::SHIFT)
        );
    }

    #[test]
    fn test_multiple_modifiers() {
        assert_eq!(
            parse_key_event("ctrl-alt-a").unwrap(),
            KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::CONTROL | KeyModifiers::ALT
            )
        );

        assert_eq!(
            parse_key_event("ctrl-shift-enter").unwrap(),
            KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL | KeyModifiers::SHIFT)
        );
    }

    #[test]
    fn test_reverse_multiple_modifiers() {
        assert_eq!(
            key_event_to_string(&KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::CONTROL | KeyModifiers::ALT
            )),
            "ctrl-alt-a".to_string()
        );
    }

    #[test]
    fn test_invalid_keys() {
        assert!(parse_key_event("invalid-key").is_err());
        assert!(parse_key_event("ctrl-invalid-key").is_err());
    }

    #[test]
    fn test_case_insensitivity() {
        assert_eq!(
            parse_key_event("CTRL-a").unwrap(),
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)
        );

        assert_eq!(
            parse_key_event("AlT-eNtEr").unwrap(),
            KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT)
        );
    }
}
