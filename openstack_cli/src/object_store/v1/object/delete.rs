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

//! Permanently deletes an object from the object store.
//! Object deletion occurs immediately at request time. Any subsequent GET,
//! HEAD, POST, or DELETE operations will return a 404 Not Found error code.
//! For static large object manifests, you can add the ?multipart-
//! manifest=delete query parameter. This operation deletes the segment objects
//! and, if all deletions succeed, this operation deletes the manifest object.
//! A DELETE request made to a symlink path will delete the symlink rather than
//! the target object.
//! An alternative to using the DELETE operation is to use the POST operation
//! with the bulk-delete query parameter.
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

use openstack_sdk::api::object_store::v1::object::delete;
use openstack_sdk::api::RawQueryAsync;

/// Permanently deletes an object from the object store.
/// Object deletion occurs immediately at request time. Any subsequent GET,
/// HEAD, POST, or DELETE operations will return a 404 Not Found error code.
/// For static large object manifests, you can add the ?multipart-
/// manifest=delete query parameter. This operation deletes the segment objects
/// and, if all deletions succeed, this operation deletes the manifest object.
/// A DELETE request made to a symlink path will delete the symlink rather than
/// the target object.
/// An alternative to using the DELETE operation is to use the POST operation
/// with the bulk-delete query parameter.
#[derive(Args, Clone, Debug)]
pub struct ObjectCommand {
    /// The unique name for the account. An account is also known as the
    /// project or tenant.
    #[arg()]
    container: String,

    /// The unique name for the object.
    #[arg()]
    object: String,

    /// If you include the multipart-manifest=get query parameter and the
    /// object is a large object, the object contents are not returned.
    /// Instead, the manifest is returned in the X-Object-Manifest response
    /// header for dynamic large objects or in the response body for static
    /// large objects.
    #[arg(long)]
    multipart_manifest: Option<String>,
}

/// Object
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Object {}

impl ObjectCommand {
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Object with {:?}", self);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = delete::Object::builder();
        // Set path parameters
        ep_builder.container(&self.container);
        ep_builder.object(&self.object);
        // Set query parameters
        if let Some(val) = &self.multipart_manifest {
            ep_builder.multipart_manifest(val);
        }
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
