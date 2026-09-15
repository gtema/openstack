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

use crate::cloud_worker::types::{self as cloud_types, ApiRequest};
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{GeneratedResourceBehaviour, ResourceBehaviour};
use crate::mode::Mode;

/// Behaviour implementation for BlockStorageBackups.
pub struct BlockStorageBackupsBehaviour;

impl ResourceBehaviour for BlockStorageBackupsBehaviour {
    type Filter = cloud_types::BlockStorageBackupList;

    fn view_key() -> &'static str {
        super::generated::backup::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::backup::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::backup::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::backup::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::backup::Generated::matches_request(request)
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
            Mode::Resource(crate::mode::BLOCK_STORAGE_BACKUP)
        );
    }

    #[test]
    fn request_from_filter_creates_request() {
        let filter = cloud_types::BlockStorageBackupList::default();
        let request = BlockStorageBackupsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::BlockStorage(cloud_types::BlockStorageApiRequest::Backup(boxreq))
            if matches!(*boxreq, cloud_types::BlockStorageBackupApiRequest::ListDetailed(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_matching() {
        let filter = cloud_types::BlockStorageBackupList::default();
        let request = BlockStorageBackupsBehaviour::request_from_filter(&filter);
        assert!(BlockStorageBackupsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::BlockStorage(cloud_types::BlockStorageApiRequest::Volume(Box::new(
            cloud_types::BlockStorageVolumeApiRequest::ListDetailed(Box::default()),
        )));
        assert!(!BlockStorageBackupsBehaviour::matches_request(&req));
    }
}
