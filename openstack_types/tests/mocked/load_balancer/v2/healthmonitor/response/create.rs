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
use uuid::Uuid;

use openstack_sdk::api::load_balancer::v2::healthmonitor::create::{
    HealthmonitorBuilder, Request, Type,
};
use openstack_sdk::api::QueryAsync;
use openstack_types::load_balancer::v2::healthmonitor::response::create::HealthmonitorResponse;

use crate::get_client;

#[tokio::test]
async fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client("load-balancer");

    let _res: HealthmonitorResponse = Request::builder()
        .healthmonitor(
            HealthmonitorBuilder::default()
                .delay(1)
                .max_retries(1)
                .pool_id(Uuid::new_v4().to_string())
                .timeout(1)
                ._type(Type::Http)
                .build()?,
        )
        .build()?
        .query_async(&client)
        .await?;

    Ok(())
}
