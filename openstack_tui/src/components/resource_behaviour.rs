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
    /// The `filter` parameter provides access to the current filter state for sub-view drill actions.
    fn filter_carry_action(
        action: &Action,
        selected: Option<&Self::Item>,
        filter: &Self::Filter,
    ) -> Vec<Action> {
        let _ = (action, selected, filter);
        Vec::new()
    }

    /// Return custom Actions (deprecated, use filter_carry_action instead for filter access).
    fn custom_action(action: &Action, selected: Option<&Self::Item>) -> Vec<Action> {
        let _ = (action, selected);
        Vec::new()
    }

    /// Return a YAML editor template for a create action. Returns (template_string, api_request_to_send_on_confirm).
    fn editor_template(_action: &Action, _filter: &Self::Filter) -> Option<(String, ApiRequest)> {
        None
    }

    /// Deserialize the edited YAML back into an ApiRequest.
    fn deserialize_edit_result(_data: &Value) -> Option<ApiRequest> {
        None
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
    /// Data comes from ApiResponsesData as a single-element Vec.
    fn handle_singular_response_data(request: &ApiRequest, data: &[Value]) -> Option<Action> {
        let _ = (request, data);
        None
    }

    /// Translate an Action into a confirmable ApiRequest (e.g., delete). Return Some(ApiRequest)
    /// to send via Action::Confirm instead of Action::PerformApiRequest.
    fn confirm_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        let _ = (action, selected);
        None
    }

    /// Handle a singular API response from a mutation (delete/create/update). Returns the actions
    /// to take, if any. Called with the original request that produced this response.
    fn handle_mutation_response(request: &ApiRequest, data: &Value) -> Option<Vec<Mutation>> {
        let _ = (request, data);
        None
    }

    /// Return true to `set_data(Vec::new())` before a filter change. The list response will
    /// be handled by ApiResponsesData. Default is false.
    fn clear_data_on_filter_change() -> bool {
        false
    }
}

/// Result of handling a mutation API response.
pub enum Mutation {
    /// Find and delete the row matching this identifier.
    DeleteRow(String),
    /// Find and update the row matching this identifier with the given data.
    UpdateRow(String, Value),
    /// Append a new row with the given data.
    AppendRow(Value),
    /// Refresh the entire list.
    Refresh,
}
