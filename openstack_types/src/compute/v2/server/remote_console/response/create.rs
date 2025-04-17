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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.
//! Response type for the POST `servers/{server_id}/remote-consoles` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// RemoteConsole response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct RemoteConsoleResponse {
    /// The protocol of remote console. The valid values are `vnc`, `spice`,
    /// `serial` and `mks`. The protocol `mks` is added since Microversion
    /// `2.8`.
    #[structable(serialize)]
    pub protocol: Protocol,

    /// The type of remote console. The valid values are `novnc`,
    /// `spice-html5`, `spice-direct`, `serial`, and `webmks`. The type
    /// `webmks` was added in Microversion `2.8`, and the type `spice-direct`
    /// was added in Microversion `2.99`.
    #[serde(rename = "type")]
    #[structable(serialize, title = "type")]
    pub _type: Type,

    /// The URL is used to connect the console.
    #[structable()]
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Protocol {
    // Mks
    #[serde(rename = "mks")]
    Mks,

    // Serial
    #[serde(rename = "serial")]
    Serial,

    // Spice
    #[serde(rename = "spice")]
    Spice,

    // Vnc
    #[serde(rename = "vnc")]
    Vnc,
}

impl std::str::FromStr for Protocol {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "mks" => Ok(Self::Mks),
            "serial" => Ok(Self::Serial),
            "spice" => Ok(Self::Spice),
            "vnc" => Ok(Self::Vnc),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    // Novnc
    #[serde(rename = "novnc")]
    Novnc,

    // Serial
    #[serde(rename = "serial")]
    Serial,

    // SpiceHtml5
    #[serde(rename = "spice-html5")]
    SpiceHtml5,

    // Webmks
    #[serde(rename = "webmks")]
    Webmks,

    // Xvpvnc
    #[serde(rename = "xvpvnc")]
    Xvpvnc,
}

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "novnc" => Ok(Self::Novnc),
            "serial" => Ok(Self::Serial),
            "spice-html5" => Ok(Self::SpiceHtml5),
            "webmks" => Ok(Self::Webmks),
            "xvpvnc" => Ok(Self::Xvpvnc),
            _ => Err(()),
        }
    }
}
