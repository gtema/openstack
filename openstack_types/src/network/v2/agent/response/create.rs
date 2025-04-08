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
//! Response type for the post agents operation

use crate::common::BoolString;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Agent response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct AgentResponse {
    admin_state_up: Option<BoolString>,

    agent_type: Option<String>,

    alive: Option<bool>,

    availability_zone: Option<String>,

    binary: Option<String>,

    configurations: Option<HashMap<String, Value>>,

    created_at: Option<String>,

    description: Option<String>,

    heartbeat_timestamp: Option<String>,

    host: Option<String>,

    id: Option<String>,

    resources_synced: Option<String>,

    started_at: Option<String>,

    topic: Option<String>,
}
