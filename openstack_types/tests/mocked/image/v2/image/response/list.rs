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

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::image::v2::image::list::Request;
use openstack_types::image::v2::image::response::list::ImageResponse;

use crate::get_client;

#[tokio::test]
async fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client("image");

    // NOTE: we do not test paginated response since schema `next` parameters is string so the mock
    // contain insane value which fails in sdk pagination.
    let _res: Vec<ImageResponse> = Request::builder().build()?.query_async(&client).await?;

    Ok(())
}
