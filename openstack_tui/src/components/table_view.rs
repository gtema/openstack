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

use crossterm::event::{KeyCode, KeyEvent};
use eyre::Result;
use itertools::Itertools;
use openstack_sdk::types::EntryStatus;
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{cmp, fmt::Display};
use structable::{StructTable, build_list_table};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, instrument};

use crate::{
    action::Action,
    components::{Component, Frame, describe::Describe},
    config::{Config, ViewConfig},
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const ITEM_HEIGHT: usize = 1;
const INFO_TEXT: &str = "(↑) move up | (↓) move down | (r) refresh | (tab) switch to describe";
const INFO_TEXT_DESCRIBE: &str = "(↑) move up | (↓) move down | (tab) switch to table";

#[derive(Hash, Eq, PartialEq)]
enum Focus {
    Table,
    Describe,
}

pub struct TableViewComponentBase<'a, T, F>
where
    T: StructTable,
    T: DeserializeOwned,
    T: ResourceKey,
    F: Default + Display,
{
    command_tx: Option<UnboundedSender<Action>>,
    pub config: Config,

    state: TableState,
    scroll_state: ScrollbarState,

    items: Vec<T>,
    raw_items: Vec<Value>,
    filter: F,

    column_widths: Vec<usize>,
    content_size: Size,
    table_headers: Row<'a>,
    table_rows: Vec<Vec<String>>,
    table_row_styles: Vec<Style>,
    describe: Describe,

    is_loading: bool,
    describe_enabled: bool,
    focus: Focus,
}

impl<T, F> Default for TableViewComponentBase<'_, T, F>
where
    T: StructTable,
    for<'a> &'a T: StructTable,
    T: DeserializeOwned,
    T: ResourceKey,
    F: Default + Display,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, F> TableViewComponentBase<'_, T, F>
where
    T: StructTable,
    for<'a> &'a T: StructTable,
    T: DeserializeOwned,
    T: ResourceKey,
    F: Default + Display,
{
    pub fn new() -> Self {
        Self {
            command_tx: None,
            config: Config::default(),
            state: TableState::default().with_selected(0),
            items: Vec::new(),
            raw_items: Vec::new(),
            filter: F::default(),
            scroll_state: ScrollbarState::new(0),
            column_widths: Vec::new(),
            content_size: Size::new(0, 0),
            table_headers: Row::default(),
            table_rows: Vec::new(),
            table_row_styles: Vec::new(),
            describe: Describe::new(),
            is_loading: false,
            describe_enabled: true,
            focus: Focus::Table,
        }
    }

    pub fn set_config(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_command_tx(&self) -> Option<&UnboundedSender<Action>> {
        self.command_tx.as_ref()
    }

    pub fn get_output_config(&mut self) -> &mut ViewConfig {
        self.config.views.entry(T::get_key().into()).or_default()
    }

    pub fn set_command_tx(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.command_tx = Some(tx);
        Ok(())
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    pub fn app_tick(&mut self) -> Result<(), TuiError> {
        Ok(())
    }

    pub fn render_tick(&mut self) -> Result<(), TuiError> {
        Ok(())
    }

    pub fn cursor_first(&mut self) -> Result<(), TuiError> {
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

    pub fn cursor_last(&mut self) -> Result<(), TuiError> {
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

    pub fn cursor_down(&mut self) -> Result<(), TuiError> {
        match self.focus {
            Focus::Table => {
                let i = match self.state.selected() {
                    Some(i) => {
                        if i < self.items.len() - 1 {
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

    pub fn cursor_up(&mut self) -> Result<(), TuiError> {
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

    pub fn cursor_page_down(&mut self) -> Result<(), TuiError> {
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

    pub fn cursor_page_up(&mut self) -> Result<(), TuiError> {
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

    pub fn cursor_left(&mut self) -> Result<(), TuiError> {
        match self.focus {
            Focus::Table => {}
            Focus::Describe => {
                self.describe.cursor_left()?;
            }
        };
        Ok(())
    }

    pub fn cursor_right(&mut self) -> Result<(), TuiError> {
        match self.focus {
            Focus::Table => {}
            Focus::Describe => {
                self.describe.cursor_right()?;
            }
        };
        Ok(())
    }

    pub fn key_tab(&mut self) -> Result<(), TuiError> {
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

    pub fn set_describe_content(&mut self) -> Result<(), TuiError> {
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

    pub fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        match key.code {
            KeyCode::Down => self.cursor_down()?,
            KeyCode::Up => self.cursor_up()?,
            KeyCode::Home => self.cursor_first()?,
            KeyCode::End => self.cursor_last()?,
            KeyCode::PageUp => self.cursor_page_up()?,
            KeyCode::PageDown => self.cursor_page_down()?,
            KeyCode::Left => self.cursor_left()?,
            KeyCode::Right => self.cursor_right()?,
            KeyCode::Tab => self.key_tab()?,
            _ => {}
        }
        Ok(None)
    }

    pub fn set_data(&mut self, data: Vec<Value>) -> Result<(), TuiError> {
        let items = serde_json::from_value::<Vec<T>>(serde_json::Value::Array(data.clone()))
            //.map_err(|err| {
            //    TuiError::deserialize(
            //        err,
            //        serde_json::to_string(&serde_json::Value::Array(
            //            data.clone()
            //                .into_iter()
            //                .filter(|item| serde_json::from_value::<T>(item.clone()).is_err())
            //                .collect(),
            //        ))
            //        .unwrap_or_else(|v| format!("{:?}", v)),
            //    )
            //})
        ?;
        if data != self.raw_items {
            self.items = items;
            self.raw_items = data.clone();
            self.state.select_first();
            self.scroll_state =
                ScrollbarState::new(self.items.len().saturating_sub(1) * ITEM_HEIGHT);
            self.sync_table_data()?;
        }
        self.set_loading(false);
        Ok(())
    }

    /// Re-sort table according to the configuration and determine column constraints
    fn prepare_table(
        &mut self,
        headers: Vec<String>,
        data: Vec<Vec<String>>,
    ) -> (Vec<String>, Vec<Vec<String>>, Vec<Option<Constraint>>) {
        let mut headers = headers;
        let mut rows = data;
        let mut column_constrains: Vec<Option<Constraint>> = vec![None; headers.len()];

        let cfg = self.get_output_config();
        // Offset from the current iteration pointer
        if headers.len() > 1 {
            let mut idx_offset: usize = 0;
            for (default_idx, field) in cfg.default_fields.iter().unique().enumerate() {
                if let Some(curr_idx) = headers
                    .iter()
                    .position(|x| x.to_lowercase() == field.to_lowercase())
                {
                    // Swap headers between current and should pos
                    if default_idx - idx_offset < headers.len() {
                        headers.swap(default_idx - idx_offset, curr_idx);
                        for row in rows.iter_mut() {
                            // Swap also data columns
                            row.swap(default_idx - idx_offset, curr_idx);
                        }
                    }
                } else {
                    // This column is not found in the data. Perhars structable returned some
                    // other name. Move the column to the very end
                    if default_idx - idx_offset < headers.len() {
                        let curr_hdr = headers.remove(default_idx - idx_offset);
                        headers.push(curr_hdr);
                        for row in rows.iter_mut() {
                            let curr_cell = row.remove(default_idx - idx_offset);
                            row.push(curr_cell);
                        }
                        // Some unmatched field moved to the end. Our "current" index should respect
                        // the offset
                        idx_offset += 1;
                    }
                }
            }
        }
        // Find field configuration
        for (idx, field) in headers.iter().enumerate() {
            if let Some(field_config) = cfg
                .fields
                .iter()
                .find(|x| x.name.to_lowercase() == field.to_lowercase())
            {
                let constraint = match (
                    field_config.width,
                    field_config.min_width,
                    field_config.max_width,
                ) {
                    (Some(fixed), _, _) => Some(Constraint::Length(fixed as u16)),
                    (None, Some(lower), _) => Some(Constraint::Min(lower as u16)),
                    (None, None, Some(upper)) => Some(Constraint::Max(upper as u16)),
                    _ => None,
                };
                column_constrains[idx] = constraint;
            }
        }
        (headers, rows, column_constrains)
    }

    /// Synchronize table data from internal vector of typed entries
    pub fn sync_table_data(&mut self) -> Result<(), TuiError> {
        let view_config = self.get_output_config().clone();
        let data = build_list_table(self.items.iter(), &view_config);
        let (table_headers, table_rows, _table_constraints) = self.prepare_table(data.0, data.1);
        let mut statuses: Vec<Option<String>> =
            self.items.iter().map(|item| item.status()).collect();

        // Ensure we have as many statuses as rows to zip them properly
        statuses.resize_with(table_rows.len(), Default::default);

        self.column_widths = table_headers
            .clone()
            .into_iter()
            .map(|col| col.len() + 1)
            .collect::<Vec<usize>>();
        self.table_headers = table_headers
            .clone()
            .into_iter()
            .map(|x| x.to_uppercase())
            .map(Cell::from)
            .collect::<Row>();
        self.table_rows = table_rows;
        for row in &self.table_rows {
            for (i, val) in row.iter().enumerate() {
                self.column_widths[i] = cmp::max(
                    table_headers[i].len(),
                    cmp::max(*self.column_widths.get(i).unwrap_or(&0), val.len()),
                );
            }
        }
        self.table_row_styles = statuses
            .iter()
            .enumerate()
            .map(|(i, status)| {
                Style::new()
                    .fg(match EntryStatus::from(status.as_ref()) {
                        EntryStatus::Error => self.config.styles.table.row_fg_error,
                        EntryStatus::Pending => self.config.styles.table.row_fg_processing,
                        EntryStatus::Inactive => self.config.styles.table.row_fg_inactive,
                        _ => self.config.styles.table.row_fg,
                    })
                    .bg(match i % 2 {
                        0 => self.config.styles.table.row_bg_normal,
                        _ => self.config.styles.table.row_bg_alt,
                    })
            })
            .collect();

        self.set_describe_content()?;
        Ok(())
    }

    /// Update single record with the new data
    pub fn update_row_data(&mut self, data: Value) -> Result<(), TuiError> {
        let updated_item: T = serde_json::from_value(data.clone())?;
        let updated_entry_id = data
            .get("id")
            .or(data.get("uuid"))
            .ok_or_else(|| TuiError::EntryIdNotPresent(data.clone()))?;
        for (idx, raw_item) in self.raw_items.iter_mut().enumerate() {
            if let Some(row_id) = raw_item.get("id").or(raw_item.get("uuid")) {
                if row_id == updated_entry_id {
                    *raw_item = data.clone();
                    if let Some(typed_row) = self.items.get_mut(idx) {
                        *typed_row = updated_item;
                        self.sync_table_data()?;
                        break;
                    }
                }
            }
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

    pub fn draw(&mut self, f: &mut Frame<'_>, area: Rect, title: &str) -> Result<(), TuiError> {
        let areas = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(area);

        self.describe_enabled = area.as_size().width >= 140;

        self.render_content(title, f, areas[0])?;
        self.render_footer(f, areas[1]);
        Ok(())
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
        let rows = self
            .table_rows
            .iter()
            .zip(self.table_row_styles.clone())
            .map(|(data, row_style)| {
                data.iter()
                    .map(|content| Cell::from(Text::from(content.to_string())))
                    .collect::<Row>()
                    .style(row_style)
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
            .row_highlight_style(selected_style)
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
    ) -> Result<(), TuiError> {
        let mut title = vec![title.as_ref().white()];
        if self.is_loading {
            title.push(Span::styled(
                " ...Loading... ",
                self.config.styles.title_loading_fg,
            ));
        } else {
            title.push(Span::styled(
                format!(" ({}) ", self.items.len()),
                self.config.styles.title_details_fg,
            ));
        }
        let filter = self.filter.to_string();
        if !filter.is_empty() {
            title.push(Span::styled(
                format!(" <{filter}> "),
                self.config.styles.title_filters_fg,
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
                Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]);
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

    pub fn get_selected(&self) -> Option<&T> {
        self.state.selected().map(|x| &self.items[x])
    }

    /// Get mutable reference to the row with the typed data matching resource id
    #[instrument(level = "debug", skip(self))]
    pub fn get_item_row_by_res_id_mut(&mut self, search_id: &String) -> Option<&mut T> {
        for (idx, raw_item) in self.raw_items.iter_mut().enumerate() {
            if let Some(row_item_id) = raw_item.get("id").or(raw_item.get("uuid")) {
                if row_item_id == search_id {
                    return self.items.get_mut(idx);
                }
            }
        }
        None
    }

    /// delete the row with the typed data matching resource id
    #[instrument(level = "debug", skip(self))]
    pub fn delete_item_row_by_res_id_mut(&mut self, search_id: &String) -> Result<Option<usize>> {
        let mut item_idx: Option<usize> = None;
        for (idx, raw_item) in self.raw_items.iter_mut().enumerate() {
            if let Some(row_item_id) = raw_item.get("id").or(raw_item.get("uuid")) {
                if row_item_id == search_id {
                    item_idx = Some(idx);
                    break;
                }
            }
        }
        if let Some(idx) = item_idx {
            self.raw_items.remove(idx);
            self.items.remove(idx);
        }
        Ok(item_idx)
    }

    pub fn get_selected_raw(&self) -> Option<&Value> {
        self.state.selected().map(|x| &self.raw_items[x])
    }

    pub fn get_selected_resource_id(&self) -> Result<Option<String>, TuiError> {
        self.get_selected_raw()
            .map(|entry| {
                entry
                    .get("id")
                    .and_then(|x| x.as_str().map(String::from))
                    .ok_or_else(|| TuiError::EntryIdNotPresent(entry.clone()))
            })
            .transpose()
    }

    pub fn describe_selected_entry(&self) -> Result<(), TuiError> {
        if let Some(command_tx) = self.get_command_tx() {
            // and have a selected entry
            if let Some(raw_value) = self.get_selected_raw() {
                command_tx.send(Action::SetDescribeApiResponseData(raw_value.clone()))?;
                command_tx.send(Action::Mode {
                    mode: Mode::Describe,
                    stack: true,
                })?;
            } else {
                debug!("No current selected entry");
            }
        } else {
            debug!("No command_tx");
        }
        Ok(())
    }
}
