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
    BlockStorageApiRequest, BlockStorageSnapshotApiRequest, BlockStorageSnapshotList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::block_storage::v3::snapshot::response::list_detailed::SnapshotResponse;

/// Behaviour implementation for BlockStorageSnapshots.
pub struct BlockStorageSnapshotsBehaviour;

impl ResourceBehaviour for BlockStorageSnapshotsBehaviour {
    type Item = SnapshotResponse;
    type Filter = BlockStorageSnapshotList;

    fn view_key() -> &'static str {
        "block_storage.snapshot"
    }
    fn title() -> &'static str {
        "Snapshots"
    }
    fn mode() -> Mode {
        Mode::BlockStorageSnapshots
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(BlockStorageSnapshotApiRequest::ListDetailed(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::BlockStorage(BlockStorageApiRequest::Snapshot(boxreq))
            if matches!(**boxreq, BlockStorageSnapshotApiRequest::ListDetailed(_))
        )
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
            Mode::BlockStorageSnapshots
        );
    }

    #[test]
    fn request_from_filter_creates_request() {
        let filter = BlockStorageSnapshotList::default();
        let request = BlockStorageSnapshotsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::BlockStorage(BlockStorageApiRequest::Snapshot(boxreq))
            if matches!(*boxreq, BlockStorageSnapshotApiRequest::ListDetailed(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_matching() {
        let filter = BlockStorageSnapshotList::default();
        let request = BlockStorageSnapshotsBehaviour::request_from_filter(&filter);
        assert!(BlockStorageSnapshotsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::BlockStorage(BlockStorageApiRequest::Volume(Box::new(
            crate::cloud_worker::block_storage::v3::BlockStorageVolumeApiRequest::ListDetailed(
                Box::new(crate::cloud_worker::block_storage::v3::BlockStorageVolumeList::default()),
            ),
        )));
        assert!(!BlockStorageSnapshotsBehaviour::matches_request(&req));
    }
}
