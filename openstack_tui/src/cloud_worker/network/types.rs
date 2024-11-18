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

/// Network filters
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkNetworkFilters {}
impl fmt::Display for NetworkNetworkFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

/// Subnet filters
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSubnetFilters {
    pub network_id: Option<String>,
    /// Name of the parent network
    pub network_name: Option<String>,
}
impl fmt::Display for NetworkSubnetFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.network_id.is_some() || self.network_name.is_some() {
            write!(
                f,
                "network: {}",
                self.network_name
                    .as_ref()
                    .or(self.network_id.as_ref())
                    .unwrap_or(&String::new())
            )?;
        }
        Ok(())
    }
}

/// Security groups
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSecurityGroupFilters {}
impl fmt::Display for NetworkSecurityGroupFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

/// Security group rules
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSecurityGroupRuleFilters {
    pub security_group_id: Option<String>,
    pub security_group_name: Option<String>,
}
impl fmt::Display for NetworkSecurityGroupRuleFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.security_group_id.is_some() || self.security_group_name.is_some() {
            write!(
                f,
                "security_group: {}",
                self.security_group_name
                    .as_ref()
                    .or(self.security_group_id.as_ref())
                    .unwrap_or(&String::new())
            )?;
        }
        Ok(())
    }
}
