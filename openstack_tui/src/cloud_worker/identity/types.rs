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

pub use crate::cloud_worker::identity::auth::*;
pub use crate::cloud_worker::identity::group::*;
pub use crate::cloud_worker::identity::project::*;
pub use crate::cloud_worker::identity::user::*;

/// Identity operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityApiRequest {
    /// Auth
    Auth(IdentityAuthApiRequest),
    /// Groups
    Group(IdentityGroupApiRequest),
    /// Projects
    Project(IdentityProjectApiRequest),
    //    Group,
    User(IdentityUserApiRequest),
}

impl From<IdentityAuthApiRequest> for IdentityApiRequest {
    fn from(item: IdentityAuthApiRequest) -> Self {
        IdentityApiRequest::Auth(item)
    }
}

impl From<IdentityGroupApiRequest> for IdentityApiRequest {
    fn from(item: IdentityGroupApiRequest) -> Self {
        IdentityApiRequest::Group(item)
    }
}

impl From<IdentityProjectApiRequest> for IdentityApiRequest {
    fn from(item: IdentityProjectApiRequest) -> Self {
        IdentityApiRequest::Project(item)
    }
}

impl From<IdentityUserApiRequest> for IdentityApiRequest {
    fn from(item: IdentityUserApiRequest) -> Self {
        IdentityApiRequest::User(item)
    }
}
