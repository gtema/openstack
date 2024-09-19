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

use crossterm::event::KeyEvent;
use eyre::{OptionExt, Report, Result};
use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{block::*, *},
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{cmp, fmt::Display};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    components::{describe::Describe, Component, Frame},
    config::Config,
    utils::{OutputConfig, StructTable},
};

const ITEM_HEIGHT: usize = 1;
const INFO_TEXT: &str = "(↑) move up | (↓) move down | (r) refresh | (tab) switch to describe";
const INFO_TEXT_DESCRIBE: &str = "(↑) move up | (↓) move down | (tab) switch to table";
const DESCRIBE_TITLE: &str = " Describe ";

#[derive(Hash, Eq, PartialEq)]
enum Focus {
    Table,
    Describe,
}

pub struct TableViewComponentBase<'a, T, F>
where
    T: StructTable,
    Vec<T>: StructTable,
    T: DeserializeOwned,
    F: Default + Display,
{
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    last_events: Vec<KeyEvent>,
    output_config: OutputConfig,

    state: TableState,
    scroll_state: ScrollbarState,

    items: Vec<T>,
    raw_items: Vec<Value>,
    filter: F,

    column_widths: Vec<usize>,
    content_size: Size,
    describe_text: Vec<String>,
    table_headers: Row<'a>,
    table_rows: Vec<Vec<String>>,
    describe: Describe,

    is_loading: bool,
    describe_enabled: bool,
    focus: Focus,
}

impl<T, F> Default for TableViewComponentBase<'_, T, F>
where
    T: StructTable,
    Vec<T>: StructTable,
    T: DeserializeOwned,
    F: Default + Display,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, F> TableViewComponentBase<'_, T, F>
