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
//! Local client configuration file operations.
//!
//! This crate is the foundation for `osc config` commands that read and
//! edit `clouds.yaml`/`secure.yaml` in place. It currently provides
//! [`yaml_edit`], a comment- and anchor-preserving YAML editor; command
//! implementations built on top of it (e.g. `clouds add`) land separately.

pub mod yaml_edit;
