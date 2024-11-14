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

    async fn get_projects(&mut self, _filters: &IdentityProjectFilters) -> Result<Vec<Value>>;
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
            Resource::IdentityProjects(ref filters) => match self.get_projects(filters).await {
                Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch available projects: {:?}",
                    err
                )))?,
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
}
