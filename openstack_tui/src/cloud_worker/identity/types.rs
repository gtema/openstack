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
pub struct IdentityUserFilters {}
impl fmt::Display for IdentityUserFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityGroupFilters {}
impl fmt::Display for IdentityGroupFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

/// Group Users filter
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityGroupUserFilters {
    /// Group id (used by API)
    pub group_id: String,
    /// Group name (Set by caller for display only)
    pub group_name: Option<String>,
}
impl fmt::Display for IdentityGroupUserFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "group: {}",
            self.group_name.as_ref().unwrap_or(&self.group_id)
        )
    }
}

/// Update user properties
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityUserUpdate {
    /// User ID
    pub id: String,
    /// New user name
    pub name: Option<String>,
    /// Enabled
    pub enabled: Option<bool>,
}

/// User Application Credentials filter
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityApplicationCredentialFilters {
    /// User id (used by API)
    pub user_id: String,
    /// User name (Set by caller for display only)
    pub user_name: Option<String>,
}
impl fmt::Display for IdentityApplicationCredentialFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "user: {}",
            self.user_name.as_ref().unwrap_or(&self.user_id)
        )
    }
}
