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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Create Pool command
//!
//! Wraps invoking of the `v2/lbaas/pools` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::load_balancer::v2::pool::create;
use openstack_types::load_balancer::v2::pool::response::create::PoolResponse;
use serde_json::Value;

/// Creates a pool for a load balancer.
///
/// The pool defines how requests should be balanced across the backend member
/// servers.
///
/// This operation provisions a pool by using the configuration that you define
/// in the request object. After the API validates the request and starts the
/// provisioning process, the API returns a response object, which contains a
/// unique ID.
///
/// In the response, the pool [provisioning status](#prov-status) is `ACTIVE`,
/// `PENDING_CREATE`, or `ERROR`.
///
/// If the status is `PENDING_CREATE`, issue GET `/v2/lbaas/pools/{pool_id}` to
/// view the progress of the provisioning operation. When the pool status
/// changes to `ACTIVE`, the pool is successfully provisioned and is ready for
/// further configuration.
///
/// At a minimum, you must specify these pool attributes:
///
/// Some attributes receive default values if you omit them from the request:
///
/// If the API cannot fulfill the request due to insufficient data or data that
/// is not valid, the service returns the HTTP `Bad Request (400)` response
/// code with information about the failure in the response body. Validation
/// errors require that you correct the error and submit the request again.
///
/// Specifying a project_id is deprecated. The pool will inherit the project_id
/// of the parent load balancer.
///
/// You can configure all documented features of the pool at creation time by
/// specifying the additional elements or attributes in the request.
///
/// To create a pool, the parent load balancer must have an `ACTIVE`
/// provisioning status.
///
/// `SOURCE_IP_PORT` algorithm is available from version 2.13.
#[derive(Args)]
#[command(about = "Create Pool")]
pub struct PoolCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Defines mandatory and optional attributes of a POST request.
    #[command(flatten)]
    pool: Pool,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Protocol {
    Http,
    Https,
    Proxy,
    Proxyv2,
    Sctp,
    Tcp,
    Udp,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum LbAlgorithm {
    LeastConnections,
    RoundRobin,
    SourceIp,
    SourceIpPort,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Type {
    AppCookie,
    HttpCookie,
    SourceIp,
}

/// SessionPersistence Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct SessionPersistence {
    #[arg(help_heading = "Body parameters", long)]
    cookie_name: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    persistence_granularity: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    persistence_timeout: Option<i32>,

    #[arg(help_heading = "Body parameters", long, required = false)]
    _type: Type,
}

/// Pool Body data
#[derive(Args, Clone)]
struct Pool {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    admin_state_up: Option<bool>,

    /// A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2
    ///
    /// **New in version 2.24**
    ///
    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    alpn_protocols: Option<Vec<String>>,

    /// The reference of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA certificate bundle for `tls_enabled`
    /// pools.
    ///
    /// **New in version 2.8**
    #[arg(help_heading = "Body parameters", long)]
    ca_tls_container_ref: Option<String>,

    /// The reference of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA revocation list file for
    /// `tls_enabled` pools.
    #[arg(help_heading = "Body parameters", long)]
    crl_container_ref: Option<String>,

    /// A human-readable description for the resource.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Defines mandatory and optional attributes of a POST request.
    #[arg(help_heading = "Body parameters", long, value_name="JSON", value_parser=crate::common::parse_json)]
    healthmonitor: Option<Value>,

    /// The load balancing algorithm for the pool. One of `LEAST_CONNECTIONS`,
    /// `ROUND_ROBIN`, `SOURCE_IP`, or `SOURCE_IP_PORT`.
    #[arg(help_heading = "Body parameters", long)]
    lb_algorithm: LbAlgorithm,

    /// The ID of the listener for the pool. Either `listener_id` or
    /// `loadbalancer_id` must be specified. The listener has some
    /// restrictions, See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    #[arg(help_heading = "Body parameters", long)]
    listener_id: Option<String>,

    /// The ID of the load balancer for the pool. Either `listener_id` or
    /// `loadbalancer_id` must be specified.
    #[arg(help_heading = "Body parameters", long)]
    loadbalancer_id: Option<String>,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=crate::common::parse_json)]
    members: Option<Vec<Value>>,

    /// Human-readable name of the resource.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The ID of the project owning this resource. (deprecated)
    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    /// The protocol for the resource. One of `HTTP`, `HTTPS`, `PROXY`,
    /// `PROXYV2`, `SCTP`, `TCP`, or `UDP`.
    #[arg(help_heading = "Body parameters", long)]
    protocol: Protocol,

    /// A JSON object specifying the session persistence for the pool or `null`
    /// for no session persistence. See
    /// [Pool Session Persistence](#session-persistence). Default is `null`.
    #[command(flatten)]
    session_persistence: Option<SessionPersistence>,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,

    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,

    /// List of ciphers in OpenSSL format (colon-separated). See
    /// <https://www.openssl.org/docs/man1.1.1/man1/ciphers.html>
    ///
    /// **New in version 2.15**
    #[arg(help_heading = "Body parameters", long)]
    tls_ciphers: Option<String>,

    /// The reference to the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PKCS12 format certificate/key bundle for
    /// `tls_enabled` pools for TLS client authentication to the member
    /// servers.
    ///
    /// **New in version 2.8**
    #[arg(help_heading = "Body parameters", long)]
    tls_container_ref: Option<String>,

    /// When `true` connections to backend member servers will use TLS
    /// encryption. Default is `false`.
    ///
    /// **New in version 2.8**
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    tls_enabled: Option<bool>,

    /// A list of TLS protocol versions. Available versions: SSLv3, TLSv1,
    /// TLSv1.1, TLSv1.2, TLSv1.3
    ///
    /// **New in version 2.17**
    ///
    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tls_versions: Option<Vec<String>>,
}

