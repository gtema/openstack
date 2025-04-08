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
//! Response type for the get os-instance_usage_audit_log operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// InstanceUsageAuditLog response representation
#[derive(Clone, Deserialize, Serialize)]
struct InstanceUsageAuditLogResponse {
    /// The number of errors.
    ///
    errors: Option<i32>,

    /// A list of the hosts whose instance audit tasks have not run.
    ///
    hosts_not_run: Option<Vec<String>>,

    /// The number of instances.
    ///
    instances: Option<i32>,

    /// The object of instance usage audit logs.
    ///
    log: Option<HashMap<String, Value>>,

    /// The log message of the instance usage audit task.
    ///
    message: Option<String>,

    /// The number of the hosts.
    ///
    num_hosts: Option<i32>,

    /// The number of the hosts whose instance audit tasks have been done.
    ///
    num_hosts_done: Option<i32>,

    /// The number of the hosts whose instance audit tasks have not run.
    ///
    num_hosts_not_run: Option<i32>,

    /// The number of the hosts whose instance audit tasks are running.
    ///
    num_hosts_running: Option<i32>,

    /// The overall status of instance audit tasks.
    ///
    /// ```text
    /// M of N hosts done. K errors.
    ///
    /// ```
    ///
    /// The `M` value is the number of hosts whose instance audit tasks have
    /// been done in the period. The `N` value is the number of all hosts. The
    /// `K` value is the number of hosts whose instance audit tasks cause
    /// errors. If instance audit tasks have been done at all hosts in the
    /// period, the overall status is as follows:
    ///
    /// ```text
    /// ALL hosts done. K errors.
    ///
    /// ```
    ///
    overall_status: Option<String>,

    /// The beginning time of the instance usage audit period. For example,
    /// `2016-05-01 00:00:00`.
    ///
    period_beginning: Option<String>,

    /// The ending time of the instance usage audit period. For example,
    /// `2016-06-01 00:00:00`.
    ///
    period_ending: Option<String>,

    /// The state of the instance usage audit task. `DONE` or `RUNNING`.
    ///
    state: Option<State>,

    /// The total number of instance audit task errors.
    ///
    total_errors: Option<i32>,

    /// The total number of VM instances in the period.
    ///
    total_instances: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum State {
    // Done
    #[serde(rename = "DONE")]
    Done,

    // Running
    #[serde(rename = "RUNNING")]
    Running,
}
