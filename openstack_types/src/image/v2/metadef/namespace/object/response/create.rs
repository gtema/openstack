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
//! Response type for the POST `metadefs/namespaces/{namespace_name}/objects` operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use structable::{StructTable, StructTableOptions};

/// Object response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ObjectResponse {
    /// Date and time of object creation
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub description: Option<String>,

    #[structable()]
    pub name: String,

    #[serde(default)]
    #[structable(optional, serialize)]
    pub properties: Option<BTreeMap<String, Properties>>,

    #[serde(default)]
    #[structable(optional, serialize)]
    pub required: Option<Vec<String>>,

    #[serde(default)]
    #[structable(optional)]
    pub schema: Option<String>,

    #[serde(default, rename = "self")]
    #[structable(optional, title = "self")]
    pub _self: Option<String>,

    /// Date and time of the last object modification
    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
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

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "array" => Ok(Self::Array),
            "boolean" => Ok(Self::Boolean),
            "integer" => Ok(Self::Integer),
            "number" => Ok(Self::Number),
            "object" => Ok(Self::Object),
            "string" => Ok(Self::String),
            _ => Err(()),
        }
    }
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
