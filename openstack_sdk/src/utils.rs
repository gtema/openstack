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

//! Utilities
//!
//use serde::{Deserialize, Deserializer};
//use serde_json::Value;

// /// Try to deserialize data and return `Default` if that fails
// pub fn deser_ok_or_default<'a, T, D>(deserializer: D) -> Result<T, D::Error>
// where
//     T: Deserialize<'a> + Default,
//     D: Deserializer<'a>,
// {
//     let v: Value = Deserialize::deserialize(deserializer)?;
//     Ok(T::deserialize(v).unwrap_or_default())
// }
