//! Creates, updates, or deletes account metadata.
//! To create, update, or delete custom metadata, use the X-Account-Meta-{name}
//! request header, where {name} is the name of the metadata item.
//! Account metadata operations work differently than how object metadata
//! operations work. Depending on the contents of your POST account metadata
//! request, the Object Storage API updates the metadata as shown in the
//! following table:
//! TODO: fill the rest
//! To delete a metadata header, send an empty value for that header, such as
//! for the X-Account-Meta-Book header. If the tool you use to communicate with
//! Object Storage, such as an older version of cURL, does not support empty
//! headers, send the X-Remove-Account- Meta-{name} header with an arbitrary
//! value. For example, X-Remove-Account-Meta-Book: x. The operation ignores
//! the arbitrary value.
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

use crate::common::parse_key_val;
use openstack_sdk::api::object_store::v1::account::post;
use openstack_sdk::api::RawQueryAsync;

/// Creates, updates, or deletes account metadata.
/// To create, update, or delete custom metadata, use the X-Account-Meta-{name}
/// request header, where {name} is the name of the metadata item.
/// Account metadata operations work differently than how object metadata
/// operations work. Depending on the contents of your POST account metadata
/// request, the Object Storage API updates the metadata as shown in the
/// following table:
/// TODO: fill the rest
/// To delete a metadata header, send an empty value for that header, such as
/// for the X-Account-Meta-Book header. If the tool you use to communicate with
/// Object Storage, such as an older version of cURL, does not support empty
/// headers, send the X-Remove-Account- Meta-{name} header with an arbitrary
/// value. For example, X-Remove-Account-Meta-Book: x. The operation ignores
/// the arbitrary value.
#[derive(Args, Clone, Debug)]
pub struct AccountArgs {
    /// Property to be set
    #[arg(long, value_name="key=value", value_parser = parse_key_val::<String, String>)]
    property: Vec<(String, String)>,
}

pub struct AccountCmd {
    pub args: AccountArgs,
}

/// Account
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Account {}

#[async_trait]
impl OSCCommand for AccountCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Account with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = post::Account::builder();
        // Set path parameters
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
        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = Account {};
        // Maybe output some headers metadata
        op.output_human::<Account>(&data)?;
        Ok(())
    }
}
