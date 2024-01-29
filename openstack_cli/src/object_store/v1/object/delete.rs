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
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
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
pub struct ObjectArgs {
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

pub struct ObjectCmd {
    pub args: ObjectArgs,
}

/// Object
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Object {}

#[async_trait]
impl OSCCommand for ObjectCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Object with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = delete::Object::builder();
        // Set path parameters
        ep_builder.container(&self.args.container);
        ep_builder.object(&self.args.object);
        // Set query parameters
        if let Some(val) = &self.args.multipart_manifest {
            ep_builder.multipart_manifest(val);
        }
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::ObjectStore)
            .await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
