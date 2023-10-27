//! Network find
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
    network::get::Network as GetNetwork, networks::get::Networks as ListNetworks,
};

/// Find for network by NameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Network<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
}

impl<'a> Network<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> NetworkBuilder<'a> {
        NetworkBuilder::default()
    }
}

impl<'a> Findable for Network<'a> {
    type G = GetNetwork<'a>;
    type L = ListNetworks<'a>;
    fn get_ep(&self) -> GetNetwork<'a> {
        GetNetwork::builder().id(self.id.clone()).build().unwrap()
    }
    fn list_ep(&self) -> ListNetworks<'a> {
        ListNetworks::builder()
            .name(self.id.clone())
            .build()
            .unwrap()
    }
}
