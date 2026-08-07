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

use crate::action::Action;
use crate::cloud_worker::compute::v2::{
    ComputeServerApiRequest, ComputeServerDeleteBuilder, ComputeServerGetConsoleOutputBuilder,
    ComputeServerInstanceActionListBuilder, ComputeServerList,
};
use crate::cloud_worker::types::{ApiRequest, ComputeApiRequest};
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{GeneratedResourceBehaviour, ResourceBehaviour};
use crate::mode::Mode;

/// Behaviour implementation for ComputeServers.
pub struct ComputeServersBehaviour;

impl ResourceBehaviour for ComputeServersBehaviour {
    type Filter = ComputeServerList;

    fn view_key() -> &'static str {
        super::generated::server::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::server::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::server::Generated::mode()
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some("display_name".into());
            filter.sort_dir = Some("asc".into());
        }
        filter
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        // `Generated::Filter` is boxed (matches `Action::SetComputeServerListFilters`'s boxed
        // payload); this behaviour's `Filter` stays unboxed, so adapt at the boundary.
        super::generated::server::Generated::request_from_filter(&Box::new(filter.clone()))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::server::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::server::Generated::handle_set_filter_action(action).map(|f| *f)
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
            let sel = selected?;
            let mut del_builder = ComputeServerDeleteBuilder::default();
            del_builder.id(crate::components::view_render::get_str(sel, "/id")?.to_string());
            if let Some(name) = crate::components::view_render::get_str(sel, "/name") {
                del_builder.name(name.to_string());
            }
            let del = del_builder.build().ok()?;
            Some(ApiRequest::from(ComputeServerApiRequest::Delete(Box::new(
                del,
            ))))
        } else {
            None
        }
    }
    fn action_to_singular_request(
        action: &Action,
        selected: Option<&serde_json::Value>,
    ) -> Option<(Vec<Action>, ApiRequest)> {
        if let Action::ShowServerConsoleOutput = action {
            let server_id = crate::components::view_render::get_str(selected?, "/id")?;
            let req = ComputeServerApiRequest::GetConsoleOutput(Box::new(
                ComputeServerGetConsoleOutputBuilder::default()
                    .id(server_id.to_string())
                    .os_get_console_output(
                        crate::cloud_worker::compute::v2::server::get_console_output::OsGetConsoleOutputBuilder::default()
                            .build()
                            .ok()?,
                    )
                    .build()
                    .ok()?,
            ));
            Some((
                vec![
                    Action::SetDescribeLoading(true),
                    Action::Mode {
                        mode: Mode::Describe,
                        stack: true,
                    },
                ],
                ApiRequest::from(req),
            ))
        } else {
            None
        }
    }
    fn matches_singular_request(request: &ApiRequest) -> bool {
        matches!(request, ApiRequest::Compute(ComputeApiRequest::Server(boxreq)) if matches!(**boxreq, ComputeServerApiRequest::GetConsoleOutput(_)))
    }
    fn handle_singular_response_data(
        request: &ApiRequest,
        data: &[serde_json::Value],
    ) -> Option<Action> {
        if let ApiRequest::Compute(ComputeApiRequest::Server(boxreq)) = request
            && let ComputeServerApiRequest::GetConsoleOutput(_) = &**boxreq
        {
            return Some(Action::SetDescribeApiResponseData(
                data.first().cloned().unwrap_or_default(),
            ));
        }
        None
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&serde_json::Value>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowResource(key) = action
            && *key == crate::mode::COMPUTE_SERVER_INSTANCE_ACTION
            && let Some(sel) = selected
            && let Some(server_id) = crate::components::view_render::get_str(sel, "/id")
        {
            let mut list_builder = ComputeServerInstanceActionListBuilder::default();
            list_builder.server_id(server_id.to_string());
            if let Some(name) = crate::components::view_render::get_str(sel, "/name") {
                list_builder.server_name(name.to_string());
            }
            if let Ok(list) = list_builder.build() {
                return vec![
                    Action::Mode {
                        mode: Mode::Resource(crate::mode::COMPUTE_SERVER_INSTANCE_ACTION),
                        stack: true,
                    },
                    Action::SetComputeServerInstanceActionListFilters(Box::new(list)),
                ];
            }
        }
        Vec::new()
    }
}

