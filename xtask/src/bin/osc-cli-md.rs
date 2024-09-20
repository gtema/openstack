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

#![deny(dead_code, unused_imports, unused_mut)]

use crate::osc_cli_md_lib::App;
use clap::builder::PossibleValue;
use clap::{Arg, ArgMatches, Command, CommandFactory};
use lazy_static::lazy_static;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::{book::Chapter, BookItem};
use openstack_cli::Cli;
use regex::Regex;
use semver::{Version, VersionReq};
use std::fmt::{self, Write};
use std::process;
use std::{io, path::PathBuf};
use xtask::{get_canonical_name, indent};

lazy_static! {
    static ref OSC: Regex = Regex::new(
        r"(?x)              # insignificant whitespace mode
        \\\{\{\#.*\}\}      # match escaped link
        |                   # or
        \{\{\s*             # link opening parens and whitespace
        \#([a-zA-Z0-9_]+)   # link type
        \s+                 # separating whitespace
        ([^}]+)             # link target path and space separated properties
        \}\}                # link closing parens",
    )
    .unwrap();
}

pub fn make_app() -> Command {
    Command::new("osc-cli-md")
        .about(
            "A mdbook preprocessor which converts {{#cmd osc}} into help markdown if the `osc` cli",
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
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

/// The actual implementation of the `Nop` preprocessor. This would usually go
/// in your main `lib.rs` file.
mod osc_cli_md_lib {
    use super::*;

    pub fn process_command_tree(
        parent_command_path: Vec<String>,
        command: &clap::Command,
        chapter: &mut Chapter,
    ) -> Result<Option<String>, Error> {
        let mut buffer = String::with_capacity(100);
        // Don't document commands marked with `clap(hide = true)` (which includes
        // `print-all-help`).
        if command.is_hide_set() {
            return Ok(None);
        }

        let title_name = get_canonical_name(command);
        // Append the name of `command` to `command_path`.
        let command_path = {
            let mut command_path = parent_command_path.clone();
            command_path.push(title_name.clone());
            command_path
        };
        writeln!(buffer, "## `{}`\n", command_path.join(" "),)?;
        if let Some(long_about) = command.get_long_about() {
            writeln!(buffer, "{}\n", long_about)?;
        } else if let Some(about) = command.get_about() {
            writeln!(buffer, "{}\n", about)?;
        }

        if let Some(help) = command.get_before_long_help() {
            writeln!(buffer, "{}\n", help)?;
        } else if let Some(help) = command.get_before_help() {
            writeln!(buffer, "{}\n", help)?;
        }

        writeln!(
            buffer,
            "**Usage:** `{}{}`\n",
            if parent_command_path.is_empty() {
                String::new()
            } else {
                let mut s = parent_command_path.join(" ");
                s.push(' ');
                s
            },
            command
                .clone()
                .render_usage()
                .to_string()
                .replace("Usage: ", "")
        )?;
        if let Some(help) = command.get_after_long_help() {
            writeln!(buffer, "{}\n", help)?;
        } else if let Some(help) = command.get_after_help() {
            writeln!(buffer, "{}\n", help)?;
        }

        //----------------------------------
        // Subcommands
        //----------------------------------

        if command.get_subcommands().next().is_some() {
            writeln!(buffer, "### **Subcommands:**\n")?;

            for subcommand in command.get_subcommands() {
                if subcommand.is_hide_set() {
                    continue;
                }

                let subcommand_title_name = get_canonical_name(subcommand);

                let about = match subcommand.get_about() {
                    Some(about) => about.to_string(),
                    None => String::new(),
                };

                writeln!(
                    buffer,
                    "* [`{subcommand_title_name}`]({}/{}.html) — {about}",
                    title_name, subcommand_title_name
                )?;
            }

            writeln!(buffer)?;
        }

        //----------------------------------
        // Arguments
        //----------------------------------

        if command.get_positionals().next().is_some() {
            writeln!(buffer, "### **Arguments:**\n")?;

            for pos_arg in command.get_positionals() {
                write_arg_markdown(&mut buffer, pos_arg)?;
            }

            writeln!(buffer)?;
        }

        //----------------------------------
        // Options
        //----------------------------------

        let non_pos: Vec<_> = command
            .get_arguments()
            .filter(|arg| !arg.is_positional() && !arg.is_hide_set())
            .collect();

        if !non_pos.is_empty() {
            writeln!(buffer, "### **Options:**\n")?;

            for arg in non_pos {
                write_arg_markdown(&mut buffer, arg)?;
            }

            write!(buffer, "\n")?;
        }

        //----------------------------------
        // Recurse to write subcommands
        //----------------------------------
        let mut command_dest_path = PathBuf::new();
        command_path.iter().for_each(|x| command_dest_path.push(x));
        let mut cnt: u32 = 0;
        for subcommand in command.get_subcommands() {
            let subcommand_name = get_canonical_name(subcommand);
            let mut subchapter_num = chapter.number.clone();
            let mut path = command_dest_path.clone();
            path.push(format!("{}.html", subcommand_name));
            let mut parents = chapter.parent_names.clone();
            parents.push(chapter.name.clone());
            if let Some(ref mut num) = subchapter_num {
                cnt += 1;
                num.push(cnt);
            }

            let mut chap = Chapter {
                name: subcommand_name,
                content: String::new(),
                number: subchapter_num.clone(),
                sub_items: Vec::new(),
                path: Some(path.clone()),
                source_path: None,
                parent_names: parents,
            };

            if let Ok(Some(subcommand_content)) =
                process_command_tree(command_path.clone(), subcommand, &mut chap)
            {
                chap.content = subcommand_content;
                chapter.sub_items.push(BookItem::Chapter(chap));
            } else {
                cnt -= 1;
            }
        }
        Ok(Some(buffer))
    }

    pub fn write_arg_markdown(buffer: &mut String, arg: &clap::Arg) -> fmt::Result {
        // Markdown list item
        write!(buffer, "* ")?;

        let value_name: String = match arg.get_value_names() {
            // TODO: What if multiple names are provided?
            Some([name, ..]) => name.as_str().to_owned(),
            Some([]) => unreachable!("clap Arg::get_value_names() returned Some(..) of empty list"),
            None => arg.get_id().to_string().to_ascii_uppercase(),
        };

        match (arg.get_short(), arg.get_long()) {
            (Some(short), Some(long)) => {
                if arg.get_action().takes_values() {
                    write!(buffer, "`-{short}`, `--{long} <{value_name}>`")?
                } else {
                    write!(buffer, "`-{short}`, `--{long}`")?
                }
            }
            (Some(short), None) => {
                if arg.get_action().takes_values() {
                    write!(buffer, "`-{short} <{value_name}>`")?
                } else {
                    write!(buffer, "`-{short}`")?
                }
            }
            (None, Some(long)) => {
                if arg.get_action().takes_values() {
                    write!(buffer, "`--{} <{value_name}>`", long)?
                } else {
                    write!(buffer, "`--{}`", long)?
                }
            }
            (None, None) => {
                debug_assert!(
                    arg.is_positional(),
                    "unexpected non-positional Arg with neither short nor long name: {arg:?}"
                );

                write!(buffer, "`<{value_name}>`",)?;
            }
        }

        if let Some(help) = arg.get_long_help() {
            // TODO: Parse formatting in the string
            buffer.push_str(&indent(&help.to_string(), " — ", "   "))
        } else if let Some(short_help) = arg.get_help() {
            writeln!(buffer, " — {short_help}")?;
        } else {
            writeln!(buffer)?;
        }

        //--------------------
        // Arg default values
        //--------------------

        if !arg.get_default_values().is_empty() {
            let default_values: String = arg
                .get_default_values()
                .iter()
                .map(|value| format!("`{}`", value.to_string_lossy()))
                .collect::<Vec<String>>()
                .join(", ");

            if arg.get_default_values().len() > 1 {
                // Plural
                writeln!(buffer, "\n  Default values: {default_values}")?;
            } else {
                // Singular
                writeln!(buffer, "\n  Default value: {default_values}")?;
            }
        }

        //--------------------
        // Arg possible values
        //--------------------

        let possible_values: Vec<PossibleValue> = arg
            .get_possible_values()
            .into_iter()
            .filter(|pv| !pv.is_hide_set())
            .collect();

        // Print possible values for options that take a value, but not for flags
        // that can only be either present or absent and do not take a value.
        if !possible_values.is_empty() && !matches!(arg.get_action(), clap::ArgAction::SetTrue) {
            let any_have_help: bool = possible_values.iter().any(|pv| pv.get_help().is_some());

            if any_have_help {
                // If any of the possible values have help text, print them
                // as a separate item in a bulleted list, and include the
                // help text for those that have it. E.g.:
                //
                //     Possible values:
                //     - `value1`:
                //       The help text
                //     - `value2`
                //     - `value3`:
                //       The help text

                let text: String = possible_values
                    .iter()
                    .map(|pv| match pv.get_help() {
                        Some(help) => {
                            format!("  - `{}`:\n    {}\n", pv.get_name(), help)
                        }
                        None => format!("  - `{}`\n", pv.get_name()),
                    })
                    .collect::<Vec<String>>()
                    .join("");

                writeln!(buffer, "\n  Possible values:\n{text}")?;
            } else {
                // If none of the possible values have any documentation, print
                // them all inline on a single line.
                let text: String = possible_values
                    .iter()
                    // TODO: Show PossibleValue::get_help(), and PossibleValue::get_name_and_aliases().
                    .map(|pv| format!("`{}`", pv.get_name()))
                    .collect::<Vec<String>>()
                    .join(", ");

                writeln!(buffer, "\n  Possible values: {text}\n")?;
            }
        }

        Ok(())
    }

    /// A preprocessor.
    pub struct App;

    impl App {
        pub fn new() -> App {
            App
        }
    }

    impl Preprocessor for App {
        fn name(&self) -> &str {
            "osc cli markdown help generator"
        }

        fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
            book.for_each_mut(|item: &mut BookItem| {
                if let BookItem::Chapter(ref mut chapter) = *item {
                    let content = &chapter.content;
                    for cap in OSC.captures_iter(&content.clone()) {
                        if let (Some(all), Some(typ), Some(subcmd)) =
                            (cap.get(0), cap.get(1), cap.get(2))
                        {
                            if typ.as_str() != "cmd" || subcmd.as_str() != "osc" {
                                break;
                            }

                            if let Ok(Some(command_content)) =
                                process_command_tree(Vec::new(), &Cli::command(), chapter)
                            {
                                chapter.content =
                                    chapter.content.replace(all.as_str(), &command_content);
                            }
                        };
                    }
                }
            });

            Ok(book)
        }

        fn supports_renderer(&self, renderer: &str) -> bool {
            renderer != "not-supported"
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn nop_preprocessor_run() {
            let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "nop": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "{{#cmd osc}}",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
            let input_json = input_json.as_bytes();

            let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
            let expected_book = book.clone();
            let result = App::new().run(&ctx, book);
            assert!(result.is_ok());

            // The nop-preprocessor should not have made any changes to the book content.
            let actual_book = result.unwrap();
            assert_ne!(actual_book, expected_book);
        }
    }
}
