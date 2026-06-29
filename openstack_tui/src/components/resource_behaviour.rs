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
use crate::cloud_worker::types::ApiRequest;
use crate::mode::Mode;
use crate::utils::ResourceKey;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Display;

/// Behaviour specifics for a particular OpenStack resource.
/// Implementors provide the concrete item type, filter type and any custom actions.
pub trait ResourceBehaviour {
    type Item: ResourceKey + DeserializeOwned;
    type Filter: Default + Display + Clone;

    /// The view configuration key used for persisting column/field settings.
    fn view_key() -> &'static str;
    /// Human readable title for the view.
    fn title() -> &'static str;
    /// The Mode that corresponds to this component (when shown, data should be loaded).
    fn mode() -> Mode;

    /// Normalise the filter before sending to the API. The default returns the filter unchanged.
    fn normalise_filter(filter: Self::Filter) -> Self::Filter {
        filter
    }

    /// Build the ApiRequest to list resources, given the current (normalised) filters.
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest;

    /// Check whether the given ApiRequest is the list-detailed request this component handles.
    fn matches_request(request: &ApiRequest) -> bool;

    /// Return the filter from a Set*Filters action. Return None if the action does not apply.
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        let _ = action;
        None
    }

    /// Translate an incoming Action (that is not a generic UI action) into an optional ApiRequest.
    /// Return `None` if the action is not handled specially for this resource.
    fn action_to_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        let _ = (action, selected);
        None
    }

    /// Return custom Actions that do not map to an ApiRequest (e.g., mode switches, filter updates).
    fn custom_action(action: &Action, selected: Option<&Self::Item>) -> Vec<Action> {
        let _ = (action, selected);
        Vec::new()
    }

    /// Map an action to a singular API request that should populate the describe pane,
    /// returning the (display actions, api request) tuple. Default returns None.
    fn action_to_singular_request(
        action: &Action,
        selected: Option<&Self::Item>,
    ) -> Option<(Vec<Action>, ApiRequest)> {
        let _ = (action, selected);
        None
    }

    /// Check whether the given ApiRequest is a singular request this component handles.
    fn matches_singular_request(request: &ApiRequest) -> bool {
        let _ = request;
        false
    }

    /// Handle the response data for a singular request. Return None if not handled.
    fn handle_singular_response_data(request: &ApiRequest, data: &[Value]) -> Option<Action> {
        let _ = (request, data);
        None
    }
}
