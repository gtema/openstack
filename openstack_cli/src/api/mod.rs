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

use tracing::info;

use anyhow::Result;
use url::Url;

use openstack_sdk::{
    api::{AsyncClient, RestClient},
    types::ServiceType,
    AsyncOpenStack,
};

use crate::common::parse_key_val;
use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;

fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}

/// Supported http methods
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
enum Method {
    /// HEAD
    Head,
    /// GET
    Get,
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
    /// Service name
    #[arg()]
    service: String,

    /// Rest URL (relative to the endpoint information
    /// from the service catalog). Do not start URL with
    /// the "/" to respect endpoint version information.
    #[arg()]
    url: String,

    /// HTTP Method
    #[arg(short, long, value_enum, default_value_t=Method::Get)]
    method: Method,

    /// Additional headers
    #[arg(long, value_name="key=value", value_parser = parse_key_val::<String, String>)]
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

        let service = ServiceType::from(self.service.as_str());

        client.discover_service_endpoint(&service).await?;

        let endpoint = client.rest_endpoint(&service, &self.url)?;

        let req = http::Request::builder()
            .method::<http::Method>(self.method.clone().into())
            .uri(url_to_http_uri(endpoint))
            .header(
                http::header::ACCEPT,
                http::HeaderValue::from_static("application/json"),
            );

        info!("Request = {:?}", req);

        let rsp = client
            .rest_async(
                req,
                self.body.clone().unwrap_or("".to_string()).into_bytes(),
            )
            .await?;

        info!("Response = {:?}", rsp);
        let data: Value = serde_json::from_slice(rsp.body())?;

        op.output_machine(data)?;

        Ok(())
    }
}
