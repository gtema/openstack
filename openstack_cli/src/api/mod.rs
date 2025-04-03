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

//! Direct API command implementation

use clap::{Parser, ValueEnum};
use http::Uri;
use serde_json::Value;
use std::io::{self, Write};
use tracing::info;
use url::Url;

use openstack_sdk::{
    AsyncOpenStack,
    api::{AsyncClient, RestClient},
    types::ServiceType,
};

use crate::Cli;
use crate::OpenStackCliError;
use crate::common::parse_key_val;
use crate::output::OutputProcessor;

fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}

/// Supported http methods
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
pub enum Method {
    /// HEAD
    Head,
    /// GET
    Get,
    /// PATCH
    Patch,
    /// PUT
    Put,
    /// POST
    Post,
    /// DELETE
    Delete,
}

impl From<Method> for http::Method {
    fn from(item: Method) -> Self {
        match item {
            Method::Head => http::Method::HEAD,
            Method::Get => http::Method::GET,
            Method::Patch => http::Method::PATCH,
            Method::Put => http::Method::PUT,
            Method::Post => http::Method::POST,
            Method::Delete => http::Method::DELETE,
        }
    }
}

/// Perform direct REST API requests with authorization
///
/// This command enables direct REST API call with the authorization and
/// version discovery handled transparently. This may be used when required
/// operation is not implemented by the `osc` or some of the parameters
/// require special handling.
///
/// Example:
///
/// ```console
/// osc --os-cloud devstack api compute flavors/detail | jq
/// ```
#[derive(Debug, Parser)]
pub struct ApiCommand {
    /// Service type as used in the service catalog
    #[arg()]
    service_type: String,

    /// Rest URL (relative to the endpoint information
    /// from the service catalog). Do not start URL with
    /// the "/" to respect endpoint version information.
    #[arg()]
    url: String,

    /// HTTP Method
    #[arg(short, long, value_enum, default_value_t=Method::Get)]
    method: Method,

    /// Additional headers
    #[arg(long, short='H', value_name="key=value", value_parser = parse_key_val::<String, String>)]
    header: Vec<(String, String)>,

    /// Request body to be used
    #[arg(long)]
    body: Option<String>,
}

impl ApiCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Perform REST API call {:?}", self);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let service_type = ServiceType::from(self.service_type.as_str());

        client.discover_service_endpoint(&service_type).await?;

        let service_endpoint = client.get_service_endpoint(&service_type, None)?;

        let endpoint = service_endpoint.build_request_url(&self.url)?;

        let mut req = http::Request::builder()
            .method::<http::Method>(self.method.clone().into())
            .uri(url_to_http_uri(endpoint))
            .header(
                http::header::ACCEPT,
                http::HeaderValue::from_static("application/json"),
            );

        let headers = req.headers_mut().unwrap();
        for (name, val) in &self.header {
            headers.insert(
                http::HeaderName::from_lowercase(name.to_lowercase().as_bytes()).unwrap(),
                http::HeaderValue::from_str(val.as_str()).unwrap(),
            );
        }

        let rsp = client
            .rest_async(req, self.body.clone().unwrap_or_default().into_bytes())
            .await?;

        info!("Response = {:?}", rsp);
        if let Some(content_type) = rsp.headers().get("content-type") {
            if content_type == "application/json" {
                if !rsp.body().is_empty() {
                    let data: Value = serde_json::from_slice(rsp.body())?;
                    op.output_machine(data)?;
                }
            } else {
                io::stdout().write_all(rsp.body())?;
            }
        }

        Ok(())
    }
}
