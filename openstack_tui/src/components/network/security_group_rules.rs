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

use crossterm::event::KeyEvent;
use eyre::{Result, WrapErr};
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

use openstack_types::network::v2::security_group_rule::response::list::SecurityGroupRuleResponse;

use crate::{
    action::Action,
    cloud_worker::network::v2::{
        NetworkApiRequest, NetworkSecurityGroupRuleApiRequest, NetworkSecurityGroupRuleDelete,
        NetworkSecurityGroupRuleDeleteBuilder, NetworkSecurityGroupRuleDeleteBuilderError,
        NetworkSecurityGroupRuleList,
    },
    cloud_worker::types::ApiRequest,
    components::{Component, table_view::TableViewComponentBase},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "SecurityGroupRules";
const VIEW_CONFIG_KEY: &str = "network.security_group_rule";

impl ResourceKey for SecurityGroupRuleResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub type NetworkSecurityGroupRules<'a> =
    TableViewComponentBase<'a, SecurityGroupRuleResponse, NetworkSecurityGroupRuleList>;

impl NetworkSecurityGroupRules<'_> {
    /// Normalize filters
    ///
    /// Add preferred sorting
    fn normalize_filters(
        &self,
        mut filters: NetworkSecurityGroupRuleList,
    ) -> NetworkSecurityGroupRuleList {
        if filters.sort_key.is_none() {
            filters.sort_key = Some(vec![
                "ethertype".into(),
                "direction".into(),
                "protocol".into(),
                "port_range_min".into(),
            ]);
            filters.sort_dir = Some(vec!["asc".into(), "asc".into(), "asc".into(), "asc".into()]);
        }
        filters
    }

    /// Normalized filters
    fn normalized_filters(&self) -> NetworkSecurityGroupRuleList {
        self.normalize_filters(self.get_filters().clone())
            .to_owned()
    }
}

impl TryFrom<&SecurityGroupRuleResponse> for NetworkSecurityGroupRuleDelete {
    type Error = NetworkSecurityGroupRuleDeleteBuilderError;
    fn try_from(value: &SecurityGroupRuleResponse) -> Result<Self, Self::Error> {
        let mut builder = NetworkSecurityGroupRuleDeleteBuilder::default();
        if let Some(val) = &value.id {
            builder.id(val.clone());
        }
        builder.build()
    }
}

