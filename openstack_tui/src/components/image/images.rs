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
use crate::cloud_worker::image::v2::{
    ImageApiRequest, ImageImageApiRequest, ImageImageDelete, ImageImageDeleteBuilder,
    ImageImageList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{
    GeneratedResourceBehaviour, Mutation, ResourceBehaviour,
};
use crate::mode::Mode;
use serde_json::Value;

impl TryFrom<&Value> for ImageImageDelete {
    type Error = crate::cloud_worker::image::v2::ImageImageDeleteBuilderError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mut builder = ImageImageDeleteBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.id(val.to_string());
        }
        if let Some(val) = crate::components::view_render::get_str(value, "/name") {
            builder.name(val.to_string());
        }
        builder.build()
    }
}

pub struct ImageImagesBehaviour;

impl ResourceBehaviour for ImageImagesBehaviour {
    type Filter = ImageImageList;

    fn view_key() -> &'static str {
        super::generated::image::Generated::view_key()
    }
    fn title() -> &'static str {
        // NOTE: `super::generated::image::Generated::title()` currently returns the wrong
        // value ("s") due to a codegenerator title-stripping bug for the "image.image"
        // resource - kept hand-written here until that's fixed upstream.
        "Images"
    }
    fn mode() -> Mode {
        super::generated::image::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::image::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::image::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::image::Generated::handle_set_filter_action(action)
    }
    fn confirm_request(action: &Action, selected: Option<&Value>) -> Option<ApiRequest> {
        if let Action::ResourceOp {
            key,
            op: crate::action::ResourceOp::Delete,
        } = action
            && *key == Self::view_key()
        {
            let del = ImageImageDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(ImageImageApiRequest::Delete(Box::new(
                del,
            ))))
        } else {
            None
        }
    }
    fn handle_mutation_response(request: &ApiRequest, _data: &Value) -> Option<Vec<Mutation>> {
        if let ApiRequest::Image(ImageApiRequest::Image(req)) = request
            && let ImageImageApiRequest::Delete(del) = &**req
        {
            return Some(vec![Mutation::DeleteRow(del.id.clone())]);
        }
        None
    }
}

pub type Images = GenericResourceView<'static, ImageImagesBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    fn make_image(id: &str, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "status": "active",
            "schema": "/v2/schemas/image",
            "tags": [],
            "container_format": "bare",
            "disk_format": "qcow2",
            "min_disk": 0,
            "min_ram": 0,
            "visibility": "public",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        })
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(ImageImagesBehaviour::view_key(), "image.image");
        assert_eq!(ImageImagesBehaviour::title(), "Images");
        assert_eq!(
            ImageImagesBehaviour::mode(),
            Mode::Resource(crate::mode::IMAGE_IMAGE)
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = ImageImageList::default();
        let request = ImageImagesBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Image(ImageApiRequest::Image(boxreq))
            if matches!(*boxreq, ImageImageApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = ImageImageList::default();
        let request = ImageImagesBehaviour::request_from_filter(&filter);
        assert!(ImageImagesBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let del = ImageImageDeleteBuilder::default()
            .id("test".into())
            .build()
            .unwrap();
        let req = ApiRequest::from(ImageImageApiRequest::Delete(Box::new(del)));
        assert!(!ImageImagesBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = ImageImageList::default();
        let action = Action::SetImageListFilters(filter);
        let result = ImageImagesBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = ImageImagesBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_delete_with_selected() {
        let img = make_image("img-1", "test-image");
        let result = ImageImagesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::IMAGE_IMAGE,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&img),
        );
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::Image(ImageApiRequest::Image(boxreq))
            if matches!(*boxreq, ImageImageApiRequest::Delete(_))
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected() {
        let result = ImageImagesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::IMAGE_IMAGE,
                op: crate::action::ResourceOp::Delete,
            },
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_ignores_delete_for_other_resource() {
        let img = make_image("img-1", "test-image");
        let result = ImageImagesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::COMPUTE_SERVER,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&img),
        );
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_returns_none_for_unrelated() {
        let img = make_image("img-1", "test-image");
        let result = ImageImagesBehaviour::confirm_request(&Action::Tick, Some(&img));
        assert!(result.is_none());
    }

    #[test]
    fn handle_mutation_response_delete() {
        let del = ImageImageDeleteBuilder::default()
            .id("img-1".into())
            .build()
            .unwrap();
        let request = ApiRequest::from(ImageImageApiRequest::Delete(Box::new(del)));
        let data = serde_json::json!({});
        let result = ImageImagesBehaviour::handle_mutation_response(&request, &data);
        let muts = result.unwrap();
        assert_eq!(muts.len(), 1);
        if let Mutation::DeleteRow(found_id) = &muts[0] {
            assert_eq!(found_id, "img-1");
        } else {
            panic!("Expected DeleteRow mutation");
        }
    }

    #[test]
    fn handle_mutation_response_non_matching() {
        let filter = ImageImageList::default();
        let request = ImageImagesBehaviour::request_from_filter(&filter);
        let data = serde_json::json!({});
        let result = ImageImagesBehaviour::handle_mutation_response(&request, &data);
        assert!(result.is_none());
    }
}
