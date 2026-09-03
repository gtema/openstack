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

/// Behaviour implementation for BlockStorageSnapshots.
pub struct BlockStorageSnapshotsBehaviour;

impl ResourceBehaviour for BlockStorageSnapshotsBehaviour {
    type Filter = cloud_types::BlockStorageSnapshotList;

    fn view_key() -> &'static str {
        super::generated::snapshot::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::snapshot::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::snapshot::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::snapshot::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::snapshot::Generated::matches_request(request)
    }
}

/// Public component for BlockStorageSnapshots using the generic view.
pub type BlockStorageSnapshots = GenericResourceView<'static, BlockStorageSnapshotsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            BlockStorageSnapshotsBehaviour::view_key(),
            "block_storage.snapshot"
        );
        assert_eq!(BlockStorageSnapshotsBehaviour::title(), "Snapshots");
        assert_eq!(
            BlockStorageSnapshotsBehaviour::mode(),
            Mode::Resource(crate::mode::BLOCK_STORAGE_SNAPSHOT)
        );
    }

    #[test]
    fn request_from_filter_creates_request() {
        let filter = cloud_types::BlockStorageSnapshotList::default();
        let request = BlockStorageSnapshotsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::BlockStorage(cloud_types::BlockStorageApiRequest::Snapshot(boxreq))
            if matches!(*boxreq, cloud_types::BlockStorageSnapshotApiRequest::ListDetailed(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_matching() {
        let filter = cloud_types::BlockStorageSnapshotList::default();
        let request = BlockStorageSnapshotsBehaviour::request_from_filter(&filter);
        assert!(BlockStorageSnapshotsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::BlockStorage(cloud_types::BlockStorageApiRequest::Volume(Box::new(
            cloud_types::BlockStorageVolumeApiRequest::ListDetailed(Box::default()),
        )));
        assert!(!BlockStorageSnapshotsBehaviour::matches_request(&req));
    }
}
