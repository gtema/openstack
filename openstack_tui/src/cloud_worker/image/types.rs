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

use crate::cloud_worker::common::ConfirmableRequest;

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

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageImageDelete {
    pub image_id: String,
    pub image_name: Option<String>,
}

impl fmt::Display for ImageImageDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl ConfirmableRequest for ImageImageDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete image {} ?",
            self.image_name.clone().unwrap_or(self.image_id.clone())
        ))
    }
}
