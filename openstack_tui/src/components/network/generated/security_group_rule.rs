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
//
// Hand-authored for now: mirrors the shape a planned `RustResourceBehaviourGenerator`
// (openstack-codegenerator, Track 2) is expected to emit. Replace with generated output once
// that generator lands.

use crate::action::Action;
use crate::cloud_worker::network::v2::{
    NetworkApiRequest, NetworkSecurityGroupRuleApiRequest, NetworkSecurityGroupRuleList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::resource_behaviour::GeneratedResourceBehaviour;
use crate::mode::Mode;
use openstack_types::network::v2::security_group_rule::response::list::SecurityGroupRuleResponse;

pub(crate) struct Generated;

impl GeneratedResourceBehaviour for Generated {
    type Item = SecurityGroupRuleResponse;
    type Filter = NetworkSecurityGroupRuleList;

    fn view_key() -> &'static str {
        crate::mode::NETWORK_SECURITY_GROUP_RULE
    }
    fn title() -> &'static str {
        "Security Group Rules"
    }
    fn mode() -> Mode {
        Mode::Resource(Self::view_key())
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(NetworkSecurityGroupRuleApiRequest::List(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(boxreq))
            if matches!(**boxreq, NetworkSecurityGroupRuleApiRequest::List(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetNetworkSecurityGroupRuleListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
}
