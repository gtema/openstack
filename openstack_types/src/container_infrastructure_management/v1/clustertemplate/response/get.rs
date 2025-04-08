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
//! Response type for the get clustertemplates/{clustertemplate_id} operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Clustertemplate response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ClustertemplateResponse {
    /// The exposed port of COE API server.
    ///
    pub apiserver_port: Option<i32>,

    /// Display the attribute `os_distro` defined as appropriate metadata in
    /// image for the cluster driver.
    ///
    pub cluster_distro: Option<String>,

    /// Specify the Container Orchestration Engine to use. Supported COEs
    /// include `kubernetes`. If your environment has additional cluster
    /// drivers installed, refer to the cluster driver documentation for the
    /// new COE names.
    ///
    pub coe: Option<Coe>,

    pub created_at: Option<String>,

    /// The DNS nameserver for the servers and containers in the cluster to
    /// use. This is configured in the private Neutron network for the cluster.
    /// The default is `8.8.8.8`.
    ///
    pub dns_nameserver: Option<String>,

    /// The name of a driver to manage the storage for the images and the
    /// container’s writable layer. The default is `devicemapper`.
    ///
    pub docker_storage_driver: Option<String>,

    /// The size in GB for the local storage on each server for the Docker
    /// daemon to cache the images and host the containers. Cinder volumes
    /// provide the storage. The default is 25 GB. For the `devicemapper`
    /// storage driver, the minimum value is 3GB. For the `overlay` storage
    /// driver, the minimum value is 1GB.
    ///
    pub docker_volume_size: Option<i32>,

    pub driver: Option<String>,

    /// The name or network ID of a Neutron network to provide connectivity to
    /// the external internet for the cluster. This network must be an external
    /// network, i.e. its attribute `router:external` must be `True`. The
    /// servers in the cluster will be connected to a private network and
    /// Magnum will create a router between this private network and the
    /// external network. This will allow the servers to download images,
    /// access discovery service, etc, and the containers to install packages,
    /// etc. In the opposite direction, floating IPs will be allocated from the
    /// external network to provide access from the external internet to
    /// servers and the container services hosted in the cluster.
    ///
    pub external_network_id: Option<String>,

    /// The name or network ID of a Neutron network to provide connectivity to
    /// the internal network for the cluster.
    ///
    pub fixed_network: Option<String>,

    /// Fixed subnet that are using to allocate network address for nodes in
    /// cluster.
    ///
    pub fixed_subnet: Option<String>,

    /// The nova flavor ID or name for booting the node servers. The default is
    /// `m1.small`.
    ///
    pub flavor_id: Option<String>,

    /// Whether enable or not using the floating IP of cloud provider. Some
    /// cloud providers used floating IP, some used public IP, thus Magnum
    /// provide this option for specifying the choice of using floating IP.
    ///
    pub floating_ip_enabled: Option<String>,

    /// Indicates whether the ClusterTemplate is hidden or not, the default
    /// value is false.
    ///
    pub hidden: Option<String>,

    /// The IP address for a proxy to use when direct http access from the
    /// servers to sites on the external internet is blocked. This may happen
    /// in certain countries or enterprises, and the proxy allows the servers
    /// and containers to access these sites. The format is a URL including a
    /// port number. The default is `None`.
    ///
    pub http_proxy: Option<String>,

    /// The IP address for a proxy to use when direct https access from the
    /// servers to sites on the external internet is blocked. This may happen
    /// in certain countries or enterprises, and the proxy allows the servers
    /// and containers to access these sites. The format is a URL including a
    /// port number. The default is `None`.
    ///
    pub https_proxy: Option<String>,

    /// The name or UUID of the base image in Glance to boot the servers for
    /// the cluster. The image must have the attribute `os_distro` defined as
    /// appropriate for the cluster driver.
    ///
    pub image_id: String,

    /// The URL pointing to users’s own private insecure docker registry to
    /// deploy and run docker containers.
    ///
    pub insecure_registry: Option<String>,

    /// The name of the SSH keypair to configure in the cluster servers for ssh
    /// access. Users will need the key to be able to ssh to the servers in the
    /// cluster. The login name is specific to the cluster driver, for example
    /// with fedora-atomic image, default login name is `fedora`.
    ///
    pub keypair_id: Option<String>,

    /// Arbitrary labels in the form of `key=value` pairs. The accepted keys
    /// and valid values are defined in the cluster drivers. They are used as a
    /// way to pass additional parameters that are specific to a cluster
    /// driver.
    ///
    pub labels: Option<HashMap<String, String>>,

    /// Links to the resources in question.
    ///
    pub links: Option<Vec<Links>>,

    /// The flavor of the master node for this cluster template.
    ///
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

    /// Name of the resource.
    ///
    pub name: Option<String>,

    /// The name of a network driver for providing the networks for the
    /// containers. Note that this is different and separate from the Neutron
    /// network for the cluster. The operation and networking model are
    /// specific to the particular driver.
    ///
    pub network_driver: Option<String>,

    /// When a proxy server is used, some sites should not go through the proxy
    /// and should be accessed normally. In this case, users can specify these
    /// sites as a comma separated list of IPs. The default is `None`.
    ///
    pub no_proxy: Option<String>,

    pub project_id: Option<String>,

    /// Access to a cluster template is normally limited to the admin, owner or
    /// users within the same tenant as the owners. Setting this flag makes the
    /// cluster template public and accessible by other users. The default is
    /// not public.
    ///
    pub public: Option<String>,

    /// Docker images by default are pulled from the public Docker registry,
    /// but in some cases, users may want to use a private registry. This
    /// option provides an alternative registry based on the Registry V2:
    /// Magnum will create a local registry in the cluster backed by swift to
    /// host the images. The default is to use the public registry.
    ///
    pub registry_enabled: Option<String>,

    /// The servers in the cluster can be `vm` or `baremetal`. This parameter
    /// selects the type of server to create for the cluster. The default is
    /// `vm`.
    ///
    pub server_type: Option<ServerType>,

    /// Administrator tags for the cluster template.
    ///
    pub tags: Option<String>,

    /// Transport Layer Security (TLS) is normally enabled to secure the
    /// cluster. In some cases, users may want to disable TLS in the cluster,
    /// for instance during development or to troubleshoot certain problems.
    /// Specifying this parameter will disable TLS so that users can access the
    /// COE endpoints without a certificate. The default is TLS enabled.
    ///
    pub tls_disabled: Option<String>,

    pub updated_at: Option<String>,

    pub user_id: Option<String>,

    /// The UUID of the cluster template.
    ///
    pub uuid: Option<String>,

    /// The name of a volume driver for managing the persistent storage for the
    /// containers. The functionality supported are specific to the driver.
    ///
    pub volume_driver: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Coe {
    // Kubernetes
    #[serde(rename = "kubernetes")]
    Kubernetes,
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
pub enum ServerType {
    // Bm
    #[serde(rename = "bm")]
    Bm,

    // Vm
    #[serde(rename = "vm")]
    Vm,
}
