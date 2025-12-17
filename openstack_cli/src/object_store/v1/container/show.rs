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

//! Shows container metadata, including the number of objects and the total
//! bytes of all objects stored in the container.
use bytes::Bytes;
use clap::Args;
use eyre::{WrapErr, eyre};
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
use openstack_sdk::api::object_store::v1::container::head::Request;

/// Shows container metadata, including the number of objects and the total
/// bytes of all objects stored in the container.
#[derive(Args, Clone, Debug)]
pub struct ContainerCommand {
    /// The unique (within an account) name for the container. The container
    /// name must be from 1 to 256 characters long and can start with any
    /// character and contain any pattern. Character set must be UTF-8. The
    /// container name cannot contain a slash (/) character because this
    /// character delimits the container and object name. For example, the path
    /// /v1/account/www/pages specifies the www container, not the www/pages
    /// container.
    #[arg()]
    container: String,
}

impl ContainerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Head Container with {:?}", self);

        let op =
            OutputProcessor::from_args(parsed_args, Some("object-store.container"), Some("show"));
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
            .ok_or_else(|| eyre!("Object Store endpoint must not point to a bare domain"))?
            .rfind(|x| !x.is_empty());
        if let Some(account) = account {
            ep_builder.account(account);
        }
        ep_builder.container(&self.container);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let mut metadata: HashMap<String, String> = HashMap::new();
        let headers = rsp.headers();

        let regexes: Vec<Regex> =
            vec![Regex::new(r"(?i)X-Container-Meta-\.*").wrap_err("failed to compile the regex")?];

        for (hdr, val) in headers.iter() {
            if [
                "x-timestamp",
                "x-container-bytes-used",
                "x-container-object-count",
                "accept-ranges",
                "x-container-meta-temp-url-key",
                "x-container-meta-temp-url-key-2",
                "x-container-meta-quota-count",
                "x-container-meta-quota-bytes",
                "x-storage-policy",
                "x-container-read",
                "x-container-write",
                "x-container-sync-key",
                "x-container-sync-to",
                "x-versions-location",
                "x-history-location",
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
