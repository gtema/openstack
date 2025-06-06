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
//! TUI Cli parameters
use clap::{Parser, ValueHint};
use std::path::PathBuf;

use crate::utils::version;

/// TUI Cli parameters
#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    /// Number of ticks per second. May be used by certain views to trigger certain actions, but
    /// most likely not used.
    #[arg(short, long, value_name = "FLOAT", default_value_t = 0.2)]
    pub tick_rate: f64,

    /// Refresh frame rate (number of frames per second)
    #[arg(short, long, value_name = "FLOAT", default_value_t = 1.0)]
    pub frame_rate: f64,

    /// Cloud name to connect to after the start
    #[arg(long, env = "OS_CLOUD")]
    pub os_cloud: Option<String>,

    /// Custom path to the `clouds.yaml` config file
    #[arg(long, env = "OS_CLIENT_CONFIG_FILE", value_hint = ValueHint::FilePath)]
    pub os_client_config_file: Option<PathBuf>,

    /// Custom path to the `secure.yaml` config file
    #[arg(long, env = "OS_CLIENT_SECURE_FILE", value_hint = ValueHint::FilePath)]
    pub os_client_secure_file: Option<PathBuf>,
}
