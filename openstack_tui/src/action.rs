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
use std::fmt;
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Resource {
    ComputeFlavors(ComputeFlavorFilters),
    ComputeServers(ComputeServerFilters),
    ComputeServerConsoleOutput(String),
    ComputeQuota,
    NetworkNetworks(NetworkNetworkFilters),
    NetworkSubnets(NetworkSubnetFilters),
    ImageImages(ImageFilters),
}

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
    ConnectedToCloud(Box<openstack_sdk::types::identity::v3::AuthToken>),
    RequestCloudResource(Resource),
    ResourceData {
        resource: Resource,
        data: serde_json::Value,
    },
    ResourcesData {
        resource: Resource,
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
    ListClouds,
    Clouds(Vec<String>),
    NetworkSubnetFilter(NetworkSubnetFilters),
    ImageFilter(ImageFilters),
    ServerConsoleOutput,
    ResetFilter,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeFlavorFilters {}
impl fmt::Display for ComputeFlavorFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeServerFilters {}
impl fmt::Display for ComputeServerFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkNetworkFilters {}
impl fmt::Display for NetworkNetworkFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSubnetFilters {
    pub network_id: Option<String>,
}
impl fmt::Display for NetworkSubnetFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(val) = &self.network_id {
            write!(f, "network: {}", val)?;
        }
        Ok(())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageFilters {
    pub visibility: Option<String>,
}
impl fmt::Display for ImageFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(val) = &self.visibility {
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}
