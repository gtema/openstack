//! Uploads binary image data. (Since Image API v2.0)
//! Set the Content-Type request header to application/octet-stream.
//! A multiple store backend support is introduced in the Rocky release as a
//! part of the EXPERIMENTAL Image API v2.8.
//! Beginning with API version 2.8, an optional X-Image-Meta-Store header may
//! be added to the request. When present, the image data will be placed into
//! the backing store whose identifier is the value of this header. If the
//! store identifier specified is not recognized, a 400 (Bad Request) response
//! is returned. When the header is not present, the image data is placed into
//! the default backing store.
//! Store identifiers are site-specific. Use the Store Discovery call to
//! determine what stores are available in a particular cloud.
//! The default store may be determined from the Store Discovery response.
//! A default store is always defined, so if you do not have a need to use a
//! particular store, simply omit this header and the default store will be
//! used.
//! For API versions before version 2.8, this header is silently ignored.
//! Preconditions
//! Before you can store binary image data, you must meet the following
//! preconditions:
//!
//!   - The image must exist.
//!
//!   - You must set the disk and container formats in the image.
//!
//!   - The image status must be queued.
//!
//!   - Your image storage quota must be sufficient.
//!
//!   - The size of the data that you want to store must not exceed the size
//! that
//!     the OpenStack Image service allows.
//!
//! Synchronous Postconditions:
//!
//!   - With correct permissions, you can see the image status as active
//! through
//!     API calls.
//!
//!   - With correct access, you can see the stored data in the storage system
//!     that the OpenStack Image Service manages.
//!
//! Troubleshooting
//!
//!   - If you cannot store the data, either your request lacks required
//!     information or you exceeded your allotted quota. Ensure that you meet
//! the
//!     preconditions and run the request again. If the request fails again,
//!     review your API request.
//!
//!   - The storage back ends for storing the data must have enough free
//! storage
//!     space to accommodate the size of the data.
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

use crate::common::build_upload_asyncread;
use openstack_sdk::api::find;
use openstack_sdk::api::image::v2::image::file::put;
use openstack_sdk::api::image::v2::image::find;
use openstack_sdk::api::RawQueryAsync;

/// Uploads binary image data. (Since Image API v2.0)
/// Set the Content-Type request header to application/octet-stream.
/// A multiple store backend support is introduced in the Rocky release as a
/// part of the EXPERIMENTAL Image API v2.8.
/// Beginning with API version 2.8, an optional X-Image-Meta-Store header may
/// be added to the request. When present, the image data will be placed into
/// the backing store whose identifier is the value of this header. If the
/// store identifier specified is not recognized, a 400 (Bad Request) response
/// is returned. When the header is not present, the image data is placed into
/// the default backing store.
/// Store identifiers are site-specific. Use the Store Discovery call to
/// determine what stores are available in a particular cloud.
/// The default store may be determined from the Store Discovery response.
/// A default store is always defined, so if you do not have a need to use a
/// particular store, simply omit this header and the default store will be
/// used.
/// For API versions before version 2.8, this header is silently ignored.
/// Preconditions
/// Before you can store binary image data, you must meet the following
/// preconditions:
///
///   - The image must exist.
///
///   - You must set the disk and container formats in the image.
///
///   - The image status must be queued.
///
///   - Your image storage quota must be sufficient.
///
///   - The size of the data that you want to store must not exceed the size
/// that
///     the OpenStack Image service allows.
///
/// Synchronous Postconditions:
///
///   - With correct permissions, you can see the image status as active
/// through
///     API calls.
///
///   - With correct access, you can see the stored data in the storage system
///     that the OpenStack Image Service manages.
///
/// Troubleshooting
///
///   - If you cannot store the data, either your request lacks required
///     information or you exceeded your allotted quota. Ensure that you meet
/// the
///     preconditions and run the request again. If the request fails again,
///     review your API request.
///
///   - The storage back ends for storing the data must have enough free
/// storage
///     space to accommodate the size of the data.
#[derive(Args, Clone, Debug)]
pub struct ImageArgs {
    /// Image ID
    #[arg()]
    id: String,

    /// Source filename (using "-" will read object from stdout)
    #[arg(long)]
    file: Option<String>,
}

pub struct ImageCmd {
    pub args: ImageArgs,
}

/// Image
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Image {}

#[async_trait]
impl Command for ImageCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Put Image with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = put::Image::builder();
        // Set path parameters
        ep_builder.id(&self.args.id);
        // Set query parameters
        // Set body parameters
        // The only supported media type
        ep_builder.header("content-type", "application/octet-stream");
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client.discover_service_endpoint("image").await?;
        let dst = self.args.file.clone();
        let data = build_upload_asyncread(dst).await?;

        let _rsp: Response<Bytes> = ep.raw_query_read_body_async(client, data).await?;
        // TODO: what if there is an interesting response
        Ok(())
    }
}
