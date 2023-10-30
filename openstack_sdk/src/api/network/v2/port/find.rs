//! Port find
use derive_builder::Builder;
use serde::de::DeserializeOwned;
use tracing::{debug, info, span, trace, Level};

use itertools::Itertools;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query};

use crate::api::network::v2::{port::get::Port as GetPort, ports::get::Ports as ListPorts};

/// Find for port by NameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Port<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
}

impl<'a> Port<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PortBuilder<'a> {
        PortBuilder::default()
    }
}

impl<'a> Findable for Port<'a> {
    type G = GetPort<'a>;
    type L = ListPorts<'a>;
    fn get_ep(&self) -> GetPort<'a> {
        GetPort::builder().id(self.id.clone()).build().unwrap()
    }
    fn list_ep(&self) -> ListPorts<'a> {
        ListPorts::builder().name(self.id.clone()).build().unwrap()
    }
}