impl Component for NetworkSecurityGroupRules<'_> {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.set_config(config)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.set_command_tx(tx)
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match action {
            Action::CloudChangeScope(_) => {
                self.set_loading(true);
            }
            Action::ConnectedToCloud(_) => {
                self.set_loading(true);
                self.set_data(Vec::new())?;
                if let Mode::NetworkSecurityGroupRules = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        NetworkSecurityGroupRuleApiRequest::List(Box::new(
                            self.normalized_filters(),
                        )),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::NetworkSecurityGroupRules,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    NetworkSecurityGroupRuleApiRequest::List(Box::new(self.normalized_filters())),
                ))));
            }
            Action::SetNetworkSecurityGroupRuleListFilters(filters) => {
                self.set_filters(filters);
                self.set_data(Vec::new())?;
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    NetworkSecurityGroupRuleApiRequest::List(Box::new(self.normalized_filters())),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(req)),
                data,
            } => {
                if let NetworkSecurityGroupRuleApiRequest::List(_) = *req {
                    self.set_data(data)?;
                } else if let NetworkSecurityGroupRuleApiRequest::Create(_) = *req {
                    self.set_data(data)?;
                }
            }
            Action::ApiResponseData {
                request: ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(req)),
                data,
            } => {
                if let NetworkSecurityGroupRuleApiRequest::Delete(del) = *req {
                    let NetworkSecurityGroupRuleDelete { ref id, .. } = *del;
                    if self.delete_item_row_by_res_id_mut(id)?.is_none() {
                        return Ok(Some(Action::Refresh));
                    }
                    self.sync_table_data()?;
                    self.set_loading(false);
                } else if let NetworkSecurityGroupRuleApiRequest::Create(_) = *req {
                    self.append_new_row(data)?;
                    self.set_loading(false);
                }
            }
            Action::DeleteNetworkSecurityGroupRule => {
                // only if we are currently in the right mode
                if current_mode == Mode::NetworkSecurityGroupRules {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            // send action to delete the selected SecurityGroupRule
                            command_tx.send(Action::Confirm(ApiRequest::from(
                                NetworkSecurityGroupRuleApiRequest::Delete(Box::new(
                                    NetworkSecurityGroupRuleDelete::try_from(selected_entry)
                                        .wrap_err("error preparing OpenStack request")?,
                                )),
                            )))?;
                        }
                    }
                }
            }
            Action::CreateNetworkSecurityGroupRule => {
                if let Some(command_tx) = self.get_command_tx() {
                    command_tx.send(Action::Edit {
                        template: format!(
                            r#"# Please provide the SecurityGroupRule data as YAML
security_group_rule:
  #  /// The security group ID to associate with this security group rule.
  security_group_id: "{}"

  #  /// The minimum port number in the range that is matched by the security
  #  /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
  #  /// value must be less than or equal to the `port_range_max` attribute
  #  /// value. If the protocol is ICMP, this value must be an ICMP type.
  # port_range_min: 

  #  /// The maximum port number in the range that is matched by the security
  #  /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
  #  /// value must be greater than or equal to the `port_range_min` attribute
  #  /// value. If the protocol is ICMP, this value must be an ICMP code.
  # port_range_max: 

  #  /// The IP protocol can be represented by a string, an integer, or `null`.
  #  /// Valid string or integer values are `any` or `0`, `ah` or `51`, `dccp`
  #  /// or `33`, `egp` or `8`, `esp` or `50`, `gre` or `47`, `icmp` or `1`,
  #  /// `icmpv6` or `58`, `igmp` or `2`, `ipip` or `4`, `ipv6-encap` or `41`,
  #  /// `ipv6-frag` or `44`, `ipv6-icmp` or `58`, `ipv6-nonxt` or `59`,
  #  /// `ipv6-opts` or `60`, `ipv6-route` or `43`, `ospf` or `89`, `pgm` or
  #  /// `113`, `rsvp` or `46`, `sctp` or `132`, `tcp` or `6`, `udp` or `17`,
  #  /// `udplite` or `136`, `vrrp` or `112`. Additionally, any integer value
  #  /// between [0-255] is also valid. The string `any` (or integer `0`) means
  #  /// `all` IP protocols. See the constants in `neutron_lib.constants` for
  #  /// the most up-to-date list of supported strings.

  # protocol: 
  #  /// Must be IPv4 or IPv6, and addresses represented in CIDR must match the
  #  /// ingress or egress rules.

  # ethertype: 

  #  /// Ingress or egress, which is the direction in which the security group
  #  /// rule is applied.
  # direction: 

  #  /// A human-readable description for the resource. Default is an empty
  #  /// string.
  # description:
"#,
                            self.get_filters()
                                .security_group_id
                                .clone()
                                .unwrap_or("<HERE>".to_string())
                        ),
                        original_action: Box::new(Action::CreateNetworkSecurityGroupRule),
                    })?;
                }
            }
            Action::EditResult {
                result,
                original_action,
            } => {
                if let Action::CreateNetworkSecurityGroupRule = *original_action {
                    tracing::debug!("Would be creating sgr with {:?}", result);
                    self.set_loading(true);
                    if let Some(command_tx) = self.get_command_tx() {
                        let data: crate::cloud_worker::network::v2::security_group_rule::NetworkSecurityGroupRuleCreate = serde_json::from_value(result)?;
                        command_tx.send(Action::Confirm(ApiRequest::from(
                            NetworkSecurityGroupRuleApiRequest::Create(Box::new(data)),
                        )))?;
                    }
                }
            }
            _ => {}
        };
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.handle_key_events(key)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        self.draw(f, area, TITLE)
    }
}
