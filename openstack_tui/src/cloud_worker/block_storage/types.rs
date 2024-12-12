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

pub use crate::cloud_worker::block_storage::backup::*;
pub use crate::cloud_worker::block_storage::snapshot::*;
pub use crate::cloud_worker::block_storage::volume::*;
use crate::cloud_worker::common::ConfirmableRequest;

/// Block Storage operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockStorageApiRequest {
    Backup(BlockStorageBackupApiRequest),
    Snapshot(BlockStorageSnapshotApiRequest),
    Volume(BlockStorageVolumeApiRequest),
}

impl From<BlockStorageBackupApiRequest> for BlockStorageApiRequest {
    fn from(item: BlockStorageBackupApiRequest) -> Self {
        BlockStorageApiRequest::Backup(item)
    }
}

impl From<BlockStorageSnapshotApiRequest> for BlockStorageApiRequest {
    fn from(item: BlockStorageSnapshotApiRequest) -> Self {
        BlockStorageApiRequest::Snapshot(item)
    }
}

impl From<BlockStorageVolumeApiRequest> for BlockStorageApiRequest {
    fn from(item: BlockStorageVolumeApiRequest) -> Self {
        BlockStorageApiRequest::Volume(item)
    }
}

impl ConfirmableRequest for BlockStorageApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            BlockStorageApiRequest::Volume(req) => req.get_confirm_message(),
            _ => None,
        }
    }
}
