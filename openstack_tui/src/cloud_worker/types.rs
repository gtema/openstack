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
use strum::Display;

use crate::action::Action;
use openstack_sdk::AsyncOpenStack;
use openstack_sdk::types::ServiceType;
use tokio::sync::mpsc::UnboundedSender;

pub use crate::cloud_worker::block_storage::*;
pub use crate::cloud_worker::common::{CloudWorkerError, ConfirmableRequest};
pub use crate::cloud_worker::compute::*;
pub use crate::cloud_worker::dns::*;
pub use crate::cloud_worker::identity::*;
pub use crate::cloud_worker::image::*;
pub use crate::cloud_worker::load_balancer::*;
pub use crate::cloud_worker::network::*;

/// OpenStack "resource"
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum ApiRequest {
    // Block storage resources
    /// Block Storage api requests
    BlockStorage(BlockStorageApiRequest),

    // Compute resources
    /// Compute api requests
    Compute(ComputeApiRequest),

    /// DNS resources
    Dns(DnsApiRequest),

    /// Identity (Keystone)
    Identity(IdentityApiRequest),

    /// Image (Glance)
    Image(ImageApiRequest),

    /// Load Balancer
    LoadBalancer(LoadBalancerApiRequest),

    // Network (Neutron)
    Network(NetworkApiRequest),
}

impl From<ApiRequest> for ServiceType {
    fn from(item: ApiRequest) -> Self {
        match item {
            ApiRequest::BlockStorage(_) => Self::BlockStorage,
            ApiRequest::Compute(_) => Self::Compute,
            ApiRequest::Dns(_) => Self::Dns,
            ApiRequest::Identity(_) => Self::Identity,
            ApiRequest::Image(_) => Self::Image,
            ApiRequest::LoadBalancer(_) => Self::LoadBalancer,
            ApiRequest::Network(_) => Self::Network,
        }
    }
}

impl ApiRequest {
    /// Whether this request is a best-effort *usage probe*: a read of a
    /// quota/usage endpoint that is not guaranteed to exist on every cloud
    /// (the service may be absent, or the quota disabled server-side, e.g.
    /// Glance image quotas). A failure of such a request is not an error; the
    /// corresponding usage is simply omitted from the usage view.
    pub fn is_usage_probe(&self) -> bool {
        match self {
            ApiRequest::Compute(ComputeApiRequest::QuotaSet(req)) => {
                matches!(&**req, ComputeQuotaSetApiRequest::Details(_))
            }
            ApiRequest::Network(NetworkApiRequest::Quota(req)) => {
                matches!(&**req, NetworkQuotaApiRequest::Details(_))
            }
            ApiRequest::BlockStorage(BlockStorageApiRequest::QuotaSet(req)) => {
                matches!(&**req, BlockStorageQuotaSetApiRequest::Get(_))
            }
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Quota(req)) => {
                matches!(&**req, LoadBalancerQuotaApiRequest::Get(_))
            }
            ApiRequest::Dns(DnsApiRequest::Quota(req)) => {
                matches!(&**req, DnsQuotaApiRequest::Get(_))
            }
            ApiRequest::Image(ImageApiRequest::Usage(req)) => {
                matches!(&**req, ImageUsageApiRequest::List(_))
            }
            _ => self.usage_count_kind().is_some(),
        }
    }

    /// Kind of usage count an unfiltered list request provides, if any.
    /// Such lists are best-effort probes of the usage view too.
    pub fn usage_count_kind(&self) -> Option<&'static str> {
        match self {
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Loadbalancer(req)) => {
                if let LoadBalancerLoadbalancerApiRequest::List(list) = &**req
                    && {
                        // Scoping the list to a project is how Home counts usage
                        let mut unscoped = (**list).clone();
                        unscoped.project_id = None;
                        unscoped == Default::default()
                    }
                {
                    return Some("loadbalancer");
                }
                None
            }
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Listener(req)) => {
                if let LoadBalancerListenerApiRequest::List(list) = &**req
                    && {
                        // Scoping the list to a project is how Home counts usage
                        let mut unscoped = (**list).clone();
                        unscoped.project_id = None;
                        unscoped == Default::default()
                    }
                {
                    return Some("listener");
                }
                None
            }
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Pool(req)) => {
                if let LoadBalancerPoolApiRequest::List(list) = &**req
                    && {
                        // Scoping the list to a project is how Home counts usage
                        let mut unscoped = (**list).clone();
                        unscoped.project_id = None;
                        unscoped == Default::default()
                    }
                {
                    return Some("pool");
                }
                None
            }
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Healthmonitor(req)) => {
                if let LoadBalancerHealthmonitorApiRequest::List(list) = &**req
                    && {
                        // Scoping the list to a project is how Home counts usage
                        let mut unscoped = (**list).clone();
                        unscoped.project_id = None;
                        unscoped == Default::default()
                    }
                {
                    return Some("healthmonitor");
                }
                None
            }
            ApiRequest::Dns(DnsApiRequest::Zone(req)) => {
                if let DnsZoneApiRequest::List(list) = &**req
                    && *list == Default::default()
                {
                    return Some("zone");
                }
                None
            }
            ApiRequest::Dns(DnsApiRequest::Recordset(req)) => {
                if let DnsRecordsetApiRequest::List(list) = &**req
                    && *list == Default::default()
                {
                    return Some("recordset");
                }
                None
            }
            _ => None,
        }
    }
}

impl ConfirmableRequest for ApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            ApiRequest::BlockStorage(x) => x.get_confirm_message(),
            ApiRequest::Compute(x) => x.get_confirm_message(),
            ApiRequest::Dns(x) => x.get_confirm_message(),
            ApiRequest::Image(x) => x.get_confirm_message(),
            ApiRequest::Network(x) => x.get_confirm_message(),
            _ => None,
        }
    }
}

pub trait ExecuteApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_quota_reads_are_usage_probes() {
        assert!(ApiRequest::from(ImageUsageApiRequest::List(Box::default())).is_usage_probe());
        assert!(
            ApiRequest::from(BlockStorageQuotaSetApiRequest::Get(Box::new(
                BlockStorageQuotaSetShow {
                    project_id: "p".into(),
                    usage: Some(true),
                },
            )))
            .is_usage_probe()
        );
        assert!(
            ApiRequest::from(DnsQuotaApiRequest::Get(Box::new(DnsQuotaShow {
                project_id: "p".into(),
            })))
            .is_usage_probe()
        );
        assert!(
            ApiRequest::from(LoadBalancerQuotaApiRequest::Get(Box::new(
                LoadBalancerQuotaShow {
                    project_id: "p".into(),
                    ..Default::default()
                },
            )))
            .is_usage_probe()
        );
    }

    #[test]
    fn other_requests_are_not_usage_probes() {
        // Only the quota *read* is a probe, not other quota operations
        assert!(
            !ApiRequest::from(LoadBalancerQuotaApiRequest::Delete(Box::new(
                LoadBalancerQuotaDelete {
                    project_id: "p".into(),
                    ..Default::default()
                },
            )))
            .is_usage_probe()
        );
        // A plain resource list is not a probe
        assert!(!ApiRequest::from(ImageImageApiRequest::List(Box::default())).is_usage_probe());
    }
}
