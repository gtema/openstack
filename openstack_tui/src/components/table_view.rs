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
use eyre::Result;
use itertools::Itertools;
use openstack_sdk::types::EntryStatus;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use serde_json::Value;
use std::{cmp, fmt::Display};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, instrument};

use crate::{
    action::Action,
    components::{
        Component, Frame,
        describe::Describe,
        search_input::{SEARCH_INPUT_HEIGHT, SearchInput},
    },
    config::{Config, ViewConfig},
    error::TuiError,
    mode::Mode,
};

const ITEM_HEIGHT: usize = 1;
const INFO_TEXT: &str =
    "(↑) move up | (↓) move down | (r) refresh | (/) search | (tab) switch to describe";
const INFO_TEXT_SEARCH: &str =
    "(enter) keep search | (esc) clear search | (↑) move up | (↓) move down";
const INFO_TEXT_DESCRIBE: &str = "(↑) move up | (↓) move down | (tab) switch to table";

/// Identifier of the entry used to match records between data updates
fn row_id(item: &Value) -> Option<&Value> {
    item.get("id").or(item.get("uuid"))
}

#[derive(Hash, Eq, PartialEq)]
enum Focus {
    Table,
    Describe,
}

pub struct TableViewComponentBase<'a, F>
where
    F: Default + Display,
{
    command_tx: Option<UnboundedSender<Action>>,
    pub config: Config,
    view_key: &'static str,

    state: TableState,
    scroll_state: ScrollbarState,

    raw_items: Vec<Value>,
    /// Indices into `raw_items` of the entries shown in the table (all of them unless a search
    /// query narrows the list). Table selection is a position in this list.
    visible: Vec<usize>,
    filter: F,
    /// Case-insensitive substring that at least one visible column of a row must contain
    search: SearchInput,

    /// Rendered headers, rows and statuses of all `raw_items` (before the search is applied)
    all_headers: Vec<String>,
    all_rows: Vec<Vec<String>>,
    all_statuses: Vec<Option<String>>,
    /// Lowercased text of every row in `all_rows` the search query is looked up in
    search_index: Vec<String>,

    column_widths: Vec<u16>,
    content_size: Size,
    table_headers: Row<'a>,
    table_rows: Vec<Vec<String>>,
    table_row_styles: Vec<Style>,
    describe: Describe,

    is_loading: bool,
    describe_enabled: bool,
    focus: Focus,
}

