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
//! Response type for the post os-keypairs operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Keypair response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct KeypairResponse {
    /// The date and time when the resource was created.
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A boolean indicates whether this keypair is deleted or not. The value
    /// is always false (not deleted).
    ///
    #[structable(optional)]
    pub deleted: Option<bool>,

    /// It is always null.
    ///
    #[structable(optional, serialize)]
    pub deleted_at: Option<String>,

    /// The fingerprint for the keypair.
    ///
    #[structable()]
    pub fingerprint: String,

    /// The keypair ID.
    ///
    #[structable()]
    pub id: i32,

    /// The name for the keypair.
    ///
    #[structable()]
    pub name: String,

    /// If you do not provide a public key on create, a new keypair will be
    /// built for you, and the private key will be returned during the initial
    /// create call. Make sure to save this, as there is no way to get this
    /// private key again in the future.
    ///
    /// **Available until version 2.91**
    ///
    #[structable(optional)]
    pub private_key: Option<String>,

    /// The keypair public key.
    ///
    #[structable()]
    pub public_key: String,

    /// The type of the keypair. Allowed values are `ssh` or `x509`.
    ///
    /// **New in version 2.2**
    ///
    #[serde(rename = "type")]
    #[structable(title = "type")]
    pub _type: String,

    /// It is always null.
    ///
    #[structable(optional, serialize)]
    pub updated_at: Option<String>,

    /// The user_id for a keypair.
    ///
    #[structable()]
    pub user_id: String,
}
