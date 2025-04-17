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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Create Namespace command
//!
//! Wraps invoking of the `v2/metadefs/namespaces` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::image::v2::metadef::namespace::create;
use openstack_types::image::v2::metadef::namespace::response::create::NamespaceResponse;
use serde_json::Value;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct NamespaceCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Provides a user friendly description of the namespace.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The user friendly name for the namespace. Used by UI if available.
    #[arg(help_heading = "Body parameters", long)]
    display_name: Option<String>,

    /// The unique namespace text.
    #[arg(help_heading = "Body parameters", long)]
    namespace: String,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=crate::common::parse_json)]
    objects: Option<Vec<Value>>,

    /// Owner of the namespace.
    #[arg(help_heading = "Body parameters", long)]
    owner: Option<String>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    properties: Option<Vec<(String, Value)>>,

    /// If true, namespace will not be deletable.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    protected: Option<bool>,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=crate::common::parse_json)]
    resource_type_associations: Option<Vec<Value>>,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,

    /// Scope of namespace accessibility.
    #[arg(help_heading = "Body parameters", long)]
    visibility: Option<Visibility>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Visibility {
    Private,
    Public,
}

impl NamespaceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Namespace");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.description data
        if let Some(arg) = &self.description {
            ep_builder.description(arg);
        }

        // Set Request.display_name data
        if let Some(arg) = &self.display_name {
            ep_builder.display_name(arg);
        }

        // Set Request.namespace data
        ep_builder.namespace(&self.namespace);

        // Set Request.objects data
        if let Some(arg) = &self.objects {
            let objects_builder: Vec<create::Objects> = arg
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Objects>(v.to_owned()))
                .collect::<Vec<create::Objects>>();
            ep_builder.objects(objects_builder);
        }

        // Set Request.owner data
        if let Some(arg) = &self.owner {
            ep_builder.owner(arg);
        }

        // Set Request.properties data
        if let Some(arg) = &self.properties {
            ep_builder.properties(
                arg.iter()
                    .map(|(k, v)| {
                        serde_json::from_value(v.to_owned()).map(|v: create::Properties| (k, v))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter(),
            );
        }

        // Set Request.protected data
        if let Some(arg) = &self.protected {
            ep_builder.protected(*arg);
        }

        // Set Request.resource_type_associations data
        if let Some(arg) = &self.resource_type_associations {
            let resource_type_associations_builder: Vec<create::ResourceTypeAssociations> = arg
                .iter()
                .flat_map(|v| {
                    serde_json::from_value::<create::ResourceTypeAssociations>(v.to_owned())
                })
                .collect::<Vec<create::ResourceTypeAssociations>>();
            ep_builder.resource_type_associations(resource_type_associations_builder);
        }

        // Set Request.tags data
        if let Some(arg) = &self.tags {
            let tags_builder: Vec<create::Tags> = arg
                .iter()
                .flat_map(|v| create::TagsBuilder::default().name(v).build())
                .collect();
            ep_builder.tags(tags_builder);
        }

        // Set Request.visibility data
        if let Some(arg) = &self.visibility {
            let tmp = match arg {
                Visibility::Private => create::Visibility::Private,
                Visibility::Public => create::Visibility::Public,
            };
            ep_builder.visibility(tmp);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<NamespaceResponse>(data)?;
        Ok(())
    }
}
