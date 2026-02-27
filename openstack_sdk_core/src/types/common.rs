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

//! Common types that can be used in responses of the API operations
use serde::{Deserialize, Serialize};

/// A reference to a resource by its Name and ID.
#[derive(Deserialize, Debug, Clone, Serialize, Eq, PartialEq)]
pub struct IdAndName {
    /// The name of the entity.
    pub name: String,
    /// The UID for the entity.
    pub id: String,
}

/// A reference to a resource by either its Name or ID.
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
pub enum NameOrId {
    /// Resource ID.
    #[serde(rename = "id")]
    Id(String),
    /// Resource name.
    #[serde(rename = "name")]
    Name(String),
}

#[cfg(test)]
mod tests {}
