//! Creates an object with data content and metadata, or replaces an existing
//! object with data content and metadata.
//! The PUT operation always creates an object. If you use this operation on an
//! existing object, you replace the existing object and metadata rather than
//! modifying the object. Consequently, this operation returns the Created
//! (201) response code.
//! If you use this operation to copy a manifest object, the new object is a
//! normal object and not a copy of the manifest. Instead it is a concatenation
//! of all the segment objects. This means that you cannot copy objects larger
//! than 5 GB.
//! Note that the provider may have limited the characters which are allowed in
//! an object name. Any name limits are exposed under the name_check key in the
//! /info discoverability response. Regardless of name_check limitations, names
//! must be URL quoted UTF-8.
//! To create custom metadata, use the X-Object-Meta-name header, where name is
//! the name of the metadata item.
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

use crate::common::build_upload_asyncread;
use openstack_sdk::api::object_store::v1::object::put;
use openstack_sdk::api::RawQueryAsync;

/// Creates an object with data content and metadata, or replaces an existing
/// object with data content and metadata.
/// The PUT operation always creates an object. If you use this operation on an
/// existing object, you replace the existing object and metadata rather than
/// modifying the object. Consequently, this operation returns the Created
/// (201) response code.
/// If you use this operation to copy a manifest object, the new object is a
/// normal object and not a copy of the manifest. Instead it is a concatenation
/// of all the segment objects. This means that you cannot copy objects larger
/// than 5 GB.
/// Note that the provider may have limited the characters which are allowed in
/// an object name. Any name limits are exposed under the name_check key in the
/// /info discoverability response. Regardless of name_check limitations, names
/// must be URL quoted UTF-8.
/// To create custom metadata, use the X-Object-Meta-name header, where name is
/// the name of the metadata item.
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

    /// Source filename (using "-" will read object from stdout)
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
impl OSCCommand for ObjectCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Put Object with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = put::Object::builder();
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
        // The only supported media type
        ep_builder.header("content-type", "*");
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::ObjectStore)
            .await?;
        let dst = self.args.file.clone();
        let data = build_upload_asyncread(dst).await?;

        let _rsp: Response<Bytes> = ep.raw_query_read_body_async(client, data).await?;
        // TODO: what if there is an interesting response
        Ok(())
    }
}
