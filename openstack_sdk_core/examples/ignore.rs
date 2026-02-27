//! Example for using ignore combinator to send API request not expecting any response. This is
//! typically used for DELETE operations
//!
//! `OS_CLOUD=devstack FLAVOR_ID=... cargo run -p openstack_sdk --example ignore`
//!
use std::borrow::Cow;
use std::env;

use derive_builder::Builder;

use openstack_sdk_core::OpenStackError;
use openstack_sdk_core::api::QueryAsync;
use openstack_sdk_core::api::ignore;
use openstack_sdk_core::api::rest_endpoint_prelude::*;
use openstack_sdk_core::{AsyncOpenStack, config::ConfigFile};

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct FlavorRequest<'a> {
    /// id parameter for /v2.1/flavors/{id} API
    #[builder(default, setter(into))]
    id: Cow<'a, str>,
}

impl RestEndpoint for FlavorRequest<'_> {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("flavors/{id}", id = self.id.as_ref(),).into()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("flavor".into())
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(2, 1))
    }
}

#[tokio::main]
async fn main() -> Result<(), OpenStackError> {
    let cfg = ConfigFile::new().unwrap();
    // Get connection config from clouds.yaml/secure.yaml
    let profile = cfg
        .get_cloud_config(env::var("OS_CLOUD").expect("OS_CLOUD variable must be set"))
        .unwrap()
        .unwrap();

    // Establish connection
    let client = AsyncOpenStack::new(&profile).await?;

    // Prepare the find combinator
    let mut ep_builder = FlavorRequestBuilder::default();

    // Set the `id` which is NAME or ID to find
    ep_builder.id(env::var("FLAVOR_ID").expect("FLAVOR_ID variable must be set"));

    // Build the endpoint
    let ep = ep_builder.build().unwrap();

    // Execute query keeping data as raw json_value
    ignore(ep).query_async(&client).await?;

    println!("Flavor was retrieved but we ignored the result");
    Ok(())
}
