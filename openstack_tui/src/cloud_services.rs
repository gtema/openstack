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

use crate::action::{
    ComputeFlavorFilters, ComputeServerFilters, IdentityAuthProjectFilters, ImageFilters,
    NetworkNetworkFilters, NetworkSubnetFilters,
};

mod compute;
mod identity;
mod image;
mod network;

pub trait ComputeExt {
    async fn get_compute_flavors(&mut self, filters: &ComputeFlavorFilters) -> Result<Vec<Value>>;
    async fn get_compute_servers(&mut self, filters: &ComputeServerFilters) -> Result<Vec<Value>>;
    async fn get_compute_server_console_output(&mut self, id: &String) -> Result<Value>;
    async fn get_compute_quota(&mut self) -> Result<Value>;
}

pub trait IdentityExt {
    async fn get_identity_auth_projects(
        &mut self,
        filters: &IdentityAuthProjectFilters,
    ) -> Result<Vec<Value>>;
}

pub trait ImageExt {
    async fn get_image_images(&mut self, filters: &ImageFilters) -> Result<Vec<Value>>;
}

pub trait NetworkExt {
    async fn get_network_networks(
        &mut self,
        _filters: &NetworkNetworkFilters,
    ) -> Result<Vec<Value>>;

    async fn get_network_subnets(&mut self, filters: &NetworkSubnetFilters) -> Result<Vec<Value>>;
}
