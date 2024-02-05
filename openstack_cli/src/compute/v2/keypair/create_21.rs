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

//! Create Keypair command [microversion = 2.1]
//!
//! Wraps invoking of the `v2.1/os-keypairs` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::keypair::create_21;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Imports (or generates) a keypair.
///
/// Normal response codes: 200, 201
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// conflict(409)
#[derive(Args)]
#[command(about = "Import (or create) Keypair (microversion = 2.1)")]
pub struct KeypairCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    keypair: Keypair,
}

/// Query parameters
#[derive(Args)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args)]
pub struct PathParameters {}
/// Keypair Body data
#[derive(Args)]
struct Keypair {
    /// A name for the keypair which will be used to reference it later.
    ///
    ///
    ///
    /// Note
    ///
    ///
    /// Since microversion 2.92, allowed characters are ASCII letters
    /// `[a-zA-Z]`, digits `[0-9]` and the following special
    /// characters: `[@.\_- ]`.
    #[arg(long)]
    name: String,

    /// The public ssh key to import.
    /// Was optional before microversion 2.92 : if you were omitting this
    /// value, a
    /// keypair was generated for you.
    #[arg(long)]
    public_key: Option<String>,
}

/// Keypair response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ResponseData {
    /// The user\_id for a keypair.
    #[serde()]
    #[structable(optional)]
    user_id: Option<String>,

    /// A boolean indicates whether this keypair is deleted or not. The value
    /// is always false (not deleted).
    #[serde()]
    #[structable(optional)]
    deleted: Option<bool>,

    /// The date and time when the resource was created.
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// It is always null.
    #[serde()]
    #[structable(optional)]
    deleted_at: Option<String>,

    /// It is always null.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The keypair ID.
    #[serde()]
    #[structable(optional)]
    id: Option<i32>,

    /// The name for the keypair.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The keypair public key.
    #[serde()]
    #[structable(optional)]
    public_key: Option<String>,

    /// The fingerprint for the keypair.
    #[serde()]
    #[structable(optional)]
    fingerprint: Option<String>,

    /// The type of the keypair. Allowed values are `ssh` or `x509`.
    ///
    ///
    /// **New in version 2.2**
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,

    /// If you do not provide a public key on create, a new keypair will
    /// be built for you, and the private key will be returned during the
    /// initial create call. Make sure to save this, as there is no way to
    /// get this private key again in the future.
    ///
    ///
    /// **Available until version 2.91**
    #[serde()]
    #[structable(optional)]
    private_key: Option<String>,
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

        let mut ep_builder = create_21::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.1");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.keypair data
        let args = &self.keypair;
        let mut keypair_builder = create_21::KeypairBuilder::default();

        keypair_builder.name(args.name.clone());

        if let Some(val) = &args.public_key {
            keypair_builder.public_key(val.clone());
        }

        ep_builder.keypair(keypair_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