impl PoolCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Pool");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.pool data
        let args = &self.pool;
        let mut pool_builder = create::PoolBuilder::default();
        if let Some(val) = &args.name {
            pool_builder.name(val);
        }

        if let Some(val) = &args.description {
            pool_builder.description(val);
        }

        if let Some(val) = &args.admin_state_up {
            pool_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.listener_id {
            pool_builder.listener_id(val);
        }

        if let Some(val) = &args.loadbalancer_id {
            pool_builder.loadbalancer_id(val);
        }

        let tmp = match &args.protocol {
            Protocol::Http => create::Protocol::Http,
            Protocol::Https => create::Protocol::Https,
            Protocol::Proxy => create::Protocol::Proxy,
            Protocol::Proxyv2 => create::Protocol::Proxyv2,
            Protocol::Sctp => create::Protocol::Sctp,
            Protocol::Tcp => create::Protocol::Tcp,
            Protocol::Udp => create::Protocol::Udp,
        };
        pool_builder.protocol(tmp);

        let tmp = match &args.lb_algorithm {
            LbAlgorithm::LeastConnections => create::LbAlgorithm::LeastConnections,
            LbAlgorithm::RoundRobin => create::LbAlgorithm::RoundRobin,
            LbAlgorithm::SourceIp => create::LbAlgorithm::SourceIp,
            LbAlgorithm::SourceIpPort => create::LbAlgorithm::SourceIpPort,
        };
        pool_builder.lb_algorithm(tmp);

        if let Some(val) = &args.session_persistence {
            let mut session_persistence_builder = create::SessionPersistenceBuilder::default();

            let tmp = match &val._type {
                Type::AppCookie => create::Type::AppCookie,
                Type::HttpCookie => create::Type::HttpCookie,
                Type::SourceIp => create::Type::SourceIp,
            };
            session_persistence_builder._type(tmp);
            if let Some(val) = &val.cookie_name {
                session_persistence_builder.cookie_name(val);
            }
            if let Some(val) = &val.persistence_timeout {
                session_persistence_builder.persistence_timeout(*val);
            }
            if let Some(val) = &val.persistence_granularity {
                session_persistence_builder.persistence_granularity(val);
            }
            pool_builder
                .session_persistence(session_persistence_builder.build().expect("A valid object"));
        }

        if let Some(val) = &args.project_id {
            pool_builder.project_id(val);
        }

        if let Some(val) = &args.healthmonitor {
            pool_builder.healthmonitor(serde_json::from_value::<create::Healthmonitor>(
                val.to_owned(),
            )?);
        }

        if let Some(val) = &args.members {
            let members_builder: Vec<create::Members> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Members>(v.to_owned()))
                .collect::<Vec<create::Members>>();
            pool_builder.members(members_builder);
        }

        if let Some(val) = &args.tags {
            pool_builder.tags(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.tls_container_ref {
            pool_builder.tls_container_ref(val);
        }

        if let Some(val) = &args.ca_tls_container_ref {
            pool_builder.ca_tls_container_ref(val);
        }

        if let Some(val) = &args.crl_container_ref {
            pool_builder.crl_container_ref(val);
        }

        if let Some(val) = &args.tls_enabled {
            pool_builder.tls_enabled(*val);
        }

        if let Some(val) = &args.tls_ciphers {
            pool_builder.tls_ciphers(val);
        }

        if let Some(val) = &args.tls_versions {
            pool_builder.tls_versions(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.alpn_protocols {
            pool_builder.alpn_protocols(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.tenant_id {
            pool_builder.tenant_id(val);
        }

        ep_builder.pool(pool_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<PoolResponse>(data)?;
        Ok(())
    }
}
