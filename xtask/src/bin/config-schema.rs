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

//! Standalone binary to generate JSON Schema for ConfigFile
//! and convert it to markdown documentation.

#[path = "../config_schema_bin/config_schema.rs"]
mod config_schema;

fn main() {
    let schema = schemars::schema_for!(openstack_sdk_core::config::ConfigFile);
    let json = serde_json::to_value(schema).expect("Failed to serialize schema");
    let mut out = std::io::BufWriter::new(std::fs::File::create("/tmp/schema.json").unwrap());
    serde_json::to_writer_pretty(&mut out, &json).unwrap();
    let markdown = config_schema::schema_to_markdown(&json);
    println!("{}", markdown);
}
