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

use crate::action::Action;
use crate::cloud_worker::block_storage::v3::{
    BlockStorageApiRequest, BlockStorageVolumeApiRequest, BlockStorageVolumeDelete,
    BlockStorageVolumeDeleteBuilder, BlockStorageVolumeList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::block_storage::v3::volume::response::list_detailed::VolumeResponse;

const VIEW_CONFIG_KEY: &str = "block_storage.volume";

impl crate::utils::ResourceKey for VolumeResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&VolumeResponse> for BlockStorageVolumeDelete {
    type Error = crate::cloud_worker::block_storage::v3::BlockStorageVolumeDeleteBuilderError;
    fn try_from(value: &VolumeResponse) -> Result<Self, Self::Error> {
        let mut builder = BlockStorageVolumeDeleteBuilder::default();
        builder.id(value.id.clone());
        if let Some(val) = &value.name {
            builder.name(val.clone());
        }
        builder.build()
    }
}

pub struct BlockStorageVolumesBehaviour;

impl ResourceBehaviour for BlockStorageVolumesBehaviour {
    type Item = VolumeResponse;
    type Filter = BlockStorageVolumeList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Volumes"
    }
    fn mode() -> Mode {
        Mode::BlockStorageVolumes
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(BlockStorageVolumeApiRequest::ListDetailed(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::BlockStorage(BlockStorageApiRequest::Volume(boxreq))
            if matches!(**boxreq, BlockStorageVolumeApiRequest::ListDetailed(_))
        )
    }
    fn confirm_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        if let Action::DeleteBlockStorageVolume = action {
            let del = BlockStorageVolumeDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(BlockStorageVolumeApiRequest::Delete(
                Box::new(del),
            )))
        } else {
            None
        }
    }
}

pub type BlockStorageVolumes = GenericResourceView<'static, BlockStorageVolumesBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::block_storage::v3::volume::response::list_detailed::VolumeResponse;

    fn make_volume(id: &str, name: &str) -> VolumeResponse {
        let json = serde_json::json!({
            "id": id,
            "name": name,
            "status": "available",
            "size": 10,
            "user_id": "user-1",
            "availability_zone": "nova",
            "created_at": "2024-01-01T00:00:00Z",
            "volume_type": "lvmdriver-1",
            "attachments": [],
            "description": null,
            "snapshot_id": null,
            "source_volid": null,
            "bootable": "true",
            "replicas": [],
            "encrypted": false,
            "consistencygroup_id": null,
            "os-vol-mig-Status.migration_status": null,
            "os-vol-host-attr.host": null,
            "os-vol-tenant-attr.tenant_id": "tenant-1",
            "os-vol-mig-status.migration_status": null,
            "os-vol-host-attr:host": null,
            "os-vol-tenant-attr:tenant_id": "tenant-1"
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            BlockStorageVolumesBehaviour::view_key(),
            "block_storage.volume"
        );
        assert_eq!(BlockStorageVolumesBehaviour::title(), "Volumes");
        assert_eq!(
            BlockStorageVolumesBehaviour::mode(),
            Mode::BlockStorageVolumes
        );
    }

    #[test]
    fn request_from_filter_creates_list_detailed() {
        let filter = BlockStorageVolumeList::default();
        let request = BlockStorageVolumesBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::BlockStorage(BlockStorageApiRequest::Volume(boxreq))
            if matches!(*boxreq, BlockStorageVolumeApiRequest::ListDetailed(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list_detailed() {
        let filter = BlockStorageVolumeList::default();
        let request = BlockStorageVolumesBehaviour::request_from_filter(&filter);
        assert!(BlockStorageVolumesBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::BlockStorage(BlockStorageApiRequest::Snapshot(Box::new(
            crate::cloud_worker::block_storage::v3::BlockStorageSnapshotApiRequest::ListDetailed(
                Box::default(),
            ),
        )));
        assert!(!BlockStorageVolumesBehaviour::matches_request(&req));
    }

    #[test]
    fn confirm_request_delete_with_selected() {
        let vol = make_volume("vol-1", "test-vol");
        let result = BlockStorageVolumesBehaviour::confirm_request(
            &Action::DeleteBlockStorageVolume,
            Some(&vol),
        );
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::BlockStorage(BlockStorageApiRequest::Volume(boxreq))
            if matches!(*boxreq, BlockStorageVolumeApiRequest::Delete(_))
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected() {
        let result =
            BlockStorageVolumesBehaviour::confirm_request(&Action::DeleteBlockStorageVolume, None);
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_returns_none_for_unrelated() {
        let vol = make_volume("vol-1", "test-vol");
        let result = BlockStorageVolumesBehaviour::confirm_request(&Action::Tick, Some(&vol));
        assert!(result.is_none());
    }
}
