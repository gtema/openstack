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

//! Creates, updates, or deletes custom metadata for a container.
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use structable_derive::StructTable;

use openstack_sdk::{
    api::RestClient,
    types::{ApiVersion, ServiceType},
    AsyncOpenStack,
};

use crate::common::parse_key_val;
use openstack_sdk::api::object_store::v1::container::set::Request;
use openstack_sdk::api::RawQueryAsync;

/// Creates, updates, or deletes custom metadata for a container.
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

    /// Property to be set
    #[arg(long, value_name="key=value", value_parser = parse_key_val::<String, String>)]
    property: Vec<(String, String)>,
}

/// Container
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Container {}

impl ContainerCommand {
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Container with {:?}", self);

        let op = OutputProcessor::from_args(parsed_args);
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
            .last();
        if let Some(account) = account {
            ep_builder.account(account);
        }
        ep_builder.container(&self.container);
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
        let data = Container {};
        // Maybe output some headers metadata
        op.output_human::<Container>(&data)?;
        Ok(())
    }
}
