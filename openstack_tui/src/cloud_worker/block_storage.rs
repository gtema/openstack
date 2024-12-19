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

use crate::cloud_worker::ConfirmableRequest;

pub mod v3;

pub use v3::*;

impl ConfirmableRequest for BlockStorageApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            BlockStorageApiRequest::Volume(req) => req.get_confirm_message(),
            _ => None,
        }
    }
}

impl ConfirmableRequest for BlockStorageVolumeApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            BlockStorageVolumeApiRequest::Delete(req) => req.get_confirm_message(),
            _ => None,
        }
    }
}
