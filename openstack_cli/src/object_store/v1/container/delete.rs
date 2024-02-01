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

//! Deletes an empty container.
//! This operation fails unless the container is empty. An empty container has
//! no objects.
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;

use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::object_store::v1::container::delete;
use openstack_sdk::api::RawQueryAsync;

/// Deletes an empty container.
/// This operation fails unless the container is empty. An empty container has
/// no objects.
#[derive(Args, Clone, Debug)]
pub struct ContainerArgs {
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

pub struct ContainerCmd {
    pub args: ContainerArgs,
}

/// Container
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Container {}

#[async_trait]
impl OSCCommand for ContainerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Container with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = delete::Container::builder();
        // Set path parameters
        ep_builder.container(&self.args.container);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::ObjectStore)
            .await?;
        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
