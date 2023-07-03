//! Downloads the object content and gets the object metadata.
//! This operation returns the object metadata in the response headers and the
//! object content in the response body.
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use crate::common::download_file;
use openstack_sdk::api::object_store::v1::object::get;
use openstack_sdk::api::RawQueryAsync;

/// Downloads the object content and gets the object metadata.
/// This operation returns the object metadata in the response headers and the
/// object content in the response body.
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

    /// Used with temporary URLs to sign the request with an HMAC-SHA1
    /// cryptographic signature that defines the allowed HTTP method,
    /// expiration date, full path to the object, and the secret key for the
    /// temporary URL. For more information about temporary URLs, see Temporary
    /// URL middleware.
    #[arg(long)]
    temp_url_sig: Option<String>,

    /// The date and time in UNIX Epoch time stamp format or ISO 8601 UTC
    /// timestamp when the signature for temporary URLs expires. For example,
    /// 1440619048 or 2015-08-26T19:57:28Z is equivalent to Mon, Wed, 26 Aug
    /// 2015 19:57:28 GMT. For more information about temporary URLs, see
    /// Temporary URL middleware.
    #[arg(long)]
    temp_url_expires: Option<u32>,

    /// Overrides the default file name. Object Storage generates a default
    /// file name for GET temporary URLs that is based on the object name.
    /// Object Storage returns this value in the Content-Disposition response
    /// header. Browsers can interpret this file name value as a file
    /// attachment to save. For more information about temporary URLs, see
    /// Temporary URL middleware.
    #[arg(long)]
    filename: Option<String>,

    /// If you include the symlink=get query parameter and the object is a
    /// symlink, then the response will include data and metadata from the
    /// symlink itself rather than from the target.
    #[arg(long)]
    symlink: Option<String>,

    /// Destination filename (using "-" will print object to stdout)
    #[arg(long)]
    file: Option<String>,
}

pub struct ObjectCmd {
    pub args: ObjectArgs,
}

/// Object
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Object {}

#[async_trait]
impl Command for ObjectCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Object with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Object::builder();
        // Set path parameters
        ep_builder.container(&self.args.container);
        ep_builder.object(&self.args.object);
        // Set query parameters
        if let Some(val) = &self.args.multipart_manifest {
            ep_builder.multipart_manifest(val);
        }
        if let Some(val) = &self.args.temp_url_sig {
            ep_builder.temp_url_sig(val);
        }
        if let Some(val) = &self.args.temp_url_expires {
            ep_builder.temp_url_expires(*val);
        }
        if let Some(val) = &self.args.filename {
            ep_builder.filename(val);
        }
        if let Some(val) = &self.args.symlink {
            ep_builder.symlink(val);
        }
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let (headers, data) = ep.download_async(client).await?;

        let size: u64 = headers
            .get("content-length")
            .map(|x| x.to_str().expect("Header is a string"))
            .unwrap_or("0")
            .parse()
            .unwrap();
        download_file(
            self.args.file.clone().unwrap_or(self.args.object.clone()),
            size,
            data,
        )
        .await?;
        Ok(())
    }
}
