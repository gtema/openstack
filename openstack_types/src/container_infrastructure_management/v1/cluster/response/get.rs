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
//! Response type for the get clusters/{cluster_id} operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cluster response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ClusterResponse {
    /// The endpoint URL of COE API exposed to end-users.
    ///
    pub api_address: Option<String>,

    /// The UUID of the cluster template.
    ///
    pub cluster_template_id: String,

    /// Version info of chosen COE in cluster for helping client in picking the
    /// right version of client.
    ///
    pub coe_version: Option<String>,

    pub container_version: Option<String>,

    /// The timeout for cluster creation in minutes. The value expected is a
    /// positive integer and the default is 60 minutes. If the timeout is
    /// reached during cluster creation process, the operation will be aborted
    /// and the cluster status will be set to `CREATE_FAILED`.
    ///
    pub create_timeout: Option<i32>,

    pub created_at: Option<String>,

    /// The custom discovery url for node discovery. This is used by the COE to
    /// discover the servers that have been created to host the containers. The
    /// actual discovery mechanism varies with the COE. In some cases, Magnum
    /// fills in the server info in the discovery service. In other cases, if
    /// the `discovery_url` is not specified, Magnum will use the public
    /// discovery service at:
    ///
    /// ```text
    /// https://discovery.etcd.io
    ///
    /// ```
    ///
    /// In this case, Magnum will generate a unique url here for each uster and
    /// store the info for the servers.
    ///
    pub discovery_url: Option<String>,

    pub docker_volume_size: Option<i32>,

    pub faults: Option<HashMap<String, String>>,

    pub fixed_network: Option<String>,

    pub fixed_subnet: Option<String>,

    pub flavor_id: Option<String>,

    /// Whether enable or not using the floating IP of cloud provider. Some
    /// cloud providers used floating IP, some used public IP, thus Magnum
    /// provide this option for specifying the choice of using floating IP. If
    /// it’s not set, the value of floating_ip_enabled in template will be
    /// used.
    ///
    pub floating_ip_enabled: Option<String>,

    pub health_status: Option<HealthStatus>,

    pub health_status_reason: Option<HashMap<String, String>>,

    /// The name of the SSH keypair to configure in the cluster servers for ssh
    /// access. Users will need the key to be able to ssh to the servers in the
    /// cluster. The login name is specific to the cluster driver, for example
    /// with fedora-atomic image, default login name is `fedora`.
    ///
    pub keypair: Option<String>,

    pub labels: Option<HashMap<String, String>>,

    pub labels_added: Option<HashMap<String, String>>,

    pub labels_overridden: Option<HashMap<String, String>>,

    pub labels_skipped: Option<HashMap<String, String>>,

    /// Links to the resources in question.
    ///
    pub links: Option<Vec<Links>>,

    /// List of floating IP of all master nodes.
    ///
    pub master_addresses: Option<Vec<String>>,

    /// The number of servers that will serve as master for the cluster. The
    /// default is 1. Set to more than 1 master to enable High Availability. If
    /// the option `master-lb-enabled` is specified in the cluster template,
    /// the master servers will be placed in a load balancer pool.
    ///
    pub master_count: Option<i32>,

    pub master_flavor_id: Option<String>,

    /// Since multiple masters may exist in a cluster, a Neutron load balancer
    /// is created to provide the API endpoint for the cluster and to direct
    /// requests to the masters. In some cases, such as when the LBaaS service
    /// is not available, this option can be set to `false` to create a cluster
    /// without the load balancer. In this case, one of the masters will serve
    /// as the API endpoint. The default is `true`, i.e. to create the load
    /// balancer for the cluster.
    ///
    pub master_lb_enabled: Option<String>,

    pub merge_labels: Option<String>,

    /// Name of the resource.
    ///
    pub name: Option<String>,

    /// List of floating IP of all servers that serve as node.
    ///
    pub node_addresses: Option<Vec<String>>,

    /// The number of servers that will serve as node in the cluster. The
    /// default is 1.
    ///
    pub node_count: Option<i32>,

    pub project_id: Option<String>,

    /// The reference UUID of orchestration stack from Heat orchestration
    /// service.
    ///
    pub stack_id: Option<String>,

    /// The current state of the cluster.
    ///
    pub status: Option<Status>,

    /// The reason of cluster current status.
    ///
    pub status_reason: Option<String>,

    pub updated_at: Option<String>,

    pub user_id: Option<String>,

    /// The UUID of the cluster.
    ///
    pub uuid: Option<String>,
}

/// A link representation.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub created_at: Option<String>,
    pub href: Option<String>,
    pub rel: Option<String>,
    pub _type: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // AdoptComplete
    #[serde(rename = "ADOPT_COMPLETE")]
    AdoptComplete,

    // CheckComplete
    #[serde(rename = "CHECK_COMPLETE")]
    CheckComplete,

    // CreateComplete
    #[serde(rename = "CREATE_COMPLETE")]
    CreateComplete,

    // CreateFailed
    #[serde(rename = "CREATE_FAILED")]
    CreateFailed,

    // CreateInProgress
    #[serde(rename = "CREATE_IN_PROGRESS")]
    CreateInProgress,

    // DeleteComplete
    #[serde(rename = "DELETE_COMPLETE")]
    DeleteComplete,

    // DeleteFailed
    #[serde(rename = "DELETE_FAILED")]
    DeleteFailed,

    // DeleteInProgress
    #[serde(rename = "DELETE_IN_PROGRESS")]
    DeleteInProgress,

    // RestoreComplete
    #[serde(rename = "RESTORE_COMPLETE")]
    RestoreComplete,

    // ResumeComplete
    #[serde(rename = "RESUME_COMPLETE")]
    ResumeComplete,

    // ResumeFailed
    #[serde(rename = "RESUME_FAILED")]
    ResumeFailed,

    // RollbackComplete
    #[serde(rename = "ROLLBACK_COMPLETE")]
    RollbackComplete,

    // RollbackFailed
    #[serde(rename = "ROLLBACK_FAILED")]
    RollbackFailed,

    // RollbackInProgress
    #[serde(rename = "ROLLBACK_IN_PROGRESS")]
    RollbackInProgress,

    // SnapshotComplete
    #[serde(rename = "SNAPSHOT_COMPLETE")]
    SnapshotComplete,

    // UpdateComplete
    #[serde(rename = "UPDATE_COMPLETE")]
    UpdateComplete,

    // UpdateFailed
    #[serde(rename = "UPDATE_FAILED")]
    UpdateFailed,

    // UpdateInProgress
    #[serde(rename = "UPDATE_IN_PROGRESS")]
    UpdateInProgress,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum HealthStatus {
    // Healthy
    #[serde(rename = "HEALTHY")]
    Healthy,

    // Unhealthy
    #[serde(rename = "UNHEALTHY")]
    Unhealthy,

    // Unknown
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
