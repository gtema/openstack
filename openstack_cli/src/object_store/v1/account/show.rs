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

//! Shows metadata for an account.
//! Because the storage system can store large amounts of data, take care when
//! you represent the total bytes response as an integer; when possible,
//! convert it to a 64-bit unsigned integer if your platform supports that
//! primitive type.
//! Do not include metadata headers in this request.
use bytes::Bytes;
use clap::Args;
use http::Response;
use regex::Regex;
use std::collections::HashMap;
use tracing::info;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::{
    AsyncOpenStack,
    api::RestClient,
    types::{ApiVersion, ServiceType},
};

use crate::common::HashMapStringString;
use openstack_sdk::api::RawQueryAsync;
use openstack_sdk::api::object_store::v1::account::head::Request;

/// Shows metadata for an account.
/// Because the storage system can store large amounts of data, take care when
/// you represent the total bytes response as an integer; when possible,
/// convert it to a 64-bit unsigned integer if your platform supports that
/// primitive type.
/// Do not include metadata headers in this request.
#[derive(Args, Clone, Debug)]
pub struct AccountCommand {}

impl AccountCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Head Account with {:?}", self);

        let op =
            OutputProcessor::from_args(parsed_args, Some("object-store.account"), Some("show"));
        op.validate_args(parsed_args)?;
        let mut ep_builder = Request::builder();
        // Set path parameters
        let ep = client.get_service_endpoint(
            &ServiceType::ObjectStore,
            Some(ApiVersion::new(1, 0)).as_ref(),
        )?;
        let account = ep
            .url()
            .path_segments()
            .expect("Object Store endpoint must not point to a bare domain")
            .filter(|x| !x.is_empty())
            .next_back();
        if let Some(account) = account {
            ep_builder.account(account);
        }
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let mut metadata: HashMap<String, String> = HashMap::new();
        let headers = rsp.headers();

        let regexes: Vec<Regex> = vec![
            Regex::new(r"(?i)X-Account-Meta-\.*").unwrap(),
            Regex::new(r"(?i)X-Account-Storage-Policy\.*Bytes-Used").unwrap(),
            Regex::new(r"(?i)X-Account-Storage-Policy\.*Container-Count").unwrap(),
            Regex::new(r"(?i)X-Account-Storage-Policy\.*Object-Count").unwrap(),
        ];

        for (hdr, val) in headers.iter() {
            if [
                "x-account-meta-temp-url-key",
                "x-account-meta-temp-url-key-2",
                "x-timestamp",
                "x-account-bytes-used",
                "x-account-container-count",
                "x-account-object-count",
                "x-account-meta-quota-bytes",
                "x-account-access-control",
            ]
            .contains(&hdr.as_str())
            {
                metadata.insert(
                    hdr.to_string(),
                    val.to_str().unwrap_or_default().to_string(),
                );
            } else if !regexes.is_empty() {
                for rex in regexes.iter() {
                    if rex.is_match(hdr.as_str()) {
                        metadata.insert(
                            hdr.to_string(),
                            val.to_str().unwrap_or_default().to_string(),
                        );
                    }
                }
            }
        }
        let data = HashMapStringString(metadata);

        op.output_single::<HashMapStringString>(serde_json::to_value(&data)?)?;
        op.show_command_hint()?;
        Ok(())
    }
}
