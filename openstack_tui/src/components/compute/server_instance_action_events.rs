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
use crate::cloud_worker::compute::v2::{
    ComputeApiRequest, ComputeServerApiRequest, ComputeServerInstanceActionApiRequest,
    ComputeServerInstanceActionShow,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use serde::Deserialize;
use serde_json::Value;
use structable::{StructTable, StructTableOptions};

const VIEW_CONFIG_KEY: &str = "compute.server/instance_action/event";

/// Event type
#[derive(Clone, Debug, Deserialize, StructTable)]
pub struct ServerInstanceActionEventData {
    /// Even details
    #[structable(optional)]
    pub details: Option<String>,

    /// Event summary
    pub event: String,

    /// Finish time of the event
    #[structable(optional)]
    pub finish_time: Option<String>,

    /// Hostname
    #[structable(optional)]
    pub host: Option<String>,

    /// Host ID
    #[structable(optional)]
    pub host_id: Option<String>,

    /// Result
    #[structable(optional)]
    pub result: Option<String>,

    /// Event start time
    #[structable(optional)]
    pub start_time: Option<String>,

    /// Traceback
    #[structable(optional)]
    pub traceback: Option<String>,
}

impl crate::utils::ResourceKey for ServerInstanceActionEventData {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct ComputeServerInstanceActionEventsBehaviour;

impl ResourceBehaviour for ComputeServerInstanceActionEventsBehaviour {
    type Item = ServerInstanceActionEventData;
    type Filter = ComputeServerInstanceActionShow;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "InstanceAction Events"
    }
    fn mode() -> Mode {
        Mode::ComputeServerInstanceActionEvents
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(ComputeServerInstanceActionApiRequest::Get(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Server(boxreq))
                if matches!(&**boxreq, ComputeServerApiRequest::InstanceAction(inner)
                    if matches!(&**inner, ComputeServerInstanceActionApiRequest::Get(_))
                )
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetComputeServerInstanceActionShowFilters(f) = action {
            Some((**f).clone())
        } else {
            None
        }
    }
    fn matches_singular_request(request: &ApiRequest) -> bool {
        Self::matches_request(request)
    }
    fn handle_singular_response_data(request: &ApiRequest, data: &[Value]) -> Option<Action> {
        for d in data {
            if let Some(events) = d.get("events")
                && let Some(ar) = events.as_array()
            {
                return Some(Action::ApiResponsesData {
                    request: request.clone(),
                    data: ar.to_vec(),
                });
            }
        }
        None
    }
}

pub type ComputeServerInstanceActionEvents =
    GenericResourceView<'static, ComputeServerInstanceActionEventsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::compute::v2::ComputeServerInstanceActionShowBuilder;
    use crate::components::resource_behaviour::ResourceBehaviour;

    fn make_filter() -> ComputeServerInstanceActionShow {
        ComputeServerInstanceActionShowBuilder::default()
            .server_id("test-server".into())
            .id("action-1".into())
            .build()
            .unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            ComputeServerInstanceActionEventsBehaviour::view_key(),
            "compute.server/instance_action/event"
        );
        assert_eq!(
            ComputeServerInstanceActionEventsBehaviour::title(),
            "InstanceAction Events"
        );
        assert_eq!(
            ComputeServerInstanceActionEventsBehaviour::mode(),
            Mode::ComputeServerInstanceActionEvents
        );
    }

    #[test]
    fn request_from_filter_creates_get_request() {
        let filter = make_filter();
        let request = ComputeServerInstanceActionEventsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Server(boxreq))
                if matches!(&*boxreq, ComputeServerApiRequest::InstanceAction(inner)
                    if matches!(&**inner, ComputeServerInstanceActionApiRequest::Get(_))
                )
        ));
    }

    #[test]
    fn matches_request_returns_true_for_get() {
        let filter = make_filter();
        let request = ComputeServerInstanceActionEventsBehaviour::request_from_filter(&filter);
        assert!(ComputeServerInstanceActionEventsBehaviour::matches_request(
            &request
        ));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let delete_request = ApiRequest::Compute(ComputeApiRequest::Server(Box::new(
            ComputeServerApiRequest::Delete(Box::new(
                crate::cloud_worker::compute::v2::ComputeServerDelete {
                    id: "test".into(),
                    name: None,
                },
            )),
        )));
        assert!(!ComputeServerInstanceActionEventsBehaviour::matches_request(&delete_request));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = make_filter();
        let action = Action::SetComputeServerInstanceActionShowFilters(Box::new(filter));
        let result = ComputeServerInstanceActionEventsBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result =
            ComputeServerInstanceActionEventsBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn matches_singular_request_delegates_to_matches_request() {
        let filter = make_filter();
        let request = ComputeServerInstanceActionEventsBehaviour::request_from_filter(&filter);
        assert!(ComputeServerInstanceActionEventsBehaviour::matches_singular_request(&request));
    }

    #[test]
    fn handle_singular_response_data_returns_events() {
        let filter = make_filter();
        let request = ComputeServerInstanceActionEventsBehaviour::request_from_filter(&filter);
        let data = vec![serde_json::json!({
            "events": [
                {"event": "boot started", "result": "SUCCESS"},
                {"event": "boot finished", "result": "SUCCESS"}
            ]
        })];
        let result = ComputeServerInstanceActionEventsBehaviour::handle_singular_response_data(
            &request, &data,
        );
        assert!(matches!(result, Some(Action::ApiResponsesData { .. })));
    }

    #[test]
    fn handle_singular_response_data_no_events_key() {
        let filter = make_filter();
        let request = ComputeServerInstanceActionEventsBehaviour::request_from_filter(&filter);
        let data = vec![serde_json::json!({})];
        let result = ComputeServerInstanceActionEventsBehaviour::handle_singular_response_data(
            &request, &data,
        );
        assert!(result.is_none());
    }

    #[test]
    fn handle_singular_response_data_events_not_array() {
        let filter = make_filter();
        let request = ComputeServerInstanceActionEventsBehaviour::request_from_filter(&filter);
        let data = vec![serde_json::json!({ "events": "not-an-array" })];
        let result = ComputeServerInstanceActionEventsBehaviour::handle_singular_response_data(
            &request, &data,
        );
        assert!(result.is_none());
    }
}
