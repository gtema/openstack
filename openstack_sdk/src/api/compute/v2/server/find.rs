use derive_builder::Builder;
use serde::de::DeserializeOwned;
use tracing::{debug, info, span, trace, Level};

use itertools::Itertools;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query};

use crate::api::compute::v2::{
    server::get::Server as GetServer, servers::detail::get::Servers as ListServers,
};

/// Find for flavor by NameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Server<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
}

impl<'a> Server<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ServerBuilder<'a> {
        ServerBuilder::default()
    }
}

impl<'a> Findable for Server<'a> {
    type G = GetServer<'a>;
    type L = ListServers<'a>;
    fn get_ep(&self) -> GetServer<'a> {
        GetServer::builder().id(self.id.clone()).build().unwrap()
    }
    fn list_ep(&self) -> ListServers<'a> {
        ListServers::builder()
            .name(self.id.clone())
            .build()
            .unwrap()
    }
}
