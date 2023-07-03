//! Perform direct REST API
//!
//! This command enables direct REST API call with
//! the authorization and version discovery
//! handeled transparently. This may be used when
//! required operation is not implemented by the
//! `osc` or some of the parameters require
//! special handling.
//!
//! Example:
//! ```console
//! osc --os-cloud devstack api compute flavors/detail | jq
//! ```

use async_trait::async_trait;
use clap::{Args, ValueEnum};
use http::{Response, Uri};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, info};

use anyhow::Result;
use url::Url;

use openstack_sdk::{
    api::{AsyncClient, RestClient},
    AsyncOpenStack,
};

use crate::common::parse_key_val;
use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}

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

#[derive(Args, Clone, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ApiArgs {
    /// Service name
    #[arg()]
    service: String,

    /// Rest URL (relative to the endpoint
    /// information from the service catalog). Do not start URL with the "/" to respect endpoint version information.
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

#[derive(Debug)]
pub struct ApiCommand {
    pub args: ApiArgs,
}

#[async_trait]
impl Command for ApiCommand {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Perform REST API call {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        client.discover_service_endpoint(&self.args.service).await?;

        let endpoint = client.rest_endpoint(&self.args.service, &self.args.url)?;

        let req = http::Request::builder()
            .method::<http::Method>(self.args.method.clone().into())
            .uri(url_to_http_uri(endpoint))
            .header(
                http::header::ACCEPT,
                http::HeaderValue::from_static("application/json"),
            );

        info!("Request = {:?}", req);

        let rsp = client
            .rest_async(
                req,
                self.args
                    .body
                    .clone()
                    .unwrap_or("".to_string())
                    .into_bytes(),
            )
            .await?;

        info!("Response = {:?}", rsp);
        let data: Value = serde_json::from_slice(rsp.body())?;

        op.output_machine(data)?;

        Ok(())
    }
}
