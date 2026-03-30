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
//! # OpenStack CLI core functionality

use comfy_table::ContentArrangement;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;

pub mod cli;
pub mod common;
pub mod config;
pub mod error;
pub mod output;
pub mod tracing_stats;

use crate::tracing_stats::HttpRequestStats;

/// Build a table of HTTP request timings
pub fn build_http_requests_timing_table(data: &HttpRequestStats) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(Vec::from(["Url", "Method", "Duration (ms)"]));

    let mut total_http_duration: u128 = 0;
    for rec in data.summarize_by_url_method() {
        total_http_duration += rec.2;
        table.add_row(vec![rec.0, rec.1, rec.2.to_string()]);
    }
    table.add_row(vec!["Total", "", &total_http_duration.to_string()]);
    table
}
