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
    DnsRecordsetList, DnsRecordsetListBuilder, DnsZoneApiRequest, DnsZoneDelete,
    DnsZoneDeleteBuilder, DnsZoneList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{GeneratedResourceBehaviour, ResourceBehaviour};
use crate::mode::Mode;

impl TryFrom<&serde_json::Value> for DnsZoneDelete {
    type Error = crate::cloud_worker::dns::v2::DnsZoneDeleteBuilderError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let mut builder = DnsZoneDeleteBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.id(val.to_string());
        }
        if let Some(val) = crate::components::view_render::get_str(value, "/name") {
            builder.name(val.to_string());
        }
        builder.build()
    }
}

impl TryFrom<&serde_json::Value> for DnsRecordsetList {
    type Error = crate::cloud_worker::dns::v2::DnsRecordsetListBuilderError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let mut builder = DnsRecordsetListBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.zone_id(val.to_string());
        }
        if let Some(val) = crate::components::view_render::get_str(value, "/name") {
            builder.zone_name(val.to_string());
        }
        builder.build()
    }
}

pub struct DnsZonesBehaviour;

impl ResourceBehaviour for DnsZonesBehaviour {
    type Filter = DnsZoneList;

    fn view_key() -> &'static str {
        super::generated::zone::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::zone::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::zone::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::zone::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::zone::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::zone::Generated::handle_set_filter_action(action)
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
            let del = DnsZoneDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(DnsZoneApiRequest::Delete(Box::new(del))))
        } else {
            None
        }
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&serde_json::Value>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowResource(key) = action
            && *key == crate::mode::DNS_RECORDSET
            && let Some(sel) = selected
            && let Ok(list) = DnsRecordsetList::try_from(sel)
        {
            return vec![
                Action::Mode {
                    mode: Mode::Resource(crate::mode::DNS_RECORDSET),
                    stack: true,
                },
                Action::SetDnsRecordsetListFilters(list),
            ];
        }
        Vec::new()
    }
}

pub type DnsZones = GenericResourceView<'static, DnsZonesBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::dns::v2::DnsApiRequest;
    use crate::components::resource_behaviour::ResourceBehaviour;

    fn make_zone(id: &str, name: &str) -> serde_json::Value {
        serde_json::json!({
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
        })
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(DnsZonesBehaviour::view_key(), "dns.zone");
        assert_eq!(DnsZonesBehaviour::title(), "Zones");
        assert_eq!(
            DnsZonesBehaviour::mode(),
            Mode::Resource(crate::mode::DNS_ZONE)
        );
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
            crate::cloud_worker::dns::v2::DnsRecordsetApiRequest::List(Box::default()),
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
        let result = DnsZonesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::DNS_ZONE,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&zone),
        );
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
        let result = DnsZonesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::DNS_ZONE,
                op: crate::action::ResourceOp::Delete,
            },
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_returns_none_for_unrelated() {
        let zone = make_zone("zone-1", "example.com");
        let result = DnsZonesBehaviour::confirm_request(&Action::Tick, Some(&zone));
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_ignores_delete_for_other_resource() {
        let zone = make_zone("zone-1", "example.com");
        let result = DnsZonesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::DNS_RECORDSET,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&zone),
        );
        assert!(result.is_none());
    }

    #[test]
    fn filter_carry_action_show_recordsets_with_selected() {
        let zone = make_zone("zone-1", "example.com");
        let actions = DnsZonesBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::DNS_RECORDSET),
            Some(&zone),
            &DnsZoneList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::Mode {
                mode: Mode::Resource(crate::mode::DNS_RECORDSET),
                stack: true
            }
        ));
        assert!(matches!(actions[1], Action::SetDnsRecordsetListFilters(_)));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = DnsZonesBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::DNS_RECORDSET),
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

    #[test]
    fn filter_carry_action_ignores_show_resource_for_other_key() {
        let zone = make_zone("zone-1", "example.com");
        let actions = DnsZonesBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::DNS_ZONE),
            Some(&zone),
            &DnsZoneList::default(),
        );
        assert!(actions.is_empty());
    }
}
