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
use crate::components::resource_behaviour::{Mutation, ResourceBehaviour};
use crate::mode::Mode;
use openstack_types::image::v2::image::response::list::ImageResponse;
use serde_json::Value;

const VIEW_CONFIG_KEY: &str = "image.image";

impl crate::utils::ResourceKey for ImageResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&ImageResponse> for ImageImageDelete {
    type Error = crate::cloud_worker::image::v2::ImageImageDeleteBuilderError;
    fn try_from(value: &ImageResponse) -> Result<Self, Self::Error> {
        let mut builder = ImageImageDeleteBuilder::default();
        if let Some(val) = &value.id {
            builder.id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.name(val.clone());
        }
        builder.build()
    }
}

pub struct ImageImagesBehaviour;

impl ResourceBehaviour for ImageImagesBehaviour {
    type Item = ImageResponse;
    type Filter = ImageImageList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Images"
    }
    fn mode() -> Mode {
        Mode::ImageImages
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(ImageImageApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Image(ImageApiRequest::Image(boxreq))
            if matches!(**boxreq, ImageImageApiRequest::List(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetImageListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
    fn confirm_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        if let Action::DeleteImage = action {
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
    use openstack_types::image::v2::image::response::list::ImageResponse;

    fn make_image(id: &str, name: &str) -> ImageResponse {
        let json = serde_json::json!({
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
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(ImageImagesBehaviour::view_key(), "image.image");
        assert_eq!(ImageImagesBehaviour::title(), "Images");
        assert_eq!(ImageImagesBehaviour::mode(), Mode::ImageImages);
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
        let result = ImageImagesBehaviour::confirm_request(&Action::DeleteImage, Some(&img));
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
        let result = ImageImagesBehaviour::confirm_request(&Action::DeleteImage, None);
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
