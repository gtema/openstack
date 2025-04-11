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
//! Response type for the get messages/{id} operation

use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// Message response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct MessageResponse {
    /// The date and time when the resource was created.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    ///
    #[structable()]
    pub created_at: String,

    /// The id of the event to this message, this id could eventually be
    /// translated into `user_message`.
    ///
    #[structable()]
    pub event_id: String,

    /// The expire time of the message, this message could be deleted after
    /// this time.
    ///
    #[structable(optional)]
    pub guaranteed_until: Option<String>,

    /// The UUID for the message.
    ///
    #[structable()]
    pub id: String,

    /// Links for the message.
    ///
    #[structable(optional, serialize)]
    pub links: Option<Vec<Links>>,

    /// The level of the message, possible value is only ‘ERROR’ now.
    ///
    #[structable()]
    pub message_level: String,

    /// The id of the request during which the message was created.
    ///
    #[structable()]
    pub request_id: String,

    /// The resource type corresponding to `resource_uuid`.
    ///
    #[structable(optional)]
    pub resource_type: Option<String>,

    /// The UUID of the resource during whose operation the message was
    /// created.
    ///
    #[structable(optional)]
    pub resource_uuid: Option<String>,

    /// The translated readable message corresponding to `event_id`.
    ///
    #[structable()]
    pub user_message: String,
}

/// Links to the resources in question. See
/// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
/// for more info.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub href: Option<String>,
    pub rel: Option<String>,
}
