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

//! Compute API v2 data types
use serde::{Deserialize, Serialize};

/// Flavors
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flavor {
    /// The name of the flavor.
    pub name: String,
    /// The UID for the Flavor.
    pub id: String,
}

/// Servers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    /// The name of the Server.
    pub name: String,
    /// The UID for the Server.
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub access_ip_v4: Option<String>,
}