impl<F> TableViewComponentBase<'_, F>
where
    F: Default + Display,
{
    pub fn new(view_key: &'static str) -> Self {
        Self {
            command_tx: None,
            config: Config::default(),
            view_key,
            state: TableState::default().with_selected(0),
            raw_items: Vec::new(),
            visible: Vec::new(),
            filter: F::default(),
            search: SearchInput::default(),
            scroll_state: ScrollbarState::new(0),
            all_headers: Vec::new(),
            all_rows: Vec::new(),
            all_statuses: Vec::new(),
            search_index: Vec::new(),
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
        self.config.views.entry(self.view_key.into()).or_default()
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
                self.state
                    .select(Some(self.visible.len().saturating_sub(1)));
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
                        if i + 1 < self.visible.len() {
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
                        self.visible.len().saturating_sub(1),
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
        let data = self.get_selected().cloned().unwrap_or(Value::Null);
        self.describe.set_data(data)?;
        Ok(())
    }

    /// Start typing a search query. The already entered query is kept and can be edited.
    pub fn start_search(&mut self) -> Result<(), TuiError> {
        self.focus = Focus::Table;
        self.describe.set_focus(false)?;
        self.search.set_active(true);
        Ok(())
    }

    /// Whether the search query is currently being typed (all keys belong to the search)
    pub fn is_searching(&self) -> bool {
        self.search.is_active()
    }

    /// Edit the search query and re-apply it to the table when it changed
    fn edit_search(&mut self, edit: impl FnOnce(&mut SearchInput)) -> Result<(), TuiError> {
        let before = self.search.value().to_string();
        edit(&mut self.search);
        if self.search.value() != before {
            self.state.select_first();
            self.apply_search()?;
        }
        Ok(())
    }

    /// Drop the search query and stop typing it
    pub fn clear_search(&mut self) -> Result<(), TuiError> {
        self.search.set_active(false);
        self.edit_search(SearchInput::clear)
    }

    /// Handle key while the search query is typed. Returns `true` when the key is consumed.
    fn handle_search_key(&mut self, key: KeyEvent) -> Result<bool, TuiError> {
        match key.code {
            KeyCode::Esc => self.clear_search()?,
            KeyCode::Enter => self.search.set_active(false),
            KeyCode::Backspace => self.edit_search(SearchInput::pop)?,
            KeyCode::Char(c)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.edit_search(|search| search.push(c))?;
            }
            // List navigation keeps working while typing
            KeyCode::Up
            | KeyCode::Down
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::Home
            | KeyCode::End => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        if self.search.is_active() && self.handle_search_key(key)? {
            return Ok(None);
        }
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
        if data != self.raw_items {
            // Keep the cursor on the same entry when it is still present
            let selected_id = self.get_selected().and_then(row_id).cloned();
            self.raw_items = data;
            self.state.select_first();
            self.sync_table_data()?;
            if let Some(id) = selected_id
                && let Some(pos) = self
                    .visible
                    .iter()
                    .position(|&idx| row_id(&self.raw_items[idx]) == Some(&id))
            {
                self.state.select(Some(pos));
                self.scroll_state = self.scroll_state.position(pos * ITEM_HEIGHT);
                self.set_describe_content()?;
            }
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
                        for row in &mut rows {
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
                        for row in &mut rows {
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

    /// Synchronize table data from internal vector of raw entries
    pub fn sync_table_data(&mut self) -> Result<(), TuiError> {
        let view_config = self.get_output_config().clone();
        let headers = crate::components::view_render::headers(&view_config);
        let rows: Vec<Vec<String>> = self
            .raw_items
            .iter()
            .map(|item| {
                crate::components::view_render::row(item, &view_config)
                    .into_iter()
                    .map(|cell| cell.unwrap_or_default())
                    .collect()
            })
            .collect();
        let mut statuses: Vec<Option<String>> = self
            .raw_items
            .iter()
            .map(|item| crate::components::view_render::status(item, &view_config))
            .collect();
        let (table_headers, table_rows, _table_constraints) = self.prepare_table(headers, rows);

        // Ensure we have as many statuses as rows to zip them properly
        statuses.resize_with(table_rows.len(), Default::default);

        self.search_index = table_rows
            .iter()
            .map(|row| row.join("\u{1f}").to_lowercase())
            .collect();
        self.all_headers = table_headers;
        self.all_rows = table_rows;
        self.all_statuses = statuses;
        self.apply_search()
    }

    /// Narrow the rendered rows to the ones matching the search query and refresh the table
    fn apply_search(&mut self) -> Result<(), TuiError> {
        let table_headers = self.all_headers.clone();
        // Narrow the table to the rows with the search query in any of the displayed columns
        let needle = self.search.value().to_lowercase();
        self.visible = self
            .search_index
            .iter()
            .enumerate()
            .filter(|(_, text)| text.contains(&needle))
            .map(|(idx, _)| idx)
            .collect();
        let table_rows: Vec<Vec<String>> = self
            .visible
            .iter()
            .map(|&idx| self.all_rows[idx].clone())
            .collect();
        let statuses: Vec<Option<String>> = self
            .visible
            .iter()
            .map(|&idx| self.all_statuses[idx].clone())
            .collect();

        // Keep the selection within the (possibly shrunk) list
        match (self.visible.len(), self.state.selected()) {
            (0, _) => self.state.select(None),
            (len, Some(idx)) if idx >= len => self.state.select(Some(len - 1)),
            (_, None) => self.state.select_first(),
            _ => {}
        }
        self.scroll_state = ScrollbarState::new(self.visible.len().saturating_sub(1) * ITEM_HEIGHT)
            .position(self.state.selected().unwrap_or_default() * ITEM_HEIGHT);

        self.column_widths = table_headers
            .clone()
            .into_iter()
            .map(|col| col.len() + 1)
            .map(TryInto::<u16>::try_into)
            .collect::<Result<Vec<u16>, _>>()?;
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
                    table_headers[i].len().try_into()?,
                    cmp::max(
                        *self.column_widths.get(i).unwrap_or(&0),
                        val.len().try_into()?,
                    ),
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
        let updated_entry_id =
            row_id(&data).ok_or_else(|| TuiError::EntryIdNotPresent(data.clone()))?;
        for raw_item in self.raw_items.iter_mut() {
            if row_id(raw_item) == Some(updated_entry_id) {
                *raw_item = data.clone();
                self.sync_table_data()?;
                break;
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
        let search_height = if self.search.is_active() {
            SEARCH_INPUT_HEIGHT
        } else {
            0
        };
        let [content, search, footer] = Layout::vertical([
            Constraint::Min(5),
            Constraint::Length(search_height),
            Constraint::Length(3),
        ])
        .areas(area);

        self.describe_enabled = area.as_size().width >= 140;

        self.render_content(title, f, content)?;
        if self.search.is_active() {
            self.search.draw(f, search, &self.config);
        }
        self.render_footer(f, footer)?;
        Ok(())
    }

    pub fn render_table(&mut self, f: &mut Frame, area: Rect) -> Result<()> {
        let (table_border_color, table_border_type) = match self.focus {
            Focus::Table => (self.config.styles.border_fg, BorderType::Double),
            Focus::Describe => (tailwind::SLATE.c600, BorderType::Plain),
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(table_border_type)
            .border_style(Style::default().fg(table_border_color));

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
                    .map(|content| Cell::from(Text::from(content.clone())))
                    .collect::<Row>()
                    .style(row_style)
                    .height(1)
            });
        let t = Table::default()
            .header(header)
            .rows(rows)
            //         // + 1 is for padding.
            .widths(self.column_widths.iter().map(|v| Constraint::Length(v + 1)))
            .row_highlight_style(selected_style)
            .bg(self.config.styles.buffer_bg)
            .block(block)
            .highlight_spacing(HighlightSpacing::Always);

        f.render_stateful_widget(t, area, &mut self.state);

        if usize::from(self.content_size.height) < self.visible.len() {
            self.render_scrollbar(f, area)?;
        }
        Ok(())
    }

    pub fn render_scrollbar(&mut self, f: &mut Frame, area: Rect) -> Result<()> {
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
        Ok(())
    }

    pub fn render_footer(&mut self, f: &mut Frame, area: Rect) -> Result<()> {
        let info_footer = Paragraph::new(Line::from(if self.search.is_active() {
            INFO_TEXT_SEARCH
        } else {
            match self.focus {
                Focus::Table => INFO_TEXT,
                Focus::Describe => INFO_TEXT_DESCRIBE,
            }
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
        Ok(())
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
                if self.search.is_empty() {
                    format!(" ({}) ", self.raw_items.len())
                } else {
                    format!(" ({}/{}) ", self.visible.len(), self.raw_items.len())
                },
                self.config.styles.title_details_fg,
            ));
        }
        if !self.search.is_empty() {
            title.push(Span::styled(
                format!(" /{} ", self.search.value()),
                self.config.styles.title_filters_fg,
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

            self.render_table(frame, content)?;
            self.describe
                .set_focus(matches!(self.focus, Focus::Describe))?;
            self.describe.draw(frame, describe)?;
        } else {
            self.render_table(frame, inner)?;
        }

        Ok(())
    }

    pub fn get_selected(&self) -> Option<&Value> {
        self.state
            .selected()
            .and_then(|idx| self.visible.get(idx))
            .and_then(|&raw_idx| self.raw_items.get(raw_idx))
    }

    /// Get mutable reference to the row matching resource id
    #[instrument(level = "debug", skip(self))]
    pub fn get_item_row_by_res_id_mut(&mut self, search_id: &String) -> Option<&mut Value> {
        self.raw_items.iter_mut().find(|raw_item| {
            raw_item
                .get("id")
                .or(raw_item.get("uuid"))
                .is_some_and(|row_id| row_id == search_id)
        })
    }

    /// delete the row matching resource id
    #[instrument(level = "debug", skip(self))]
    pub fn delete_item_row_by_res_id_mut(&mut self, search_id: &String) -> Result<Option<usize>> {
        let item_idx = self.raw_items.iter().position(|raw_item| {
            raw_item
                .get("id")
                .or(raw_item.get("uuid"))
                .is_some_and(|row_id| row_id == search_id)
        });
        if let Some(idx) = item_idx {
            self.raw_items.remove(idx);
            self.sync_table_data()?;
        }
        self.set_loading(false);
        Ok(item_idx)
    }

    pub fn get_selected_resource_id(&self) -> Result<Option<String>, TuiError> {
        self.get_selected()
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
            if let Some(raw_value) = self.get_selected() {
                command_tx.send(Action::Mode {
                    mode: Mode::Describe,
                    stack: true,
                })?;
                command_tx.send(Action::SetDescribeApiResponseData(raw_value.clone()))?;
            } else {
                debug!("No current selected entry");
            }
        } else {
            debug!("No command_tx");
        }
        Ok(())
    }

    /// append new row.
    #[instrument(level = "debug", skip(self))]
    pub fn append_new_row(&mut self, data: Value) -> Result<(), TuiError> {
        self.raw_items.push(data);
        self.sync_table_data()?;
        self.set_loading(false);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn delete_item_row_by_res_id_mut_syncs_table_rows() {
        let mut base: TableViewComponentBase<'_, String> = TableViewComponentBase::new("test_view");
        base.set_data(vec![
            json!({"id": "a", "name": "foo"}),
            json!({"id": "b", "name": "bar"}),
        ])
        .unwrap();
        assert_eq!(base.table_rows.len(), 2);

        base.delete_item_row_by_res_id_mut(&"a".to_string())
            .unwrap();

        assert_eq!(base.raw_items.len(), 1);
        // Regression: table_rows is the cache render_table actually draws from --
        // deleting must resync it, not just raw_items, or the row stays visible.
        assert_eq!(base.table_rows.len(), 1);
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn type_text(base: &mut TableViewComponentBase<'_, String>, text: &str) {
        for c in text.chars() {
            base.handle_key_events(key(KeyCode::Char(c))).unwrap();
        }
    }

    fn search_base() -> TableViewComponentBase<'static, String> {
        let mut base: TableViewComponentBase<'_, String> = TableViewComponentBase::new("test_view");
        base.get_output_config().default_fields = vec!["id".into(), "name".into()];
        base.set_data(vec![
            json!({"id": "a", "name": "Foo"}),
            json!({"id": "b", "name": "bar"}),
            json!({"id": "c", "name": "food"}),
        ])
        .unwrap();
        base
    }

    #[test]
    fn search_narrows_rows_case_insensitively() {
        let mut base = search_base();
        base.start_search().unwrap();
        assert!(base.is_searching());
        type_text(&mut base, "FOO");

        assert_eq!(base.table_rows.len(), 2);
        assert_eq!(base.raw_items.len(), 3);
        assert_eq!(base.get_selected().unwrap()["id"], "a");
        base.cursor_down().unwrap();
        assert_eq!(base.get_selected().unwrap()["id"], "c");
    }

    #[test]
    fn search_matches_any_displayed_column() {
        let mut base = search_base();
        base.start_search().unwrap();
        type_text(&mut base, "b");
        // "b" is the id of the second row and part of the "bar" name
        assert_eq!(base.table_rows.len(), 1);
        assert_eq!(base.get_selected().unwrap()["id"], "b");
    }

    #[test]
    fn search_ignores_columns_that_are_not_displayed() {
        let mut base = search_base();
        base.set_data(vec![json!({"id": "a", "name": "foo", "hidden": "secret"})])
            .unwrap();
        base.start_search().unwrap();
        type_text(&mut base, "secret");
        assert!(base.table_rows.is_empty());
    }

    #[test]
    fn search_without_matches_is_safe_to_navigate() {
        let mut base = search_base();
        base.start_search().unwrap();
        type_text(&mut base, "zzz");
        assert!(base.table_rows.is_empty());
        assert!(base.get_selected().is_none());
        for code in [KeyCode::Down, KeyCode::PageDown, KeyCode::End, KeyCode::Up] {
            base.handle_key_events(key(code)).unwrap();
        }
        // Backspacing restores the rows and a selection
        for _ in 0..3 {
            base.handle_key_events(key(KeyCode::Backspace)).unwrap();
        }
        assert_eq!(base.table_rows.len(), 3);
        assert!(base.get_selected().is_some());
    }

    #[test]
    fn search_enter_keeps_and_esc_clears_query() {
        let mut base = search_base();
        base.start_search().unwrap();
        type_text(&mut base, "bar");
        base.handle_key_events(key(KeyCode::Enter)).unwrap();
        assert!(!base.is_searching());
        assert_eq!(base.table_rows.len(), 1);

        base.start_search().unwrap();
        base.handle_key_events(key(KeyCode::Esc)).unwrap();
        assert!(!base.is_searching());
        assert_eq!(base.table_rows.len(), 3);
    }

    #[test]
    fn search_survives_data_refresh_and_row_update() {
        let mut base = search_base();
        base.start_search().unwrap();
        type_text(&mut base, "foo");
        base.set_data(vec![
            json!({"id": "a", "name": "foo"}),
            json!({"id": "b", "name": "bar"}),
        ])
        .unwrap();
        assert_eq!(base.table_rows.len(), 1);
        base.update_row_data(json!({"id": "b", "name": "foobar"}))
            .unwrap();
        assert_eq!(base.table_rows.len(), 2);
    }

    #[test]
    fn search_keys_with_modifiers_are_not_typed() {
        let mut base = search_base();
        base.start_search().unwrap();
        base.handle_key_events(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL))
            .unwrap();
        assert_eq!(base.table_rows.len(), 3);
    }

    #[test]
    fn refresh_keeps_selected_entry() {
        let mut base = search_base();
        base.cursor_down().unwrap();
        assert_eq!(base.get_selected().unwrap()["id"], "b");
        base.set_data(vec![
            json!({"id": "z", "name": "new"}),
            json!({"id": "a", "name": "Foo"}),
            json!({"id": "b", "name": "bar2"}),
        ])
        .unwrap();
        assert_eq!(base.get_selected().unwrap()["id"], "b");

        // Selected entry gone: back to the first one
        base.set_data(vec![json!({"id": "z", "name": "new"})])
            .unwrap();
        assert_eq!(base.get_selected().unwrap()["id"], "z");
    }

    #[test]
    fn clear_search_restores_rows() {
        let mut base = search_base();
        base.start_search().unwrap();
        type_text(&mut base, "bar");
        base.clear_search().unwrap();
        assert!(!base.is_searching());
        assert_eq!(base.table_rows.len(), 3);
    }
}
