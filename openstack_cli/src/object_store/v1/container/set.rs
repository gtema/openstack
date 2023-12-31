//! Creates, updates, or deletes custom metadata for a container.
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
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_key_val;
use openstack_sdk::api::object_store::v1::container::post;
use openstack_sdk::api::RawQueryAsync;

/// Creates, updates, or deletes custom metadata for a container.
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

    /// Property to be set
    #[arg(long, value_name="key=value", value_parser = parse_key_val::<String, String>)]
    property: Vec<(String, String)>,
}

pub struct ContainerCmd {
    pub args: ContainerArgs,
}

/// Container
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Container {}

#[async_trait]
impl Command for ContainerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Container with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = post::Container::builder();
        // Set path parameters
        ep_builder.container(&self.args.container);
        // Set query parameters
        // Set body parameters
        ep_builder.headers(self.args.property.iter().map(|(k, v)| {
            (
                Some(HeaderName::from_bytes(k.as_bytes()).expect("HeaderName is a string")),
                HeaderValue::from_str(v.as_str()).expect("Header Value is a string"),
            )
        }));
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::ObjectStore)
            .await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = Container {};
        // Maybe output some headers metadata
        op.output_human::<Container>(&data)?;
        Ok(())
    }
}
