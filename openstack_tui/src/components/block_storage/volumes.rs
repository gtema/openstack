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
use crate::cloud_worker::types::{self as cloud_types, ApiRequest};
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{GeneratedResourceBehaviour, ResourceBehaviour};
use crate::mode::Mode;

impl TryFrom<&serde_json::Value> for cloud_types::BlockStorageVolumeDelete {
    type Error = crate::cloud_worker::block_storage::v3::BlockStorageVolumeDeleteBuilderError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let mut builder =
            crate::cloud_worker::block_storage::v3::BlockStorageVolumeDeleteBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.id(val.to_string());
        }
        if let Some(val) = crate::components::view_render::get_str(value, "/name") {
            builder.name(val.to_string());
        }
        builder.build()
    }
}

pub struct BlockStorageVolumesBehaviour;

impl ResourceBehaviour for BlockStorageVolumesBehaviour {
    type Filter = cloud_types::BlockStorageVolumeList;

    fn view_key() -> &'static str {
        super::generated::volume::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::volume::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::volume::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::volume::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::volume::Generated::matches_request(request)
    }
    fn confirm_request(
        action: &Action,
        selected: Option<&serde_json::Value>,
    ) -> Option<ApiRequest> {
        if let Action::ResourceOp {
            key,
            op: crate::action::ResourceOp::Delete,
        } = action
            && *key == Self::view_key()
        {
            let del = cloud_types::BlockStorageVolumeDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(
                cloud_types::BlockStorageVolumeApiRequest::Delete(Box::new(del)),
            ))
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

    fn make_volume(id: &str, name: &str) -> serde_json::Value {
        serde_json::json!({
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
        })
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
            Mode::Resource(crate::mode::BLOCK_STORAGE_VOLUME)
        );
    }

    #[test]
    fn request_from_filter_creates_list_detailed() {
        let filter = cloud_types::BlockStorageVolumeList::default();
        let request = BlockStorageVolumesBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::BlockStorage(cloud_types::BlockStorageApiRequest::Volume(boxreq))
            if matches!(*boxreq, cloud_types::BlockStorageVolumeApiRequest::ListDetailed(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list_detailed() {
        let filter = cloud_types::BlockStorageVolumeList::default();
        let request = BlockStorageVolumesBehaviour::request_from_filter(&filter);
        assert!(BlockStorageVolumesBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req =
            ApiRequest::BlockStorage(cloud_types::BlockStorageApiRequest::Snapshot(Box::new(
                cloud_types::BlockStorageSnapshotApiRequest::ListDetailed(Box::default()),
            )));
        assert!(!BlockStorageVolumesBehaviour::matches_request(&req));
    }

    #[test]
    fn confirm_request_delete_with_selected() {
        let vol = make_volume("vol-1", "test-vol");
        let result = BlockStorageVolumesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::BLOCK_STORAGE_VOLUME,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&vol),
        );
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::BlockStorage(cloud_types::BlockStorageApiRequest::Volume(boxreq))
            if matches!(*boxreq, cloud_types::BlockStorageVolumeApiRequest::Delete(_))
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected() {
        let result = BlockStorageVolumesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::BLOCK_STORAGE_VOLUME,
                op: crate::action::ResourceOp::Delete,
            },
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_returns_none_for_unrelated() {
        let vol = make_volume("vol-1", "test-vol");
        let result = BlockStorageVolumesBehaviour::confirm_request(&Action::Tick, Some(&vol));
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_ignores_delete_for_other_resource() {
        let vol = make_volume("vol-1", "test-vol");
        let result = BlockStorageVolumesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::BLOCK_STORAGE_SNAPSHOT,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&vol),
        );
        assert!(result.is_none());
    }
}
