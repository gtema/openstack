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
//! Response type for the post metadefs/namespaces operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Namespace response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct NamespaceResponse {
    /// Date and time of namespace creation
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// Provides a user friendly description of the namespace.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// The user friendly name for the namespace. Used by UI if available.
    ///
    #[structable(optional)]
    pub display_name: Option<String>,

    /// The unique namespace text.
    ///
    #[structable()]
    pub namespace: String,

    #[structable(optional, serialize)]
    pub objects: Option<Vec<Objects>>,

    /// Owner of the namespace.
    ///
    #[structable(optional)]
    pub owner: Option<String>,

    #[structable(optional, serialize)]
    pub properties: Option<HashMap<String, Properties>>,

    /// If true, namespace will not be deletable.
    ///
    #[structable(optional)]
    pub protected: Option<bool>,

    #[structable(optional, serialize)]
    pub resource_type_associations: Option<Vec<ResourceTypeAssociations>>,

    #[structable(optional)]
    pub schema: Option<String>,

    #[serde(rename = "self")]
    #[structable(optional, title = "self")]
    pub _self: Option<String>,

    #[structable(optional, serialize)]
    pub tags: Option<Vec<Tags>>,

    /// Date and time of the last namespace modification
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,

    /// Scope of namespace accessibility.
    ///
    #[structable(optional, serialize)]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Visibility {
    // Private
    #[serde(rename = "private")]
    Private,

    // Public
    #[serde(rename = "public")]
    Public,
}

/// `ResourceTypeAssociations` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResourceTypeAssociations {
    pub name: Option<String>,
    pub prefix: Option<String>,
    pub properties_target: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // Array
    #[serde(rename = "array")]
    Array,

    // Boolean
    #[serde(rename = "boolean")]
    Boolean,

    // Integer
    #[serde(rename = "integer")]
    Integer,

    // Number
    #[serde(rename = "number")]
    Number,

    // Object
    #[serde(rename = "object")]
    Object,

    // String
    #[serde(rename = "string")]
    String,
}

/// `Items` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Items {
    pub _enum: Option<Vec<String>>,
    pub _type: Option<Type>,
}

/// `Properties` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Properties {
    pub additional_items: Option<bool>,
    pub _default: Option<Value>,
    pub description: Option<String>,
    pub _enum: Option<Vec<String>>,
    pub items: Option<Items>,
    pub maximum: Option<f32>,
    pub max_items: Option<i32>,
    pub max_length: Option<i32>,
    pub minimum: Option<f32>,
    pub min_items: Option<i32>,
    pub min_length: Option<i32>,
    pub name: Option<String>,
    pub operators: Option<Vec<String>>,
    pub pattern: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<Vec<String>>,
    pub title: String,
    pub _type: Type,
    pub unique_items: Option<bool>,
}

/// `Objects` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Objects {
    pub description: Option<String>,
    pub name: Option<String>,
    pub properties: Option<HashMap<String, Properties>>,
    pub required: Option<Vec<String>>,
}

/// `Tags` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tags {
    pub name: Option<String>,
}
