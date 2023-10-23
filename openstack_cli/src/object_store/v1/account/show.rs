//! Shows metadata for an account.
//! Because the storage system can store large amounts of data, take care when
//! you represent the total bytes response as an integer; when possible,
//! convert it to a 64-bit unsigned integer if your platform supports that
//! primitive type.
//! Do not include metadata headers in this request.
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

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::HashMapStringString;
use openstack_sdk::api::object_store::v1::account::head;
use openstack_sdk::api::RawQueryAsync;
use regex::Regex;

/// Shows metadata for an account.
/// Because the storage system can store large amounts of data, take care when
/// you represent the total bytes response as an integer; when possible,
/// convert it to a 64-bit unsigned integer if your platform supports that
/// primitive type.
/// Do not include metadata headers in this request.
#[derive(Args, Clone, Debug)]
pub struct AccountArgs {}

pub struct AccountCmd {
    pub args: AccountArgs,
}

/// Account
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Account {
    #[structable(title = "metadata")]
    metadata: HashMapStringString,
}

#[async_trait]
impl Command for AccountCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Head Account with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = head::Account::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::ObjectStore)
            .await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let mut metadata: HashMap<String, String> = HashMap::new();
        let headers = rsp.headers();

        let mut regexes: Vec<Regex> = vec![
            Regex::new(r"(?i)X-Account-Meta-\.*").unwrap(),
            Regex::new(r"(?i)X-Account-Storage-Policy\.*Bytes-Used").unwrap(),
            Regex::new(r"(?i)X-Account-Storage-Policy\.*Container-Count").unwrap(),
            Regex::new(r"(?i)X-Account-Storage-Policy\.*Object-Count").unwrap(),
        ];

        for (hdr, val) in headers.iter() {
            if [
                "x-account-meta-temp-url-key",
                "x-account-meta-temp-url-key-2",
                "x-timestamp",
                "x-account-bytes-used",
                "x-account-container-count",
                "x-account-object-count",
                "x-account-meta-quota-bytes",
                "x-account-access-control",
            ]
            .contains(&hdr.as_str())
            {
                metadata.insert(
                    hdr.to_string(),
                    val.to_str().unwrap_or_default().to_string(),
                );
            } else {
                if !regexes.is_empty() {
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
        }
        let data = Account {
            metadata: metadata.into(),
        };
        // Maybe output some headers metadata
        op.output_human::<Account>(&data)?;
        Ok(())
    }
}
