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

//! Downloads the object content and gets the object metadata.
//! This operation returns the object metadata in the response headers and the
//! object content in the response body.

use clap::Args;

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;
use structable::{StructTable, StructTableOptions};

use openstack_sdk::{
    AsyncOpenStack,
    api::RestClient,
    types::{ApiVersion, ServiceType},
};

use crate::common::download_file;
use openstack_sdk::api::RawQueryAsync;
use openstack_sdk::api::object_store::v1::object::get::Request;

/// Downloads the object content and gets the object metadata.
/// This operation returns the object metadata in the response headers and the
/// object content in the response body.
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
    temp_url_expires: Option<i32>,

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

/// Object
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Object {}

impl ObjectCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Object with {:?}", self);

        let op =
            OutputProcessor::from_args(parsed_args, Some("object-store.object"), Some("download"));
        op.validate_args(parsed_args)?;
        let mut ep_builder = Request::builder();
        // Set path parameters
        let ep = client.get_service_endpoint(
            &ServiceType::ObjectStore,
            Some(ApiVersion::new(1, 0)).as_ref(),
        )?;
        let account = ep
            .url()
            .path_segments()
            .expect("Object Store endpoint must not point to a bare domain")
            .filter(|x| !x.is_empty())
            .next_back();
        if let Some(account) = account {
            ep_builder.account(account);
        }
        ep_builder.container(&self.container);
        ep_builder.object(&self.object);
        // Set query parameters
        if let Some(val) = &self.multipart_manifest {
            ep_builder.multipart_manifest(val);
        }
        if let Some(val) = &self.temp_url_sig {
            ep_builder.temp_url_sig(val);
        }
        if let Some(val) = &self.temp_url_expires {
            ep_builder.temp_url_expires(*val);
        }
        if let Some(val) = &self.filename {
            ep_builder.filename(val);
        }
        if let Some(val) = &self.symlink {
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
        download_file(self.file.clone().unwrap_or(self.object.clone()), size, data).await?;
        op.show_command_hint()?;
        Ok(())
    }
}
