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

//! Update a user.
//!
//! PATCH /v3/users/{user_id}
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
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::user::find;
use openstack_sdk::api::identity::v3::user::patch;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::BTreeMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct UserArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long)]
    default_project_id: Option<String>,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)]
    domain_id: Option<String>,
    #[arg(action=clap::ArgAction::Set, long)]
    enabled: Option<bool>,
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    federated: Option<Vec<Value>>,
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    password: Option<String>,
    #[command(flatten)]
    options: Option<Options>,
    /// Additional properties to be sent with the request
    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    properties: Option<Vec<(String, Value)>>,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[arg()]
    id: String,
}
/// Options Body data
#[derive(Args, Debug, Clone)]
struct Options {
    #[arg(action=clap::ArgAction::Set, long)]
    ignore_change_password_upon_first_use: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    ignore_password_expiry: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    ignore_lockout_failure_attempts: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    lock_password: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    ignore_user_inactivity: Option<bool>,

    #[arg(action=clap::ArgAction::Append, long)]
    multi_factor_auth_rules: Option<Vec<String>>,

    #[arg(action=clap::ArgAction::Set, long)]
    multi_factor_auth_enabled: Option<bool>,
}

/// User set command
pub struct UserCmd {
    pub args: UserArgs,
}
/// User response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    default_project_id: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    #[serde()]
    #[structable(optional, wide)]
    federated: Option<VecResponseFederated>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    password: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    options: Option<ResponseOptions>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseProtocols {
    protocol_id: String,
    unique_id: String,
}

impl fmt::Display for ResponseProtocols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!("protocol_id={}", self.protocol_id),
            format!("unique_id={}", self.unique_id),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseProtocols(Vec<ResponseProtocols>);
impl fmt::Display for VecResponseProtocols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseFederated {
    idp_id: String,
    protocols: VecResponseProtocols,
}

impl fmt::Display for ResponseFederated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!("idp_id={}", self.idp_id),
            format!("protocols={}", self.protocols),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseFederated(Vec<ResponseFederated>);
impl fmt::Display for VecResponseFederated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseOptions {
    ignore_change_password_upon_first_use: Option<bool>,
    ignore_password_expiry: Option<bool>,
    ignore_lockout_failure_attempts: Option<bool>,
    lock_password: Option<bool>,
    ignore_user_inactivity: Option<bool>,
    multi_factor_auth_rules: Option<VecString>,
    multi_factor_auth_enabled: Option<bool>,
}

impl fmt::Display for ResponseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ignore_change_password_upon_first_use={}",
                self.ignore_change_password_upon_first_use
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_password_expiry={}",
                self.ignore_password_expiry
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_lockout_failure_attempts={}",
                self.ignore_lockout_failure_attempts
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "lock_password={}",
                self.lock_password
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_user_inactivity={}",
                self.ignore_user_inactivity
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "multi_factor_auth_rules={}",
                self.multi_factor_auth_rules
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "multi_factor_auth_enabled={}",
                self.multi_factor_auth_enabled
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

#[async_trait]
impl OSCCommand for UserCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set User with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.args.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;
        let mut ep_builder = patch::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.default_project_id data
        if let Some(args) = &self.args.default_project_id {
            ep_builder.default_project_id(Some(args.into()));
        }

        // Set Request.description data
        if let Some(args) = &self.args.description {
            ep_builder.description(Some(args.into()));
        }

        // Set Request.domain_id data
        if let Some(args) = &self.args.domain_id {
            ep_builder.domain_id(args);
        }

        // Set Request.enabled data
        if let Some(args) = &self.args.enabled {
            ep_builder.enabled(*args);
        }

        // Set Request.federated data
        if let Some(args) = &self.args.federated {
            let sub: Vec<patch::Federated> = args
                .iter()
                .flat_map(|v| serde_json::from_value::<patch::Federated>(v.clone()))
                .collect::<Vec<patch::Federated>>();
            ep_builder.federated(sub);
        }

        // Set Request.name data
        if let Some(args) = &self.args.name {
            ep_builder.name(args);
        }

        // Set Request.password data
        if let Some(args) = &self.args.password {
            ep_builder.password(Some(args.into()));
        }

        // Set Request.options data
        if let Some(args) = &self.args.options {
            let mut options_builder = patch::OptionsBuilder::default();
            if let Some(val) = &args.ignore_change_password_upon_first_use {
                options_builder.ignore_change_password_upon_first_use(*val);
            }

            if let Some(val) = &args.ignore_password_expiry {
                options_builder.ignore_password_expiry(*val);
            }

            if let Some(val) = &args.ignore_lockout_failure_attempts {
                options_builder.ignore_lockout_failure_attempts(*val);
            }

            if let Some(val) = &args.lock_password {
                options_builder.lock_password(*val);
            }

            if let Some(val) = &args.ignore_user_inactivity {
                options_builder.ignore_user_inactivity(*val);
            }

            if let Some(val) = &args.multi_factor_auth_rules {
                options_builder
                    .multi_factor_auth_rules(val.iter().map(|v| v.into()).collect::<Vec<_>>());
            }

            if let Some(val) = &args.multi_factor_auth_enabled {
                options_builder.multi_factor_auth_enabled(*val);
            }

            ep_builder.options(options_builder.build().unwrap());
        }

        if let Some(properties) = &self.args.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        // Patching resource requires fetching and calculating diff
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();

        let data: ResponseData = serde_json::from_value(find_data)?;
        let mut new = data.clone();
        if let Some(val) = &self.args.default_project_id {
            new.default_project_id = Some(val.into());
        }
        if let Some(val) = &self.args.description {
            new.description = Some(val.into());
        }
        if let Some(val) = &self.args.domain_id {
            new.domain_id = Some(val.into());
        }
        if let Some(val) = &self.args.enabled {
            new.enabled = Some(*val);
        }
        if let Some(val) = &self.args.federated {
            new.federated = Some(serde_json::from_value(serde_json::Value::from(
                val.clone(),
            ))?);
        }
        if let Some(val) = &self.args.name {
            new.name = Some(val.into());
        }
        if let Some(val) = &self.args.password {
            new.password = Some(val.into());
        }
        if let Some(val) = &self.args.options {
            new.options = Some(val.into());
        }

        let curr_json = serde_json::to_value(&data).unwrap();
        let mut new_json = serde_json::to_value(&new).unwrap();
        if let Some(properties) = &self.args.properties {
            for (key, val) in properties {
                new_json[key] = json!(val);
            }
        }

        let patch = diff(&curr_json, &new_json);

        let mut patch_ep_builder = patch::Request::builder();
        patch_ep_builder.id(&resource_id);
        patch_ep_builder.patch(patch);

        let patch_ep = patch_ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let new_data = patch_ep.query_async(client).await?;
        op.output_single::<ResponseData>(new_data)?;
        Ok(())
    }
}
