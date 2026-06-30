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
    DnsApiRequest, DnsRecordsetList, DnsRecordsetListBuilder, DnsZoneApiRequest, DnsZoneDelete,
    DnsZoneDeleteBuilder, DnsZoneList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::dns::v2::zone::response::list::ZoneResponse;

const VIEW_CONFIG_KEY: &str = "dns.zone";

impl crate::utils::ResourceKey for ZoneResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&ZoneResponse> for DnsZoneDelete {
    type Error = crate::cloud_worker::dns::v2::DnsZoneDeleteBuilderError;
    fn try_from(value: &ZoneResponse) -> Result<Self, Self::Error> {
        let mut builder = DnsZoneDeleteBuilder::default();
        if let Some(val) = &value.id {
            builder.id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.name(val.clone());
        }
        builder.build()
    }
}

impl TryFrom<&ZoneResponse> for DnsRecordsetList {
    type Error = crate::cloud_worker::dns::v2::DnsRecordsetListBuilderError;
    fn try_from(value: &ZoneResponse) -> Result<Self, Self::Error> {
        let mut builder = DnsRecordsetListBuilder::default();
        if let Some(val) = &value.id {
            builder.zone_id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.zone_name(val.clone());
        }
        builder.build()
    }
}

pub struct DnsZonesBehaviour;

impl ResourceBehaviour for DnsZonesBehaviour {
    type Item = ZoneResponse;
    type Filter = DnsZoneList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "DNS Zones"
    }
    fn mode() -> Mode {
        Mode::DnsZones
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(DnsZoneApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Dns(DnsApiRequest::Zone(boxreq))
            if matches!(**boxreq, DnsZoneApiRequest::List(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetDnsZoneListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
    fn confirm_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        if let Action::DeleteDnsZone = action {
            let del = DnsZoneDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(DnsZoneApiRequest::Delete(Box::new(del))))
        } else {
            None
        }
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&Self::Item>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowDnsZoneRecordsets = action
            && let Some(sel) = selected
            && let Ok(list) = DnsRecordsetList::try_from(sel)
        {
            return vec![
                Action::SetDnsRecordsetListFilters(list),
                Action::Mode {
                    mode: Mode::DnsRecordsets,
                    stack: true,
                },
            ];
        }
        Vec::new()
    }
}

pub type DnsZones = GenericResourceView<'static, DnsZonesBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::dns::v2::zone::response::list::ZoneResponse;

    fn make_zone(id: &str, name: &str) -> ZoneResponse {
        let json = serde_json::json!({
            "id": id,
            "name": name,
            "email": "admin@example.com",
            "ttl": 3600,
            "status": "ACTIVE",
            "description": "test zone",
            "zone_type": "PRIMARY",
            "pool_id": "pool1",
            "masters": [],
            "version": 1,
            "transferred_at": null,
            "locked": false,
            "created_at": "2024-01-01T00:00:00",
            "updated_at": "2024-01-01T00:00:00",
            "attributes": {}
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(DnsZonesBehaviour::view_key(), "dns.zone");
        assert_eq!(DnsZonesBehaviour::title(), "DNS Zones");
        assert_eq!(DnsZonesBehaviour::mode(), Mode::DnsZones);
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = DnsZoneList::default();
        let request = DnsZonesBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Dns(DnsApiRequest::Zone(boxreq))
            if matches!(*boxreq, DnsZoneApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = DnsZoneList::default();
        let request = DnsZonesBehaviour::request_from_filter(&filter);
        assert!(DnsZonesBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Dns(DnsApiRequest::Recordset(Box::new(
            crate::cloud_worker::dns::v2::DnsRecordsetApiRequest::List(Box::new(
                crate::cloud_worker::dns::v2::DnsRecordsetList::default(),
            )),
        )));
        assert!(!DnsZonesBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = DnsZoneList::default();
        let action = Action::SetDnsZoneListFilters(filter);
        let result = DnsZonesBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = DnsZonesBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_delete_with_selected() {
        let zone = make_zone("zone-1", "example.com");
        let result = DnsZonesBehaviour::confirm_request(&Action::DeleteDnsZone, Some(&zone));
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::Dns(DnsApiRequest::Zone(boxreq))
            if matches!(*boxreq, DnsZoneApiRequest::Delete(_))
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected() {
        let result = DnsZonesBehaviour::confirm_request(&Action::DeleteDnsZone, None);
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_returns_none_for_unrelated() {
        let zone = make_zone("zone-1", "example.com");
        let result = DnsZonesBehaviour::confirm_request(&Action::Tick, Some(&zone));
        assert!(result.is_none());
    }

    #[test]
    fn filter_carry_action_show_recordsets_with_selected() {
        let zone = make_zone("zone-1", "example.com");
        let actions = DnsZonesBehaviour::filter_carry_action(
            &Action::ShowDnsZoneRecordsets,
            Some(&zone),
            &DnsZoneList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(actions[0], Action::SetDnsRecordsetListFilters(_)));
        assert!(matches!(
            actions[1],
            Action::Mode {
                mode: Mode::DnsRecordsets,
                stack: true
            }
        ));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = DnsZonesBehaviour::filter_carry_action(
            &Action::ShowDnsZoneRecordsets,
            None,
            &DnsZoneList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let zone = make_zone("zone-1", "example.com");
        let actions = DnsZonesBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&zone),
            &DnsZoneList::default(),
        );
        assert!(actions.is_empty());
    }
}
