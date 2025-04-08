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
//! Response type for the get metadefs/namespaces/{namespace_name}/objects operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Object response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ObjectResponse {
    /// Date and time of object creation
    ///
    pub created_at: Option<String>,

    pub description: Option<String>,

    pub name: String,

    pub properties: Option<HashMap<String, Properties>>,

    pub required: Option<Vec<String>>,

    pub schema: Option<String>,

    #[serde(rename = "self")]
    pub _self: Option<String>,

    /// Date and time of the last object modification
    ///
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // String
    #[serde(rename = "string")]
    String,

    // Array
    #[serde(rename = "array")]
    Array,

    // Integer
    #[serde(rename = "integer")]
    Integer,

    // Boolean
    #[serde(rename = "boolean")]
    Boolean,

    // Number
    #[serde(rename = "number")]
    Number,

    // Object
    #[serde(rename = "object")]
    Object,
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
