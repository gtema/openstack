use derive_builder::Builder;
use serde::de::DeserializeOwned;
use tracing::trace;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query, RestClient};

use crate::api::network::v2::router::{get as Get, list as List};

/// Find for router by nameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
}

impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl<'a> Findable for Request<'a> {
    type G = Get::Request<'a>;
    type L = List::Request<'a>;
    fn get_ep(&self) -> Get::Request<'a> {
        Get::Request::builder().id(self.id.clone()).build().unwrap()
    }
    fn list_ep(&self) -> List::Request<'a> {
        List::Request::builder()
            .name(self.id.clone())
            .build()
            .unwrap()
    }
}
