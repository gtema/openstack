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
//! Response type for the get lbaas/availabilityzoneprofiles/{availabilityzoneprofile_id} operation

use serde::{Deserialize, Serialize};

/// AvailabilityZoneProfile response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct AvailabilityZoneProfileResponse {
    pub availability_zone_data: Option<String>,

    pub id: Option<String>,

    pub name: Option<String>,

    pub provider_name: Option<String>,
}
