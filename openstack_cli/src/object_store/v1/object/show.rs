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

//! Shows object metadata.
use bytes::Bytes;
use clap::Args;
use http::Response;
use regex::Regex;
use std::collections::HashMap;
use tracing::info;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::{
    AsyncOpenStack,
    api::RestClient,
    types::{ApiVersion, ServiceType},
};

use crate::common::HashMapStringString;
use openstack_sdk::api::RawQueryAsync;
use openstack_sdk::api::object_store::v1::object::head::Request;

/// Shows object metadata.
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
}

impl ObjectCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Head Object with {:?}", self);

        let op = OutputProcessor::from_args(parsed_args, Some("object-store.object"), Some("show"));
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
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let mut metadata: HashMap<String, String> = HashMap::new();
        let headers = rsp.headers();

        let regexes: Vec<Regex> = vec![Regex::new(r"(?i)X-Object-Meta-\.*").unwrap()];

        for (hdr, val) in headers.iter() {
            if [
                "content-length",
                "content-disposition",
                "content-type",
                "content-encoding",
                "accept-ranges",
                "last-modified",
                "x-delete-at",
                "x-object-manifest",
                "etag",
                "x-timestamp",
                "x-trans-id",
                "date",
                "x-static-large-object",
                "x-symlink-target",
                "x-symlink-target-account",
            ]
            .contains(&hdr.as_str())
            {
                metadata.insert(
                    hdr.to_string(),
                    val.to_str().unwrap_or_default().to_string(),
                );
            } else if !regexes.is_empty() {
                for rex in regexes.iter() {
                    if rex.is_match(hdr.as_str()) {
                        metadata.insert(
                            hdr.to_string(),
                            val.to_str().unwrap_or_default().to_string(),
                        );
                    }
                }
            }
        }
        let data = HashMapStringString(metadata);

        op.output_single::<HashMapStringString>(serde_json::to_value(&data)?)?;
        op.show_command_hint()?;
        Ok(())
    }
}
