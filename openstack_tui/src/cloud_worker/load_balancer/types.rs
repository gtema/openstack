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

use serde::{Deserialize, Serialize};
use std::fmt;

//use crate::cloud_worker::common::ConfirmableRequest;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadBalancerFilters {}

impl fmt::Display for LoadBalancerFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadBalancerListenerFilters {
    pub loadbalancer_id: Option<String>,
    pub loadbalancer_name: Option<String>,
}

impl fmt::Display for LoadBalancerListenerFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.loadbalancer_id.is_some() || self.loadbalancer_name.is_some() {
            parts.push(format!(
                "lb: {}",
                self.loadbalancer_name
                    .as_ref()
                    .or(self.loadbalancer_id.as_ref())
                    .unwrap_or(&String::new())
            ));
        }
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&LoadBalancerListenerFilters>
    for openstack_sdk::api::load_balancer::v2::listener::list::RequestBuilder
{
    type Error = eyre::Report;

    fn try_from(value: &LoadBalancerListenerFilters) -> Result<Self, Self::Error> {
        let ep_builder = openstack_sdk::api::load_balancer::v2::listener::list::Request::builder();

        if let Some(_lb_id) = &value.loadbalancer_id {
            // TODO
            //ep_builder.loadbalancer_id(lb_id.clone());
        }

        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadBalancerPoolFilters {
    //pub loadbalancer_id: Option<String>,
    //pub loadbalancer_name: Option<String>,
}

impl fmt::Display for LoadBalancerPoolFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
        //let mut parts: Vec<String> = Vec::new();
        //if self.loadbalancer_id.is_some() || self.loadbalancer_name.is_some() {
        //    parts.push(format!(
        //        "lb: {}",
        //        self.loadbalancer_name
        //            .as_ref()
        //            .or(self.loadbalancer_id.as_ref())
        //            .unwrap_or(&String::new())
        //    ));
        //}
        //write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&LoadBalancerPoolFilters>
    for openstack_sdk::api::load_balancer::v2::pool::list::RequestBuilder
{
    type Error = eyre::Report;

    fn try_from(_value: &LoadBalancerPoolFilters) -> Result<Self, Self::Error> {
        let ep_builder = openstack_sdk::api::load_balancer::v2::pool::list::Request::builder();

        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadBalancerPoolMemberFilters {
    pub pool_id: String,
    pub pool_name: Option<String>,
}

impl fmt::Display for LoadBalancerPoolMemberFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "pool: {}",
            self.pool_name.as_ref().unwrap_or(&self.pool_id)
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&LoadBalancerPoolMemberFilters>
    for openstack_sdk::api::load_balancer::v2::pool::member::list::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(value: &LoadBalancerPoolMemberFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::load_balancer::v2::pool::member::list::Request::builder();

        ep_builder.pool_id(value.pool_id.clone());

        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadBalancerHealthMonitorFilters {
    //pub loadbalancer_id: Option<String>,
    //pub loadbalancer_name: Option<String>,
}

impl fmt::Display for LoadBalancerHealthMonitorFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
        //let mut parts: Vec<String> = Vec::new();
        //if self.loadbalancer_id.is_some() || self.loadbalancer_name.is_some() {
        //    parts.push(format!(
        //        "lb: {}",
        //        self.loadbalancer_name
        //            .as_ref()
        //            .or(self.loadbalancer_id.as_ref())
        //            .unwrap_or(&String::new())
        //    ));
        //}
        //write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&LoadBalancerHealthMonitorFilters>
    for openstack_sdk::api::load_balancer::v2::healthmonitor::list::RequestBuilder
{
    type Error = eyre::Report;

    fn try_from(_value: &LoadBalancerHealthMonitorFilters) -> Result<Self, Self::Error> {
        let ep_builder =
            openstack_sdk::api::load_balancer::v2::healthmonitor::list::Request::builder();

        Ok(ep_builder)
    }
}
