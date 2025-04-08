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
//! Response type for the get os-quota-sets/{id}/detail operation

use serde::{Deserialize, Serialize};

/// QuotaSet response representation
#[derive(Clone, Deserialize, Serialize)]
struct QuotaSetResponse {
    /// The object of detailed cores quota, including in_use, limit and
    /// reserved number of cores.
    ///
    cores: Option<Cores>,

    /// The object of detailed fixed ips quota, including in_use, limit and
    /// reserved number of fixed ips.
    ///
    /// **Available until version 2.35**
    ///
    fixed_ips: Option<FixedIps>,

    /// The object of detailed floating ips quota, including in_use, limit and
    /// reserved number of floating ips.
    ///
    /// **Available until version 2.35**
    ///
    floating_ips: Option<FloatingIps>,

    /// The UUID of the tenant/user the quotas listed for.
    ///
    id: String,

    /// The object of detailed injected files quota, including in_use, limit
    /// and reserved number of injected files.
    ///
    /// **Available until version 2.56**
    ///
    injected_files: Option<InjectedFiles>,

    /// The object of detailed injected file content bytes quota, including
    /// in_use, limit and reserved number of injected file content bytes.
    ///
    injected_files_content_bytes: Option<InjectedFilesContentBytes>,

    /// The object of detailed injected file path bytes quota, including
    /// in_use, limit and reserved number of injected file path bytes.
    ///
    injected_files_path_bytes: Option<InjectedFilesPathBytes>,

    /// The object of detailed servers quota, including in_use, limit and
    /// reserved number of instances.
    ///
    instances: Option<Instances>,

    /// The object of detailed key pairs quota, including in_use, limit and
    /// reserved number of key pairs.
    ///
    /// Note
    ///
    /// `in_use` field value for keypair quota details is always zero. In Nova,
    /// key_pairs are a user-level resource, not a project- level resource, so
    /// for legacy reasons, the keypair in-use information is not counted.
    ///
    key_pairs: Option<KeyPairs>,

    /// The object of detailed key metadata items quota, including in_use,
    /// limit and reserved number of metadata items.
    ///
    metadata_items: Option<MetadataItems>,

    /// The number of private networks that can be created per project.
    ///
    /// **Available until version 2.35**
    ///
    networks: Option<Networks>,

    /// The object of detailed key ram quota, including in_use, limit and
    /// reserved number of ram.
    ///
    ram: Option<Ram>,

    /// The object of detailed security group rules quota, including in_use,
    /// limit and reserved number of security group rules.
    ///
    /// **Available until version 2.35**
    ///
    security_group_rules: Option<SecurityGroupRules>,

    /// The object of detailed security groups, including in_use, limit and
    /// reserved number of security groups.
    ///
    /// **Available until version 2.35**
    ///
    security_groups: Option<SecurityGroups>,

    /// The object of detailed server group members, including in_use, limit
    /// and reserved number of server group members.
    ///
    server_group_members: Option<ServerGroupMembers>,

    /// The object of detailed server groups, including in_use, limit and
    /// reserved number of server groups.
    ///
    server_groups: Option<ServerGroups>,
}

/// The object of detailed servers quota, including in_use, limit and reserved
/// number of instances.
///
/// `Instances` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Instances {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed cores quota, including in_use, limit and reserved
/// number of cores.
///
/// `Cores` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Cores {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed key ram quota, including in_use, limit and reserved
/// number of ram.
///
/// `Ram` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Ram {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed floating ips quota, including in_use, limit and
/// reserved number of floating ips.
///
/// **Available until version 2.35**
///
/// `FloatingIps` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct FloatingIps {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed fixed ips quota, including in_use, limit and
/// reserved number of fixed ips.
///
/// **Available until version 2.35**
///
/// `FixedIps` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct FixedIps {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed key metadata items quota, including in_use, limit
/// and reserved number of metadata items.
///
/// `MetadataItems` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct MetadataItems {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed key pairs quota, including in_use, limit and
/// reserved number of key pairs.
///
/// Note
///
/// `in_use` field value for keypair quota details is always zero. In Nova,
/// key_pairs are a user-level resource, not a project- level resource, so for
/// legacy reasons, the keypair in-use information is not counted.
///
/// `KeyPairs` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct KeyPairs {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed security groups, including in_use, limit and
/// reserved number of security groups.
///
/// **Available until version 2.35**
///
/// `SecurityGroups` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct SecurityGroups {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed security group rules quota, including in_use, limit
/// and reserved number of security group rules.
///
/// **Available until version 2.35**
///
/// `SecurityGroupRules` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct SecurityGroupRules {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed injected files quota, including in_use, limit and
/// reserved number of injected files.
///
/// **Available until version 2.56**
///
/// `InjectedFiles` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct InjectedFiles {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed injected file content bytes quota, including in_use,
/// limit and reserved number of injected file content bytes.
///
/// `InjectedFilesContentBytes` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct InjectedFilesContentBytes {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed injected file path bytes quota, including in_use,
/// limit and reserved number of injected file path bytes.
///
/// `InjectedFilesPathBytes` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct InjectedFilesPathBytes {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed server groups, including in_use, limit and reserved
/// number of server groups.
///
/// `ServerGroups` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct ServerGroups {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The object of detailed server group members, including in_use, limit and
/// reserved number of server group members.
///
/// `ServerGroupMembers` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct ServerGroupMembers {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}

/// The number of private networks that can be created per project.
///
/// **Available until version 2.35**
///
/// `Networks` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Networks {
    in_use: Option<i32>,
    limit: Option<i32>,
    reserved: Option<i32>,
}