/// Public component for ComputeServers using the generic view.
pub type ComputeServers = GenericResourceView<'static, ComputeServersBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::compute::v2::ComputeServerDelete;
    use crate::components::resource_behaviour::ResourceBehaviour;

    fn make_server(id: &str, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "status": "ACTIVE",
            "tenant_id": "tenant1",
            "user_id": "user1",
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-01T00:00:00Z",
            "accessIPv4": "",
            "accessIPv6": "",
            "flavor": {"id": "flavor1"},
            "image": "image1",
            "OS-DCF:diskConfig": "AUTO",
            "OS-EXT-AZ:availability_zone": "nova",
            "OS-EXT-STS:power_state": 1,
            "os-extended-volumes:volumes_attached": [],
            "metadata": {},
            "addresses": {},
            "config_drive": "",
            "hostId": "host1",
            "key_name": null,
            "security_groups": []
        })
    }

    #[test]
    fn normalise_filter_sets_defaults_when_sort_key_none() {
        let filter = ComputeServerList::default();
        let normalized = ComputeServersBehaviour::normalise_filter(filter);
        assert_eq!(normalized.sort_key, Some("display_name".into()));
        assert_eq!(normalized.sort_dir, Some("asc".into()));
    }

    #[test]
    fn normalise_filter_preserves_existing_sort_key() {
        let filter = ComputeServerList {
            sort_key: Some("id".into()),
            sort_dir: Some("desc".into()),
            ..Default::default()
        };
        let normalized = ComputeServersBehaviour::normalise_filter(filter);
        assert_eq!(normalized.sort_key, Some("id".into()));
        assert_eq!(normalized.sort_dir, Some("desc".into()));
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(ComputeServersBehaviour::view_key(), "compute.server");
        assert_eq!(ComputeServersBehaviour::title(), "Servers");
        assert_eq!(
            ComputeServersBehaviour::mode(),
            Mode::Resource(crate::mode::COMPUTE_SERVER)
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = ComputeServerList::default();
        let request = ComputeServersBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Server(boxreq))
            if matches!(*boxreq, ComputeServerApiRequest::ListDetailed(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list_detailed() {
        let filter = ComputeServerList::default();
        let request = ComputeServersBehaviour::request_from_filter(&filter);
        assert!(ComputeServersBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_non_matching() {
        let request = ApiRequest::from(ComputeServerApiRequest::Delete(Box::new(
            ComputeServerDelete {
                id: "test".into(),
                name: None,
            },
        )));
        assert!(!ComputeServersBehaviour::matches_request(&request));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = ComputeServerList {
            sort_key: Some("name".into()),
            ..Default::default()
        };
        let action = Action::SetComputeServerListFilters(Box::new(filter));
        let result = ComputeServersBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
        assert_eq!(result.unwrap().sort_key, Some("name".into()));
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated_action() {
        let action = Action::Tick;
        let result = ComputeServersBehaviour::handle_set_filter_action(&action);
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_delete_with_selected_server() {
        let server = make_server("server-1", "test-server");
        let result = ComputeServersBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::COMPUTE_SERVER,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&server),
        );
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Server(boxreq))
            if matches!(*boxreq, ComputeServerApiRequest::Delete(_))
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected_server() {
        let result = ComputeServersBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::COMPUTE_SERVER,
                op: crate::action::ResourceOp::Delete,
            },
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_ignores_delete_for_other_resource() {
        let server = make_server("server-1", "test-server");
        let result = ComputeServersBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::COMPUTE_AGGREGATE,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&server),
        );
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_returns_none_for_unrelated_action() {
        let server = make_server("server-1", "test-server");
        let result = ComputeServersBehaviour::confirm_request(&Action::Tick, Some(&server));
        assert!(result.is_none());
    }

    #[test]
    fn action_to_singular_request_console_output_with_server() {
        let server = make_server("server-1", "test-server");
        let result = ComputeServersBehaviour::action_to_singular_request(
            &Action::ShowServerConsoleOutput,
            Some(&server),
        );
        assert!(result.is_some());
        let (actions, request) = result.unwrap();
        assert_eq!(actions.len(), 2);
        assert!(matches!(actions[0], Action::SetDescribeLoading(true)));
        assert!(matches!(
            actions[1],
            Action::Mode {
                mode: Mode::Describe,
                stack: true
            }
        ));
        assert!(matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Server(boxreq))
            if matches!(*boxreq, ComputeServerApiRequest::GetConsoleOutput(_))
        ));
    }

    #[test]
    fn action_to_singular_request_console_output_without_server() {
        let result = ComputeServersBehaviour::action_to_singular_request(
            &Action::ShowServerConsoleOutput,
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn action_to_singular_request_returns_none_for_unrelated_action() {
        let server = make_server("server-1", "test-server");
        let result =
            ComputeServersBehaviour::action_to_singular_request(&Action::Tick, Some(&server));
        assert!(result.is_none());
    }

    #[test]
    fn matches_singular_request_returns_true_for_console_output() {
        let server = make_server("server-1", "test-server");
        let result = ComputeServersBehaviour::action_to_singular_request(
            &Action::ShowServerConsoleOutput,
            Some(&server),
        );
        let (_actions, request) = result.unwrap();
        assert!(ComputeServersBehaviour::matches_singular_request(&request));
    }

    #[test]
    fn matches_singular_request_returns_false_for_non_matching() {
        let filter = ComputeServerList::default();
        let request = ComputeServersBehaviour::request_from_filter(&filter);
        assert!(!ComputeServersBehaviour::matches_singular_request(&request));
    }

    #[test]
    fn handle_singular_response_data_sets_describe_data() {
        let server = make_server("server-1", "test-server");
        let result = ComputeServersBehaviour::action_to_singular_request(
            &Action::ShowServerConsoleOutput,
            Some(&server),
        );
        let (_actions, request) = result.unwrap();
        let data = vec![serde_json::json!({ "output": "test" })];
        let action = ComputeServersBehaviour::handle_singular_response_data(&request, &data);
        assert!(matches!(
            action,
            Some(Action::SetDescribeApiResponseData(_))
        ));
    }

    #[test]
    fn handle_singular_response_data_returns_none_for_non_matching_request() {
        let filter = ComputeServerList::default();
        let request = ComputeServersBehaviour::request_from_filter(&filter);
        let data = vec![serde_json::json!({ "output": "test" })];
        let action = ComputeServersBehaviour::handle_singular_response_data(&request, &data);
        assert!(action.is_none());
    }

    #[test]
    fn filter_carry_action_instance_actions_with_server() {
        let server = make_server("server-1", "test-server");
        let result = ComputeServersBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::COMPUTE_SERVER_INSTANCE_ACTION),
            Some(&server),
            &ComputeServerList::default(),
        );
        assert_eq!(result.len(), 2);
        assert!(matches!(
            result[0],
            Action::Mode {
                mode: Mode::Resource(crate::mode::COMPUTE_SERVER_INSTANCE_ACTION),
                stack: true
            }
        ));
        assert!(matches!(
            result[1],
            Action::SetComputeServerInstanceActionListFilters(_)
        ));
    }

    #[test]
    fn filter_carry_action_instance_actions_without_server() {
        let result = ComputeServersBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::COMPUTE_SERVER_INSTANCE_ACTION),
            None,
            &ComputeServerList::default(),
        );
        assert!(result.is_empty());
    }

    #[test]
    fn filter_carry_action_ignores_show_resource_for_other_key() {
        let server = make_server("server-1", "test-server");
        let result = ComputeServersBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::COMPUTE_SERVER),
            Some(&server),
            &ComputeServerList::default(),
        );
        assert!(result.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated_action() {
        let server = make_server("server-1", "test-server");
        let result = ComputeServersBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&server),
            &ComputeServerList::default(),
        );
        assert!(result.is_empty());
    }
}
