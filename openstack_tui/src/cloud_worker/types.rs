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

pub use crate::cloud_worker::compute::{ComputeFlavorFilters, ComputeServerFilters};

/// OpenStack "resource"
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Resource {
    ComputeFlavors(ComputeFlavorFilters),
    ComputeServers(ComputeServerFilters),
    ComputeServerConsoleOutput(String),
    ComputeQuota,
    IdentityAuthProjects(IdentityAuthProjectFilters),
    IdentityProjects(IdentityProjectFilters),
    ImageImages(ImageFilters),
    NetworkNetworks(NetworkNetworkFilters),
    NetworkSubnets(NetworkSubnetFilters),
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
pub struct IdentityAuthProjectFilters {}
impl fmt::Display for IdentityAuthProjectFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityProjectFilters {}
impl fmt::Display for IdentityProjectFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
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
