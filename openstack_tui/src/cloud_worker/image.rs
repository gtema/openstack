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
#![allow(clippy::module_inception)]

use crate::cloud_worker::ConfirmableRequest;

pub mod v2;

pub use v2::*;

impl ConfirmableRequest for ImageApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            ImageApiRequest::Image(req) => req.get_confirm_message(),
        }
    }
}

impl ConfirmableRequest for ImageImageApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            ImageImageApiRequest::Delete(req) => req.get_confirm_message(),
            _ => None,
        }
    }
}
