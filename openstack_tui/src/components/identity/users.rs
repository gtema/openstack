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

use openstack_types::identity::v3::user::response::list::UserResponse;

use crate::{
    action::Action,
    cloud_worker::identity::v3::{
        IdentityApiRequest, IdentityUserApiRequest, IdentityUserApplicationCredentialList,
        IdentityUserApplicationCredentialListBuilder,
        IdentityUserApplicationCredentialListBuilderError, IdentityUserDelete,
        IdentityUserDeleteBuilder, IdentityUserDeleteBuilderError, IdentityUserList,
        IdentityUserSetBuilder,
    },
    cloud_worker::types::ApiRequest,
    components::{Component, table_view::TableViewComponentBase},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "Identity Users";
const VIEW_CONFIG_KEY: &str = "identity.user";

impl ResourceKey for UserResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&UserResponse> for IdentityUserDelete {
    type Error = IdentityUserDeleteBuilderError;
    fn try_from(value: &UserResponse) -> Result<Self, Self::Error> {
        let mut builder = IdentityUserDeleteBuilder::default();
        builder.id(value.id.clone());
        builder.name(value.name.clone());
        builder.build()
    }
}

impl TryFrom<&UserResponse> for IdentityUserApplicationCredentialList {
    type Error = IdentityUserApplicationCredentialListBuilderError;
    fn try_from(value: &UserResponse) -> Result<Self, Self::Error> {
        let mut builder = IdentityUserApplicationCredentialListBuilder::default();
        builder.user_id(value.id.clone());
        builder.user_name(value.name.clone());
        builder.build()
    }
}

pub type IdentityUsers<'a> = TableViewComponentBase<'a, UserResponse, IdentityUserList>;

impl Component for IdentityUsers<'_> {
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
                if let Mode::IdentityUsers = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        IdentityUserApiRequest::List(Box::new(self.get_filters().clone())),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::IdentityUsers,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    IdentityUserApiRequest::List(Box::new(self.get_filters().clone())),
                ))));
            }
            Action::IdentityUserFlipEnable => {
                // only if we are currently in the proper mode
                if current_mode == Mode::IdentityUsers {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_row) = self.get_selected() {
                            // send action to set GroupUserListFilters
                            command_tx.send(Action::PerformApiRequest(ApiRequest::from(
                                IdentityUserApiRequest::Set(Box::new(
                                    IdentityUserSetBuilder::default()
                                        .id(selected_row.id.clone())
                                        .user(crate::cloud_worker::identity::v3::user::set::UserBuilder::default().enabled(!selected_row.enabled).build().wrap_err("cannot prepare user data structure")?)
                                        .build().wrap_err("error preparing OpenStack request")?
                                )))))?;
                            self.set_loading(true);
                        }
                    }
                }
            }
            Action::IdentityUserDelete => {
                // only if we are currently in the proper mode
                if current_mode == Mode::IdentityUsers {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_row) = self.get_selected() {
                            // send action to set Delete User
                            command_tx.send(Action::Confirm(ApiRequest::from(
                                IdentityUserApiRequest::Delete(Box::new(
                                    IdentityUserDelete::try_from(selected_row)
                                        .wrap_err("error preparing OpenStack request")?,
                                )),
                            )))?;
                        }
                    }
                }
            }
            Action::ShowIdentityUserApplicationCredentials => {
                // only if we are currently in the proper mode
                if current_mode == Mode::IdentityUsers {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_row) = self.get_selected() {
                            // send action to set GroupUserListFilters
                            command_tx.send(
                                Action::SetIdentityApplicationCredentialListFilters(
                                    IdentityUserApplicationCredentialList::try_from(selected_row)
                                        .wrap_err("error preparing OpenStack request")?,
                                ),
                            )?;
                            command_tx.send(Action::Mode {
                                mode: Mode::IdentityApplicationCredentials,
                                stack: true,
                            })?;
                        }
                    }
                }
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Identity(IdentityApiRequest::User(req)),
                data,
            } => {
                if let IdentityUserApiRequest::List(_) = *req {
                    self.set_data(data)?;
                }
            }
            Action::ApiResponseData {
                request: ApiRequest::Identity(IdentityApiRequest::User(req)),
                data,
            } => {
                if let IdentityUserApiRequest::Set(_) = *req {
                    // Since user update only returns some info (i.e. it doesn't contain email) we need
                    // to update record manually
                    let updated_user: UserResponse = serde_json::from_value(data.clone())?;
                    if let Some(item_row) = self.get_item_row_by_res_id_mut(&updated_user.id) {
                        item_row.enabled = updated_user.enabled;
                        item_row.name = updated_user.name;
                        self.sync_table_data()?;
                    }
                    self.set_loading(false);
                } else if let IdentityUserApiRequest::Delete(del) = *req
                    && let IdentityUserDelete { id, .. } = *del
                {
                    if self.delete_item_row_by_res_id_mut(&id)?.is_none() {
                        return Ok(Some(Action::Refresh));
                    }
                    self.sync_table_data()?;
                    self.set_loading(false);
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
