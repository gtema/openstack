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
//! Response type for the post metadefs/namespaces/{namespace_name}/objects operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Object response representation
#[derive(Clone, Deserialize, Serialize)]
struct ObjectResponse {
    /// Date and time of object creation
    ///
    created_at: Option<String>,

    description: Option<String>,

    name: String,

    properties: Option<HashMap<String, Properties>>,

    required: Option<Vec<String>>,

    schema: Option<String>,

    #[serde(rename = "self")]
    _self: Option<String>,

    /// Date and time of the last object modification
    ///
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // Array
    #[serde(rename = "array")]
    Array,

    // Number
    #[serde(rename = "number")]
    Number,

    // String
    #[serde(rename = "string")]
    String,

    // Boolean
    #[serde(rename = "boolean")]
    Boolean,

    // Integer
    #[serde(rename = "integer")]
    Integer,

    // Object
    #[serde(rename = "object")]
    Object,
}

/// `Items` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Items {
    _enum: Option<Vec<String>>,
    _type: Option<Type>,
}

/// `Properties` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Properties {
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