where
    T: StructTable,
    Vec<T>: StructTable,
    T: DeserializeOwned,
    F: Default + Display,
{
    pub fn new() -> Self {
        Self {
            command_tx: None,
            config: Config::default(),
            last_events: Vec::new(),
            state: TableState::default().with_selected(0),
            items: Vec::new(),
            raw_items: Vec::new(),
            filter: F::default(),
            scroll_state: ScrollbarState::new(0),
            column_widths: Vec::new(),
            content_size: Size::new(0, 0),
            describe_text: Vec::new(),
            output_config: OutputConfig::default(),
            table_headers: Row::default(),
            table_rows: Vec::new(),
            describe: Describe::new(),
            is_loading: false,
            describe_enabled: true,
            focus: Focus::Table,
        }
    }

    pub fn set_config(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    pub fn get_command_tx(&self) -> Option<&UnboundedSender<Action>> {
        self.command_tx.as_ref()
    }

    pub fn set_command_tx(&mut self, tx: UnboundedSender<Action>) {
        self.command_tx = Some(tx);
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    pub fn app_tick(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn render_tick(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn cursor_first(&mut self) -> Result<()> {
        match self.focus {
            Focus::Table => {
                self.state.select_first();
                self.scroll_state.first();
                self.set_describe_content()?;
            }
            Focus::Describe => {
                self.describe.cursor_first()?;
            }
        };
        Ok(())
    }

    pub fn cursor_last(&mut self) -> Result<()> {
        match self.focus {
            Focus::Table => {
                self.state.select(Some(self.items.len().saturating_sub(1)));
                self.scroll_state.last();
                self.set_describe_content()?;
            }
            Focus::Describe => {
                self.describe.cursor_last()?;
            }
        };
        Ok(())
    }

    pub fn cursor_down(&mut self) -> Result<()> {
        match self.focus {
            Focus::Table => {
                let i = match self.state.selected() {
                    Some(i) => {
                        if i < self.items.len() {
                            i + 1
                        } else {
                            i
                        }
                    }
                    None => 0,
                };
                self.state.select(Some(i));
                self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
                self.set_describe_content()?;
            }
            Focus::Describe => {
                self.describe.cursor_down()?;
            }
        };
        Ok(())
    }

    pub fn cursor_up(&mut self) -> Result<()> {
        match self.focus {
            Focus::Table => {
                let i = match self.state.selected() {
                    Some(i) => i.saturating_sub(1),
                    None => 0,
                };
                self.state.select(Some(i));
                self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
                self.set_describe_content()?;
            }
            Focus::Describe => {
                self.describe.cursor_up()?;
            }
        };
        Ok(())
    }

    pub fn cursor_page_down(&mut self) -> Result<()> {
        match self.focus {
            Focus::Table => {
                let i = match self.state.selected() {
                    Some(i) => cmp::min(
                        i.saturating_add(self.content_size.height as usize),
                        self.items.len() - 1,
                    ),
                    None => 0,
                };
                self.state.select(Some(i));
                self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
                self.set_describe_content()?;
            }
            Focus::Describe => {
                self.describe.cursor_page_down()?;
            }
        }
        Ok(())
    }

    pub fn cursor_page_up(&mut self) -> Result<()> {
        match self.focus {
            Focus::Table => {
                let i = match self.state.selected() {
                    Some(i) => i.saturating_sub(self.content_size.height as usize),
                    None => 0,
                };
                self.state.select(Some(i));
                self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
                self.set_describe_content()?;
            }
            Focus::Describe => {
                self.describe.cursor_page_up()?;
            }
        };
        Ok(())
    }

    pub fn cursor_left(&mut self) -> Result<()> {
        match self.focus {
            Focus::Table => {}
            Focus::Describe => {
                self.describe.cursor_left()?;
            }
        };
        Ok(())
    }

    pub fn cursor_right(&mut self) -> Result<()> {
        match self.focus {
            Focus::Table => {}
            Focus::Describe => {
                self.describe.cursor_right()?;
            }
        };
        Ok(())
    }

    pub fn key_tab(&mut self) -> Result<()> {
        if self.describe_enabled {
            self.focus = match self.focus {
                Focus::Table => Focus::Describe,
                Focus::Describe => Focus::Table,
            };
            self.describe
                .set_focus(matches!(self.focus, Focus::Describe))?;
        }
        Ok(())
    }

    pub fn set_describe_content(&mut self) -> Result<()> {
        if let Some(selected_idx) = self.state.selected() {
            if selected_idx < self.raw_items.len() {
                self.describe
                    .set_data(self.raw_items[selected_idx].clone())?;
            } else {
                self.describe.set_data(Value::Null)?;
            }
        } else {
            self.describe.set_data(Value::Null)?;
        }
        Ok(())
    }

    pub fn set_data(&mut self, data: Vec<Value>) -> Result<()> {
        let items: Vec<T> = serde_json::from_value(serde_json::Value::Array(data.clone()))?;
        if data != self.raw_items {
            self.items = items;
            self.raw_items = data.clone();
            self.state.select_first();
            self.scroll_state =
                ScrollbarState::new(self.items.len().saturating_sub(1) * ITEM_HEIGHT);
            let (headers, rows) = self.items.build(&self.output_config);
            self.column_widths.clear();
            self.column_widths.resize(headers.len(), 1);
            self.table_headers = headers.clone().into_iter().map(Cell::from).collect::<Row>();
            self.table_rows = rows;
            for row in &self.table_rows {
                for (i, val) in row.iter().enumerate() {
                    self.column_widths[i] = cmp::max(
                        headers[i].len(),
                        cmp::max(*self.column_widths.get(i).unwrap_or(&0), val.len()),
                    );
                }
            }
            self.set_describe_content()?;
        }
        self.set_loading(false);
        Ok(())
    }

    pub fn get_filters(&self) -> &F {
        &self.filter
    }

    pub fn set_filters(&mut self, filters: F) {
        self.filter = filters;
    }

    pub fn render_table(&mut self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::RIGHT)
            .padding(Padding::right(1))
            .border_style(Style::default().fg(self.config.styles.border_fg));

        self.content_size = block.inner(area).as_size();

        let header_style = Style::default()
            .fg(self.config.styles.table.header_fg)
            .bg(self.config.styles.table.header_bg);
        let selected_style = Style::default().add_modifier(Modifier::REVERSED).fg(self
            .config
            .styles
            .table
            .row_fg_selected);

        let header = self.table_headers.clone().style(header_style).height(1);
        let rows = self.table_rows.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.config.styles.table.row_bg_normal,
                _ => self.config.styles.table.row_bg_alt,
            };
            data.iter()
                .map(|content| Cell::from(Text::from(content.to_string())))
                .collect::<Row>()
                .style(Style::new().fg(self.config.styles.table.row_fg).bg(color))
                .height(1)
        });
        let t = Table::default()
            .header(header)
            .rows(rows)
            //         // + 1 is for padding.
            .widths(
                self.column_widths
                    .iter()
                    .map(|v| Constraint::Length((v + 1).try_into().unwrap())),
            )
            .highlight_style(selected_style)
            .bg(self.config.styles.buffer_bg)
            .block(block)
            .highlight_spacing(HighlightSpacing::Always);

        f.render_stateful_widget(t, area, &mut self.state);

        if usize::from(self.content_size.height) < self.items.len() {
            self.render_scrollbar(f, area);
        }
    }

    pub fn render_scrollbar(&mut self, f: &mut Frame, area: Rect) {
        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(self.config.styles.border_fg)),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    pub fn render_footer(&mut self, f: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Line::from(match self.focus {
            Focus::Table => INFO_TEXT,
            Focus::Describe => INFO_TEXT_DESCRIBE,
        }))
        .style(
            Style::new()
                .fg(self.config.styles.table.row_fg)
                .bg(self.config.styles.buffer_bg),
        )
        .centered()
        .block(
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(self.config.styles.table.footer_border)),
        );
        f.render_widget(info_footer, area);
    }

    pub fn render_content<S: AsRef<str>>(
        &mut self,
        title: S,
        frame: &mut Frame,
        area: Rect,
    ) -> Result<()> {
        let mut title = vec![title.as_ref().white()];
        if self.is_loading {
            title.push(Span::styled(" ...Loading... ", tailwind::PINK.c400));
        } else {
            title.push(Span::styled(
                format!(" ({}) ", self.items.len()),
                tailwind::BLUE.c400,
            ));
        }
        let filter = self.filter.to_string();
        if !filter.is_empty() {
            title.push(Span::styled(
                format!(" <{}> ", filter),
                tailwind::VIOLET.c400,
            ));
        }
        let block = Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(self.config.styles.border_fg));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.describe_enabled {
            let content_layout =
                Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
            let [content, describe] = content_layout.areas(inner);

            self.render_table(frame, content);
            self.describe
                .set_focus(matches!(self.focus, Focus::Describe))?;
            self.describe.draw(frame, describe)?;
        } else {
            self.render_table(frame, inner);
        }

        Ok(())
    }

    pub fn get_selected_raw(&self) -> &Value {
        &self.raw_items[self.state.selected().unwrap()]
    }

    pub fn get_selected_resource_id(&self) -> Result<String, Report> {
        self.get_selected_raw()
            .get("id")
            .ok_or_eyre("Resource ID must be known")?
            .as_str()
            .map(String::from)
            .ok_or_eyre("Resource ID must be string")
    }
}
