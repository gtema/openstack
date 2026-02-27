//! Example for using paged combinator to list all available flavors (paginating through the
//! results)
//!
//! `OS_CLOUD=devstack FLAVOR_MIN_RAM=32768 cargo run -p openstack_sdk --example paged`

#![allow(dead_code)]

use derive_builder::Builder;
use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::env;

use openstack_sdk_core::api::paged;
use openstack_sdk_core::api::rest_endpoint_prelude::*;
use openstack_sdk_core::api::Pagination;
use openstack_sdk_core::api::QueryAsync;
use openstack_sdk_core::OpenStackError;
use openstack_sdk_core::{config::ConfigFile, AsyncOpenStack};

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(default, setter(into))]
    is_public: Option<Cow<'a, str>>,

    #[builder(default)]
    limit: Option<u32>,

    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    min_disk: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    min_ram: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    sort_dir: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    sort_key: Option<Cow<'a, str>>,
}

impl RestEndpoint for Request<'_> {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "flavors".to_string().into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("is_public", self.is_public.as_ref());
        params.push_opt("limit", self.limit);
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("minDisk", self.min_disk.as_ref());
        params.push_opt("minRam", self.min_ram.as_ref());
        params.push_opt("sort_dir", self.sort_dir.as_ref());
        params.push_opt("sort_key", self.sort_key.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("flavors".into())
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(2, 1))
    }
}
impl Pageable for Request<'_> {}

#[derive(Deserialize, Debug, Clone)]
struct Flavor {
    disk: Option<i32>,
    extra_specs: Option<Value>,
    id: Option<String>,
    name: Option<String>,
    #[serde(rename = "os-flavor-access:is_public")]
    os_flavor_access_is_public: Option<bool>,
    ram: Option<i32>,
    vcpus: Option<i32>,
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

    // use the list_detailed endpoint
    let mut ep_builder = RequestBuilder::default();
    if let Ok(min_ram) = env::var("FLAVOR_MIN_RAM") {
        // set query parameter
        ep_builder.min_ram(min_ram);
    }

    // Build the endpoint with parameters
    let ep = ep_builder.build().unwrap();

    // Execute the query enabling pagination
    let data: Vec<Flavor> = paged(ep, Pagination::Limit(100))
        .query_async(&client)
        .await?;

    println!("Flavors = {data:?}");
    Ok(())
}
