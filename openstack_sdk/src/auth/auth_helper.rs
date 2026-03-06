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

//! Authentication helper.
//!
//! Authentication data may not be present in the configuration file, not provided during the
//! calling operation or simply not known in advance. When authentication requires a TOTP or other
//! temporary token - may be during the re-authentication or session renewal - the is no other way
//! rather than the client to provide a callback that the authentication routine may invoke to
//! request for such additional authentication data. This module defines such interface.

pub use openstack_sdk_core::auth::auth_helper::*;
