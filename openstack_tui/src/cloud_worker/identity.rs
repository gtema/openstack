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

use eyre::Result;
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;

use openstack_sdk::api::QueryAsync;

use crate::action::Action;
use crate::cloud_worker::{Cloud, Resource};

pub mod types;
use types::*;

pub trait IdentityExt {
    async fn query_resource(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        resource: Resource,
    ) -> Result<()>;

    async fn get_auth_projects(
        &mut self,
        filters: &IdentityAuthProjectFilters,
    ) -> Result<Vec<Value>>;

    async fn get_groups(&mut self, _filters: &IdentityGroupFilters) -> Result<Vec<Value>>;
    async fn get_group_users(&mut self, _filters: &IdentityGroupUserFilters) -> Result<Vec<Value>>;
    async fn get_projects(&mut self, _filters: &IdentityProjectFilters) -> Result<Vec<Value>>;
    /// Get all users
    async fn get_users(&mut self, _filters: &IdentityUserFilters) -> Result<Vec<Value>>;
    /// Update user
    async fn update_user(&mut self, data: &IdentityUserUpdate) -> Result<Value>;
    /// Get user application credentials
    async fn get_user_application_credentials(
        &mut self,
        _filters: &IdentityApplicationCredentialFilters,
    ) -> Result<Vec<Value>>;
}

impl IdentityExt for Cloud {
    async fn query_resource(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        resource: Resource,
    ) -> Result<()> {
        match resource {
            Resource::IdentityAuthProjects(ref filters) => {
                match self.get_auth_projects(filters).await {
                    Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch available project scopes: {:?}",
                        err
                    )))?,
                }
            }
            Resource::IdentityGroups(ref filters) => match self.get_groups(filters).await {
                Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch available groups\n\nSome clouds require to use domain scope with the user having `manager` role\n{:?}",
                    err
                )))?,
            },
            Resource::IdentityGroupUsers(ref filters) => match self.get_group_users(filters).await {
                Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch available group users\n\nSome clouds require to use domain scope with the user having `manager` role\n{:?}",
                    err
                )))?,
            },
            Resource::IdentityProjects(ref filters) => match self.get_projects(filters).await {
                Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch available projects: {:?}",
                    err
                )))?,
            },
            Resource::IdentityUsers(ref filters) => match self.get_users(filters).await {
                Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch available users\n\nSome clouds require to use domain scope with the user having `manager` role\n{:?}",
                    err
                )))?,
            },
            Resource::IdentityUserUpdate(ref data) => match self.update_user(data).await {
                Ok(data) => app_tx.send(Action::ResourceData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to update user\n\nSome clouds require to use domain scope with the user having `manager` role\n{:?}",
                    err
                )))?,
            },
            Resource::IdentityApplicationCredentials(ref filters) => {
                let mut maybe_changed_filters = filters.clone();
                if maybe_changed_filters.user_id.is_empty() {
                    if let Some(session) = &self.cloud {
                        if let Some(auth) = session.get_auth_info() {
                            maybe_changed_filters.user_id = auth.token.user.id;
                            maybe_changed_filters.user_name = Some(auth.token.user.name);
                            app_tx.send(Action::IdentityApplicationCredentialFilter(maybe_changed_filters))?;
                        }
                    }
                } else {
                    match self.get_user_application_credentials(&maybe_changed_filters).await {
                        Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                        Err(err) => app_tx.send(Action::Error(format!(
                            "Failed to fetch available application credentials\n\nSome clouds require to use domain scope with the user having `manager` role\n{:?}",
                            err
                        )))?,
                    }
                }
            },
            _ => {
                todo!()
            }
        }
        Ok(())
    }

    async fn get_auth_projects(
        &mut self,
        _filters: &IdentityAuthProjectFilters,
    ) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep_builder =
                openstack_sdk::api::identity::v3::auth::project::list::Request::builder();

            //if let Some(vis) = &filters.visibility {
            //    ep_builder.visibility(vis);
            //}
            let ep = ep_builder.build()?;
            let res: Vec<Value> = ep.query_async(session).await?;
            //let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_projects(&mut self, _filters: &IdentityProjectFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep_builder = openstack_sdk::api::identity::v3::project::list::Request::builder();

            //if let Some(vis) = &filters.visibility {
            //    ep_builder.visibility(vis);
            //}
            let ep = ep_builder.build()?;
            let res: Vec<Value> = ep.query_async(session).await?;
            //let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_groups(&mut self, _filters: &IdentityGroupFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep_builder = openstack_sdk::api::identity::v3::group::list::Request::builder();

            //if let Some(vis) = &filters.visibility {
            //    ep_builder.visibility(vis);
            //}
            let ep = ep_builder.build()?;
            let res: Vec<Value> = ep.query_async(session).await?;
            //let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_group_users(&mut self, filters: &IdentityGroupUserFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep = openstack_sdk::api::identity::v3::group::user::list::Request::builder()
                .group_id(&filters.group_id)
                .build()?;

            let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_users(&mut self, _filters: &IdentityUserFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep_builder = openstack_sdk::api::identity::v3::user::list::Request::builder();

            //if let Some(vis) = &filters.visibility {
            //    ep_builder.visibility(vis);
            //}
            let ep = ep_builder.build()?;
            let res: Vec<Value> = ep.query_async(session).await?;
            //let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn update_user(&mut self, data: &IdentityUserUpdate) -> Result<Value> {
        if let Some(session) = &self.cloud {
            let mut ep_builder = openstack_sdk::api::identity::v3::user::set::Request::builder();
            let mut user_builder =
                openstack_sdk::api::identity::v3::user::set::UserBuilder::default();
            ep_builder.id(data.id.clone());
            if let Some(name) = &data.name {
                user_builder.name(name);
            }
            if let Some(enabled) = &data.enabled {
                user_builder.enabled(*enabled);
            }
            ep_builder.user(user_builder.build()?);

            let ep = ep_builder.build()?;
            let res: Value = ep.query_async(session).await?;
            debug!("Updated user information: {:?}", res);
            return Ok(res);
        }
        Ok(Value::Null)
    }

    async fn get_user_application_credentials(
        &mut self,
        filters: &IdentityApplicationCredentialFilters,
    ) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let mut ep_builder = openstack_sdk::api::identity::v3::user::application_credential::list::Request::builder();
            if filters.user_id.is_empty() {
                if let Some(auth) = session.get_auth_info() {
                    ep_builder.user_id(auth.token.user.id);
                }
            } else {
                ep_builder.user_id(filters.user_id.clone());
            }

            let ep = ep_builder.build()?;

            let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }
}
