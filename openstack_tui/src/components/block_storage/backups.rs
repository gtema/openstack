// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use crate::cloud_worker::block_storage::v3::{
    BlockStorageApiRequest, BlockStorageBackupApiRequest, BlockStorageBackupList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::block_storage::v3::backup::response::list_detailed::BackupResponse;

/// Behaviour implementation for BlockStorageBackups.
pub struct BlockStorageBackupsBehaviour;

impl ResourceBehaviour for BlockStorageBackupsBehaviour {
    type Item = BackupResponse;
    type Filter = BlockStorageBackupList;

    fn view_key() -> &'static str {
        "block_storage.backup"
    }
    fn title() -> &'static str {
        "Backups"
    }
    fn mode() -> Mode {
        Mode::BlockStorageBackups
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(BlockStorageBackupApiRequest::ListDetailed(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::BlockStorage(BlockStorageApiRequest::Backup(boxreq))
            if matches!(**boxreq, BlockStorageBackupApiRequest::ListDetailed(_))
        )
    }
}

/// Public component for BlockStorageBackups using the generic view.
pub type BlockStorageBackups = GenericResourceView<'static, BlockStorageBackupsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            BlockStorageBackupsBehaviour::view_key(),
            "block_storage.backup"
        );
        assert_eq!(BlockStorageBackupsBehaviour::title(), "Backups");
        assert_eq!(
            BlockStorageBackupsBehaviour::mode(),
            Mode::BlockStorageBackups
        );
    }

    #[test]
    fn request_from_filter_creates_request() {
        let filter = BlockStorageBackupList::default();
        let request = BlockStorageBackupsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::BlockStorage(BlockStorageApiRequest::Backup(boxreq))
            if matches!(*boxreq, BlockStorageBackupApiRequest::ListDetailed(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_matching() {
        let filter = BlockStorageBackupList::default();
        let request = BlockStorageBackupsBehaviour::request_from_filter(&filter);
        assert!(BlockStorageBackupsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::BlockStorage(BlockStorageApiRequest::Volume(Box::new(
            crate::cloud_worker::block_storage::v3::BlockStorageVolumeApiRequest::ListDetailed(
                Box::default(),
            ),
        )));
        assert!(!BlockStorageBackupsBehaviour::matches_request(&req));
    }
}
