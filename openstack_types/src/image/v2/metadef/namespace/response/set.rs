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
//! Response type for the put metadefs/namespaces/{namespace_name} operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Namespace response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct NamespaceResponse {
    /// Date and time of namespace creation
    ///
    created_at: Option<String>,

    /// Provides a user friendly description of the namespace.
    ///
    description: Option<String>,

    /// The user friendly name for the namespace. Used by UI if available.
    ///
    display_name: Option<String>,

    /// The unique namespace text.
    ///
    namespace: String,

    objects: Option<Vec<Objects>>,

    /// Owner of the namespace.
    ///
    owner: Option<String>,

    properties: Option<HashMap<String, Properties>>,

    /// If true, namespace will not be deletable.
    ///
    protected: Option<bool>,

    resource_type_associations: Option<Vec<ResourceTypeAssociations>>,

    schema: Option<String>,

    #[serde(rename = "self")]
    _self: Option<String>,

    tags: Option<Vec<Tags>>,

    /// Date and time of the last namespace modification
    ///
    updated_at: Option<String>,

    /// Scope of namespace accessibility.
    ///
    visibility: Option<Visibility>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Visibility {
    // Public
    #[serde(rename = "public")]
    Public,

    // Private
    #[serde(rename = "private")]
    Private,
}

/// `ResourceTypeAssociations` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResourceTypeAssociations {
    name: Option<String>,
    prefix: Option<String>,
    properties_target: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // String
    #[serde(rename = "string")]
    String,

    // Number
    #[serde(rename = "number")]
    Number,

    // Object
    #[serde(rename = "object")]
    Object,

    // Boolean
    #[serde(rename = "boolean")]
    Boolean,

    // Array
    #[serde(rename = "array")]
    Array,

    // Integer
    #[serde(rename = "integer")]
    Integer,
}

/// `Items` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Items {
    _enum: Option<Vec<String>>,
    _type: Option<Type>,
}

/// `Properties` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Properties {
    additional_items: Option<bool>,
    _default: Option<Value>,
    description: Option<String>,
    _enum: Option<Vec<String>>,
    items: Option<Items>,
    maximum: Option<f32>,
    max_items: Option<i32>,
    max_length: Option<i32>,
    minimum: Option<f32>,
    min_items: Option<i32>,
    min_length: Option<i32>,
    name: Option<String>,
    operators: Option<Vec<String>>,
    pattern: Option<String>,
    readonly: Option<bool>,
    required: Option<Vec<String>>,
    title: String,
    _type: Type,
    unique_items: Option<bool>,
}

/// `Objects` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Objects {
    description: Option<String>,
    name: Option<String>,
    properties: Option<HashMap<String, Properties>>,
    required: Option<Vec<String>>,
}

/// `Tags` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tags {
    name: Option<String>,
}
