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
use serde_json::Value;
use std::fmt::Display;

/// The mechanical subset of `ResourceBehaviour`: methods fully derivable from a resource's
/// metadata (view key, request/filter types, mode). Intended to be implemented by a generated
/// `Generated` companion type per resource; `ResourceBehaviour`'s defaults delegate to it.
pub trait GeneratedResourceBehaviour {
    type Filter: Default + Display + Clone;

    /// The view configuration key used for persisting column/field settings.
    fn view_key() -> &'static str;
    /// Human readable title for the view.
    fn title() -> &'static str;
    /// The Mode that corresponds to this component (when shown, data should be loaded).
    fn mode() -> Mode;
    /// Build the ApiRequest to list resources, given the current (normalised) filters.
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest;
    /// Check whether the given ApiRequest is the list-detailed request this component handles.
    fn matches_request(request: &ApiRequest) -> bool;
    /// Return the filter from a Set*Filters action. Return None if the action does not apply.
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        let _ = action;
        None
    }
}

/// Behaviour specifics for a particular OpenStack resource.
/// Implementors provide the concrete item type, filter type and any custom actions.
///
/// The `view_key`/`title`/`mode`/`request_from_filter`/`matches_request`/`handle_set_filter_action`
/// methods below are the "mechanical" tier also described by `GeneratedResourceBehaviour`: for
/// resources that have a generated companion module, implementors should give each of these a
/// one-line body that forwards to it (e.g. `fn view_key() -> &'static str {
/// generated::security_group::Generated::view_key() }`) rather than duplicating the logic here.
/// They stay required, non-default methods on this trait itself so that resources without a
/// generated companion module yet are unaffected.
pub trait ResourceBehaviour {
    type Filter: Default + Display + Clone;

    /// The view configuration key used for persisting column/field settings.
    fn view_key() -> &'static str;
    /// Human readable title for the view.
    fn title() -> &'static str;
    /// The Mode that corresponds to this component (when shown, data should be loaded).
    fn mode() -> Mode;

    /// Seed a freshly-authenticated user's identity into the filter, if this resource cares.
    /// Called on every `ConnectedToCloud`, regardless of whether this resource's view is
    /// currently active, so the filter is already correct by the time the user navigates here.
    /// Default is a no-op; override where a resource needs to default-scope to the current user.
    ///
    /// This only *seeds*, it never clobbers: it's also called after `CloudChangeScope`/
    /// `SwitchToRegion` reconnects (same user, different project/region), where a value
    /// already in the filter (e.g. a user explicitly drilled down to) must survive. Values
    /// that must NOT survive a genuine cloud switch belong in `reset_filter_on_cloud_switch`
    /// instead, which runs only for `ConnectToCloud`.
    fn seed_filter_from_current_user(
        filter: Self::Filter,
        _token: &openstack_sdk::types::identity::v3::TokenInfo,
    ) -> Self::Filter {
        filter
    }

    /// Clear any cloud-scoped value from the filter before a genuine cloud switch
    /// (`Action::ConnectToCloud`, as opposed to `CloudChangeScope`/`SwitchToRegion`, which
    /// keep the same authenticated user). Called before the new cloud's `ConnectedToCloud`
    /// arrives, so `seed_filter_from_current_user`'s "don't clobber an existing value" guard
    /// doesn't mistake a stale value from the *previous* cloud for a deliberate selection made
    /// on this one. Default is a no-op; override where a resource seeds from the current user's
    /// identity (which is only ever valid within one cloud).
    fn reset_filter_on_cloud_switch(filter: Self::Filter) -> Self::Filter {
        filter
    }

    /// Normalise the filter before sending to the API. The default returns the filter unchanged.
    fn normalise_filter(filter: Self::Filter) -> Self::Filter {
        filter
    }

