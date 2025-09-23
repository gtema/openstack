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

//! Creates, updates, or deletes account metadata.
//! To create, update, or delete custom metadata, use the X-Account-Meta-{name}
//! request header, where {name} is the name of the metadata item.
//! Account metadata operations work differently than how object metadata
//! operations work. Depending on the contents of your POST account metadata
//! request, the Object Storage API updates the metadata as shown in the
//! following table:
//! TODO: fill the rest
//! To delete a metadata header, send an empty value for that header, such as
//! for the X-Account-Meta-Book header. If the tool you use to communicate with
//! Object Storage, such as an older version of cURL, does not support empty
//! headers, send the X-Remove-Account- Meta-{name} header with an arbitrary
//! value. For example, X-Remove-Account-Meta-Book: x. The operation ignores
//! the arbitrary value.
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;
use structable::{StructTable, StructTableOptions};

use openstack_sdk::{
    AsyncOpenStack,
    api::RestClient,
    types::{ApiVersion, ServiceType},
};

use crate::common::HashMapStringString;
use crate::common::parse_key_val;
use openstack_sdk::api::RawQueryAsync;
use openstack_sdk::api::object_store::v1::account::head::Request as GetRequest;
use openstack_sdk::api::object_store::v1::account::set::Request;

/// Creates, updates, or deletes account metadata.
/// To create, update, or delete custom metadata, use the X-Account-Meta-{name}
/// request header, where {name} is the name of the metadata item.
/// Account metadata operations work differently than how object metadata
/// operations work. Depending on the contents of your POST account metadata
/// request, the Object Storage API updates the metadata as shown in the
/// following table:
/// TODO: fill the rest
/// To delete a metadata header, send an empty value for that header, such as
/// for the X-Account-Meta-Book header. If the tool you use to communicate with
/// Object Storage, such as an older version of cURL, does not support empty
/// headers, send the X-Remove-Account- Meta-{name} header with an arbitrary
/// value. For example, X-Remove-Account-Meta-Book: x. The operation ignores
/// the arbitrary value.
#[derive(Args, Clone, Debug)]
pub struct AccountCommand {
    /// Property to be set
    #[arg(long, value_name="key=value", value_parser = parse_key_val::<String, String>)]
    property: Vec<(String, String)>,
}

/// Account
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Account {}

impl AccountCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Account with {:?}", self);

        let op = OutputProcessor::from_args(parsed_args, Some("object-store.account"), Some("set"));
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
        ep_builder.headers(self.property.iter().map(|(k, v)| {
            (
                Some(HeaderName::from_bytes(k.as_bytes()).expect("HeaderName is a string")),
                HeaderValue::from_str(v.as_str()).expect("Header Value is a string"),
            )
        }));
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;

        // Refetch the container with the actual data
        let mut ep_builder = GetRequest::builder();
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
