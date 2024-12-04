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

//! Identity Endpoint Filter commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod endpoint;
mod endpoint_group;
mod project;

/// OS-EP-FILTER API
///
/// This API enables the creation of custom catalogs using project scope. The result is the ability
/// to advertise specific endpoints based on the project in use. The association can be done two
/// different ways. The first is by building a direct association between the project and the
/// endpoint, which implies that all tokens scoped to that particular project will receive a
/// specific endpoint, or set of endpoints, in the service catalog. The second is by creating an
/// endpoint group. An endpoint group is a filter that consists of at least one endpoint attribute.
/// By associating a project to an endpoint group, all service catalogs scoped to that project will
/// contain endpoints that match the attributes defined in the endpoint group. Using endpoint
/// groups is a way to dynamically associate an endpoint, or a group of endpoints, to a specific
/// project.
#[derive(Parser)]
pub struct EndpointFilterCommand {
    #[command(subcommand)]
    command: EndpointFilterCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum EndpointFilterCommands {
    Endpoint(Box<endpoint::EndpointCommand>),
    EndpointGroup(Box<endpoint_group::EndpointGroupCommand>),
    Project(Box<project::ProjectCommand>),
}

impl EndpointFilterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            EndpointFilterCommands::Endpoint(cmd) => cmd.take_action(parsed_args, session).await,
            EndpointFilterCommands::EndpointGroup(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            EndpointFilterCommands::Project(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