    /// Whether the (normalised) filter has everything it needs to issue a list request.
    /// Checked before every fetch this view would otherwise trigger (login, refresh, mode
    /// switch, filter change, post-mutation reload). Default is always ready. Override for a
    /// resource whose filter can be transiently incomplete -- e.g. a required id that's only
    /// empty before `ConnectedToCloud` seeds it or right after `ConnectToCloud` clears it on a
    /// cloud switch -- so a request is never sent with a value the API would reject.
    fn is_filter_ready(filter: &Self::Filter) -> bool {
        let _ = filter;
        true
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
    fn action_to_request(action: &Action, selected: Option<&Value>) -> Option<ApiRequest> {
        let _ = (action, selected);
        None
    }

    /// Return custom Actions that do not map to an ApiRequest (e.g., mode switches, filter updates).
    /// The `filter` parameter provides access to the current filter state for sub-view drill actions.
    fn filter_carry_action(
        action: &Action,
        selected: Option<&Value>,
        filter: &Self::Filter,
    ) -> Vec<Action> {
        let _ = (action, selected, filter);
        Vec::new()
    }

    /// Return custom Actions (deprecated, use filter_carry_action instead for filter access).
    fn custom_action(action: &Action, selected: Option<&Value>) -> Vec<Action> {
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

    /// JSON Schema of the request body backing `editor_template`'s
    /// template, if this resource has one. When present, the edited buffer
    /// is validated against it (required fields, enums, ranges, ...) before
    /// being sent back for `deserialize_edit_result`, catching mistakes
    /// plain YAML parsing can't. Default is no schema (parse-only
    /// validation, the pre-existing behaviour).
    fn editor_schema(_action: &Action) -> Option<&'static str> {
        None
    }

    /// Map an action to a singular API request that should populate the describe pane,
    /// returning the (display actions, api request) tuple. Default returns None.
    fn action_to_singular_request(
        action: &Action,
        selected: Option<&Value>,
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
    fn confirm_request(action: &Action, selected: Option<&Value>) -> Option<ApiRequest> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, Clone)]
    struct Filter;
    impl Display for Filter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    struct DefaultBehaviour;
    impl ResourceBehaviour for DefaultBehaviour {
        type Filter = Filter;
        fn view_key() -> &'static str {
            "test.item"
        }
        fn title() -> &'static str {
            "Items"
        }
        fn mode() -> Mode {
            Mode::Resource(Self::view_key())
        }
        fn request_from_filter(_filter: &Self::Filter) -> ApiRequest {
            unimplemented!()
        }
        fn matches_request(_request: &ApiRequest) -> bool {
            false
        }
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(DefaultBehaviour::view_key(), "test.item");
        assert_eq!(DefaultBehaviour::title(), "Items");
        assert_eq!(DefaultBehaviour::mode(), Mode::Resource("test.item"));
    }

    #[test]
    fn editor_schema_default_is_none() {
        assert!(DefaultBehaviour::editor_schema(&Action::Tick).is_none());
    }

    #[test]
    fn action_to_request_default_is_none() {
        let value = serde_json::json!({"id": "a"});
        assert!(DefaultBehaviour::action_to_request(&Action::Tick, Some(&value)).is_none());
    }

    #[test]
    fn filter_carry_action_default_is_empty() {
        let value = serde_json::json!({"id": "a"});
        assert!(
            DefaultBehaviour::filter_carry_action(&Action::Tick, Some(&value), &Filter).is_empty()
        );
    }

    #[test]
    fn seed_filter_from_current_user_default_is_noop() {
        let token_info = openstack_sdk::types::identity::v3::TokenInfo::default();
        let result = DefaultBehaviour::seed_filter_from_current_user(Filter, &token_info);
        assert_eq!(format!("{result}"), "");
    }

    #[test]
    fn reset_filter_on_cloud_switch_default_is_noop() {
        let result = DefaultBehaviour::reset_filter_on_cloud_switch(Filter);
        assert_eq!(format!("{result}"), "");
    }

    #[test]
    fn is_filter_ready_default_is_always_ready() {
        assert!(DefaultBehaviour::is_filter_ready(&Filter));
    }
}
