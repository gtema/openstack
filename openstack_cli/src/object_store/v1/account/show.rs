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

use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::HashMapStringString;
use openstack_sdk::api::object_store::v1::account::head;
use openstack_sdk::api::RawQueryAsync;
use regex::Regex;
use std::collections::HashMap;

/// Shows metadata for an account.
/// Because the storage system can store large amounts of data, take care when
/// you represent the total bytes response as an integer; when possible,
/// convert it to a 64-bit unsigned integer if your platform supports that
/// primitive type.
/// Do not include metadata headers in this request.
#[derive(Args, Clone, Debug)]
pub struct AccountCommand {}

/// Account
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Account {
    #[structable(title = "metadata")]
    metadata: HashMapStringString,
}

impl AccountCommand {
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Head Account with {:?}", self);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let ep_builder = head::Account::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::ObjectStore)
            .await?;
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
        let data = Account {
            metadata: metadata.into(),
        };
        // Maybe output some headers metadata
        op.output_human::<Account>(&data)?;
        Ok(())
    }
}
