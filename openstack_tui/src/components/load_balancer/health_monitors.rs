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
use crate::cloud_worker::load_balancer::v2::{
    LoadBalancerApiRequest, LoadBalancerHealthmonitorApiRequest, LoadBalancerHealthmonitorList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::load_balancer::v2::healthmonitor::response::list::HealthmonitorResponse;

const VIEW_CONFIG_KEY: &str = "load-balancer.healthmonitor";

impl crate::utils::ResourceKey for HealthmonitorResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct LoadBalancerHealthMonitorsBehaviour;

impl ResourceBehaviour for LoadBalancerHealthMonitorsBehaviour {
    type Item = HealthmonitorResponse;
    type Filter = LoadBalancerHealthmonitorList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "LB HealthMonitors"
    }
    fn mode() -> Mode {
        Mode::LoadBalancerHealthMonitors
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(LoadBalancerHealthmonitorApiRequest::List(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Healthmonitor(boxreq))
            if matches!(**boxreq, LoadBalancerHealthmonitorApiRequest::List(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetLoadBalancerHealthMonitorListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
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
            "LB HealthMonitors"
        );
        assert_eq!(
            LoadBalancerHealthMonitorsBehaviour::mode(),
            Mode::LoadBalancerHealthMonitors
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = LoadBalancerHealthmonitorList::default();
        let request = LoadBalancerHealthMonitorsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Healthmonitor(boxreq))
            if matches!(*boxreq, LoadBalancerHealthmonitorApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = LoadBalancerHealthmonitorList::default();
        let request = LoadBalancerHealthMonitorsBehaviour::request_from_filter(&filter);
        assert!(LoadBalancerHealthMonitorsBehaviour::matches_request(
            &request
        ));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::LoadBalancer(LoadBalancerApiRequest::Listener(Box::new(
            crate::cloud_worker::load_balancer::v2::LoadBalancerListenerApiRequest::List(Box::new(
                crate::cloud_worker::load_balancer::v2::LoadBalancerListenerList::default(),
            )),
        )));
        assert!(!LoadBalancerHealthMonitorsBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = LoadBalancerHealthmonitorList::default();
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
