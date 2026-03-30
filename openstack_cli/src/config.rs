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
//! OpenStackClient configuration
//!
//! It is possible to configure different aspects of the OpenStackClient (not the clouds connection
//! credentials) using the configuration file (`$XDG_CONFIG_DIR/osc/config.yaml`). This enables
//! user to configurate which columns should be returned when no corresponding run time arguments
//! on a resource base.
//!
//! ```yaml
//! views:
//!   compute.server:
//!     # Listing compute servers will only return ID, NAME and IMAGE columns unless `-o wide` or
//!     `-f XXX` parameters are being passed
//!     fields: [id, name, image]
//!   dns.zone/recordset:
//!     # DNS zone recordsets are listed in the wide mode by default.
//!     wide: true
//! ```

pub use openstack_cli_core::config::*;
