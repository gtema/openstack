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

pub struct LoadBalancerHealthMonitorsBehaviour;

impl ResourceBehaviour for LoadBalancerHealthMonitorsBehaviour {
    type Filter = cloud_types::LoadBalancerHealthmonitorList;

    fn view_key() -> &'static str {
        super::generated::healthmonitor::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::healthmonitor::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::healthmonitor::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::healthmonitor::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::healthmonitor::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::healthmonitor::Generated::handle_set_filter_action(action)
    }
}

pub type LoadBalancerHealthMonitors =
    GenericResourceView<'static, LoadBalancerHealthMonitorsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            LoadBalancerHealthMonitorsBehaviour::view_key(),
            "load-balancer.healthmonitor"
        );
        assert_eq!(
            LoadBalancerHealthMonitorsBehaviour::title(),
            "Health Monitors"
        );
        assert_eq!(
            LoadBalancerHealthMonitorsBehaviour::mode(),
            Mode::Resource(crate::mode::LB_HEALTHMONITOR)
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = cloud_types::LoadBalancerHealthmonitorList::default();
        let request = LoadBalancerHealthMonitorsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::LoadBalancer(cloud_types::LoadBalancerApiRequest::Healthmonitor(boxreq))
            if matches!(*boxreq, cloud_types::LoadBalancerHealthmonitorApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = cloud_types::LoadBalancerHealthmonitorList::default();
        let request = LoadBalancerHealthMonitorsBehaviour::request_from_filter(&filter);
        assert!(LoadBalancerHealthMonitorsBehaviour::matches_request(
            &request
        ));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req =
            ApiRequest::LoadBalancer(cloud_types::LoadBalancerApiRequest::Listener(Box::new(
                cloud_types::LoadBalancerListenerApiRequest::List(Box::default()),
            )));
        assert!(!LoadBalancerHealthMonitorsBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = cloud_types::LoadBalancerHealthmonitorList::default();
        let action = Action::SetLoadBalancerHealthMonitorListFilters(filter);
        let result = LoadBalancerHealthMonitorsBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = LoadBalancerHealthMonitorsBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }
}
