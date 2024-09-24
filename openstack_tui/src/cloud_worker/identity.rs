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

use openstack_sdk::api::QueryAsync;

use crate::cloud_worker::types::{IdentityAuthProjectFilters, IdentityProjectFilters};
use crate::cloud_worker::Cloud;

pub trait IdentityExt {
    async fn get_auth_projects(
        &mut self,
        filters: &IdentityAuthProjectFilters,
    ) -> Result<Vec<Value>>;

    async fn get_projects(&mut self, _filters: &IdentityProjectFilters) -> Result<Vec<Value>>;
}

impl IdentityExt for Cloud {
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
