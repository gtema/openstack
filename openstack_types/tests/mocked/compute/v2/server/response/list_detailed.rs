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

use openstack_sdk::api::compute::v2::server::list_detailed::Request;
use openstack_sdk::api::{Pagination, QueryAsync, paged};
use openstack_types::compute::v2::server::response;

use crate::get_client;

#[tokio::test]
async fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client("compute");

    let res: Vec<serde_json::Value> = paged(Request::builder().build()?, Pagination::Limit(10))
        .query_async(&client)
        .await?;

    // Need to iterate over all possible candidate schemas
    if let Some(val) = res.first() {
        assert!(
            serde_json::from_value::<response::list_detailed_21::ServerResponse>(val.clone())
                .is_ok()
                || serde_json::from_value::<response::list_detailed_23::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_29::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_216::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_219::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_226::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_247::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_263::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_269_a::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_269_b::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_273_a::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_273_b::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_290_a::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_290_b::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_296_a::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_296_b::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_298_a::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_298_b::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_2100_a::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
                || serde_json::from_value::<response::list_detailed_2100_b::ServerResponse>(
                    val.clone(),
                )
                .is_ok()
        );
    }

    Ok(())
}
