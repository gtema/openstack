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

//! Mock tests for the `openstack_types` crate
use std::env;
use url::Url;

use openstack_sdk::auth::{authtoken::AuthToken, Auth};
use openstack_sdk::test::client::FakeOpenStackClient;

#[cfg(feature = "block_storage")]
mod block_storage;
#[cfg(feature = "compute")]
mod compute;
#[cfg(feature = "container_infra")]
mod container_infrastructure_management;
#[cfg(feature = "dns")]
mod dns;
#[cfg(feature = "identity")]
mod identity;
#[cfg(feature = "image")]
mod image;
#[cfg(feature = "load_balancer")]
mod load_balancer;
#[cfg(feature = "network")]
mod network;
#[cfg(feature = "object_store")]
mod object_store;
#[cfg(feature = "placement")]
mod placement;

/// Initialize fake OpenStack client
pub fn get_client<S: AsRef<str>>(service: S) -> FakeOpenStackClient {
    let mut client: FakeOpenStackClient = FakeOpenStackClient::new("http://localhost:4010");
    match service.as_ref() {
        "block-storage" => {
            client.add_endpoint(
                "block-storage",
                Url::parse(&env::var("OPENSTACK_BLOCK_STORAGE_ENDPOINT").expect(
                    "OPENSTACK_BLOCK_STORAGE_ENDPOINT environment variable must be present",
                ))
                .expect("url must be valid uri"),
            );
        }
        "network" => {
            client.add_endpoint(
                service.as_ref(),
                Url::parse(
                    &env::var("OPENSTACK_NETWORK_ENDPOINT")
                        .expect("OPENSTACK_NETWORK_ENDPOINT environment variable must be present"),
                )
                .expect("url must be valid uri"),
            );
        }
        _ => {}
    };
    client.set_auth(Some(Auth::AuthToken(Box::new(AuthToken::from("fake")))));
    client
}
