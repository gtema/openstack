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

//! mdbook preprocessor that replaces `{{#configschema}}` with
//! auto-generated markdown documentation from the ConfigFile JSON Schema.

#[path = "../config_schema_bin/config_schema.rs"]
mod config_schema;

use clap::{Arg, Command};
use mdbook_core::book::{Book, BookItem};
use mdbook_core::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext, parse_input};
use regex::Regex;
use semver::{Version, VersionReq};
use std::io;
use std::process;

static CONFIG_SCHEMA_REGEX: &str = r"\{\{\s*#configschema\s*\}\}";

pub fn make_app() -> Command {
    Command::new("config-schema-md")
        .about(
            "A mdbook preprocessor which replaces {{#configschema}} with markdown tables from ConfigFile JSON Schema",
        )
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = App::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook_core::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
              but we're being called from version {}",
            pre.name(),
            mdbook_core::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &clap::ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer).unwrap_or(true);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

/// The config-schema preprocessor.
pub struct App;

impl App {
    pub fn new() -> App {
        App
    }
}

impl Preprocessor for App {
    fn name(&self) -> &str {
        "config-schema-md"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let re = Regex::new(CONFIG_SCHEMA_REGEX)
            .map_err(|e| Error::msg(format!("Failed to compile regex: {}", e)))?;

        let schema = schemars::schema_for!(openstack_sdk_core::config::ConfigFile);
        let json = serde_json::to_value(schema)
            .map_err(|e| Error::msg(format!("Failed to serialize schema: {}", e)))?;
        let markdown = config_schema::schema_to_markdown(&json);

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                let content = chapter.content.clone();
                for cap in re.captures_iter(&content) {
                    if let Some(all) = cap.get(0) {
                        chapter.content = chapter.content.replace(all.as_str(), &markdown);
                    }
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer != "not-supported")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_schema_preprocessor_replaces_placeholder() {
        // Create a minimal book directly
        let mut book = mdbook_core::book::Book::new_with_items(vec![BookItem::Chapter(
            mdbook_core::book::Chapter {
                name: "Chapter 1".to_string(),
                content: "{{#configschema}}".to_string(),
                number: None,
                sub_items: vec![],
                path: Some("chapter_1.md".into()),
                source_path: Some("chapter_1.md".into()),
                parent_names: vec![],
            },
        )]);

        let schema = schemars::schema_for!(openstack_sdk_core::config::ConfigFile);
        let json = serde_json::to_value(&schema).unwrap();
        let markdown = config_schema::schema_to_markdown(&json);

        let re = Regex::new(CONFIG_SCHEMA_REGEX).unwrap();
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                let content = chapter.content.clone();
                for cap in re.captures_iter(&content) {
                    if let Some(all) = cap.get(0) {
                        chapter.content = chapter.content.replace(all.as_str(), &markdown);
                    }
                }
            }
        });

        // Verify the placeholder was replaced
        if let BookItem::Chapter(ref chapter) = book.items[0] {
            assert!(!chapter.content.contains("{{#configschema}}"));
            assert!(chapter.content.contains("## `Auth`"));
        } else {
            panic!("Expected chapter");
        }
    }
}
