//! Shows container metadata, including the number of objects and the total
//! bytes of all objects stored in the container.
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

use crate::common::HashMapStringString;
use openstack_sdk::api::object_store::v1::container::head;
use openstack_sdk::api::RawQueryAsync;
use regex::Regex;

/// Shows container metadata, including the number of objects and the total
/// bytes of all objects stored in the container.
#[derive(Args, Clone, Debug)]
pub struct ContainerArgs {
    /// The unique (within an account) name for the container. The container
    /// name must be from 1 to 256 characters long and can start with any
    /// character and contain any pattern. Character set must be UTF-8. The
    /// container name cannot contain a slash (/) character because this
    /// character delimits the container and object name. For example, the path
    /// /v1/account/www/pages specifies the www container, not the www/pages
    /// container.
    #[arg()]
    container: String,
}

pub struct ContainerCmd {
    pub args: ContainerArgs,
}

/// Container
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Container {
    #[structable(title = "metadata")]
    metadata: HashMapStringString,
}

#[async_trait]
impl Command for ContainerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Head Container with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = head::Container::builder();
        // Set path parameters
        ep_builder.container(&self.args.container);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let mut metadata: HashMap<String, String> = HashMap::new();
        let headers = rsp.headers();

        let mut regexes: Vec<Regex> = vec![Regex::new(r"(?i)X-Container-Meta-\.*").unwrap()];

        for (hdr, val) in headers.iter() {
            if [
                "x-timestamp",
                "x-container-bytes-used",
                "x-container-object-count",
                "accept-ranges",
                "x-container-meta-temp-url-key",
                "x-container-meta-temp-url-key-2",
                "x-container-meta-quota-count",
                "x-container-meta-quota-bytes",
                "x-storage-policy",
                "x-container-read",
                "x-container-write",
                "x-container-sync-key",
                "x-container-sync-to",
                "x-versions-location",
                "x-history-location",
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
        let data = Container {
            metadata: metadata.into(),
        };
        // Maybe output some headers metadata
        op.output_human::<Container>(&data)?;
        Ok(())
    }
}
