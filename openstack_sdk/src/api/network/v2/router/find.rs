//! Router find
use derive_builder::Builder;
use serde::de::DeserializeOwned;
use tracing::{debug, info, span, trace, Level};

use itertools::Itertools;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query};

use crate::api::network::v2::{
    router::get::Router as GetRouter, routers::get::Routers as ListRouters,
};

/// Find for router by NameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Router<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
}

impl<'a> Router<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RouterBuilder<'a> {
        RouterBuilder::default()
    }
}

impl<'a> Findable for Router<'a> {
    type G = GetRouter<'a>;
    type L = ListRouters<'a>;
    fn get_ep(&self) -> GetRouter<'a> {
        GetRouter::builder().id(self.id.clone()).build().unwrap()
    }
    fn list_ep(&self) -> ListRouters<'a> {
        ListRouters::builder()
            .name(self.id.clone())
            .build()
            .unwrap()
    }
}
