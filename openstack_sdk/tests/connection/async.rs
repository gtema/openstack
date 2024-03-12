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

use std::env;

use openstack_sdk::types::ServiceType;
use openstack_sdk::{config::ConfigFile, AsyncOpenStack};

#[tokio::test]
async fn async_connection() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = ConfigFile::new().unwrap();
    // Get connection config from clouds.yaml/secure.yaml
    let profile = cfg
        .get_cloud_config(env::var("OS_CLOUD").expect("OS_CLOUD variable set"))
        .unwrap()
        .unwrap();
    // Establish connection
    let mut session = AsyncOpenStack::new(&profile).await?;

    // Invoke service discovery when desired.
    session
        .discover_service_endpoint(&ServiceType::Compute)
        .await?;

    assert!(session.get_auth_token().is_some());

    Ok(())
}
