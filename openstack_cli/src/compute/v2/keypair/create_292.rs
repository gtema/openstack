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

//! Create Keypair command [microversion = 2.92]
//!
//! Wraps invoking of the `v2.1/os-keypairs` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::keypair::create_292;
use openstack_types::compute::v2::keypair::response::create::KeypairResponse;

/// Imports (or generates) a keypair.
///
/// Normal response codes: 200, 201
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// conflict(409)
#[derive(Args)]
#[command(about = "Import (or create) Keypair (microversion = 2.92)")]
pub struct KeypairCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Keypair object
    #[command(flatten)]
    keypair: Keypair,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Type {
    Ssh,
    X509,
}

/// Keypair Body data
#[derive(Args, Clone)]
struct Keypair {
    /// A name for the keypair which will be used to reference it later.
    ///
    /// Note
    ///
    /// Since microversion 2.92, allowed characters are ASCII letters
    /// `[a-zA-Z]`, digits `[0-9]` and the following special characters:
    /// `[@._- ]`.
    #[arg(help_heading = "Body parameters", long)]
    name: String,

    /// The public ssh key to import. Was optional before microversion 2.92 :
    /// if you were omitting this value, a keypair was generated for you.
    #[arg(help_heading = "Body parameters", long)]
    public_key: String,

    /// The type of the keypair. Allowed values are `ssh` or `x509`.
    ///
    /// **New in version 2.2**
    #[arg(help_heading = "Body parameters", long)]
    _type: Option<Type>,

    /// The user_id for a keypair. This allows administrative users to upload
    /// keys for other users than themselves.
    ///
    /// **New in version 2.10**
    #[arg(help_heading = "Body parameters", long)]
    user_id: Option<String>,
}

impl KeypairCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Keypair");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_292::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.92");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.keypair data
        let args = &self.keypair;
        let mut keypair_builder = create_292::KeypairBuilder::default();

        keypair_builder.name(&args.name);

        if let Some(val) = &args._type {
            let tmp = match val {
                Type::Ssh => create_292::Type::Ssh,
                Type::X509 => create_292::Type::X509,
            };
            keypair_builder._type(tmp);
        }

        keypair_builder.public_key(&args.public_key);

        if let Some(val) = &args.user_id {
            keypair_builder.user_id(val);
        }

        ep_builder.keypair(keypair_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<KeypairResponse>(data)?;
        Ok(())
    }
}
