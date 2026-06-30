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
use crate::cloud_worker::dns::v2::{
    DnsApiRequest, DnsRecordsetApiRequest, DnsRecordsetList, DnsZoneApiRequest,
    DnsZoneRecordsetApiRequest, DnsZoneRecordsetList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::dns::v2::recordset::response::list::RecordsetResponse;

const VIEW_CONFIG_KEY: &str = "dns.recordset";

impl crate::utils::ResourceKey for RecordsetResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl From<DnsRecordsetList> for DnsZoneRecordsetList {
    fn from(value: DnsRecordsetList) -> Self {
        Self {
            _type: value._type,
            data: value.data,
            description: value.description,
            limit: value.limit,
            marker: value.marker,
            name: value.name,
            sort_dir: value.sort_dir,
            sort_key: value.sort_key,
            status: value.status,
            ttl: value.ttl,
            zone_name: value.zone_name,
            zone_id: value.zone_id.unwrap_or_default(),
        }
    }
}

pub struct DnsRecordsetsBehaviour;

impl ResourceBehaviour for DnsRecordsetsBehaviour {
    type Item = RecordsetResponse;
    type Filter = DnsRecordsetList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "DNS Recordsets"
    }
    fn mode() -> Mode {
        Mode::DnsRecordsets
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        if filter.zone_id.is_some() {
            ApiRequest::from(DnsZoneRecordsetApiRequest::List(Box::new(
                filter.clone().into(),
            )))
        } else {
            ApiRequest::from(DnsRecordsetApiRequest::List(Box::new(filter.clone())))
        }
    }
    fn matches_request(request: &ApiRequest) -> bool {
        match request {
            ApiRequest::Dns(DnsApiRequest::Recordset(_)) => true,
            ApiRequest::Dns(DnsApiRequest::Zone(sub)) => {
                matches!(**sub, DnsZoneApiRequest::Recordset(_))
            }
            _ => false,
        }
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetDnsRecordsetListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
}

pub type DnsRecordsets = GenericResourceView<'static, DnsRecordsetsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    fn make_filter_with_zone() -> DnsRecordsetList {
        DnsRecordsetList {
            zone_id: Some("zone-1".into()),
            ..Default::default()
        }
    }

    fn make_filter_without_zone() -> DnsRecordsetList {
        DnsRecordsetList::default()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(DnsRecordsetsBehaviour::view_key(), "dns.recordset");
        assert_eq!(DnsRecordsetsBehaviour::title(), "DNS Recordsets");
        assert_eq!(DnsRecordsetsBehaviour::mode(), Mode::DnsRecordsets);
    }

    #[test]
    fn request_from_filter_with_zone_id_creates_zone_recordset_list() {
        let filter = make_filter_with_zone();
        let request = DnsRecordsetsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Dns(DnsApiRequest::Zone(boxreq))
            if matches!(*boxreq, DnsZoneApiRequest::Recordset(_))
        ));
    }

    #[test]
    fn request_from_filter_without_zone_id_creates_recordset_list() {
        let filter = make_filter_without_zone();
        let request = DnsRecordsetsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Dns(DnsApiRequest::Recordset(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_recordset_list() {
        let filter = make_filter_without_zone();
        let request = DnsRecordsetsBehaviour::request_from_filter(&filter);
        assert!(DnsRecordsetsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_true_for_zone_recordset() {
        let filter = make_filter_with_zone();
        let request = DnsRecordsetsBehaviour::request_from_filter(&filter);
        assert!(DnsRecordsetsBehaviour::matches_request(&request));
    }
    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let del = crate::cloud_worker::dns::v2::DnsZoneDeleteBuilder::default()
            .id("test".into())
            .build()
            .unwrap();
        let unrelated = ApiRequest::from(DnsZoneApiRequest::Delete(Box::new(del)));
        assert!(!DnsRecordsetsBehaviour::matches_request(&unrelated));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = make_filter_with_zone();
        let action = Action::SetDnsRecordsetListFilters(filter);
        let result = DnsRecordsetsBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
        assert_eq!(result.unwrap().zone_id, Some("zone-1".into()));
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = DnsRecordsetsBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }
}
