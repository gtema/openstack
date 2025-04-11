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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.
//! Response type for the get local_ips/{local_ip_id}/port_associations operation

use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// PortAssociation response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct PortAssociationResponse {
    /// The IP of the port associated with the Local IP.
    ///
    #[structable(optional)]
    pub fixed_ip: Option<String>,

    /// The ID of the port associated with the Local IP.
    ///
    #[structable(optional)]
    pub fixed_port_id: Option<String>,

    /// The host of the port associated with the Local IP.
    ///
    #[structable(optional)]
    pub host: Option<String>,

    /// The actual IP address of the Local IP.
    ///
    #[structable(optional)]
    pub local_ip_address: Option<String>,

    /// The ID of the Local IP.
    ///
    #[structable(optional)]
    pub local_ip_id: Option<String>,
}
