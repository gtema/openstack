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

use serde::{Deserialize, Serialize};
use strum::Display;

use crate::cloud_worker::types as cloud_types;

/// TUI action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Info,
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Mode(crate::mode::Mode),
    Error(String),
    Help,
    ConnectToCloud(String),
    CloudChangeScope(openstack_sdk::auth::authtoken::AuthTokenScope),
    ConnectedToCloud(Box<openstack_sdk::types::identity::v3::AuthToken>),
    RequestCloudResource(cloud_types::Resource),
    ResourceData {
        resource: cloud_types::Resource,
        data: serde_json::Value,
    },
    ResourcesData {
        resource: cloud_types::Resource,
        data: Vec<serde_json::Value>,
    },
    ResourceSelect,
    Refresh,
    Next,
    Prev,
    End,
    PageUp,
    PageDown,
    Describe(serde_json::Value),
    CloudSelect,
    SelectProject,
    ListClouds,
    Clouds(Vec<String>),
    ComputeServerFilter(cloud_types::ComputeServerFilters),
    NetworkSubnetFilter(cloud_types::NetworkSubnetFilters),
    ImageFilter(cloud_types::ImageFilters),
    ServerConsoleOutput,
    ResetFilter,
}
